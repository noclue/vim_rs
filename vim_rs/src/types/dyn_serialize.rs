use serde::Serialize;
use super::struct_enum::StructType;
use super::vim_object_trait::VimObjectTrait;
use super::structs::*;

/// Serialize a polymorphic VimObjectTrait into serde::Serializer
pub fn serialize_polymorphic<S>(p: &dyn VimObjectTrait, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data_type = p.data_type();
    match data_type {
        StructType::ManagedObjectReference => ManagedObjectReference::serialize(
            p.as_any_ref().downcast_ref::<ManagedObjectReference>().unwrap(),
            serializer,
        ),
        StructType::DataObject => DataObject::serialize(
            p.as_any_ref().downcast_ref::<DataObject>().unwrap(),
            serializer,
        ),
        StructType::AboutInfo => AboutInfo::serialize(
            p.as_any_ref().downcast_ref::<AboutInfo>().unwrap(),
            serializer,
        ),
        StructType::AuthorizationDescription => AuthorizationDescription::serialize(
            p.as_any_ref().downcast_ref::<AuthorizationDescription>().unwrap(),
            serializer,
        ),
        StructType::EntityPrivilege => EntityPrivilege::serialize(
            p.as_any_ref().downcast_ref::<EntityPrivilege>().unwrap(),
            serializer,
        ),
        StructType::Permission => Permission::serialize(
            p.as_any_ref().downcast_ref::<Permission>().unwrap(),
            serializer,
        ),
        StructType::AuthorizationPrivilege => AuthorizationPrivilege::serialize(
            p.as_any_ref().downcast_ref::<AuthorizationPrivilege>().unwrap(),
            serializer,
        ),
        StructType::PrivilegeAvailability => PrivilegeAvailability::serialize(
            p.as_any_ref().downcast_ref::<PrivilegeAvailability>().unwrap(),
            serializer,
        ),
        StructType::AuthorizationRole => AuthorizationRole::serialize(
            p.as_any_ref().downcast_ref::<AuthorizationRole>().unwrap(),
            serializer,
        ),
        StructType::UserPrivilegeResult => UserPrivilegeResult::serialize(
            p.as_any_ref().downcast_ref::<UserPrivilegeResult>().unwrap(),
            serializer,
        ),
        StructType::BatchResult => BatchResult::serialize(
            p.as_any_ref().downcast_ref::<BatchResult>().unwrap(),
            serializer,
        ),
        StructType::Capability => Capability::serialize(
            p.as_any_ref().downcast_ref::<Capability>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceClusterConfigResult => ClusterComputeResourceClusterConfigResult::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceClusterConfigResult>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceDvsSetting => ClusterComputeResourceDvsSetting::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceDvsSetting>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping => ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceDvsProfile => ClusterComputeResourceDvsProfile::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceDvsProfile>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping => ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHciConfigInfo => ClusterComputeResourceHciConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHciConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHciConfigSpec => ClusterComputeResourceHciConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHciConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHostConfigurationInput => ClusterComputeResourceHostConfigurationInput::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationInput>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHostConfigurationProfile => ClusterComputeResourceHostConfigurationProfile::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationProfile>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHostVmkNicInfo => ClusterComputeResourceHostVmkNicInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHostVmkNicInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceVcProfile => ClusterComputeResourceVcProfile::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceVcProfile>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceValidationResultBase => ClusterComputeResourceValidationResultBase::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceValidationResultBase>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceDvsConfigurationValidation => ClusterComputeResourceDvsConfigurationValidation::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceDvsConfigurationValidation>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceHostConfigurationValidation => ClusterComputeResourceHostConfigurationValidation::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceHostConfigurationValidation>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceVcsSlots => ClusterComputeResourceVcsSlots::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceVcsSlots>().unwrap(),
            serializer,
        ),
        StructType::ComputeResourceConfigInfo => ComputeResourceConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ComputeResourceConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterConfigInfoEx => ClusterConfigInfoEx::serialize(
            p.as_any_ref().downcast_ref::<ClusterConfigInfoEx>().unwrap(),
            serializer,
        ),
        StructType::ComputeResourceConfigSpec => ComputeResourceConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ComputeResourceConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterConfigSpecEx => ClusterConfigSpecEx::serialize(
            p.as_any_ref().downcast_ref::<ClusterConfigSpecEx>().unwrap(),
            serializer,
        ),
        StructType::ComputeResourceHostSpbmLicenseInfo => ComputeResourceHostSpbmLicenseInfo::serialize(
            p.as_any_ref().downcast_ref::<ComputeResourceHostSpbmLicenseInfo>().unwrap(),
            serializer,
        ),
        StructType::ComputeResourceSummary => ComputeResourceSummary::serialize(
            p.as_any_ref().downcast_ref::<ComputeResourceSummary>().unwrap(),
            serializer,
        ),
        StructType::ClusterComputeResourceSummary => ClusterComputeResourceSummary::serialize(
            p.as_any_ref().downcast_ref::<ClusterComputeResourceSummary>().unwrap(),
            serializer,
        ),
        StructType::CustomFieldDef => CustomFieldDef::serialize(
            p.as_any_ref().downcast_ref::<CustomFieldDef>().unwrap(),
            serializer,
        ),
        StructType::CustomFieldValue => CustomFieldValue::serialize(
            p.as_any_ref().downcast_ref::<CustomFieldValue>().unwrap(),
            serializer,
        ),
        StructType::CustomFieldStringValue => CustomFieldStringValue::serialize(
            p.as_any_ref().downcast_ref::<CustomFieldStringValue>().unwrap(),
            serializer,
        ),
        StructType::CustomizationSpecInfo => CustomizationSpecInfo::serialize(
            p.as_any_ref().downcast_ref::<CustomizationSpecInfo>().unwrap(),
            serializer,
        ),
        StructType::CustomizationSpecItem => CustomizationSpecItem::serialize(
            p.as_any_ref().downcast_ref::<CustomizationSpecItem>().unwrap(),
            serializer,
        ),
        StructType::DatacenterBasicConnectInfo => DatacenterBasicConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<DatacenterBasicConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::DatacenterConfigInfo => DatacenterConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<DatacenterConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::DatacenterConfigSpec => DatacenterConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DatacenterConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DatastoreCapability => DatastoreCapability::serialize(
            p.as_any_ref().downcast_ref::<DatastoreCapability>().unwrap(),
            serializer,
        ),
        StructType::DatastoreHostMount => DatastoreHostMount::serialize(
            p.as_any_ref().downcast_ref::<DatastoreHostMount>().unwrap(),
            serializer,
        ),
        StructType::DatastoreInfo => DatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<DatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::LocalDatastoreInfo => LocalDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<LocalDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::NasDatastoreInfo => NasDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<NasDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::PMemDatastoreInfo => PMemDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<PMemDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreInfo => VmfsDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanDatastoreInfo => VsanDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::VvolDatastoreInfo => VvolDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<VvolDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::DatastoreMountPathDatastorePair => DatastoreMountPathDatastorePair::serialize(
            p.as_any_ref().downcast_ref::<DatastoreMountPathDatastorePair>().unwrap(),
            serializer,
        ),
        StructType::DatastoreSummary => DatastoreSummary::serialize(
            p.as_any_ref().downcast_ref::<DatastoreSummary>().unwrap(),
            serializer,
        ),
        StructType::DatastoreVVolContainerFailoverPair => DatastoreVVolContainerFailoverPair::serialize(
            p.as_any_ref().downcast_ref::<DatastoreVVolContainerFailoverPair>().unwrap(),
            serializer,
        ),
        StructType::DatastoreNamespaceManagerDirectoryInfo => DatastoreNamespaceManagerDirectoryInfo::serialize(
            p.as_any_ref().downcast_ref::<DatastoreNamespaceManagerDirectoryInfo>().unwrap(),
            serializer,
        ),
        StructType::Description => Description::serialize(
            p.as_any_ref().downcast_ref::<Description>().unwrap(),
            serializer,
        ),
        StructType::ElementDescription => ElementDescription::serialize(
            p.as_any_ref().downcast_ref::<ElementDescription>().unwrap(),
            serializer,
        ),
        StructType::EvcMode => EvcMode::serialize(
            p.as_any_ref().downcast_ref::<EvcMode>().unwrap(),
            serializer,
        ),
        StructType::ExtendedElementDescription => ExtendedElementDescription::serialize(
            p.as_any_ref().downcast_ref::<ExtendedElementDescription>().unwrap(),
            serializer,
        ),
        StructType::FeatureEvcMode => FeatureEvcMode::serialize(
            p.as_any_ref().downcast_ref::<FeatureEvcMode>().unwrap(),
            serializer,
        ),
        StructType::OptionDef => OptionDef::serialize(
            p.as_any_ref().downcast_ref::<OptionDef>().unwrap(),
            serializer,
        ),
        StructType::ExtendedDescription => ExtendedDescription::serialize(
            p.as_any_ref().downcast_ref::<ExtendedDescription>().unwrap(),
            serializer,
        ),
        StructType::MethodDescription => MethodDescription::serialize(
            p.as_any_ref().downcast_ref::<MethodDescription>().unwrap(),
            serializer,
        ),
        StructType::TypeDescription => TypeDescription::serialize(
            p.as_any_ref().downcast_ref::<TypeDescription>().unwrap(),
            serializer,
        ),
        StructType::ScheduledTaskDetail => ScheduledTaskDetail::serialize(
            p.as_any_ref().downcast_ref::<ScheduledTaskDetail>().unwrap(),
            serializer,
        ),
        StructType::DesiredSoftwareSpec => DesiredSoftwareSpec::serialize(
            p.as_any_ref().downcast_ref::<DesiredSoftwareSpec>().unwrap(),
            serializer,
        ),
        StructType::DesiredSoftwareSpecBaseImageSpec => DesiredSoftwareSpecBaseImageSpec::serialize(
            p.as_any_ref().downcast_ref::<DesiredSoftwareSpecBaseImageSpec>().unwrap(),
            serializer,
        ),
        StructType::DesiredSoftwareSpecComponentSpec => DesiredSoftwareSpecComponentSpec::serialize(
            p.as_any_ref().downcast_ref::<DesiredSoftwareSpecComponentSpec>().unwrap(),
            serializer,
        ),
        StructType::DesiredSoftwareSpecVendorAddOnSpec => DesiredSoftwareSpecVendorAddOnSpec::serialize(
            p.as_any_ref().downcast_ref::<DesiredSoftwareSpecVendorAddOnSpec>().unwrap(),
            serializer,
        ),
        StructType::DiagnosticManagerAuditRecordResult => DiagnosticManagerAuditRecordResult::serialize(
            p.as_any_ref().downcast_ref::<DiagnosticManagerAuditRecordResult>().unwrap(),
            serializer,
        ),
        StructType::DiagnosticManagerBundleInfo => DiagnosticManagerBundleInfo::serialize(
            p.as_any_ref().downcast_ref::<DiagnosticManagerBundleInfo>().unwrap(),
            serializer,
        ),
        StructType::DiagnosticManagerLogDescriptor => DiagnosticManagerLogDescriptor::serialize(
            p.as_any_ref().downcast_ref::<DiagnosticManagerLogDescriptor>().unwrap(),
            serializer,
        ),
        StructType::DiagnosticManagerLogHeader => DiagnosticManagerLogHeader::serialize(
            p.as_any_ref().downcast_ref::<DiagnosticManagerLogHeader>().unwrap(),
            serializer,
        ),
        StructType::DvsBackupRestoreCapability => DvsBackupRestoreCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsBackupRestoreCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsCapability => DvsCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsConfigInfo => DvsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsConfigInfo => VMwareDvsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsConfigSpec => DvsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsConfigSpec => VMwareDvsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsContactInfo => DvsContactInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsContactInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsCreateSpec => DvsCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsFeatureCapability => DvsFeatureCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsFeatureCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsFeatureCapability => VMwareDvsFeatureCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsFeatureCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsHealthCheckConfig => DvsHealthCheckConfig::serialize(
            p.as_any_ref().downcast_ref::<DvsHealthCheckConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsHealthCheckConfig => VMwareDvsHealthCheckConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsHealthCheckConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsTeamingHealthCheckConfig => VMwareDvsTeamingHealthCheckConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsVlanMtuHealthCheckConfig => VMwareDvsVlanMtuHealthCheckConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsVlanMtuHealthCheckConfig>().unwrap(),
            serializer,
        ),
        StructType::DvsHealthCheckCapability => DvsHealthCheckCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsHealthCheckCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsHealthCheckCapability => VMwareDvsHealthCheckCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsHealthCheckCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsHostInfrastructureTrafficResource => DvsHostInfrastructureTrafficResource::serialize(
            p.as_any_ref().downcast_ref::<DvsHostInfrastructureTrafficResource>().unwrap(),
            serializer,
        ),
        StructType::DvsHostInfrastructureTrafficResourceAllocation => DvsHostInfrastructureTrafficResourceAllocation::serialize(
            p.as_any_ref().downcast_ref::<DvsHostInfrastructureTrafficResourceAllocation>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkResourceManagementCapability => DvsNetworkResourceManagementCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkResourceManagementCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsResourceRuntimeInfo => DvsResourceRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsResourceRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsRollbackCapability => DvsRollbackCapability::serialize(
            p.as_any_ref().downcast_ref::<DvsRollbackCapability>().unwrap(),
            serializer,
        ),
        StructType::DvsRuntimeInfo => DvsRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsSummary => DvsSummary::serialize(
            p.as_any_ref().downcast_ref::<DvsSummary>().unwrap(),
            serializer,
        ),
        StructType::DvsPolicy => DvsPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsUplinkPortPolicy => DvsUplinkPortPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsUplinkPortPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsNameArrayUplinkPortPolicy => DvsNameArrayUplinkPortPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsNameArrayUplinkPortPolicy>().unwrap(),
            serializer,
        ),
        StructType::EnumDescription => EnumDescription::serialize(
            p.as_any_ref().downcast_ref::<EnumDescription>().unwrap(),
            serializer,
        ),
        StructType::EnvironmentBrowserConfigOptionQuerySpec => EnvironmentBrowserConfigOptionQuerySpec::serialize(
            p.as_any_ref().downcast_ref::<EnvironmentBrowserConfigOptionQuerySpec>().unwrap(),
            serializer,
        ),
        StructType::Extension => Extension::serialize(
            p.as_any_ref().downcast_ref::<Extension>().unwrap(),
            serializer,
        ),
        StructType::ExtensionClientInfo => ExtensionClientInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionClientInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionEventTypeInfo => ExtensionEventTypeInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionEventTypeInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionFaultTypeInfo => ExtensionFaultTypeInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionFaultTypeInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionHealthInfo => ExtensionHealthInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionHealthInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionOvfConsumerInfo => ExtensionOvfConsumerInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionOvfConsumerInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionPrivilegeInfo => ExtensionPrivilegeInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionPrivilegeInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionResourceInfo => ExtensionResourceInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionResourceInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionServerInfo => ExtensionServerInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionServerInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionTaskTypeInfo => ExtensionTaskTypeInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtensionTaskTypeInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtensionManagerIpAllocationUsage => ExtensionManagerIpAllocationUsage::serialize(
            p.as_any_ref().downcast_ref::<ExtensionManagerIpAllocationUsage>().unwrap(),
            serializer,
        ),
        StructType::FaultsByHost => FaultsByHost::serialize(
            p.as_any_ref().downcast_ref::<FaultsByHost>().unwrap(),
            serializer,
        ),
        StructType::FaultsByVm => FaultsByVm::serialize(
            p.as_any_ref().downcast_ref::<FaultsByVm>().unwrap(),
            serializer,
        ),
        StructType::FileLockInfo => FileLockInfo::serialize(
            p.as_any_ref().downcast_ref::<FileLockInfo>().unwrap(),
            serializer,
        ),
        StructType::FileLockInfoResult => FileLockInfoResult::serialize(
            p.as_any_ref().downcast_ref::<FileLockInfoResult>().unwrap(),
            serializer,
        ),
        StructType::FolderBatchAddHostsToClusterResult => FolderBatchAddHostsToClusterResult::serialize(
            p.as_any_ref().downcast_ref::<FolderBatchAddHostsToClusterResult>().unwrap(),
            serializer,
        ),
        StructType::FolderBatchAddStandaloneHostsResult => FolderBatchAddStandaloneHostsResult::serialize(
            p.as_any_ref().downcast_ref::<FolderBatchAddStandaloneHostsResult>().unwrap(),
            serializer,
        ),
        StructType::FolderFailedHostResult => FolderFailedHostResult::serialize(
            p.as_any_ref().downcast_ref::<FolderFailedHostResult>().unwrap(),
            serializer,
        ),
        StructType::FolderNewHostSpec => FolderNewHostSpec::serialize(
            p.as_any_ref().downcast_ref::<FolderNewHostSpec>().unwrap(),
            serializer,
        ),
        StructType::HbrManagerReplicationVmInfo => HbrManagerReplicationVmInfo::serialize(
            p.as_any_ref().downcast_ref::<HbrManagerReplicationVmInfo>().unwrap(),
            serializer,
        ),
        StructType::ReplicationVmProgressInfo => ReplicationVmProgressInfo::serialize(
            p.as_any_ref().downcast_ref::<ReplicationVmProgressInfo>().unwrap(),
            serializer,
        ),
        StructType::HbrManagerVmReplicationCapability => HbrManagerVmReplicationCapability::serialize(
            p.as_any_ref().downcast_ref::<HbrManagerVmReplicationCapability>().unwrap(),
            serializer,
        ),
        StructType::HealthUpdate => HealthUpdate::serialize(
            p.as_any_ref().downcast_ref::<HealthUpdate>().unwrap(),
            serializer,
        ),
        StructType::HealthUpdateInfo => HealthUpdateInfo::serialize(
            p.as_any_ref().downcast_ref::<HealthUpdateInfo>().unwrap(),
            serializer,
        ),
        StructType::PerfInterval => PerfInterval::serialize(
            p.as_any_ref().downcast_ref::<PerfInterval>().unwrap(),
            serializer,
        ),
        StructType::HostServiceTicket => HostServiceTicket::serialize(
            p.as_any_ref().downcast_ref::<HostServiceTicket>().unwrap(),
            serializer,
        ),
        StructType::HostSystemComplianceCheckState => HostSystemComplianceCheckState::serialize(
            p.as_any_ref().downcast_ref::<HostSystemComplianceCheckState>().unwrap(),
            serializer,
        ),
        StructType::HostSystemReconnectSpec => HostSystemReconnectSpec::serialize(
            p.as_any_ref().downcast_ref::<HostSystemReconnectSpec>().unwrap(),
            serializer,
        ),
        StructType::HostSystemRemediationState => HostSystemRemediationState::serialize(
            p.as_any_ref().downcast_ref::<HostSystemRemediationState>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseCapabilities => HttpNfcLeaseCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseDatastoreLeaseInfo => HttpNfcLeaseDatastoreLeaseInfo::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseDatastoreLeaseInfo>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseDeviceUrl => HttpNfcLeaseDeviceUrl::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseDeviceUrl>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseHostInfo => HttpNfcLeaseHostInfo::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseHostInfo>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseInfo => HttpNfcLeaseInfo::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseInfo>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseManifestEntry => HttpNfcLeaseManifestEntry::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseManifestEntry>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseProbeResult => HttpNfcLeaseProbeResult::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseProbeResult>().unwrap(),
            serializer,
        ),
        StructType::HttpNfcLeaseSourceFile => HttpNfcLeaseSourceFile::serialize(
            p.as_any_ref().downcast_ref::<HttpNfcLeaseSourceFile>().unwrap(),
            serializer,
        ),
        StructType::ImportSpec => ImportSpec::serialize(
            p.as_any_ref().downcast_ref::<ImportSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualAppImportSpec => VirtualAppImportSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualAppImportSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineImportSpec => VirtualMachineImportSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineImportSpec>().unwrap(),
            serializer,
        ),
        StructType::InheritablePolicy => InheritablePolicy::serialize(
            p.as_any_ref().downcast_ref::<InheritablePolicy>().unwrap(),
            serializer,
        ),
        StructType::BoolPolicy => BoolPolicy::serialize(
            p.as_any_ref().downcast_ref::<BoolPolicy>().unwrap(),
            serializer,
        ),
        StructType::IntPolicy => IntPolicy::serialize(
            p.as_any_ref().downcast_ref::<IntPolicy>().unwrap(),
            serializer,
        ),
        StructType::LongPolicy => LongPolicy::serialize(
            p.as_any_ref().downcast_ref::<LongPolicy>().unwrap(),
            serializer,
        ),
        StructType::StringPolicy => StringPolicy::serialize(
            p.as_any_ref().downcast_ref::<StringPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsFilterConfig => DvsFilterConfig::serialize(
            p.as_any_ref().downcast_ref::<DvsFilterConfig>().unwrap(),
            serializer,
        ),
        StructType::DvsFilterConfigSpec => DvsFilterConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsFilterConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsTrafficFilterConfig => DvsTrafficFilterConfig::serialize(
            p.as_any_ref().downcast_ref::<DvsTrafficFilterConfig>().unwrap(),
            serializer,
        ),
        StructType::DvsTrafficFilterConfigSpec => DvsTrafficFilterConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsTrafficFilterConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsFilterPolicy => DvsFilterPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsFilterPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsTrafficShapingPolicy => DvsTrafficShapingPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsTrafficShapingPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsVendorSpecificConfig => DvsVendorSpecificConfig::serialize(
            p.as_any_ref().downcast_ref::<DvsVendorSpecificConfig>().unwrap(),
            serializer,
        ),
        StructType::DvsFailureCriteria => DvsFailureCriteria::serialize(
            p.as_any_ref().downcast_ref::<DvsFailureCriteria>().unwrap(),
            serializer,
        ),
        StructType::DvsMacLearningPolicy => DvsMacLearningPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsMacLearningPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsMacManagementPolicy => DvsMacManagementPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsMacManagementPolicy>().unwrap(),
            serializer,
        ),
        StructType::DvsSecurityPolicy => DvsSecurityPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvsSecurityPolicy>().unwrap(),
            serializer,
        ),
        StructType::VMwareUplinkLacpPolicy => VMwareUplinkLacpPolicy::serialize(
            p.as_any_ref().downcast_ref::<VMwareUplinkLacpPolicy>().unwrap(),
            serializer,
        ),
        StructType::VMwareUplinkPortOrderPolicy => VMwareUplinkPortOrderPolicy::serialize(
            p.as_any_ref().downcast_ref::<VMwareUplinkPortOrderPolicy>().unwrap(),
            serializer,
        ),
        StructType::VmwareUplinkPortTeamingPolicy => VmwareUplinkPortTeamingPolicy::serialize(
            p.as_any_ref().downcast_ref::<VmwareUplinkPortTeamingPolicy>().unwrap(),
            serializer,
        ),
        StructType::VmwareDistributedVirtualSwitchVlanSpec => VmwareDistributedVirtualSwitchVlanSpec::serialize(
            p.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanSpec>().unwrap(),
            serializer,
        ),
        StructType::VmwareDistributedVirtualSwitchPvlanSpec => VmwareDistributedVirtualSwitchPvlanSpec::serialize(
            p.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchPvlanSpec>().unwrap(),
            serializer,
        ),
        StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec => VmwareDistributedVirtualSwitchTrunkVlanSpec::serialize(
            p.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchTrunkVlanSpec>().unwrap(),
            serializer,
        ),
        StructType::VmwareDistributedVirtualSwitchVlanIdSpec => VmwareDistributedVirtualSwitchVlanIdSpec::serialize(
            p.as_any_ref().downcast_ref::<VmwareDistributedVirtualSwitchVlanIdSpec>().unwrap(),
            serializer,
        ),
        StructType::IoFilterInfo => IoFilterInfo::serialize(
            p.as_any_ref().downcast_ref::<IoFilterInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterIoFilterInfo => ClusterIoFilterInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterIoFilterInfo>().unwrap(),
            serializer,
        ),
        StructType::HostIoFilterInfo => HostIoFilterInfo::serialize(
            p.as_any_ref().downcast_ref::<HostIoFilterInfo>().unwrap(),
            serializer,
        ),
        StructType::IoFilterQueryIssueResult => IoFilterQueryIssueResult::serialize(
            p.as_any_ref().downcast_ref::<IoFilterQueryIssueResult>().unwrap(),
            serializer,
        ),
        StructType::IoFilterHostIssue => IoFilterHostIssue::serialize(
            p.as_any_ref().downcast_ref::<IoFilterHostIssue>().unwrap(),
            serializer,
        ),
        StructType::IpPoolManagerIpAllocation => IpPoolManagerIpAllocation::serialize(
            p.as_any_ref().downcast_ref::<IpPoolManagerIpAllocation>().unwrap(),
            serializer,
        ),
        StructType::KeyValue => KeyValue::serialize(
            p.as_any_ref().downcast_ref::<KeyValue>().unwrap(),
            serializer,
        ),
        StructType::LatencySensitivity => LatencySensitivity::serialize(
            p.as_any_ref().downcast_ref::<LatencySensitivity>().unwrap(),
            serializer,
        ),
        StructType::LicenseAssignmentManagerLicenseAssignment => LicenseAssignmentManagerLicenseAssignment::serialize(
            p.as_any_ref().downcast_ref::<LicenseAssignmentManagerLicenseAssignment>().unwrap(),
            serializer,
        ),
        StructType::LicenseAvailabilityInfo => LicenseAvailabilityInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseAvailabilityInfo>().unwrap(),
            serializer,
        ),
        StructType::LicenseDiagnostics => LicenseDiagnostics::serialize(
            p.as_any_ref().downcast_ref::<LicenseDiagnostics>().unwrap(),
            serializer,
        ),
        StructType::LicenseManagerEvaluationInfo => LicenseManagerEvaluationInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseManagerEvaluationInfo>().unwrap(),
            serializer,
        ),
        StructType::LicenseFeatureInfo => LicenseFeatureInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseFeatureInfo>().unwrap(),
            serializer,
        ),
        StructType::HostLicensableResourceInfo => HostLicensableResourceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostLicensableResourceInfo>().unwrap(),
            serializer,
        ),
        StructType::LicenseManagerLicenseInfo => LicenseManagerLicenseInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseManagerLicenseInfo>().unwrap(),
            serializer,
        ),
        StructType::LicenseSource => LicenseSource::serialize(
            p.as_any_ref().downcast_ref::<LicenseSource>().unwrap(),
            serializer,
        ),
        StructType::EvaluationLicenseSource => EvaluationLicenseSource::serialize(
            p.as_any_ref().downcast_ref::<EvaluationLicenseSource>().unwrap(),
            serializer,
        ),
        StructType::LicenseServerSource => LicenseServerSource::serialize(
            p.as_any_ref().downcast_ref::<LicenseServerSource>().unwrap(),
            serializer,
        ),
        StructType::LocalLicenseSource => LocalLicenseSource::serialize(
            p.as_any_ref().downcast_ref::<LocalLicenseSource>().unwrap(),
            serializer,
        ),
        StructType::LicenseUsageInfo => LicenseUsageInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseUsageInfo>().unwrap(),
            serializer,
        ),
        StructType::LicenseReservationInfo => LicenseReservationInfo::serialize(
            p.as_any_ref().downcast_ref::<LicenseReservationInfo>().unwrap(),
            serializer,
        ),
        StructType::LocalizationManagerMessageCatalog => LocalizationManagerMessageCatalog::serialize(
            p.as_any_ref().downcast_ref::<LocalizationManagerMessageCatalog>().unwrap(),
            serializer,
        ),
        StructType::NegatableExpression => NegatableExpression::serialize(
            p.as_any_ref().downcast_ref::<NegatableExpression>().unwrap(),
            serializer,
        ),
        StructType::IntExpression => IntExpression::serialize(
            p.as_any_ref().downcast_ref::<IntExpression>().unwrap(),
            serializer,
        ),
        StructType::IpAddress => IpAddress::serialize(
            p.as_any_ref().downcast_ref::<IpAddress>().unwrap(),
            serializer,
        ),
        StructType::IpRange => IpRange::serialize(
            p.as_any_ref().downcast_ref::<IpRange>().unwrap(),
            serializer,
        ),
        StructType::SingleIp => SingleIp::serialize(
            p.as_any_ref().downcast_ref::<SingleIp>().unwrap(),
            serializer,
        ),
        StructType::MacAddress => MacAddress::serialize(
            p.as_any_ref().downcast_ref::<MacAddress>().unwrap(),
            serializer,
        ),
        StructType::MacRange => MacRange::serialize(
            p.as_any_ref().downcast_ref::<MacRange>().unwrap(),
            serializer,
        ),
        StructType::SingleMac => SingleMac::serialize(
            p.as_any_ref().downcast_ref::<SingleMac>().unwrap(),
            serializer,
        ),
        StructType::StringExpression => StringExpression::serialize(
            p.as_any_ref().downcast_ref::<StringExpression>().unwrap(),
            serializer,
        ),
        StructType::DvsIpPort => DvsIpPort::serialize(
            p.as_any_ref().downcast_ref::<DvsIpPort>().unwrap(),
            serializer,
        ),
        StructType::DvsIpPortRange => DvsIpPortRange::serialize(
            p.as_any_ref().downcast_ref::<DvsIpPortRange>().unwrap(),
            serializer,
        ),
        StructType::DvsSingleIpPort => DvsSingleIpPort::serialize(
            p.as_any_ref().downcast_ref::<DvsSingleIpPort>().unwrap(),
            serializer,
        ),
        StructType::NetworkSummary => NetworkSummary::serialize(
            p.as_any_ref().downcast_ref::<NetworkSummary>().unwrap(),
            serializer,
        ),
        StructType::OpaqueNetworkSummary => OpaqueNetworkSummary::serialize(
            p.as_any_ref().downcast_ref::<OpaqueNetworkSummary>().unwrap(),
            serializer,
        ),
        StructType::NumericRange => NumericRange::serialize(
            p.as_any_ref().downcast_ref::<NumericRange>().unwrap(),
            serializer,
        ),
        StructType::OpaqueNetworkCapability => OpaqueNetworkCapability::serialize(
            p.as_any_ref().downcast_ref::<OpaqueNetworkCapability>().unwrap(),
            serializer,
        ),
        StructType::OvfConsumerOstNode => OvfConsumerOstNode::serialize(
            p.as_any_ref().downcast_ref::<OvfConsumerOstNode>().unwrap(),
            serializer,
        ),
        StructType::OvfConsumerOvfSection => OvfConsumerOvfSection::serialize(
            p.as_any_ref().downcast_ref::<OvfConsumerOvfSection>().unwrap(),
            serializer,
        ),
        StructType::OvfManagerCommonParams => OvfManagerCommonParams::serialize(
            p.as_any_ref().downcast_ref::<OvfManagerCommonParams>().unwrap(),
            serializer,
        ),
        StructType::OvfCreateImportSpecParams => OvfCreateImportSpecParams::serialize(
            p.as_any_ref().downcast_ref::<OvfCreateImportSpecParams>().unwrap(),
            serializer,
        ),
        StructType::OvfParseDescriptorParams => OvfParseDescriptorParams::serialize(
            p.as_any_ref().downcast_ref::<OvfParseDescriptorParams>().unwrap(),
            serializer,
        ),
        StructType::OvfValidateHostParams => OvfValidateHostParams::serialize(
            p.as_any_ref().downcast_ref::<OvfValidateHostParams>().unwrap(),
            serializer,
        ),
        StructType::OvfCreateDescriptorParams => OvfCreateDescriptorParams::serialize(
            p.as_any_ref().downcast_ref::<OvfCreateDescriptorParams>().unwrap(),
            serializer,
        ),
        StructType::OvfCreateDescriptorResult => OvfCreateDescriptorResult::serialize(
            p.as_any_ref().downcast_ref::<OvfCreateDescriptorResult>().unwrap(),
            serializer,
        ),
        StructType::OvfCreateImportSpecResult => OvfCreateImportSpecResult::serialize(
            p.as_any_ref().downcast_ref::<OvfCreateImportSpecResult>().unwrap(),
            serializer,
        ),
        StructType::OvfDeploymentOption => OvfDeploymentOption::serialize(
            p.as_any_ref().downcast_ref::<OvfDeploymentOption>().unwrap(),
            serializer,
        ),
        StructType::OvfFileItem => OvfFileItem::serialize(
            p.as_any_ref().downcast_ref::<OvfFileItem>().unwrap(),
            serializer,
        ),
        StructType::OvfNetworkInfo => OvfNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<OvfNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::OvfNetworkMapping => OvfNetworkMapping::serialize(
            p.as_any_ref().downcast_ref::<OvfNetworkMapping>().unwrap(),
            serializer,
        ),
        StructType::OvfFile => OvfFile::serialize(
            p.as_any_ref().downcast_ref::<OvfFile>().unwrap(),
            serializer,
        ),
        StructType::OvfOptionInfo => OvfOptionInfo::serialize(
            p.as_any_ref().downcast_ref::<OvfOptionInfo>().unwrap(),
            serializer,
        ),
        StructType::OvfParseDescriptorResult => OvfParseDescriptorResult::serialize(
            p.as_any_ref().downcast_ref::<OvfParseDescriptorResult>().unwrap(),
            serializer,
        ),
        StructType::OvfResourceMap => OvfResourceMap::serialize(
            p.as_any_ref().downcast_ref::<OvfResourceMap>().unwrap(),
            serializer,
        ),
        StructType::OvfValidateHostResult => OvfValidateHostResult::serialize(
            p.as_any_ref().downcast_ref::<OvfValidateHostResult>().unwrap(),
            serializer,
        ),
        StructType::PasswordField => PasswordField::serialize(
            p.as_any_ref().downcast_ref::<PasswordField>().unwrap(),
            serializer,
        ),
        StructType::PerformanceDescription => PerformanceDescription::serialize(
            p.as_any_ref().downcast_ref::<PerformanceDescription>().unwrap(),
            serializer,
        ),
        StructType::PerfCompositeMetric => PerfCompositeMetric::serialize(
            p.as_any_ref().downcast_ref::<PerfCompositeMetric>().unwrap(),
            serializer,
        ),
        StructType::PerfCounterInfo => PerfCounterInfo::serialize(
            p.as_any_ref().downcast_ref::<PerfCounterInfo>().unwrap(),
            serializer,
        ),
        StructType::PerformanceManagerCounterLevelMapping => PerformanceManagerCounterLevelMapping::serialize(
            p.as_any_ref().downcast_ref::<PerformanceManagerCounterLevelMapping>().unwrap(),
            serializer,
        ),
        StructType::PerfEntityMetricBase => PerfEntityMetricBase::serialize(
            p.as_any_ref().downcast_ref::<PerfEntityMetricBase>().unwrap(),
            serializer,
        ),
        StructType::PerfEntityMetric => PerfEntityMetric::serialize(
            p.as_any_ref().downcast_ref::<PerfEntityMetric>().unwrap(),
            serializer,
        ),
        StructType::PerfEntityMetricCsv => PerfEntityMetricCsv::serialize(
            p.as_any_ref().downcast_ref::<PerfEntityMetricCsv>().unwrap(),
            serializer,
        ),
        StructType::PerfMetricId => PerfMetricId::serialize(
            p.as_any_ref().downcast_ref::<PerfMetricId>().unwrap(),
            serializer,
        ),
        StructType::PerfMetricSeries => PerfMetricSeries::serialize(
            p.as_any_ref().downcast_ref::<PerfMetricSeries>().unwrap(),
            serializer,
        ),
        StructType::PerfMetricIntSeries => PerfMetricIntSeries::serialize(
            p.as_any_ref().downcast_ref::<PerfMetricIntSeries>().unwrap(),
            serializer,
        ),
        StructType::PerfMetricSeriesCsv => PerfMetricSeriesCsv::serialize(
            p.as_any_ref().downcast_ref::<PerfMetricSeriesCsv>().unwrap(),
            serializer,
        ),
        StructType::PerfProviderSummary => PerfProviderSummary::serialize(
            p.as_any_ref().downcast_ref::<PerfProviderSummary>().unwrap(),
            serializer,
        ),
        StructType::PerfQuerySpec => PerfQuerySpec::serialize(
            p.as_any_ref().downcast_ref::<PerfQuerySpec>().unwrap(),
            serializer,
        ),
        StructType::PerfSampleInfo => PerfSampleInfo::serialize(
            p.as_any_ref().downcast_ref::<PerfSampleInfo>().unwrap(),
            serializer,
        ),
        StructType::PrivilegePolicyDef => PrivilegePolicyDef::serialize(
            p.as_any_ref().downcast_ref::<PrivilegePolicyDef>().unwrap(),
            serializer,
        ),
        StructType::ResourceAllocationInfo => ResourceAllocationInfo::serialize(
            p.as_any_ref().downcast_ref::<ResourceAllocationInfo>().unwrap(),
            serializer,
        ),
        StructType::ResourceAllocationOption => ResourceAllocationOption::serialize(
            p.as_any_ref().downcast_ref::<ResourceAllocationOption>().unwrap(),
            serializer,
        ),
        StructType::ResourceConfigOption => ResourceConfigOption::serialize(
            p.as_any_ref().downcast_ref::<ResourceConfigOption>().unwrap(),
            serializer,
        ),
        StructType::ResourceConfigSpec => ResourceConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ResourceConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DatabaseSizeEstimate => DatabaseSizeEstimate::serialize(
            p.as_any_ref().downcast_ref::<DatabaseSizeEstimate>().unwrap(),
            serializer,
        ),
        StructType::DatabaseSizeParam => DatabaseSizeParam::serialize(
            p.as_any_ref().downcast_ref::<DatabaseSizeParam>().unwrap(),
            serializer,
        ),
        StructType::InventoryDescription => InventoryDescription::serialize(
            p.as_any_ref().downcast_ref::<InventoryDescription>().unwrap(),
            serializer,
        ),
        StructType::PerformanceStatisticsDescription => PerformanceStatisticsDescription::serialize(
            p.as_any_ref().downcast_ref::<PerformanceStatisticsDescription>().unwrap(),
            serializer,
        ),
        StructType::ResourcePoolResourceUsage => ResourcePoolResourceUsage::serialize(
            p.as_any_ref().downcast_ref::<ResourcePoolResourceUsage>().unwrap(),
            serializer,
        ),
        StructType::ResourcePoolRuntimeInfo => ResourcePoolRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<ResourcePoolRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::ResourcePoolSummary => ResourcePoolSummary::serialize(
            p.as_any_ref().downcast_ref::<ResourcePoolSummary>().unwrap(),
            serializer,
        ),
        StructType::VirtualAppSummary => VirtualAppSummary::serialize(
            p.as_any_ref().downcast_ref::<VirtualAppSummary>().unwrap(),
            serializer,
        ),
        StructType::ResourcePoolQuickStats => ResourcePoolQuickStats::serialize(
            p.as_any_ref().downcast_ref::<ResourcePoolQuickStats>().unwrap(),
            serializer,
        ),
        StructType::SddcBase => SddcBase::serialize(
            p.as_any_ref().downcast_ref::<SddcBase>().unwrap(),
            serializer,
        ),
        StructType::SelectionSet => SelectionSet::serialize(
            p.as_any_ref().downcast_ref::<SelectionSet>().unwrap(),
            serializer,
        ),
        StructType::DvPortgroupSelection => DvPortgroupSelection::serialize(
            p.as_any_ref().downcast_ref::<DvPortgroupSelection>().unwrap(),
            serializer,
        ),
        StructType::DvsSelection => DvsSelection::serialize(
            p.as_any_ref().downcast_ref::<DvsSelection>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionCompatibility => HostVMotionCompatibility::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionCompatibility>().unwrap(),
            serializer,
        ),
        StructType::ProductComponentInfo => ProductComponentInfo::serialize(
            p.as_any_ref().downcast_ref::<ProductComponentInfo>().unwrap(),
            serializer,
        ),
        StructType::ServiceContent => ServiceContent::serialize(
            p.as_any_ref().downcast_ref::<ServiceContent>().unwrap(),
            serializer,
        ),
        StructType::ServiceLocator => ServiceLocator::serialize(
            p.as_any_ref().downcast_ref::<ServiceLocator>().unwrap(),
            serializer,
        ),
        StructType::ServiceLocatorCredential => ServiceLocatorCredential::serialize(
            p.as_any_ref().downcast_ref::<ServiceLocatorCredential>().unwrap(),
            serializer,
        ),
        StructType::ServiceLocatorNamePassword => ServiceLocatorNamePassword::serialize(
            p.as_any_ref().downcast_ref::<ServiceLocatorNamePassword>().unwrap(),
            serializer,
        ),
        StructType::ServiceLocatorSamlCredential => ServiceLocatorSamlCredential::serialize(
            p.as_any_ref().downcast_ref::<ServiceLocatorSamlCredential>().unwrap(),
            serializer,
        ),
        StructType::ServiceManagerServiceInfo => ServiceManagerServiceInfo::serialize(
            p.as_any_ref().downcast_ref::<ServiceManagerServiceInfo>().unwrap(),
            serializer,
        ),
        StructType::SessionManagerGenericServiceTicket => SessionManagerGenericServiceTicket::serialize(
            p.as_any_ref().downcast_ref::<SessionManagerGenericServiceTicket>().unwrap(),
            serializer,
        ),
        StructType::SessionManagerLocalTicket => SessionManagerLocalTicket::serialize(
            p.as_any_ref().downcast_ref::<SessionManagerLocalTicket>().unwrap(),
            serializer,
        ),
        StructType::SessionManagerServiceRequestSpec => SessionManagerServiceRequestSpec::serialize(
            p.as_any_ref().downcast_ref::<SessionManagerServiceRequestSpec>().unwrap(),
            serializer,
        ),
        StructType::SessionManagerHttpServiceRequestSpec => SessionManagerHttpServiceRequestSpec::serialize(
            p.as_any_ref().downcast_ref::<SessionManagerHttpServiceRequestSpec>().unwrap(),
            serializer,
        ),
        StructType::SessionManagerVmomiServiceRequestSpec => SessionManagerVmomiServiceRequestSpec::serialize(
            p.as_any_ref().downcast_ref::<SessionManagerVmomiServiceRequestSpec>().unwrap(),
            serializer,
        ),
        StructType::SharesInfo => SharesInfo::serialize(
            p.as_any_ref().downcast_ref::<SharesInfo>().unwrap(),
            serializer,
        ),
        StructType::SharesOption => SharesOption::serialize(
            p.as_any_ref().downcast_ref::<SharesOption>().unwrap(),
            serializer,
        ),
        StructType::SiteInfo => SiteInfo::serialize(
            p.as_any_ref().downcast_ref::<SiteInfo>().unwrap(),
            serializer,
        ),
        StructType::StoragePodSummary => StoragePodSummary::serialize(
            p.as_any_ref().downcast_ref::<StoragePodSummary>().unwrap(),
            serializer,
        ),
        StructType::StorageIoAllocationInfo => StorageIoAllocationInfo::serialize(
            p.as_any_ref().downcast_ref::<StorageIoAllocationInfo>().unwrap(),
            serializer,
        ),
        StructType::StorageIoAllocationOption => StorageIoAllocationOption::serialize(
            p.as_any_ref().downcast_ref::<StorageIoAllocationOption>().unwrap(),
            serializer,
        ),
        StructType::StorageIormInfo => StorageIormInfo::serialize(
            p.as_any_ref().downcast_ref::<StorageIormInfo>().unwrap(),
            serializer,
        ),
        StructType::StorageIormConfigOption => StorageIormConfigOption::serialize(
            p.as_any_ref().downcast_ref::<StorageIormConfigOption>().unwrap(),
            serializer,
        ),
        StructType::StorageIormConfigSpec => StorageIormConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageIormConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::PodStorageDrsEntry => PodStorageDrsEntry::serialize(
            p.as_any_ref().downcast_ref::<PodStorageDrsEntry>().unwrap(),
            serializer,
        ),
        StructType::StoragePerformanceSummary => StoragePerformanceSummary::serialize(
            p.as_any_ref().downcast_ref::<StoragePerformanceSummary>().unwrap(),
            serializer,
        ),
        StructType::StorageResourceManagerStorageProfileStatistics => StorageResourceManagerStorageProfileStatistics::serialize(
            p.as_any_ref().downcast_ref::<StorageResourceManagerStorageProfileStatistics>().unwrap(),
            serializer,
        ),
        StructType::Tag => Tag::serialize(
            p.as_any_ref().downcast_ref::<Tag>().unwrap(),
            serializer,
        ),
        StructType::TaskDescription => TaskDescription::serialize(
            p.as_any_ref().downcast_ref::<TaskDescription>().unwrap(),
            serializer,
        ),
        StructType::TaskFilterSpec => TaskFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<TaskFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::TaskFilterSpecByEntity => TaskFilterSpecByEntity::serialize(
            p.as_any_ref().downcast_ref::<TaskFilterSpecByEntity>().unwrap(),
            serializer,
        ),
        StructType::TaskFilterSpecByTime => TaskFilterSpecByTime::serialize(
            p.as_any_ref().downcast_ref::<TaskFilterSpecByTime>().unwrap(),
            serializer,
        ),
        StructType::TaskFilterSpecByUsername => TaskFilterSpecByUsername::serialize(
            p.as_any_ref().downcast_ref::<TaskFilterSpecByUsername>().unwrap(),
            serializer,
        ),
        StructType::TaskInfo => TaskInfo::serialize(
            p.as_any_ref().downcast_ref::<TaskInfo>().unwrap(),
            serializer,
        ),
        StructType::TaskReason => TaskReason::serialize(
            p.as_any_ref().downcast_ref::<TaskReason>().unwrap(),
            serializer,
        ),
        StructType::TaskReasonAlarm => TaskReasonAlarm::serialize(
            p.as_any_ref().downcast_ref::<TaskReasonAlarm>().unwrap(),
            serializer,
        ),
        StructType::TaskReasonSchedule => TaskReasonSchedule::serialize(
            p.as_any_ref().downcast_ref::<TaskReasonSchedule>().unwrap(),
            serializer,
        ),
        StructType::TaskReasonSystem => TaskReasonSystem::serialize(
            p.as_any_ref().downcast_ref::<TaskReasonSystem>().unwrap(),
            serializer,
        ),
        StructType::TaskReasonUser => TaskReasonUser::serialize(
            p.as_any_ref().downcast_ref::<TaskReasonUser>().unwrap(),
            serializer,
        ),
        StructType::UpdateVirtualMachineFilesResult => UpdateVirtualMachineFilesResult::serialize(
            p.as_any_ref().downcast_ref::<UpdateVirtualMachineFilesResult>().unwrap(),
            serializer,
        ),
        StructType::UpdateVirtualMachineFilesResultFailedVmFileInfo => UpdateVirtualMachineFilesResultFailedVmFileInfo::serialize(
            p.as_any_ref().downcast_ref::<UpdateVirtualMachineFilesResultFailedVmFileInfo>().unwrap(),
            serializer,
        ),
        StructType::UserSearchResult => UserSearchResult::serialize(
            p.as_any_ref().downcast_ref::<UserSearchResult>().unwrap(),
            serializer,
        ),
        StructType::PosixUserSearchResult => PosixUserSearchResult::serialize(
            p.as_any_ref().downcast_ref::<PosixUserSearchResult>().unwrap(),
            serializer,
        ),
        StructType::UserSession => UserSession::serialize(
            p.as_any_ref().downcast_ref::<UserSession>().unwrap(),
            serializer,
        ),
        StructType::VVolVmConfigFileUpdateResult => VVolVmConfigFileUpdateResult::serialize(
            p.as_any_ref().downcast_ref::<VVolVmConfigFileUpdateResult>().unwrap(),
            serializer,
        ),
        StructType::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo => VVolVmConfigFileUpdateResultFailedVmConfigFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VVolVmConfigFileUpdateResultFailedVmConfigFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VasaStorageArray => VasaStorageArray::serialize(
            p.as_any_ref().downcast_ref::<VasaStorageArray>().unwrap(),
            serializer,
        ),
        StructType::VasaStorageArrayDiscoveryFcTransport => VasaStorageArrayDiscoveryFcTransport::serialize(
            p.as_any_ref().downcast_ref::<VasaStorageArrayDiscoveryFcTransport>().unwrap(),
            serializer,
        ),
        StructType::VasaStorageArrayDiscoveryIpTransport => VasaStorageArrayDiscoveryIpTransport::serialize(
            p.as_any_ref().downcast_ref::<VasaStorageArrayDiscoveryIpTransport>().unwrap(),
            serializer,
        ),
        StructType::VasaStorageArrayDiscoverySvcInfo => VasaStorageArrayDiscoverySvcInfo::serialize(
            p.as_any_ref().downcast_ref::<VasaStorageArrayDiscoverySvcInfo>().unwrap(),
            serializer,
        ),
        StructType::VasaProviderContainerSpec => VasaProviderContainerSpec::serialize(
            p.as_any_ref().downcast_ref::<VasaProviderContainerSpec>().unwrap(),
            serializer,
        ),
        StructType::VimVasaProvider => VimVasaProvider::serialize(
            p.as_any_ref().downcast_ref::<VimVasaProvider>().unwrap(),
            serializer,
        ),
        StructType::VimVasaProviderStatePerArray => VimVasaProviderStatePerArray::serialize(
            p.as_any_ref().downcast_ref::<VimVasaProviderStatePerArray>().unwrap(),
            serializer,
        ),
        StructType::VimVasaProviderVirtualHostConfig => VimVasaProviderVirtualHostConfig::serialize(
            p.as_any_ref().downcast_ref::<VimVasaProviderVirtualHostConfig>().unwrap(),
            serializer,
        ),
        StructType::VimVasaProviderInfo => VimVasaProviderInfo::serialize(
            p.as_any_ref().downcast_ref::<VimVasaProviderInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualAppLinkInfo => VirtualAppLinkInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualAppLinkInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSpec => VirtualDiskSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSpec>().unwrap(),
            serializer,
        ),
        StructType::DeviceBackedVirtualDiskSpec => DeviceBackedVirtualDiskSpec::serialize(
            p.as_any_ref().downcast_ref::<DeviceBackedVirtualDiskSpec>().unwrap(),
            serializer,
        ),
        StructType::FileBackedVirtualDiskSpec => FileBackedVirtualDiskSpec::serialize(
            p.as_any_ref().downcast_ref::<FileBackedVirtualDiskSpec>().unwrap(),
            serializer,
        ),
        StructType::SeSparseVirtualDiskSpec => SeSparseVirtualDiskSpec::serialize(
            p.as_any_ref().downcast_ref::<SeSparseVirtualDiskSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConnection => VirtualMachineConnection::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConnection>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMksConnection => VirtualMachineMksConnection::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMksConnection>().unwrap(),
            serializer,
        ),
        StructType::DiskChangeInfo => DiskChangeInfo::serialize(
            p.as_any_ref().downcast_ref::<DiskChangeInfo>().unwrap(),
            serializer,
        ),
        StructType::DiskChangeExtent => DiskChangeExtent::serialize(
            p.as_any_ref().downcast_ref::<DiskChangeExtent>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDisplayTopology => VirtualMachineDisplayTopology::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDisplayTopology>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMksTicket => VirtualMachineMksTicket::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMksTicket>().unwrap(),
            serializer,
        ),
        StructType::StorageRequirement => StorageRequirement::serialize(
            p.as_any_ref().downcast_ref::<StorageRequirement>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineTicket => VirtualMachineTicket::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineTicket>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineWipeResult => VirtualMachineWipeResult::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineWipeResult>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemNetworkPartitionInfo => VsanUpgradeSystemNetworkPartitionInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemNetworkPartitionInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemPreflightCheckIssue => VsanUpgradeSystemPreflightCheckIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemPreflightCheckIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemApiBrokenIssue => VsanUpgradeSystemApiBrokenIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemApiBrokenIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue => VsanUpgradeSystemAutoClaimEnabledOnHostsIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemHostsDisconnectedIssue => VsanUpgradeSystemHostsDisconnectedIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemHostsDisconnectedIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemMissingHostsInClusterIssue => VsanUpgradeSystemMissingHostsInClusterIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemMissingHostsInClusterIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemNetworkPartitionIssue => VsanUpgradeSystemNetworkPartitionIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemNetworkPartitionIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue => VsanUpgradeSystemNotEnoughFreeCapacityIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemNotEnoughFreeCapacityIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemRogueHostsInClusterIssue => VsanUpgradeSystemRogueHostsInClusterIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemRogueHostsInClusterIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue => VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemWrongEsxVersionIssue => VsanUpgradeSystemWrongEsxVersionIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemWrongEsxVersionIssue>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemPreflightCheckResult => VsanUpgradeSystemPreflightCheckResult::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemPreflightCheckResult>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemUpgradeHistoryItem => VsanUpgradeSystemUpgradeHistoryItem::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryItem>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp => VsanUpgradeSystemUpgradeHistoryDiskGroupOp::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail => VsanUpgradeSystemUpgradeHistoryPreflightFail::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeHistoryPreflightFail>().unwrap(),
            serializer,
        ),
        StructType::VsanUpgradeSystemUpgradeStatus => VsanUpgradeSystemUpgradeStatus::serialize(
            p.as_any_ref().downcast_ref::<VsanUpgradeSystemUpgradeStatus>().unwrap(),
            serializer,
        ),
        StructType::Action => Action::serialize(
            p.as_any_ref().downcast_ref::<Action>().unwrap(),
            serializer,
        ),
        StructType::CreateTaskAction => CreateTaskAction::serialize(
            p.as_any_ref().downcast_ref::<CreateTaskAction>().unwrap(),
            serializer,
        ),
        StructType::MethodAction => MethodAction::serialize(
            p.as_any_ref().downcast_ref::<MethodAction>().unwrap(),
            serializer,
        ),
        StructType::RunScriptAction => RunScriptAction::serialize(
            p.as_any_ref().downcast_ref::<RunScriptAction>().unwrap(),
            serializer,
        ),
        StructType::SendEmailAction => SendEmailAction::serialize(
            p.as_any_ref().downcast_ref::<SendEmailAction>().unwrap(),
            serializer,
        ),
        StructType::SendSnmpAction => SendSnmpAction::serialize(
            p.as_any_ref().downcast_ref::<SendSnmpAction>().unwrap(),
            serializer,
        ),
        StructType::MethodActionArgument => MethodActionArgument::serialize(
            p.as_any_ref().downcast_ref::<MethodActionArgument>().unwrap(),
            serializer,
        ),
        StructType::AlarmAction => AlarmAction::serialize(
            p.as_any_ref().downcast_ref::<AlarmAction>().unwrap(),
            serializer,
        ),
        StructType::AlarmTriggeringAction => AlarmTriggeringAction::serialize(
            p.as_any_ref().downcast_ref::<AlarmTriggeringAction>().unwrap(),
            serializer,
        ),
        StructType::GroupAlarmAction => GroupAlarmAction::serialize(
            p.as_any_ref().downcast_ref::<GroupAlarmAction>().unwrap(),
            serializer,
        ),
        StructType::AlarmDescription => AlarmDescription::serialize(
            p.as_any_ref().downcast_ref::<AlarmDescription>().unwrap(),
            serializer,
        ),
        StructType::AlarmExpression => AlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<AlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::AndAlarmExpression => AndAlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<AndAlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::EventAlarmExpression => EventAlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<EventAlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::MetricAlarmExpression => MetricAlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<MetricAlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::OrAlarmExpression => OrAlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<OrAlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::StateAlarmExpression => StateAlarmExpression::serialize(
            p.as_any_ref().downcast_ref::<StateAlarmExpression>().unwrap(),
            serializer,
        ),
        StructType::AlarmFilterSpec => AlarmFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<AlarmFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::AlarmSetting => AlarmSetting::serialize(
            p.as_any_ref().downcast_ref::<AlarmSetting>().unwrap(),
            serializer,
        ),
        StructType::AlarmSpec => AlarmSpec::serialize(
            p.as_any_ref().downcast_ref::<AlarmSpec>().unwrap(),
            serializer,
        ),
        StructType::AlarmInfo => AlarmInfo::serialize(
            p.as_any_ref().downcast_ref::<AlarmInfo>().unwrap(),
            serializer,
        ),
        StructType::AlarmState => AlarmState::serialize(
            p.as_any_ref().downcast_ref::<AlarmState>().unwrap(),
            serializer,
        ),
        StructType::AlarmTriggeringActionTransitionSpec => AlarmTriggeringActionTransitionSpec::serialize(
            p.as_any_ref().downcast_ref::<AlarmTriggeringActionTransitionSpec>().unwrap(),
            serializer,
        ),
        StructType::EventAlarmExpressionComparison => EventAlarmExpressionComparison::serialize(
            p.as_any_ref().downcast_ref::<EventAlarmExpressionComparison>().unwrap(),
            serializer,
        ),
        StructType::ClusterAction => ClusterAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterClusterInitialPlacementAction => ClusterClusterInitialPlacementAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterClusterInitialPlacementAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterHostInfraUpdateHaModeAction => ClusterHostInfraUpdateHaModeAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterHostInfraUpdateHaModeAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterHostPowerAction => ClusterHostPowerAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterHostPowerAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterInitialPlacementAction => ClusterInitialPlacementAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterInitialPlacementAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterMigrationAction => ClusterMigrationAction::serialize(
            p.as_any_ref().downcast_ref::<ClusterMigrationAction>().unwrap(),
            serializer,
        ),
        StructType::PlacementAction => PlacementAction::serialize(
            p.as_any_ref().downcast_ref::<PlacementAction>().unwrap(),
            serializer,
        ),
        StructType::HbrDiskMigrationAction => HbrDiskMigrationAction::serialize(
            p.as_any_ref().downcast_ref::<HbrDiskMigrationAction>().unwrap(),
            serializer,
        ),
        StructType::StorageMigrationAction => StorageMigrationAction::serialize(
            p.as_any_ref().downcast_ref::<StorageMigrationAction>().unwrap(),
            serializer,
        ),
        StructType::StoragePlacementAction => StoragePlacementAction::serialize(
            p.as_any_ref().downcast_ref::<StoragePlacementAction>().unwrap(),
            serializer,
        ),
        StructType::ClusterActionHistory => ClusterActionHistory::serialize(
            p.as_any_ref().downcast_ref::<ClusterActionHistory>().unwrap(),
            serializer,
        ),
        StructType::ClusterAttemptedVmInfo => ClusterAttemptedVmInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterAttemptedVmInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterConfigInfo => ClusterConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterConfigSpec => ClusterConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterCryptoConfigInfo => ClusterCryptoConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterCryptoConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAamNodeState => ClusterDasAamNodeState::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAamNodeState>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAdmissionControlInfo => ClusterDasAdmissionControlInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAdmissionControlInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverHostAdmissionControlInfo => ClusterFailoverHostAdmissionControlInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverLevelAdmissionControlInfo => ClusterFailoverLevelAdmissionControlInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverResourcesAdmissionControlInfo => ClusterFailoverResourcesAdmissionControlInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAdmissionControlPolicy => ClusterDasAdmissionControlPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAdmissionControlPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverHostAdmissionControlPolicy => ClusterFailoverHostAdmissionControlPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverLevelAdmissionControlPolicy => ClusterFailoverLevelAdmissionControlPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverResourcesAdmissionControlPolicy => ClusterFailoverResourcesAdmissionControlPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAdvancedRuntimeInfo => ClusterDasAdvancedRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAdvancedRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo => ClusterDasFailoverLevelAdvancedRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::DasHeartbeatDatastoreInfo => DasHeartbeatDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<DasHeartbeatDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo => ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasConfigInfo => ClusterDasConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasData => ClusterDasData::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasData>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasDataSummary => ClusterDasDataSummary::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasDataSummary>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots => ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo => ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots => ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasFdmHostState => ClusterDasFdmHostState::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasFdmHostState>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasHostInfo => ClusterDasHostInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasHostInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasAamHostInfo => ClusterDasAamHostInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasAamHostInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasHostRecommendation => ClusterDasHostRecommendation::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasHostRecommendation>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasVmConfigInfo => ClusterDasVmConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasVmConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasVmSettings => ClusterDasVmSettings::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasVmSettings>().unwrap(),
            serializer,
        ),
        StructType::ClusterDpmConfigInfo => ClusterDpmConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDpmConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDpmHostConfigInfo => ClusterDpmHostConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDpmHostConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsConfigInfo => ClusterDrsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsFaults => ClusterDrsFaults::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsFaults>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsFaultsFaultsByVm => ClusterDrsFaultsFaultsByVm::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsFaultsFaultsByVm>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsFaultsFaultsByVirtualDisk => ClusterDrsFaultsFaultsByVirtualDisk::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsFaultsFaultsByVirtualDisk>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsMigration => ClusterDrsMigration::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsMigration>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsRecommendation => ClusterDrsRecommendation::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsRecommendation>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsVmConfigInfo => ClusterDrsVmConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsVmConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterEvcManagerCheckResult => ClusterEvcManagerCheckResult::serialize(
            p.as_any_ref().downcast_ref::<ClusterEvcManagerCheckResult>().unwrap(),
            serializer,
        ),
        StructType::ClusterEvcManagerEvcState => ClusterEvcManagerEvcState::serialize(
            p.as_any_ref().downcast_ref::<ClusterEvcManagerEvcState>().unwrap(),
            serializer,
        ),
        StructType::ClusterEnterMaintenanceResult => ClusterEnterMaintenanceResult::serialize(
            p.as_any_ref().downcast_ref::<ClusterEnterMaintenanceResult>().unwrap(),
            serializer,
        ),
        StructType::ClusterFailoverHostAdmissionControlInfoHostStatus => ClusterFailoverHostAdmissionControlInfoHostStatus::serialize(
            p.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlInfoHostStatus>().unwrap(),
            serializer,
        ),
        StructType::ClusterGroupInfo => ClusterGroupInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterGroupInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterHostGroup => ClusterHostGroup::serialize(
            p.as_any_ref().downcast_ref::<ClusterHostGroup>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmGroup => ClusterVmGroup::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmGroup>().unwrap(),
            serializer,
        ),
        StructType::ClusterHostRecommendation => ClusterHostRecommendation::serialize(
            p.as_any_ref().downcast_ref::<ClusterHostRecommendation>().unwrap(),
            serializer,
        ),
        StructType::ClusterInfraUpdateHaConfigInfo => ClusterInfraUpdateHaConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterInfraUpdateHaConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterNotAttemptedVmInfo => ClusterNotAttemptedVmInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterNotAttemptedVmInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterOrchestrationInfo => ClusterOrchestrationInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterOrchestrationInfo>().unwrap(),
            serializer,
        ),
        StructType::PlacementResult => PlacementResult::serialize(
            p.as_any_ref().downcast_ref::<PlacementResult>().unwrap(),
            serializer,
        ),
        StructType::PlacementSpec => PlacementSpec::serialize(
            p.as_any_ref().downcast_ref::<PlacementSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterPowerOnVmResult => ClusterPowerOnVmResult::serialize(
            p.as_any_ref().downcast_ref::<ClusterPowerOnVmResult>().unwrap(),
            serializer,
        ),
        StructType::ClusterPreemptibleVmPairInfo => ClusterPreemptibleVmPairInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterPreemptibleVmPairInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterProactiveDrsConfigInfo => ClusterProactiveDrsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterProactiveDrsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterRecommendation => ClusterRecommendation::serialize(
            p.as_any_ref().downcast_ref::<ClusterRecommendation>().unwrap(),
            serializer,
        ),
        StructType::ClusterResourceUsageSummary => ClusterResourceUsageSummary::serialize(
            p.as_any_ref().downcast_ref::<ClusterResourceUsageSummary>().unwrap(),
            serializer,
        ),
        StructType::ClusterRuleInfo => ClusterRuleInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterRuleInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterAffinityRuleSpec => ClusterAffinityRuleSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterAffinityRuleSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterAntiAffinityRuleSpec => ClusterAntiAffinityRuleSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterAntiAffinityRuleSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterDependencyRuleInfo => ClusterDependencyRuleInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterDependencyRuleInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmHostRuleInfo => ClusterVmHostRuleInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmHostRuleInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskAntiAffinityRuleSpec => VirtualDiskAntiAffinityRuleSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskAntiAffinityRuleSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskRuleSpec => VirtualDiskRuleSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskRuleSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterSlotPolicy => ClusterSlotPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterSlotPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterFixedSizeSlotPolicy => ClusterFixedSizeSlotPolicy::serialize(
            p.as_any_ref().downcast_ref::<ClusterFixedSizeSlotPolicy>().unwrap(),
            serializer,
        ),
        StructType::ClusterSystemVMsConfigInfo => ClusterSystemVMsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterSystemVMsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterSystemVMsConfigSpec => ClusterSystemVMsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterSystemVMsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterUsageSummary => ClusterUsageSummary::serialize(
            p.as_any_ref().downcast_ref::<ClusterUsageSummary>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmComponentProtectionSettings => ClusterVmComponentProtectionSettings::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmComponentProtectionSettings>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmOrchestrationInfo => ClusterVmOrchestrationInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmOrchestrationInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmReadiness => ClusterVmReadiness::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmReadiness>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmToolsMonitoringSettings => ClusterVmToolsMonitoringSettings::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmToolsMonitoringSettings>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualPort => DistributedVirtualPort::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualPort>().unwrap(),
            serializer,
        ),
        StructType::DvPortConfigInfo => DvPortConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<DvPortConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::DvPortConfigSpec => DvPortConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvPortConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsFilterParameter => DvsFilterParameter::serialize(
            p.as_any_ref().downcast_ref::<DvsFilterParameter>().unwrap(),
            serializer,
        ),
        StructType::DvsHostLocalPortInfo => DvsHostLocalPortInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsHostLocalPortInfo>().unwrap(),
            serializer,
        ),
        StructType::DvPortStatus => DvPortStatus::serialize(
            p.as_any_ref().downcast_ref::<DvPortStatus>().unwrap(),
            serializer,
        ),
        StructType::DvPortSetting => DvPortSetting::serialize(
            p.as_any_ref().downcast_ref::<DvPortSetting>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsPortSetting => VMwareDvsPortSetting::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsPortSetting>().unwrap(),
            serializer,
        ),
        StructType::DvPortState => DvPortState::serialize(
            p.as_any_ref().downcast_ref::<DvPortState>().unwrap(),
            serializer,
        ),
        StructType::DvPortgroupConfigInfo => DvPortgroupConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<DvPortgroupConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::DvPortgroupConfigSpec => DvPortgroupConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvPortgroupConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualPortgroupNsxPortgroupOperationResult => DistributedVirtualPortgroupNsxPortgroupOperationResult::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualPortgroupNsxPortgroupOperationResult>().unwrap(),
            serializer,
        ),
        StructType::DvPortgroupPolicy => DvPortgroupPolicy::serialize(
            p.as_any_ref().downcast_ref::<DvPortgroupPolicy>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsPortgroupPolicy => VMwareDvsPortgroupPolicy::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsPortgroupPolicy>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualPortgroupProblem => DistributedVirtualPortgroupProblem::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualPortgroupProblem>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualPortgroupInfo => DistributedVirtualPortgroupInfo::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualPortgroupInfo>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchInfo => DistributedVirtualSwitchInfo::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchInfo>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerCompatibilityResult => DistributedVirtualSwitchManagerCompatibilityResult::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerCompatibilityResult>().unwrap(),
            serializer,
        ),
        StructType::DvsManagerDvsConfigTarget => DvsManagerDvsConfigTarget::serialize(
            p.as_any_ref().downcast_ref::<DvsManagerDvsConfigTarget>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerDvsProductSpec => DistributedVirtualSwitchManagerDvsProductSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerDvsProductSpec>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerHostContainer => DistributedVirtualSwitchManagerHostContainer::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostContainer>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec => DistributedVirtualSwitchManagerHostDvsFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerHostArrayFilter => DistributedVirtualSwitchManagerHostArrayFilter::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostArrayFilter>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerHostContainerFilter => DistributedVirtualSwitchManagerHostContainerFilter::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostContainerFilter>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter => DistributedVirtualSwitchManagerHostDvsMembershipFilter::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerHostDvsMembershipFilter>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchManagerImportResult => DistributedVirtualSwitchManagerImportResult::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchManagerImportResult>().unwrap(),
            serializer,
        ),
        StructType::DvsManagerPhysicalNicsList => DvsManagerPhysicalNicsList::serialize(
            p.as_any_ref().downcast_ref::<DvsManagerPhysicalNicsList>().unwrap(),
            serializer,
        ),
        StructType::EntityBackup => EntityBackup::serialize(
            p.as_any_ref().downcast_ref::<EntityBackup>().unwrap(),
            serializer,
        ),
        StructType::EntityBackupConfig => EntityBackupConfig::serialize(
            p.as_any_ref().downcast_ref::<EntityBackupConfig>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMember => DistributedVirtualSwitchHostMember::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMember>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberBacking => DistributedVirtualSwitchHostMemberBacking::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberBacking>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberPnicBacking => DistributedVirtualSwitchHostMemberPnicBacking::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberPnicBacking>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberConfigInfo => DistributedVirtualSwitchHostMemberConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberConfigSpec => DistributedVirtualSwitchHostMemberConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostMemberHealthCheckResult => HostMemberHealthCheckResult::serialize(
            p.as_any_ref().downcast_ref::<HostMemberHealthCheckResult>().unwrap(),
            serializer,
        ),
        StructType::HostMemberUplinkHealthCheckResult => HostMemberUplinkHealthCheckResult::serialize(
            p.as_any_ref().downcast_ref::<HostMemberUplinkHealthCheckResult>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsMtuHealthCheckResult => VMwareDvsMtuHealthCheckResult::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsMtuHealthCheckResult>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsVlanHealthCheckResult => VMwareDvsVlanHealthCheckResult::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsVlanHealthCheckResult>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsTeamingHealthCheckResult => VMwareDvsTeamingHealthCheckResult::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsTeamingHealthCheckResult>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberPnicSpec => DistributedVirtualSwitchHostMemberPnicSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberPnicSpec>().unwrap(),
            serializer,
        ),
        StructType::HostMemberRuntimeInfo => HostMemberRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostMemberRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberRuntimeState => DistributedVirtualSwitchHostMemberRuntimeState::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberRuntimeState>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostMemberTransportZoneInfo => DistributedVirtualSwitchHostMemberTransportZoneInfo::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostMemberTransportZoneInfo>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchHostProductSpec => DistributedVirtualSwitchHostProductSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchHostProductSpec>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchKeyedOpaqueBlob => DistributedVirtualSwitchKeyedOpaqueBlob::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchKeyedOpaqueBlob>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchNetworkOffloadSpec => DistributedVirtualSwitchNetworkOffloadSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchNetworkOffloadSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkResourcePool => DvsNetworkResourcePool::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkResourcePool>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkResourcePoolAllocationInfo => DvsNetworkResourcePoolAllocationInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkResourcePoolAllocationInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkResourcePoolConfigSpec => DvsNetworkResourcePoolConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkResourcePoolConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchPortConnectee => DistributedVirtualSwitchPortConnectee::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortConnectee>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchPortConnection => DistributedVirtualSwitchPortConnection::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortConnection>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchPortCriteria => DistributedVirtualSwitchPortCriteria::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortCriteria>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchPortStatistics => DistributedVirtualSwitchPortStatistics::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchPortStatistics>().unwrap(),
            serializer,
        ),
        StructType::DistributedVirtualSwitchProductSpec => DistributedVirtualSwitchProductSpec::serialize(
            p.as_any_ref().downcast_ref::<DistributedVirtualSwitchProductSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsTrafficRule => DvsTrafficRule::serialize(
            p.as_any_ref().downcast_ref::<DvsTrafficRule>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkRuleAction => DvsNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsAcceptNetworkRuleAction => DvsAcceptNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsAcceptNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsCopyNetworkRuleAction => DvsCopyNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsCopyNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsDropNetworkRuleAction => DvsDropNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsDropNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsGreEncapNetworkRuleAction => DvsGreEncapNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsGreEncapNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsLogNetworkRuleAction => DvsLogNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsLogNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsMacRewriteNetworkRuleAction => DvsMacRewriteNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsMacRewriteNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsPuntNetworkRuleAction => DvsPuntNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsPuntNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsRateLimitNetworkRuleAction => DvsRateLimitNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsRateLimitNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsUpdateTagNetworkRuleAction => DvsUpdateTagNetworkRuleAction::serialize(
            p.as_any_ref().downcast_ref::<DvsUpdateTagNetworkRuleAction>().unwrap(),
            serializer,
        ),
        StructType::DvsNetworkRuleQualifier => DvsNetworkRuleQualifier::serialize(
            p.as_any_ref().downcast_ref::<DvsNetworkRuleQualifier>().unwrap(),
            serializer,
        ),
        StructType::DvsIpNetworkRuleQualifier => DvsIpNetworkRuleQualifier::serialize(
            p.as_any_ref().downcast_ref::<DvsIpNetworkRuleQualifier>().unwrap(),
            serializer,
        ),
        StructType::DvsMacNetworkRuleQualifier => DvsMacNetworkRuleQualifier::serialize(
            p.as_any_ref().downcast_ref::<DvsMacNetworkRuleQualifier>().unwrap(),
            serializer,
        ),
        StructType::DvsSystemTrafficNetworkRuleQualifier => DvsSystemTrafficNetworkRuleQualifier::serialize(
            p.as_any_ref().downcast_ref::<DvsSystemTrafficNetworkRuleQualifier>().unwrap(),
            serializer,
        ),
        StructType::DvsTrafficRuleset => DvsTrafficRuleset::serialize(
            p.as_any_ref().downcast_ref::<DvsTrafficRuleset>().unwrap(),
            serializer,
        ),
        StructType::DvsVmVnicNetworkResourcePool => DvsVmVnicNetworkResourcePool::serialize(
            p.as_any_ref().downcast_ref::<DvsVmVnicNetworkResourcePool>().unwrap(),
            serializer,
        ),
        StructType::DvsVmVnicResourcePoolConfigSpec => DvsVmVnicResourcePoolConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<DvsVmVnicResourcePoolConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::DvsVmVnicResourceAllocation => DvsVmVnicResourceAllocation::serialize(
            p.as_any_ref().downcast_ref::<DvsVmVnicResourceAllocation>().unwrap(),
            serializer,
        ),
        StructType::DvsVmVnicNetworkResourcePoolRuntimeInfo => DvsVmVnicNetworkResourcePoolRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<DvsVmVnicNetworkResourcePoolRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::DvsVnicAllocatedResource => DvsVnicAllocatedResource::serialize(
            p.as_any_ref().downcast_ref::<DvsVnicAllocatedResource>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsDpuCapability => VMwareDvsDpuCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsDpuCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareIpfixConfig => VMwareIpfixConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareIpfixConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsIpfixCapability => VMwareDvsIpfixCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsIpfixCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsLacpCapability => VMwareDvsLacpCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsLacpCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsLacpGroupConfig => VMwareDvsLacpGroupConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsLacpGroupConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsLacpGroupSpec => VMwareDvsLacpGroupSpec::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsLacpGroupSpec>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsLagIpfixConfig => VMwareDvsLagIpfixConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsLagIpfixConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsLagVlanConfig => VMwareDvsLagVlanConfig::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsLagVlanConfig>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsMtuCapability => VMwareDvsMtuCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsMtuCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsPvlanConfigSpec => VMwareDvsPvlanConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsPvlanConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsPvlanMapEntry => VMwareDvsPvlanMapEntry::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsPvlanMapEntry>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsVspanConfigSpec => VMwareDvsVspanConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsVspanConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VMwareDvsVspanCapability => VMwareDvsVspanCapability::serialize(
            p.as_any_ref().downcast_ref::<VMwareDvsVspanCapability>().unwrap(),
            serializer,
        ),
        StructType::VMwareVspanPort => VMwareVspanPort::serialize(
            p.as_any_ref().downcast_ref::<VMwareVspanPort>().unwrap(),
            serializer,
        ),
        StructType::VMwareVspanSession => VMwareVspanSession::serialize(
            p.as_any_ref().downcast_ref::<VMwareVspanSession>().unwrap(),
            serializer,
        ),
        StructType::CryptoKeyId => CryptoKeyId::serialize(
            p.as_any_ref().downcast_ref::<CryptoKeyId>().unwrap(),
            serializer,
        ),
        StructType::CryptoKeyPlain => CryptoKeyPlain::serialize(
            p.as_any_ref().downcast_ref::<CryptoKeyPlain>().unwrap(),
            serializer,
        ),
        StructType::CryptoKeyResult => CryptoKeyResult::serialize(
            p.as_any_ref().downcast_ref::<CryptoKeyResult>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerHostKeyStatus => CryptoManagerHostKeyStatus::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerHostKeyStatus>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipCertSignRequest => CryptoManagerKmipCertSignRequest::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipCertSignRequest>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipCertificateInfo => CryptoManagerKmipCertificateInfo::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipCertificateInfo>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipClusterStatus => CryptoManagerKmipClusterStatus::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipClusterStatus>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipCryptoKeyStatus => CryptoManagerKmipCryptoKeyStatus::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipCryptoKeyStatus>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipCustomAttributeSpec => CryptoManagerKmipCustomAttributeSpec::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipCustomAttributeSpec>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipServerCertInfo => CryptoManagerKmipServerCertInfo::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipServerCertInfo>().unwrap(),
            serializer,
        ),
        StructType::CryptoManagerKmipServerStatus => CryptoManagerKmipServerStatus::serialize(
            p.as_any_ref().downcast_ref::<CryptoManagerKmipServerStatus>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpec => CryptoSpec::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpec>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecDecrypt => CryptoSpecDecrypt::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecDecrypt>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecDeepRecrypt => CryptoSpecDeepRecrypt::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecDeepRecrypt>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecEncrypt => CryptoSpecEncrypt::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecEncrypt>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecNoOp => CryptoSpecNoOp::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecNoOp>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecRegister => CryptoSpecRegister::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecRegister>().unwrap(),
            serializer,
        ),
        StructType::CryptoSpecShallowRecrypt => CryptoSpecShallowRecrypt::serialize(
            p.as_any_ref().downcast_ref::<CryptoSpecShallowRecrypt>().unwrap(),
            serializer,
        ),
        StructType::KeyProviderId => KeyProviderId::serialize(
            p.as_any_ref().downcast_ref::<KeyProviderId>().unwrap(),
            serializer,
        ),
        StructType::KmipClusterInfo => KmipClusterInfo::serialize(
            p.as_any_ref().downcast_ref::<KmipClusterInfo>().unwrap(),
            serializer,
        ),
        StructType::KmipServerInfo => KmipServerInfo::serialize(
            p.as_any_ref().downcast_ref::<KmipServerInfo>().unwrap(),
            serializer,
        ),
        StructType::KmipServerSpec => KmipServerSpec::serialize(
            p.as_any_ref().downcast_ref::<KmipServerSpec>().unwrap(),
            serializer,
        ),
        StructType::KmipServerStatus => KmipServerStatus::serialize(
            p.as_any_ref().downcast_ref::<KmipServerStatus>().unwrap(),
            serializer,
        ),
        StructType::ChangesInfoEventArgument => ChangesInfoEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ChangesInfoEventArgument>().unwrap(),
            serializer,
        ),
        StructType::DvsOutOfSyncHostArgument => DvsOutOfSyncHostArgument::serialize(
            p.as_any_ref().downcast_ref::<DvsOutOfSyncHostArgument>().unwrap(),
            serializer,
        ),
        StructType::Event => Event::serialize(
            p.as_any_ref().downcast_ref::<Event>().unwrap(),
            serializer,
        ),
        StructType::EventArgument => EventArgument::serialize(
            p.as_any_ref().downcast_ref::<EventArgument>().unwrap(),
            serializer,
        ),
        StructType::EntityEventArgument => EntityEventArgument::serialize(
            p.as_any_ref().downcast_ref::<EntityEventArgument>().unwrap(),
            serializer,
        ),
        StructType::AlarmEventArgument => AlarmEventArgument::serialize(
            p.as_any_ref().downcast_ref::<AlarmEventArgument>().unwrap(),
            serializer,
        ),
        StructType::ComputeResourceEventArgument => ComputeResourceEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ComputeResourceEventArgument>().unwrap(),
            serializer,
        ),
        StructType::DatacenterEventArgument => DatacenterEventArgument::serialize(
            p.as_any_ref().downcast_ref::<DatacenterEventArgument>().unwrap(),
            serializer,
        ),
        StructType::DatastoreEventArgument => DatastoreEventArgument::serialize(
            p.as_any_ref().downcast_ref::<DatastoreEventArgument>().unwrap(),
            serializer,
        ),
        StructType::DvsEventArgument => DvsEventArgument::serialize(
            p.as_any_ref().downcast_ref::<DvsEventArgument>().unwrap(),
            serializer,
        ),
        StructType::FolderEventArgument => FolderEventArgument::serialize(
            p.as_any_ref().downcast_ref::<FolderEventArgument>().unwrap(),
            serializer,
        ),
        StructType::HostEventArgument => HostEventArgument::serialize(
            p.as_any_ref().downcast_ref::<HostEventArgument>().unwrap(),
            serializer,
        ),
        StructType::ManagedEntityEventArgument => ManagedEntityEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ManagedEntityEventArgument>().unwrap(),
            serializer,
        ),
        StructType::NetworkEventArgument => NetworkEventArgument::serialize(
            p.as_any_ref().downcast_ref::<NetworkEventArgument>().unwrap(),
            serializer,
        ),
        StructType::ResourcePoolEventArgument => ResourcePoolEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ResourcePoolEventArgument>().unwrap(),
            serializer,
        ),
        StructType::ScheduledTaskEventArgument => ScheduledTaskEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ScheduledTaskEventArgument>().unwrap(),
            serializer,
        ),
        StructType::VmEventArgument => VmEventArgument::serialize(
            p.as_any_ref().downcast_ref::<VmEventArgument>().unwrap(),
            serializer,
        ),
        StructType::ProfileEventArgument => ProfileEventArgument::serialize(
            p.as_any_ref().downcast_ref::<ProfileEventArgument>().unwrap(),
            serializer,
        ),
        StructType::RoleEventArgument => RoleEventArgument::serialize(
            p.as_any_ref().downcast_ref::<RoleEventArgument>().unwrap(),
            serializer,
        ),
        StructType::EventDescription => EventDescription::serialize(
            p.as_any_ref().downcast_ref::<EventDescription>().unwrap(),
            serializer,
        ),
        StructType::EventArgDesc => EventArgDesc::serialize(
            p.as_any_ref().downcast_ref::<EventArgDesc>().unwrap(),
            serializer,
        ),
        StructType::EventDescriptionEventDetail => EventDescriptionEventDetail::serialize(
            p.as_any_ref().downcast_ref::<EventDescriptionEventDetail>().unwrap(),
            serializer,
        ),
        StructType::EventFilterSpec => EventFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<EventFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::EventFilterSpecByEntity => EventFilterSpecByEntity::serialize(
            p.as_any_ref().downcast_ref::<EventFilterSpecByEntity>().unwrap(),
            serializer,
        ),
        StructType::EventFilterSpecByTime => EventFilterSpecByTime::serialize(
            p.as_any_ref().downcast_ref::<EventFilterSpecByTime>().unwrap(),
            serializer,
        ),
        StructType::EventFilterSpecByUsername => EventFilterSpecByUsername::serialize(
            p.as_any_ref().downcast_ref::<EventFilterSpecByUsername>().unwrap(),
            serializer,
        ),
        StructType::ExtendedEventPair => ExtendedEventPair::serialize(
            p.as_any_ref().downcast_ref::<ExtendedEventPair>().unwrap(),
            serializer,
        ),
        StructType::VnicPortArgument => VnicPortArgument::serialize(
            p.as_any_ref().downcast_ref::<VnicPortArgument>().unwrap(),
            serializer,
        ),
        StructType::ExtExtendedProductInfo => ExtExtendedProductInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtExtendedProductInfo>().unwrap(),
            serializer,
        ),
        StructType::ManagedByInfo => ManagedByInfo::serialize(
            p.as_any_ref().downcast_ref::<ManagedByInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtManagedEntityInfo => ExtManagedEntityInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtManagedEntityInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtSolutionManagerInfo => ExtSolutionManagerInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtSolutionManagerInfo>().unwrap(),
            serializer,
        ),
        StructType::ExtSolutionManagerInfoTabInfo => ExtSolutionManagerInfoTabInfo::serialize(
            p.as_any_ref().downcast_ref::<ExtSolutionManagerInfoTabInfo>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileUpdateFailure => AnswerFileUpdateFailure::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileUpdateFailure>().unwrap(),
            serializer,
        ),
        StructType::ConflictingConfigurationConfig => ConflictingConfigurationConfig::serialize(
            p.as_any_ref().downcast_ref::<ConflictingConfigurationConfig>().unwrap(),
            serializer,
        ),
        StructType::DatacenterMismatchArgument => DatacenterMismatchArgument::serialize(
            p.as_any_ref().downcast_ref::<DatacenterMismatchArgument>().unwrap(),
            serializer,
        ),
        StructType::DvsApplyOperationFaultFaultOnObject => DvsApplyOperationFaultFaultOnObject::serialize(
            p.as_any_ref().downcast_ref::<DvsApplyOperationFaultFaultOnObject>().unwrap(),
            serializer,
        ),
        StructType::DvsOperationBulkFaultFaultOnHost => DvsOperationBulkFaultFaultOnHost::serialize(
            p.as_any_ref().downcast_ref::<DvsOperationBulkFaultFaultOnHost>().unwrap(),
            serializer,
        ),
        StructType::ImportOperationBulkFaultFaultOnImport => ImportOperationBulkFaultFaultOnImport::serialize(
            p.as_any_ref().downcast_ref::<ImportOperationBulkFaultFaultOnImport>().unwrap(),
            serializer,
        ),
        StructType::MultipleCertificatesVerifyFaultThumbprintData => MultipleCertificatesVerifyFaultThumbprintData::serialize(
            p.as_any_ref().downcast_ref::<MultipleCertificatesVerifyFaultThumbprintData>().unwrap(),
            serializer,
        ),
        StructType::NoPermissionEntityPrivileges => NoPermissionEntityPrivileges::serialize(
            p.as_any_ref().downcast_ref::<NoPermissionEntityPrivileges>().unwrap(),
            serializer,
        ),
        StructType::ProfileUpdateFailedUpdateFailure => ProfileUpdateFailedUpdateFailure::serialize(
            p.as_any_ref().downcast_ref::<ProfileUpdateFailedUpdateFailure>().unwrap(),
            serializer,
        ),
        StructType::HostActiveDirectory => HostActiveDirectory::serialize(
            p.as_any_ref().downcast_ref::<HostActiveDirectory>().unwrap(),
            serializer,
        ),
        StructType::HostActiveDirectorySpec => HostActiveDirectorySpec::serialize(
            p.as_any_ref().downcast_ref::<HostActiveDirectorySpec>().unwrap(),
            serializer,
        ),
        StructType::HostAssignableHardwareBinding => HostAssignableHardwareBinding::serialize(
            p.as_any_ref().downcast_ref::<HostAssignableHardwareBinding>().unwrap(),
            serializer,
        ),
        StructType::HostAssignableHardwareConfig => HostAssignableHardwareConfig::serialize(
            p.as_any_ref().downcast_ref::<HostAssignableHardwareConfig>().unwrap(),
            serializer,
        ),
        StructType::HostAssignableHardwareConfigAttributeOverride => HostAssignableHardwareConfigAttributeOverride::serialize(
            p.as_any_ref().downcast_ref::<HostAssignableHardwareConfigAttributeOverride>().unwrap(),
            serializer,
        ),
        StructType::HostAuthenticationManagerInfo => HostAuthenticationManagerInfo::serialize(
            p.as_any_ref().downcast_ref::<HostAuthenticationManagerInfo>().unwrap(),
            serializer,
        ),
        StructType::HostAuthenticationStoreInfo => HostAuthenticationStoreInfo::serialize(
            p.as_any_ref().downcast_ref::<HostAuthenticationStoreInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDirectoryStoreInfo => HostDirectoryStoreInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDirectoryStoreInfo>().unwrap(),
            serializer,
        ),
        StructType::HostActiveDirectoryInfo => HostActiveDirectoryInfo::serialize(
            p.as_any_ref().downcast_ref::<HostActiveDirectoryInfo>().unwrap(),
            serializer,
        ),
        StructType::HostLocalAuthenticationInfo => HostLocalAuthenticationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostLocalAuthenticationInfo>().unwrap(),
            serializer,
        ),
        StructType::AutoStartPowerInfo => AutoStartPowerInfo::serialize(
            p.as_any_ref().downcast_ref::<AutoStartPowerInfo>().unwrap(),
            serializer,
        ),
        StructType::HostAutoStartManagerConfig => HostAutoStartManagerConfig::serialize(
            p.as_any_ref().downcast_ref::<HostAutoStartManagerConfig>().unwrap(),
            serializer,
        ),
        StructType::AutoStartDefaults => AutoStartDefaults::serialize(
            p.as_any_ref().downcast_ref::<AutoStartDefaults>().unwrap(),
            serializer,
        ),
        StructType::HostBiosInfo => HostBiosInfo::serialize(
            p.as_any_ref().downcast_ref::<HostBiosInfo>().unwrap(),
            serializer,
        ),
        StructType::HostBootDeviceInfo => HostBootDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostBootDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::HostBootDevice => HostBootDevice::serialize(
            p.as_any_ref().downcast_ref::<HostBootDevice>().unwrap(),
            serializer,
        ),
        StructType::HostCacheConfigurationInfo => HostCacheConfigurationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostCacheConfigurationInfo>().unwrap(),
            serializer,
        ),
        StructType::HostCacheConfigurationSpec => HostCacheConfigurationSpec::serialize(
            p.as_any_ref().downcast_ref::<HostCacheConfigurationSpec>().unwrap(),
            serializer,
        ),
        StructType::HostCapability => HostCapability::serialize(
            p.as_any_ref().downcast_ref::<HostCapability>().unwrap(),
            serializer,
        ),
        StructType::HostCertificateManagerCertificateInfo => HostCertificateManagerCertificateInfo::serialize(
            p.as_any_ref().downcast_ref::<HostCertificateManagerCertificateInfo>().unwrap(),
            serializer,
        ),
        StructType::HostCertificateManagerCertificateSpec => HostCertificateManagerCertificateSpec::serialize(
            p.as_any_ref().downcast_ref::<HostCertificateManagerCertificateSpec>().unwrap(),
            serializer,
        ),
        StructType::HostConfigChange => HostConfigChange::serialize(
            p.as_any_ref().downcast_ref::<HostConfigChange>().unwrap(),
            serializer,
        ),
        StructType::HostConfigInfo => HostConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostConfigManager => HostConfigManager::serialize(
            p.as_any_ref().downcast_ref::<HostConfigManager>().unwrap(),
            serializer,
        ),
        StructType::HostConfigSpec => HostConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostConnectInfo => HostConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreConnectInfo => HostDatastoreConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreExistsConnectInfo => HostDatastoreExistsConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreExistsConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreNameConflictConnectInfo => HostDatastoreNameConflictConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreNameConflictConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostLicenseConnectInfo => HostLicenseConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostLicenseConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostConnectInfoNetworkInfo => HostConnectInfoNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<HostConnectInfoNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNewNetworkConnectInfo => HostNewNetworkConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNewNetworkConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::HostConnectSpec => HostConnectSpec::serialize(
            p.as_any_ref().downcast_ref::<HostConnectSpec>().unwrap(),
            serializer,
        ),
        StructType::HostCpuIdInfo => HostCpuIdInfo::serialize(
            p.as_any_ref().downcast_ref::<HostCpuIdInfo>().unwrap(),
            serializer,
        ),
        StructType::HostCpuInfo => HostCpuInfo::serialize(
            p.as_any_ref().downcast_ref::<HostCpuInfo>().unwrap(),
            serializer,
        ),
        StructType::HostCpuPackage => HostCpuPackage::serialize(
            p.as_any_ref().downcast_ref::<HostCpuPackage>().unwrap(),
            serializer,
        ),
        StructType::HostCpuPowerManagementInfo => HostCpuPowerManagementInfo::serialize(
            p.as_any_ref().downcast_ref::<HostCpuPowerManagementInfo>().unwrap(),
            serializer,
        ),
        StructType::HostHyperThreadScheduleInfo => HostHyperThreadScheduleInfo::serialize(
            p.as_any_ref().downcast_ref::<HostHyperThreadScheduleInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDataTransportConnectionInfo => HostDataTransportConnectionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDataTransportConnectionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNfcConnectionInfo => HostNfcConnectionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNfcConnectionInfo>().unwrap(),
            serializer,
        ),
        StructType::FileInfo => FileInfo::serialize(
            p.as_any_ref().downcast_ref::<FileInfo>().unwrap(),
            serializer,
        ),
        StructType::FloppyImageFileInfo => FloppyImageFileInfo::serialize(
            p.as_any_ref().downcast_ref::<FloppyImageFileInfo>().unwrap(),
            serializer,
        ),
        StructType::FolderFileInfo => FolderFileInfo::serialize(
            p.as_any_ref().downcast_ref::<FolderFileInfo>().unwrap(),
            serializer,
        ),
        StructType::IsoImageFileInfo => IsoImageFileInfo::serialize(
            p.as_any_ref().downcast_ref::<IsoImageFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VmConfigFileInfo => VmConfigFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VmConfigFileInfo>().unwrap(),
            serializer,
        ),
        StructType::TemplateConfigFileInfo => TemplateConfigFileInfo::serialize(
            p.as_any_ref().downcast_ref::<TemplateConfigFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VmDiskFileInfo => VmDiskFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VmDiskFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VmLogFileInfo => VmLogFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VmLogFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VmNvramFileInfo => VmNvramFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VmNvramFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VmSnapshotFileInfo => VmSnapshotFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VmSnapshotFileInfo>().unwrap(),
            serializer,
        ),
        StructType::FileQueryFlags => FileQueryFlags::serialize(
            p.as_any_ref().downcast_ref::<FileQueryFlags>().unwrap(),
            serializer,
        ),
        StructType::FileQuery => FileQuery::serialize(
            p.as_any_ref().downcast_ref::<FileQuery>().unwrap(),
            serializer,
        ),
        StructType::FloppyImageFileQuery => FloppyImageFileQuery::serialize(
            p.as_any_ref().downcast_ref::<FloppyImageFileQuery>().unwrap(),
            serializer,
        ),
        StructType::FolderFileQuery => FolderFileQuery::serialize(
            p.as_any_ref().downcast_ref::<FolderFileQuery>().unwrap(),
            serializer,
        ),
        StructType::IsoImageFileQuery => IsoImageFileQuery::serialize(
            p.as_any_ref().downcast_ref::<IsoImageFileQuery>().unwrap(),
            serializer,
        ),
        StructType::VmConfigFileQuery => VmConfigFileQuery::serialize(
            p.as_any_ref().downcast_ref::<VmConfigFileQuery>().unwrap(),
            serializer,
        ),
        StructType::TemplateConfigFileQuery => TemplateConfigFileQuery::serialize(
            p.as_any_ref().downcast_ref::<TemplateConfigFileQuery>().unwrap(),
            serializer,
        ),
        StructType::VmDiskFileQuery => VmDiskFileQuery::serialize(
            p.as_any_ref().downcast_ref::<VmDiskFileQuery>().unwrap(),
            serializer,
        ),
        StructType::VmLogFileQuery => VmLogFileQuery::serialize(
            p.as_any_ref().downcast_ref::<VmLogFileQuery>().unwrap(),
            serializer,
        ),
        StructType::VmNvramFileQuery => VmNvramFileQuery::serialize(
            p.as_any_ref().downcast_ref::<VmNvramFileQuery>().unwrap(),
            serializer,
        ),
        StructType::VmSnapshotFileQuery => VmSnapshotFileQuery::serialize(
            p.as_any_ref().downcast_ref::<VmSnapshotFileQuery>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreBrowserSearchResults => HostDatastoreBrowserSearchResults::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreBrowserSearchResults>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreBrowserSearchSpec => HostDatastoreBrowserSearchSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreBrowserSearchSpec>().unwrap(),
            serializer,
        ),
        StructType::VmConfigFileEncryptionInfo => VmConfigFileEncryptionInfo::serialize(
            p.as_any_ref().downcast_ref::<VmConfigFileEncryptionInfo>().unwrap(),
            serializer,
        ),
        StructType::VmConfigFileQueryFlags => VmConfigFileQueryFlags::serialize(
            p.as_any_ref().downcast_ref::<VmConfigFileQueryFlags>().unwrap(),
            serializer,
        ),
        StructType::VmConfigFileQueryFilter => VmConfigFileQueryFilter::serialize(
            p.as_any_ref().downcast_ref::<VmConfigFileQueryFilter>().unwrap(),
            serializer,
        ),
        StructType::VmDiskFileEncryptionInfo => VmDiskFileEncryptionInfo::serialize(
            p.as_any_ref().downcast_ref::<VmDiskFileEncryptionInfo>().unwrap(),
            serializer,
        ),
        StructType::VmDiskFileQueryFlags => VmDiskFileQueryFlags::serialize(
            p.as_any_ref().downcast_ref::<VmDiskFileQueryFlags>().unwrap(),
            serializer,
        ),
        StructType::VmDiskFileQueryFilter => VmDiskFileQueryFilter::serialize(
            p.as_any_ref().downcast_ref::<VmDiskFileQueryFilter>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreSystemCapabilities => HostDatastoreSystemCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreSystemCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreSystemDatastoreResult => HostDatastoreSystemDatastoreResult::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreSystemDatastoreResult>().unwrap(),
            serializer,
        ),
        StructType::HostDatastoreSystemVvolDatastoreSpec => HostDatastoreSystemVvolDatastoreSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDatastoreSystemVvolDatastoreSpec>().unwrap(),
            serializer,
        ),
        StructType::HostDateTimeConfig => HostDateTimeConfig::serialize(
            p.as_any_ref().downcast_ref::<HostDateTimeConfig>().unwrap(),
            serializer,
        ),
        StructType::HostDateTimeInfo => HostDateTimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDateTimeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDateTimeSystemServiceTestResult => HostDateTimeSystemServiceTestResult::serialize(
            p.as_any_ref().downcast_ref::<HostDateTimeSystemServiceTestResult>().unwrap(),
            serializer,
        ),
        StructType::HostDateTimeSystemTimeZone => HostDateTimeSystemTimeZone::serialize(
            p.as_any_ref().downcast_ref::<HostDateTimeSystemTimeZone>().unwrap(),
            serializer,
        ),
        StructType::HostDeploymentInfo => HostDeploymentInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDeploymentInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDevice => HostDevice::serialize(
            p.as_any_ref().downcast_ref::<HostDevice>().unwrap(),
            serializer,
        ),
        StructType::ScsiLun => ScsiLun::serialize(
            p.as_any_ref().downcast_ref::<ScsiLun>().unwrap(),
            serializer,
        ),
        StructType::HostScsiDisk => HostScsiDisk::serialize(
            p.as_any_ref().downcast_ref::<HostScsiDisk>().unwrap(),
            serializer,
        ),
        StructType::HostDhcpService => HostDhcpService::serialize(
            p.as_any_ref().downcast_ref::<HostDhcpService>().unwrap(),
            serializer,
        ),
        StructType::HostDhcpServiceConfig => HostDhcpServiceConfig::serialize(
            p.as_any_ref().downcast_ref::<HostDhcpServiceConfig>().unwrap(),
            serializer,
        ),
        StructType::HostDhcpServiceSpec => HostDhcpServiceSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDhcpServiceSpec>().unwrap(),
            serializer,
        ),
        StructType::HostDiagnosticPartition => HostDiagnosticPartition::serialize(
            p.as_any_ref().downcast_ref::<HostDiagnosticPartition>().unwrap(),
            serializer,
        ),
        StructType::HostDiagnosticPartitionCreateDescription => HostDiagnosticPartitionCreateDescription::serialize(
            p.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateDescription>().unwrap(),
            serializer,
        ),
        StructType::HostDiagnosticPartitionCreateOption => HostDiagnosticPartitionCreateOption::serialize(
            p.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateOption>().unwrap(),
            serializer,
        ),
        StructType::HostDiagnosticPartitionCreateSpec => HostDiagnosticPartitionCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDiagnosticPartitionCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::HostDigestInfo => HostDigestInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDigestInfo>().unwrap(),
            serializer,
        ),
        StructType::HostTpmDigestInfo => HostTpmDigestInfo::serialize(
            p.as_any_ref().downcast_ref::<HostTpmDigestInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDiskConfigurationResult => HostDiskConfigurationResult::serialize(
            p.as_any_ref().downcast_ref::<HostDiskConfigurationResult>().unwrap(),
            serializer,
        ),
        StructType::HostDiskDimensions => HostDiskDimensions::serialize(
            p.as_any_ref().downcast_ref::<HostDiskDimensions>().unwrap(),
            serializer,
        ),
        StructType::HostDiskDimensionsChs => HostDiskDimensionsChs::serialize(
            p.as_any_ref().downcast_ref::<HostDiskDimensionsChs>().unwrap(),
            serializer,
        ),
        StructType::HostDiskDimensionsLba => HostDiskDimensionsLba::serialize(
            p.as_any_ref().downcast_ref::<HostDiskDimensionsLba>().unwrap(),
            serializer,
        ),
        StructType::HostDiskPartitionInfo => HostDiskPartitionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDiskPartitionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDiskPartitionBlockRange => HostDiskPartitionBlockRange::serialize(
            p.as_any_ref().downcast_ref::<HostDiskPartitionBlockRange>().unwrap(),
            serializer,
        ),
        StructType::HostDiskPartitionLayout => HostDiskPartitionLayout::serialize(
            p.as_any_ref().downcast_ref::<HostDiskPartitionLayout>().unwrap(),
            serializer,
        ),
        StructType::HostDiskPartitionAttributes => HostDiskPartitionAttributes::serialize(
            p.as_any_ref().downcast_ref::<HostDiskPartitionAttributes>().unwrap(),
            serializer,
        ),
        StructType::HostDiskPartitionSpec => HostDiskPartitionSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDiskPartitionSpec>().unwrap(),
            serializer,
        ),
        StructType::HostDnsConfig => HostDnsConfig::serialize(
            p.as_any_ref().downcast_ref::<HostDnsConfig>().unwrap(),
            serializer,
        ),
        StructType::HostDnsConfigSpec => HostDnsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostDnsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostDvxClass => HostDvxClass::serialize(
            p.as_any_ref().downcast_ref::<HostDvxClass>().unwrap(),
            serializer,
        ),
        StructType::HostEnterMaintenanceResult => HostEnterMaintenanceResult::serialize(
            p.as_any_ref().downcast_ref::<HostEnterMaintenanceResult>().unwrap(),
            serializer,
        ),
        StructType::HostEsxAgentHostManagerConfigInfo => HostEsxAgentHostManagerConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostEsxAgentHostManagerConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFaultToleranceManagerComponentHealthInfo => HostFaultToleranceManagerComponentHealthInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFaultToleranceManagerComponentHealthInfo>().unwrap(),
            serializer,
        ),
        StructType::FcoeConfig => FcoeConfig::serialize(
            p.as_any_ref().downcast_ref::<FcoeConfig>().unwrap(),
            serializer,
        ),
        StructType::FcoeConfigFcoeCapabilities => FcoeConfigFcoeCapabilities::serialize(
            p.as_any_ref().downcast_ref::<FcoeConfigFcoeCapabilities>().unwrap(),
            serializer,
        ),
        StructType::FcoeConfigFcoeSpecification => FcoeConfigFcoeSpecification::serialize(
            p.as_any_ref().downcast_ref::<FcoeConfigFcoeSpecification>().unwrap(),
            serializer,
        ),
        StructType::FcoeConfigVlanRange => FcoeConfigVlanRange::serialize(
            p.as_any_ref().downcast_ref::<FcoeConfigVlanRange>().unwrap(),
            serializer,
        ),
        StructType::HostFeatureCapability => HostFeatureCapability::serialize(
            p.as_any_ref().downcast_ref::<HostFeatureCapability>().unwrap(),
            serializer,
        ),
        StructType::HostFeatureMask => HostFeatureMask::serialize(
            p.as_any_ref().downcast_ref::<HostFeatureMask>().unwrap(),
            serializer,
        ),
        StructType::HostFeatureVersionInfo => HostFeatureVersionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFeatureVersionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFibreChannelOverEthernetHbaLinkInfo => HostFibreChannelOverEthernetHbaLinkInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHbaLinkInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFileAccess => HostFileAccess::serialize(
            p.as_any_ref().downcast_ref::<HostFileAccess>().unwrap(),
            serializer,
        ),
        StructType::ModeInfo => ModeInfo::serialize(
            p.as_any_ref().downcast_ref::<ModeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFileSystemMountInfo => HostFileSystemMountInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFileSystemMountInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFileSystemVolume => HostFileSystemVolume::serialize(
            p.as_any_ref().downcast_ref::<HostFileSystemVolume>().unwrap(),
            serializer,
        ),
        StructType::HostLocalFileSystemVolume => HostLocalFileSystemVolume::serialize(
            p.as_any_ref().downcast_ref::<HostLocalFileSystemVolume>().unwrap(),
            serializer,
        ),
        StructType::HostNasVolume => HostNasVolume::serialize(
            p.as_any_ref().downcast_ref::<HostNasVolume>().unwrap(),
            serializer,
        ),
        StructType::HostPMemVolume => HostPMemVolume::serialize(
            p.as_any_ref().downcast_ref::<HostPMemVolume>().unwrap(),
            serializer,
        ),
        StructType::HostVfatVolume => HostVfatVolume::serialize(
            p.as_any_ref().downcast_ref::<HostVfatVolume>().unwrap(),
            serializer,
        ),
        StructType::HostVffsVolume => HostVffsVolume::serialize(
            p.as_any_ref().downcast_ref::<HostVffsVolume>().unwrap(),
            serializer,
        ),
        StructType::HostVmfsVolume => HostVmfsVolume::serialize(
            p.as_any_ref().downcast_ref::<HostVmfsVolume>().unwrap(),
            serializer,
        ),
        StructType::HostVvolVolume => HostVvolVolume::serialize(
            p.as_any_ref().downcast_ref::<HostVvolVolume>().unwrap(),
            serializer,
        ),
        StructType::HostFileSystemVolumeInfo => HostFileSystemVolumeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFileSystemVolumeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallConfig => HostFirewallConfig::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallConfig>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallConfigRuleSetConfig => HostFirewallConfigRuleSetConfig::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallConfigRuleSetConfig>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallInfo => HostFirewallInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallDefaultPolicy => HostFirewallDefaultPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallDefaultPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostFlagInfo => HostFlagInfo::serialize(
            p.as_any_ref().downcast_ref::<HostFlagInfo>().unwrap(),
            serializer,
        ),
        StructType::HostForceMountedInfo => HostForceMountedInfo::serialize(
            p.as_any_ref().downcast_ref::<HostForceMountedInfo>().unwrap(),
            serializer,
        ),
        StructType::HostFru => HostFru::serialize(
            p.as_any_ref().downcast_ref::<HostFru>().unwrap(),
            serializer,
        ),
        StructType::HostGatewaySpec => HostGatewaySpec::serialize(
            p.as_any_ref().downcast_ref::<HostGatewaySpec>().unwrap(),
            serializer,
        ),
        StructType::HostGraphicsConfig => HostGraphicsConfig::serialize(
            p.as_any_ref().downcast_ref::<HostGraphicsConfig>().unwrap(),
            serializer,
        ),
        StructType::HostGraphicsConfigDeviceType => HostGraphicsConfigDeviceType::serialize(
            p.as_any_ref().downcast_ref::<HostGraphicsConfigDeviceType>().unwrap(),
            serializer,
        ),
        StructType::HostGraphicsInfo => HostGraphicsInfo::serialize(
            p.as_any_ref().downcast_ref::<HostGraphicsInfo>().unwrap(),
            serializer,
        ),
        StructType::HostHardwareInfo => HostHardwareInfo::serialize(
            p.as_any_ref().downcast_ref::<HostHardwareInfo>().unwrap(),
            serializer,
        ),
        StructType::HostHardwareStatusInfo => HostHardwareStatusInfo::serialize(
            p.as_any_ref().downcast_ref::<HostHardwareStatusInfo>().unwrap(),
            serializer,
        ),
        StructType::DpuStatusInfoOperationalInfo => DpuStatusInfoOperationalInfo::serialize(
            p.as_any_ref().downcast_ref::<DpuStatusInfoOperationalInfo>().unwrap(),
            serializer,
        ),
        StructType::HostHardwareElementInfo => HostHardwareElementInfo::serialize(
            p.as_any_ref().downcast_ref::<HostHardwareElementInfo>().unwrap(),
            serializer,
        ),
        StructType::DpuStatusInfo => DpuStatusInfo::serialize(
            p.as_any_ref().downcast_ref::<DpuStatusInfo>().unwrap(),
            serializer,
        ),
        StructType::HostStorageElementInfo => HostStorageElementInfo::serialize(
            p.as_any_ref().downcast_ref::<HostStorageElementInfo>().unwrap(),
            serializer,
        ),
        StructType::HostStorageOperationalInfo => HostStorageOperationalInfo::serialize(
            p.as_any_ref().downcast_ref::<HostStorageOperationalInfo>().unwrap(),
            serializer,
        ),
        StructType::HostHbaCreateSpec => HostHbaCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<HostHbaCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::HostTcpHbaCreateSpec => HostTcpHbaCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<HostTcpHbaCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::HealthSystemRuntime => HealthSystemRuntime::serialize(
            p.as_any_ref().downcast_ref::<HealthSystemRuntime>().unwrap(),
            serializer,
        ),
        StructType::HostAccessControlEntry => HostAccessControlEntry::serialize(
            p.as_any_ref().downcast_ref::<HostAccessControlEntry>().unwrap(),
            serializer,
        ),
        StructType::HostHostBusAdapter => HostHostBusAdapter::serialize(
            p.as_any_ref().downcast_ref::<HostHostBusAdapter>().unwrap(),
            serializer,
        ),
        StructType::HostBlockHba => HostBlockHba::serialize(
            p.as_any_ref().downcast_ref::<HostBlockHba>().unwrap(),
            serializer,
        ),
        StructType::HostFibreChannelHba => HostFibreChannelHba::serialize(
            p.as_any_ref().downcast_ref::<HostFibreChannelHba>().unwrap(),
            serializer,
        ),
        StructType::HostFibreChannelOverEthernetHba => HostFibreChannelOverEthernetHba::serialize(
            p.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHba>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHba => HostInternetScsiHba::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHba>().unwrap(),
            serializer,
        ),
        StructType::HostParallelScsiHba => HostParallelScsiHba::serialize(
            p.as_any_ref().downcast_ref::<HostParallelScsiHba>().unwrap(),
            serializer,
        ),
        StructType::HostPcieHba => HostPcieHba::serialize(
            p.as_any_ref().downcast_ref::<HostPcieHba>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaHba => HostRdmaHba::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaHba>().unwrap(),
            serializer,
        ),
        StructType::HostSerialAttachedHba => HostSerialAttachedHba::serialize(
            p.as_any_ref().downcast_ref::<HostSerialAttachedHba>().unwrap(),
            serializer,
        ),
        StructType::HostTcpHba => HostTcpHba::serialize(
            p.as_any_ref().downcast_ref::<HostTcpHba>().unwrap(),
            serializer,
        ),
        StructType::HostProxySwitch => HostProxySwitch::serialize(
            p.as_any_ref().downcast_ref::<HostProxySwitch>().unwrap(),
            serializer,
        ),
        StructType::HostProxySwitchConfig => HostProxySwitchConfig::serialize(
            p.as_any_ref().downcast_ref::<HostProxySwitchConfig>().unwrap(),
            serializer,
        ),
        StructType::HostProxySwitchEnsInfo => HostProxySwitchEnsInfo::serialize(
            p.as_any_ref().downcast_ref::<HostProxySwitchEnsInfo>().unwrap(),
            serializer,
        ),
        StructType::HostProxySwitchHostLagConfig => HostProxySwitchHostLagConfig::serialize(
            p.as_any_ref().downcast_ref::<HostProxySwitchHostLagConfig>().unwrap(),
            serializer,
        ),
        StructType::HostProxySwitchSpec => HostProxySwitchSpec::serialize(
            p.as_any_ref().downcast_ref::<HostProxySwitchSpec>().unwrap(),
            serializer,
        ),
        StructType::HostImageProfileSummary => HostImageProfileSummary::serialize(
            p.as_any_ref().downcast_ref::<HostImageProfileSummary>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaAuthenticationCapabilities => HostInternetScsiHbaAuthenticationCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaAuthenticationCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaAuthenticationProperties => HostInternetScsiHbaAuthenticationProperties::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaAuthenticationProperties>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaDigestCapabilities => HostInternetScsiHbaDigestCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaDigestCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaDigestProperties => HostInternetScsiHbaDigestProperties::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaDigestProperties>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaDiscoveryCapabilities => HostInternetScsiHbaDiscoveryCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaDiscoveryCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaDiscoveryProperties => HostInternetScsiHbaDiscoveryProperties::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaDiscoveryProperties>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaIpCapabilities => HostInternetScsiHbaIpCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaIpCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaIpProperties => HostInternetScsiHbaIpProperties::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaIpProperties>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaIPv6Properties => HostInternetScsiHbaIPv6Properties::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaIPv6Properties>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaIscsiIpv6Address => HostInternetScsiHbaIscsiIpv6Address::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaIscsiIpv6Address>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaSendTarget => HostInternetScsiHbaSendTarget::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaSendTarget>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaStaticTarget => HostInternetScsiHbaStaticTarget::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaStaticTarget>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaTargetSet => HostInternetScsiHbaTargetSet::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaTargetSet>().unwrap(),
            serializer,
        ),
        StructType::HostIpConfig => HostIpConfig::serialize(
            p.as_any_ref().downcast_ref::<HostIpConfig>().unwrap(),
            serializer,
        ),
        StructType::HostIpConfigIpV6Address => HostIpConfigIpV6Address::serialize(
            p.as_any_ref().downcast_ref::<HostIpConfigIpV6Address>().unwrap(),
            serializer,
        ),
        StructType::HostIpConfigIpV6AddressConfiguration => HostIpConfigIpV6AddressConfiguration::serialize(
            p.as_any_ref().downcast_ref::<HostIpConfigIpV6AddressConfiguration>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteConfig => HostIpRouteConfig::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteConfig>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteConfigSpec => HostIpRouteConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteEntry => HostIpRouteEntry::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteEntry>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteOp => HostIpRouteOp::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteOp>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteTableConfig => HostIpRouteTableConfig::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteTableConfig>().unwrap(),
            serializer,
        ),
        StructType::HostIpRouteTableInfo => HostIpRouteTableInfo::serialize(
            p.as_any_ref().downcast_ref::<HostIpRouteTableInfo>().unwrap(),
            serializer,
        ),
        StructType::HostIpmiInfo => HostIpmiInfo::serialize(
            p.as_any_ref().downcast_ref::<HostIpmiInfo>().unwrap(),
            serializer,
        ),
        StructType::IscsiDependencyEntity => IscsiDependencyEntity::serialize(
            p.as_any_ref().downcast_ref::<IscsiDependencyEntity>().unwrap(),
            serializer,
        ),
        StructType::IscsiMigrationDependency => IscsiMigrationDependency::serialize(
            p.as_any_ref().downcast_ref::<IscsiMigrationDependency>().unwrap(),
            serializer,
        ),
        StructType::IscsiPortInfo => IscsiPortInfo::serialize(
            p.as_any_ref().downcast_ref::<IscsiPortInfo>().unwrap(),
            serializer,
        ),
        StructType::IscsiStatus => IscsiStatus::serialize(
            p.as_any_ref().downcast_ref::<IscsiStatus>().unwrap(),
            serializer,
        ),
        StructType::KernelModuleInfo => KernelModuleInfo::serialize(
            p.as_any_ref().downcast_ref::<KernelModuleInfo>().unwrap(),
            serializer,
        ),
        StructType::KernelModuleSectionInfo => KernelModuleSectionInfo::serialize(
            p.as_any_ref().downcast_ref::<KernelModuleSectionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostLicenseSpec => HostLicenseSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLicenseSpec>().unwrap(),
            serializer,
        ),
        StructType::LinkDiscoveryProtocolConfig => LinkDiscoveryProtocolConfig::serialize(
            p.as_any_ref().downcast_ref::<LinkDiscoveryProtocolConfig>().unwrap(),
            serializer,
        ),
        StructType::HostAccountSpec => HostAccountSpec::serialize(
            p.as_any_ref().downcast_ref::<HostAccountSpec>().unwrap(),
            serializer,
        ),
        StructType::HostPosixAccountSpec => HostPosixAccountSpec::serialize(
            p.as_any_ref().downcast_ref::<HostPosixAccountSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLocalFileSystemVolumeSpec => HostLocalFileSystemVolumeSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLocalFileSystemVolumeSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerDiskLayoutSpec => HostLowLevelProvisioningManagerDiskLayoutSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerDiskLayoutSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerFileDeleteResult => HostLowLevelProvisioningManagerFileDeleteResult::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileDeleteResult>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerFileDeleteSpec => HostLowLevelProvisioningManagerFileDeleteSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileDeleteSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerFileReserveResult => HostLowLevelProvisioningManagerFileReserveResult::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileReserveResult>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerFileReserveSpec => HostLowLevelProvisioningManagerFileReserveSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerFileReserveSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerSnapshotLayoutSpec => HostLowLevelProvisioningManagerSnapshotLayoutSpec::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerSnapshotLayoutSpec>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerVmMigrationStatus => HostLowLevelProvisioningManagerVmMigrationStatus::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerVmMigrationStatus>().unwrap(),
            serializer,
        ),
        StructType::HostLowLevelProvisioningManagerVmRecoveryInfo => HostLowLevelProvisioningManagerVmRecoveryInfo::serialize(
            p.as_any_ref().downcast_ref::<HostLowLevelProvisioningManagerVmRecoveryInfo>().unwrap(),
            serializer,
        ),
        StructType::HostMaintenanceSpec => HostMaintenanceSpec::serialize(
            p.as_any_ref().downcast_ref::<HostMaintenanceSpec>().unwrap(),
            serializer,
        ),
        StructType::ServiceConsoleReservationInfo => ServiceConsoleReservationInfo::serialize(
            p.as_any_ref().downcast_ref::<ServiceConsoleReservationInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMemoryReservationInfo => VirtualMachineMemoryReservationInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMemoryReservationInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMemoryReservationSpec => VirtualMachineMemoryReservationSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMemoryReservationSpec>().unwrap(),
            serializer,
        ),
        StructType::HostMemorySpec => HostMemorySpec::serialize(
            p.as_any_ref().downcast_ref::<HostMemorySpec>().unwrap(),
            serializer,
        ),
        StructType::HostMemoryTierInfo => HostMemoryTierInfo::serialize(
            p.as_any_ref().downcast_ref::<HostMemoryTierInfo>().unwrap(),
            serializer,
        ),
        StructType::HostMountInfo => HostMountInfo::serialize(
            p.as_any_ref().downcast_ref::<HostMountInfo>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfo => HostMultipathInfo::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfo>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoLogicalUnit => HostMultipathInfoLogicalUnit::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnit>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoLogicalUnitPolicy => HostMultipathInfoLogicalUnitPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnitPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoFixedLogicalUnitPolicy => HostMultipathInfoFixedLogicalUnitPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoFixedLogicalUnitPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoHppLogicalUnitPolicy => HostMultipathInfoHppLogicalUnitPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoHppLogicalUnitPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoLogicalUnitStorageArrayTypePolicy => HostMultipathInfoLogicalUnitStorageArrayTypePolicy::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoLogicalUnitStorageArrayTypePolicy>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathInfoPath => HostMultipathInfoPath::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathInfoPath>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathStateInfo => HostMultipathStateInfo::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathStateInfo>().unwrap(),
            serializer,
        ),
        StructType::HostMultipathStateInfoPath => HostMultipathStateInfoPath::serialize(
            p.as_any_ref().downcast_ref::<HostMultipathStateInfoPath>().unwrap(),
            serializer,
        ),
        StructType::HostNasVolumeConfig => HostNasVolumeConfig::serialize(
            p.as_any_ref().downcast_ref::<HostNasVolumeConfig>().unwrap(),
            serializer,
        ),
        StructType::HostNasVolumeSpec => HostNasVolumeSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNasVolumeSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNasVolumeUserInfo => HostNasVolumeUserInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNasVolumeUserInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNatService => HostNatService::serialize(
            p.as_any_ref().downcast_ref::<HostNatService>().unwrap(),
            serializer,
        ),
        StructType::HostNatServiceConfig => HostNatServiceConfig::serialize(
            p.as_any_ref().downcast_ref::<HostNatServiceConfig>().unwrap(),
            serializer,
        ),
        StructType::HostNatServiceNameServiceSpec => HostNatServiceNameServiceSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNatServiceNameServiceSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNatServicePortForwardSpec => HostNatServicePortForwardSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNatServicePortForwardSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNatServiceSpec => HostNatServiceSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNatServiceSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNetCapabilities => HostNetCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostNetCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostNetOffloadCapabilities => HostNetOffloadCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostNetOffloadCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostNetStackInstance => HostNetStackInstance::serialize(
            p.as_any_ref().downcast_ref::<HostNetStackInstance>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkConfig => HostNetworkConfig::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkConfig>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkConfigNetStackSpec => HostNetworkConfigNetStackSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkConfigNetStackSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkConfigResult => HostNetworkConfigResult::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkConfigResult>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkInfo => HostNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkPolicy => HostNetworkPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostNicFailureCriteria => HostNicFailureCriteria::serialize(
            p.as_any_ref().downcast_ref::<HostNicFailureCriteria>().unwrap(),
            serializer,
        ),
        StructType::HostNicOrderPolicy => HostNicOrderPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostNicOrderPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostNicTeamingPolicy => HostNicTeamingPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostNicTeamingPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkSecurityPolicy => HostNetworkSecurityPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkSecurityPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkTrafficShapingPolicy => HostNetworkTrafficShapingPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkTrafficShapingPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostNtpConfig => HostNtpConfig::serialize(
            p.as_any_ref().downcast_ref::<HostNtpConfig>().unwrap(),
            serializer,
        ),
        StructType::HostNumaInfo => HostNumaInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNumaInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNumaNode => HostNumaNode::serialize(
            p.as_any_ref().downcast_ref::<HostNumaNode>().unwrap(),
            serializer,
        ),
        StructType::HostNumericSensorInfo => HostNumericSensorInfo::serialize(
            p.as_any_ref().downcast_ref::<HostNumericSensorInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmDimmInfo => NvdimmDimmInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmDimmInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmGuid => NvdimmGuid::serialize(
            p.as_any_ref().downcast_ref::<NvdimmGuid>().unwrap(),
            serializer,
        ),
        StructType::NvdimmHealthInfo => NvdimmHealthInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmHealthInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmInterleaveSetInfo => NvdimmInterleaveSetInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmInterleaveSetInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmNamespaceCreateSpec => NvdimmNamespaceCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<NvdimmNamespaceCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::NvdimmNamespaceDeleteSpec => NvdimmNamespaceDeleteSpec::serialize(
            p.as_any_ref().downcast_ref::<NvdimmNamespaceDeleteSpec>().unwrap(),
            serializer,
        ),
        StructType::NvdimmNamespaceDetails => NvdimmNamespaceDetails::serialize(
            p.as_any_ref().downcast_ref::<NvdimmNamespaceDetails>().unwrap(),
            serializer,
        ),
        StructType::NvdimmNamespaceInfo => NvdimmNamespaceInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmNamespaceInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmSystemInfo => NvdimmSystemInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmSystemInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmPMemNamespaceCreateSpec => NvdimmPMemNamespaceCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<NvdimmPMemNamespaceCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::NvdimmRegionInfo => NvdimmRegionInfo::serialize(
            p.as_any_ref().downcast_ref::<NvdimmRegionInfo>().unwrap(),
            serializer,
        ),
        StructType::NvdimmSummary => NvdimmSummary::serialize(
            p.as_any_ref().downcast_ref::<NvdimmSummary>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeController => HostNvmeController::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeController>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeDisconnectSpec => HostNvmeDisconnectSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeDisconnectSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeDiscoveryLog => HostNvmeDiscoveryLog::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeDiscoveryLog>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeDiscoveryLogEntry => HostNvmeDiscoveryLogEntry::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeDiscoveryLogEntry>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeNamespace => HostNvmeNamespace::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeNamespace>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeSpec => HostNvmeSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeConnectSpec => HostNvmeConnectSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeConnectSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeDiscoverSpec => HostNvmeDiscoverSpec::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeDiscoverSpec>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeTopology => HostNvmeTopology::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeTopology>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeTopologyInterface => HostNvmeTopologyInterface::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeTopologyInterface>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeTransportParameters => HostNvmeTransportParameters::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeTransportParameters>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeOpaqueTransportParameters => HostNvmeOpaqueTransportParameters::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeOpaqueTransportParameters>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeOverFibreChannelParameters => HostNvmeOverFibreChannelParameters::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeOverFibreChannelParameters>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeOverRdmaParameters => HostNvmeOverRdmaParameters::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeOverRdmaParameters>().unwrap(),
            serializer,
        ),
        StructType::HostNvmeOverTcpParameters => HostNvmeOverTcpParameters::serialize(
            p.as_any_ref().downcast_ref::<HostNvmeOverTcpParameters>().unwrap(),
            serializer,
        ),
        StructType::HostOpaqueNetworkInfo => HostOpaqueNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<HostOpaqueNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::HostOpaqueSwitch => HostOpaqueSwitch::serialize(
            p.as_any_ref().downcast_ref::<HostOpaqueSwitch>().unwrap(),
            serializer,
        ),
        StructType::HostOpaqueSwitchPhysicalNicZone => HostOpaqueSwitchPhysicalNicZone::serialize(
            p.as_any_ref().downcast_ref::<HostOpaqueSwitchPhysicalNicZone>().unwrap(),
            serializer,
        ),
        StructType::HostPatchManagerLocator => HostPatchManagerLocator::serialize(
            p.as_any_ref().downcast_ref::<HostPatchManagerLocator>().unwrap(),
            serializer,
        ),
        StructType::HostPatchManagerPatchManagerOperationSpec => HostPatchManagerPatchManagerOperationSpec::serialize(
            p.as_any_ref().downcast_ref::<HostPatchManagerPatchManagerOperationSpec>().unwrap(),
            serializer,
        ),
        StructType::HostPatchManagerResult => HostPatchManagerResult::serialize(
            p.as_any_ref().downcast_ref::<HostPatchManagerResult>().unwrap(),
            serializer,
        ),
        StructType::HostPatchManagerStatus => HostPatchManagerStatus::serialize(
            p.as_any_ref().downcast_ref::<HostPatchManagerStatus>().unwrap(),
            serializer,
        ),
        StructType::HostPatchManagerStatusPrerequisitePatch => HostPatchManagerStatusPrerequisitePatch::serialize(
            p.as_any_ref().downcast_ref::<HostPatchManagerStatusPrerequisitePatch>().unwrap(),
            serializer,
        ),
        StructType::HostPathSelectionPolicyOption => HostPathSelectionPolicyOption::serialize(
            p.as_any_ref().downcast_ref::<HostPathSelectionPolicyOption>().unwrap(),
            serializer,
        ),
        StructType::HostPciDevice => HostPciDevice::serialize(
            p.as_any_ref().downcast_ref::<HostPciDevice>().unwrap(),
            serializer,
        ),
        StructType::HostPciPassthruConfig => HostPciPassthruConfig::serialize(
            p.as_any_ref().downcast_ref::<HostPciPassthruConfig>().unwrap(),
            serializer,
        ),
        StructType::HostSriovConfig => HostSriovConfig::serialize(
            p.as_any_ref().downcast_ref::<HostSriovConfig>().unwrap(),
            serializer,
        ),
        StructType::HostPciPassthruInfo => HostPciPassthruInfo::serialize(
            p.as_any_ref().downcast_ref::<HostPciPassthruInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSriovInfo => HostSriovInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSriovInfo>().unwrap(),
            serializer,
        ),
        StructType::HostPersistentMemoryInfo => HostPersistentMemoryInfo::serialize(
            p.as_any_ref().downcast_ref::<HostPersistentMemoryInfo>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNic => PhysicalNic::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNic>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicCdpDeviceCapability => PhysicalNicCdpDeviceCapability::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicCdpDeviceCapability>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicCdpInfo => PhysicalNicCdpInfo::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicCdpInfo>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicConfig => PhysicalNicConfig::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicConfig>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicLinkInfo => PhysicalNicLinkInfo::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicLinkInfo>().unwrap(),
            serializer,
        ),
        StructType::LinkLayerDiscoveryProtocolInfo => LinkLayerDiscoveryProtocolInfo::serialize(
            p.as_any_ref().downcast_ref::<LinkLayerDiscoveryProtocolInfo>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicHintInfo => PhysicalNicHintInfo::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicHintInfo>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicHint => PhysicalNicHint::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicHint>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicIpHint => PhysicalNicIpHint::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicIpHint>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicNameHint => PhysicalNicNameHint::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicNameHint>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicSpec => PhysicalNicSpec::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicSpec>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopology => HostPlugStoreTopology::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopology>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopologyAdapter => HostPlugStoreTopologyAdapter::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopologyAdapter>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopologyDevice => HostPlugStoreTopologyDevice::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopologyDevice>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopologyPath => HostPlugStoreTopologyPath::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopologyPath>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopologyPlugin => HostPlugStoreTopologyPlugin::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopologyPlugin>().unwrap(),
            serializer,
        ),
        StructType::HostPlugStoreTopologyTarget => HostPlugStoreTopologyTarget::serialize(
            p.as_any_ref().downcast_ref::<HostPlugStoreTopologyTarget>().unwrap(),
            serializer,
        ),
        StructType::HostPortGroup => HostPortGroup::serialize(
            p.as_any_ref().downcast_ref::<HostPortGroup>().unwrap(),
            serializer,
        ),
        StructType::HostPortGroupConfig => HostPortGroupConfig::serialize(
            p.as_any_ref().downcast_ref::<HostPortGroupConfig>().unwrap(),
            serializer,
        ),
        StructType::HostPortGroupPort => HostPortGroupPort::serialize(
            p.as_any_ref().downcast_ref::<HostPortGroupPort>().unwrap(),
            serializer,
        ),
        StructType::HostPortGroupSpec => HostPortGroupSpec::serialize(
            p.as_any_ref().downcast_ref::<HostPortGroupSpec>().unwrap(),
            serializer,
        ),
        StructType::PowerSystemCapability => PowerSystemCapability::serialize(
            p.as_any_ref().downcast_ref::<PowerSystemCapability>().unwrap(),
            serializer,
        ),
        StructType::PowerSystemInfo => PowerSystemInfo::serialize(
            p.as_any_ref().downcast_ref::<PowerSystemInfo>().unwrap(),
            serializer,
        ),
        StructType::HostPowerPolicy => HostPowerPolicy::serialize(
            p.as_any_ref().downcast_ref::<HostPowerPolicy>().unwrap(),
            serializer,
        ),
        StructType::HostProtocolEndpoint => HostProtocolEndpoint::serialize(
            p.as_any_ref().downcast_ref::<HostProtocolEndpoint>().unwrap(),
            serializer,
        ),
        StructType::HostPtpConfig => HostPtpConfig::serialize(
            p.as_any_ref().downcast_ref::<HostPtpConfig>().unwrap(),
            serializer,
        ),
        StructType::HostPtpConfigPtpPort => HostPtpConfigPtpPort::serialize(
            p.as_any_ref().downcast_ref::<HostPtpConfigPtpPort>().unwrap(),
            serializer,
        ),
        StructType::HostQualifiedName => HostQualifiedName::serialize(
            p.as_any_ref().downcast_ref::<HostQualifiedName>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaDevice => HostRdmaDevice::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaDevice>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaDeviceBacking => HostRdmaDeviceBacking::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaDeviceBacking>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaDevicePnicBacking => HostRdmaDevicePnicBacking::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaDevicePnicBacking>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaDeviceCapability => HostRdmaDeviceCapability::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaDeviceCapability>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaDeviceConnectionInfo => HostRdmaDeviceConnectionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaDeviceConnectionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostReliableMemoryInfo => HostReliableMemoryInfo::serialize(
            p.as_any_ref().downcast_ref::<HostReliableMemoryInfo>().unwrap(),
            serializer,
        ),
        StructType::HostResignatureRescanResult => HostResignatureRescanResult::serialize(
            p.as_any_ref().downcast_ref::<HostResignatureRescanResult>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallRuleset => HostFirewallRuleset::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallRuleset>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallRulesetIpList => HostFirewallRulesetIpList::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallRulesetIpList>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallRulesetIpNetwork => HostFirewallRulesetIpNetwork::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallRulesetIpNetwork>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallRule => HostFirewallRule::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallRule>().unwrap(),
            serializer,
        ),
        StructType::HostFirewallRulesetRulesetSpec => HostFirewallRulesetRulesetSpec::serialize(
            p.as_any_ref().downcast_ref::<HostFirewallRulesetRulesetSpec>().unwrap(),
            serializer,
        ),
        StructType::HostRuntimeInfo => HostRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostRuntimeInfoNetStackInstanceRuntimeInfo => HostRuntimeInfoNetStackInstanceRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostRuntimeInfoNetStackInstanceRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostNetworkResourceRuntime => HostNetworkResourceRuntime::serialize(
            p.as_any_ref().downcast_ref::<HostNetworkResourceRuntime>().unwrap(),
            serializer,
        ),
        StructType::HostRuntimeInfoNetworkRuntimeInfo => HostRuntimeInfoNetworkRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostRuntimeInfoNetworkRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostPlacedVirtualNicIdentifier => HostPlacedVirtualNicIdentifier::serialize(
            p.as_any_ref().downcast_ref::<HostPlacedVirtualNicIdentifier>().unwrap(),
            serializer,
        ),
        StructType::HostPnicNetworkResourceInfo => HostPnicNetworkResourceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostPnicNetworkResourceInfo>().unwrap(),
            serializer,
        ),
        StructType::HostRuntimeInfoStateEncryptionInfo => HostRuntimeInfoStateEncryptionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostRuntimeInfoStateEncryptionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostScsiDiskPartition => HostScsiDiskPartition::serialize(
            p.as_any_ref().downcast_ref::<HostScsiDiskPartition>().unwrap(),
            serializer,
        ),
        StructType::ScsiLunCapabilities => ScsiLunCapabilities::serialize(
            p.as_any_ref().downcast_ref::<ScsiLunCapabilities>().unwrap(),
            serializer,
        ),
        StructType::ScsiLunDescriptor => ScsiLunDescriptor::serialize(
            p.as_any_ref().downcast_ref::<ScsiLunDescriptor>().unwrap(),
            serializer,
        ),
        StructType::ScsiLunDurableName => ScsiLunDurableName::serialize(
            p.as_any_ref().downcast_ref::<ScsiLunDurableName>().unwrap(),
            serializer,
        ),
        StructType::HostScsiTopology => HostScsiTopology::serialize(
            p.as_any_ref().downcast_ref::<HostScsiTopology>().unwrap(),
            serializer,
        ),
        StructType::HostScsiTopologyInterface => HostScsiTopologyInterface::serialize(
            p.as_any_ref().downcast_ref::<HostScsiTopologyInterface>().unwrap(),
            serializer,
        ),
        StructType::HostScsiTopologyLun => HostScsiTopologyLun::serialize(
            p.as_any_ref().downcast_ref::<HostScsiTopologyLun>().unwrap(),
            serializer,
        ),
        StructType::HostScsiTopologyTarget => HostScsiTopologyTarget::serialize(
            p.as_any_ref().downcast_ref::<HostScsiTopologyTarget>().unwrap(),
            serializer,
        ),
        StructType::HostSecuritySpec => HostSecuritySpec::serialize(
            p.as_any_ref().downcast_ref::<HostSecuritySpec>().unwrap(),
            serializer,
        ),
        StructType::HostService => HostService::serialize(
            p.as_any_ref().downcast_ref::<HostService>().unwrap(),
            serializer,
        ),
        StructType::HostServiceSourcePackage => HostServiceSourcePackage::serialize(
            p.as_any_ref().downcast_ref::<HostServiceSourcePackage>().unwrap(),
            serializer,
        ),
        StructType::HostServiceConfig => HostServiceConfig::serialize(
            p.as_any_ref().downcast_ref::<HostServiceConfig>().unwrap(),
            serializer,
        ),
        StructType::HostServiceInfo => HostServiceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostServiceInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSevInfo => HostSevInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSevInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSgxInfo => HostSgxInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSgxInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSgxRegistrationInfo => HostSgxRegistrationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSgxRegistrationInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSharedGpuCapabilities => HostSharedGpuCapabilities::serialize(
            p.as_any_ref().downcast_ref::<HostSharedGpuCapabilities>().unwrap(),
            serializer,
        ),
        StructType::HostSnmpSystemAgentLimits => HostSnmpSystemAgentLimits::serialize(
            p.as_any_ref().downcast_ref::<HostSnmpSystemAgentLimits>().unwrap(),
            serializer,
        ),
        StructType::HostSnmpConfigSpec => HostSnmpConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostSnmpConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostSnmpDestination => HostSnmpDestination::serialize(
            p.as_any_ref().downcast_ref::<HostSnmpDestination>().unwrap(),
            serializer,
        ),
        StructType::SoftwarePackage => SoftwarePackage::serialize(
            p.as_any_ref().downcast_ref::<SoftwarePackage>().unwrap(),
            serializer,
        ),
        StructType::SoftwarePackageCapability => SoftwarePackageCapability::serialize(
            p.as_any_ref().downcast_ref::<SoftwarePackageCapability>().unwrap(),
            serializer,
        ),
        StructType::Relation => Relation::serialize(
            p.as_any_ref().downcast_ref::<Relation>().unwrap(),
            serializer,
        ),
        StructType::HostSriovDevicePoolInfo => HostSriovDevicePoolInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSriovDevicePoolInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSriovNetworkDevicePoolInfo => HostSriovNetworkDevicePoolInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSriovNetworkDevicePoolInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSslThumbprintInfo => HostSslThumbprintInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSslThumbprintInfo>().unwrap(),
            serializer,
        ),
        StructType::HostStorageArrayTypePolicyOption => HostStorageArrayTypePolicyOption::serialize(
            p.as_any_ref().downcast_ref::<HostStorageArrayTypePolicyOption>().unwrap(),
            serializer,
        ),
        StructType::HostStorageDeviceInfo => HostStorageDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostStorageDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::HostStorageSystemDiskLocatorLedResult => HostStorageSystemDiskLocatorLedResult::serialize(
            p.as_any_ref().downcast_ref::<HostStorageSystemDiskLocatorLedResult>().unwrap(),
            serializer,
        ),
        StructType::HostStorageSystemScsiLunResult => HostStorageSystemScsiLunResult::serialize(
            p.as_any_ref().downcast_ref::<HostStorageSystemScsiLunResult>().unwrap(),
            serializer,
        ),
        StructType::HostStorageSystemVmfsVolumeResult => HostStorageSystemVmfsVolumeResult::serialize(
            p.as_any_ref().downcast_ref::<HostStorageSystemVmfsVolumeResult>().unwrap(),
            serializer,
        ),
        StructType::HostListSummary => HostListSummary::serialize(
            p.as_any_ref().downcast_ref::<HostListSummary>().unwrap(),
            serializer,
        ),
        StructType::HostConfigSummary => HostConfigSummary::serialize(
            p.as_any_ref().downcast_ref::<HostConfigSummary>().unwrap(),
            serializer,
        ),
        StructType::HostListSummaryGatewaySummary => HostListSummaryGatewaySummary::serialize(
            p.as_any_ref().downcast_ref::<HostListSummaryGatewaySummary>().unwrap(),
            serializer,
        ),
        StructType::HostHardwareSummary => HostHardwareSummary::serialize(
            p.as_any_ref().downcast_ref::<HostHardwareSummary>().unwrap(),
            serializer,
        ),
        StructType::HostListSummaryQuickStats => HostListSummaryQuickStats::serialize(
            p.as_any_ref().downcast_ref::<HostListSummaryQuickStats>().unwrap(),
            serializer,
        ),
        StructType::SystemEventInfo => SystemEventInfo::serialize(
            p.as_any_ref().downcast_ref::<SystemEventInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSystemHealthInfo => HostSystemHealthInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSystemHealthInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSystemIdentificationInfo => HostSystemIdentificationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSystemIdentificationInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSystemInfo => HostSystemInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSystemInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSystemResourceInfo => HostSystemResourceInfo::serialize(
            p.as_any_ref().downcast_ref::<HostSystemResourceInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfiguration => HostSystemSwapConfiguration::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfiguration>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfigurationSystemSwapOption => HostSystemSwapConfigurationSystemSwapOption::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfigurationSystemSwapOption>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfigurationDatastoreOption => HostSystemSwapConfigurationDatastoreOption::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDatastoreOption>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfigurationDisabledOption => HostSystemSwapConfigurationDisabledOption::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfigurationDisabledOption>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfigurationHostCacheOption => HostSystemSwapConfigurationHostCacheOption::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostCacheOption>().unwrap(),
            serializer,
        ),
        StructType::HostSystemSwapConfigurationHostLocalSwapOption => HostSystemSwapConfigurationHostLocalSwapOption::serialize(
            p.as_any_ref().downcast_ref::<HostSystemSwapConfigurationHostLocalSwapOption>().unwrap(),
            serializer,
        ),
        StructType::HostTargetTransport => HostTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostBlockAdapterTargetTransport => HostBlockAdapterTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostBlockAdapterTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostFibreChannelTargetTransport => HostFibreChannelTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostFibreChannelTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostFibreChannelOverEthernetTargetTransport => HostFibreChannelOverEthernetTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiTargetTransport => HostInternetScsiTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostParallelScsiTargetTransport => HostParallelScsiTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostParallelScsiTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostPcieTargetTransport => HostPcieTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostPcieTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostRdmaTargetTransport => HostRdmaTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostRdmaTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostSerialAttachedTargetTransport => HostSerialAttachedTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostSerialAttachedTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostTcpTargetTransport => HostTcpTargetTransport::serialize(
            p.as_any_ref().downcast_ref::<HostTcpTargetTransport>().unwrap(),
            serializer,
        ),
        StructType::HostTpmAttestationInfo => HostTpmAttestationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostTpmAttestationInfo>().unwrap(),
            serializer,
        ),
        StructType::HostTpmAttestationReport => HostTpmAttestationReport::serialize(
            p.as_any_ref().downcast_ref::<HostTpmAttestationReport>().unwrap(),
            serializer,
        ),
        StructType::HostTpmEventDetails => HostTpmEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmBootCompleteEventDetails => HostTpmBootCompleteEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmBootCompleteEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmBootSecurityOptionEventDetails => HostTpmBootSecurityOptionEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmBootSecurityOptionEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmNvTagEventDetails => HostTpmNvTagEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmNvTagEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmSignerEventDetails => HostTpmSignerEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmSignerEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmCommandEventDetails => HostTpmCommandEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmCommandEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmOptionEventDetails => HostTpmOptionEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmOptionEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmSoftwareComponentEventDetails => HostTpmSoftwareComponentEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmSoftwareComponentEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmVersionEventDetails => HostTpmVersionEventDetails::serialize(
            p.as_any_ref().downcast_ref::<HostTpmVersionEventDetails>().unwrap(),
            serializer,
        ),
        StructType::HostTpmEventLogEntry => HostTpmEventLogEntry::serialize(
            p.as_any_ref().downcast_ref::<HostTpmEventLogEntry>().unwrap(),
            serializer,
        ),
        StructType::HostTrustAuthorityAttestationInfo => HostTrustAuthorityAttestationInfo::serialize(
            p.as_any_ref().downcast_ref::<HostTrustAuthorityAttestationInfo>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsExtent => HostUnresolvedVmfsExtent::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsExtent>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsResignatureSpec => HostUnresolvedVmfsResignatureSpec::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsResignatureSpec>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsResolutionResult => HostUnresolvedVmfsResolutionResult::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsResolutionResult>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsResolutionSpec => HostUnresolvedVmfsResolutionSpec::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsResolutionSpec>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsVolume => HostUnresolvedVmfsVolume::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsVolume>().unwrap(),
            serializer,
        ),
        StructType::HostUnresolvedVmfsVolumeResolveStatus => HostUnresolvedVmfsVolumeResolveStatus::serialize(
            p.as_any_ref().downcast_ref::<HostUnresolvedVmfsVolumeResolveStatus>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashCacheConfigInfo => HostVFlashManagerVFlashCacheConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption => HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashCacheConfigSpec => HostVFlashManagerVFlashCacheConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashCacheConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashConfigInfo => HostVFlashManagerVFlashConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashResourceConfigInfo => HostVFlashManagerVFlashResourceConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashResourceConfigSpec => HostVFlashManagerVFlashResourceConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashManagerVFlashResourceRunTimeInfo => HostVFlashManagerVFlashResourceRunTimeInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashManagerVFlashResourceRunTimeInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVFlashResourceConfigurationResult => HostVFlashResourceConfigurationResult::serialize(
            p.as_any_ref().downcast_ref::<HostVFlashResourceConfigurationResult>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionConfig => HostVMotionConfig::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionInfo => HostVMotionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionManagerDstInstantCloneResult => HostVMotionManagerDstInstantCloneResult::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionManagerDstInstantCloneResult>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionManagerSrcInstantCloneResult => HostVMotionManagerSrcInstantCloneResult::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionManagerSrcInstantCloneResult>().unwrap(),
            serializer,
        ),
        StructType::HostVMotionNetConfig => HostVMotionNetConfig::serialize(
            p.as_any_ref().downcast_ref::<HostVMotionNetConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVffsSpec => HostVffsSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVffsSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNic => HostVirtualNic::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNic>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicConfig => HostVirtualNicConfig::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicIpRouteSpec => HostVirtualNicIpRouteSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicIpRouteSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicOpaqueNetworkSpec => HostVirtualNicOpaqueNetworkSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicOpaqueNetworkSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicSpec => HostVirtualNicSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicConnection => HostVirtualNicConnection::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicConnection>().unwrap(),
            serializer,
        ),
        StructType::VirtualNicManagerNetConfig => VirtualNicManagerNetConfig::serialize(
            p.as_any_ref().downcast_ref::<VirtualNicManagerNetConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicManagerNicTypeSelection => HostVirtualNicManagerNicTypeSelection::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicManagerNicTypeSelection>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualNicManagerInfo => HostVirtualNicManagerInfo::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualNicManagerInfo>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitch => HostVirtualSwitch::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitch>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchBeaconConfig => HostVirtualSwitchBeaconConfig::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchBeaconConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchBridge => HostVirtualSwitchBridge::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchBridge>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchAutoBridge => HostVirtualSwitchAutoBridge::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchAutoBridge>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchBondBridge => HostVirtualSwitchBondBridge::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchBondBridge>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchSimpleBridge => HostVirtualSwitchSimpleBridge::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchSimpleBridge>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchConfig => HostVirtualSwitchConfig::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchConfig>().unwrap(),
            serializer,
        ),
        StructType::HostVirtualSwitchSpec => HostVirtualSwitchSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVirtualSwitchSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVmciAccessManagerAccessSpec => HostVmciAccessManagerAccessSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVmciAccessManagerAccessSpec>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreOption => VmfsDatastoreOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreOption>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreBaseOption => VmfsDatastoreBaseOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreBaseOption>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreMultipleExtentOption => VmfsDatastoreMultipleExtentOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreMultipleExtentOption>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreSingleExtentOption => VmfsDatastoreSingleExtentOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreSingleExtentOption>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreAllExtentOption => VmfsDatastoreAllExtentOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreAllExtentOption>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreSpec => VmfsDatastoreSpec::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreSpec>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreCreateSpec => VmfsDatastoreCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreExpandSpec => VmfsDatastoreExpandSpec::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreExpandSpec>().unwrap(),
            serializer,
        ),
        StructType::VmfsDatastoreExtendSpec => VmfsDatastoreExtendSpec::serialize(
            p.as_any_ref().downcast_ref::<VmfsDatastoreExtendSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVmfsRescanResult => HostVmfsRescanResult::serialize(
            p.as_any_ref().downcast_ref::<HostVmfsRescanResult>().unwrap(),
            serializer,
        ),
        StructType::VmfsConfigOption => VmfsConfigOption::serialize(
            p.as_any_ref().downcast_ref::<VmfsConfigOption>().unwrap(),
            serializer,
        ),
        StructType::HostVmfsSpec => HostVmfsSpec::serialize(
            p.as_any_ref().downcast_ref::<HostVmfsSpec>().unwrap(),
            serializer,
        ),
        StructType::VmfsUnmapBandwidthSpec => VmfsUnmapBandwidthSpec::serialize(
            p.as_any_ref().downcast_ref::<VmfsUnmapBandwidthSpec>().unwrap(),
            serializer,
        ),
        StructType::HostVsanInternalSystemCmmdsQuery => HostVsanInternalSystemCmmdsQuery::serialize(
            p.as_any_ref().downcast_ref::<HostVsanInternalSystemCmmdsQuery>().unwrap(),
            serializer,
        ),
        StructType::HostVsanInternalSystemDeleteVsanObjectsResult => HostVsanInternalSystemDeleteVsanObjectsResult::serialize(
            p.as_any_ref().downcast_ref::<HostVsanInternalSystemDeleteVsanObjectsResult>().unwrap(),
            serializer,
        ),
        StructType::VsanNewPolicyBatch => VsanNewPolicyBatch::serialize(
            p.as_any_ref().downcast_ref::<VsanNewPolicyBatch>().unwrap(),
            serializer,
        ),
        StructType::VsanPolicyChangeBatch => VsanPolicyChangeBatch::serialize(
            p.as_any_ref().downcast_ref::<VsanPolicyChangeBatch>().unwrap(),
            serializer,
        ),
        StructType::VsanPolicyCost => VsanPolicyCost::serialize(
            p.as_any_ref().downcast_ref::<VsanPolicyCost>().unwrap(),
            serializer,
        ),
        StructType::VsanPolicySatisfiability => VsanPolicySatisfiability::serialize(
            p.as_any_ref().downcast_ref::<VsanPolicySatisfiability>().unwrap(),
            serializer,
        ),
        StructType::HostVsanInternalSystemVsanObjectOperationResult => HostVsanInternalSystemVsanObjectOperationResult::serialize(
            p.as_any_ref().downcast_ref::<HostVsanInternalSystemVsanObjectOperationResult>().unwrap(),
            serializer,
        ),
        StructType::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult => HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult::serialize(
            p.as_any_ref().downcast_ref::<HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>().unwrap(),
            serializer,
        ),
        StructType::HostVvolNqn => HostVvolNqn::serialize(
            p.as_any_ref().downcast_ref::<HostVvolNqn>().unwrap(),
            serializer,
        ),
        StructType::VVolHostPe => VVolHostPe::serialize(
            p.as_any_ref().downcast_ref::<VVolHostPe>().unwrap(),
            serializer,
        ),
        StructType::HostVvolVolumeHostVvolNqn => HostVvolVolumeHostVvolNqn::serialize(
            p.as_any_ref().downcast_ref::<HostVvolVolumeHostVvolNqn>().unwrap(),
            serializer,
        ),
        StructType::HostVvolVolumeSpecification => HostVvolVolumeSpecification::serialize(
            p.as_any_ref().downcast_ref::<HostVvolVolumeSpecification>().unwrap(),
            serializer,
        ),
        StructType::NetDhcpConfigInfo => NetDhcpConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<NetDhcpConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::NetDhcpConfigInfoDhcpOptions => NetDhcpConfigInfoDhcpOptions::serialize(
            p.as_any_ref().downcast_ref::<NetDhcpConfigInfoDhcpOptions>().unwrap(),
            serializer,
        ),
        StructType::NetDhcpConfigSpec => NetDhcpConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<NetDhcpConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::NetDhcpConfigSpecDhcpOptionsSpec => NetDhcpConfigSpecDhcpOptionsSpec::serialize(
            p.as_any_ref().downcast_ref::<NetDhcpConfigSpecDhcpOptionsSpec>().unwrap(),
            serializer,
        ),
        StructType::NetDnsConfigInfo => NetDnsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<NetDnsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::NetDnsConfigSpec => NetDnsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<NetDnsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpConfigInfo => NetIpConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<NetIpConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::NetIpConfigInfoIpAddress => NetIpConfigInfoIpAddress::serialize(
            p.as_any_ref().downcast_ref::<NetIpConfigInfoIpAddress>().unwrap(),
            serializer,
        ),
        StructType::NetIpConfigSpec => NetIpConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<NetIpConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpConfigSpecIpAddressSpec => NetIpConfigSpecIpAddressSpec::serialize(
            p.as_any_ref().downcast_ref::<NetIpConfigSpecIpAddressSpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigInfo => NetIpRouteConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigInfoGateway => NetIpRouteConfigInfoGateway::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigInfoGateway>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigInfoIpRoute => NetIpRouteConfigInfoIpRoute::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigInfoIpRoute>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigSpec => NetIpRouteConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigSpecGatewaySpec => NetIpRouteConfigSpecGatewaySpec::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigSpecGatewaySpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpRouteConfigSpecIpRouteSpec => NetIpRouteConfigSpecIpRouteSpec::serialize(
            p.as_any_ref().downcast_ref::<NetIpRouteConfigSpecIpRouteSpec>().unwrap(),
            serializer,
        ),
        StructType::NetIpStackInfo => NetIpStackInfo::serialize(
            p.as_any_ref().downcast_ref::<NetIpStackInfo>().unwrap(),
            serializer,
        ),
        StructType::NetIpStackInfoDefaultRouter => NetIpStackInfoDefaultRouter::serialize(
            p.as_any_ref().downcast_ref::<NetIpStackInfoDefaultRouter>().unwrap(),
            serializer,
        ),
        StructType::NetIpStackInfoNetToMedia => NetIpStackInfoNetToMedia::serialize(
            p.as_any_ref().downcast_ref::<NetIpStackInfoNetToMedia>().unwrap(),
            serializer,
        ),
        StructType::NetBiosConfigInfo => NetBiosConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<NetBiosConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::WinNetBiosConfigInfo => WinNetBiosConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<WinNetBiosConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ArrayUpdateSpec => ArrayUpdateSpec::serialize(
            p.as_any_ref().downcast_ref::<ArrayUpdateSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterDasVmConfigSpec => ClusterDasVmConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterDasVmConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterDatastoreUpdateSpec => ClusterDatastoreUpdateSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterDatastoreUpdateSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterDpmHostConfigSpec => ClusterDpmHostConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterDpmHostConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterDrsVmConfigSpec => ClusterDrsVmConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterDrsVmConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterGroupSpec => ClusterGroupSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterGroupSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterPreemptibleVmPairSpec => ClusterPreemptibleVmPairSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterPreemptibleVmPairSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterRuleSpec => ClusterRuleSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterRuleSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterTagCategoryUpdateSpec => ClusterTagCategoryUpdateSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterTagCategoryUpdateSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterVmOrchestrationSpec => ClusterVmOrchestrationSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterVmOrchestrationSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsOptionSpec => StorageDrsOptionSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsOptionSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsVmConfigSpec => StorageDrsVmConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsVmConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VAppOvfSectionSpec => VAppOvfSectionSpec::serialize(
            p.as_any_ref().downcast_ref::<VAppOvfSectionSpec>().unwrap(),
            serializer,
        ),
        StructType::VAppProductSpec => VAppProductSpec::serialize(
            p.as_any_ref().downcast_ref::<VAppProductSpec>().unwrap(),
            serializer,
        ),
        StructType::VAppPropertySpec => VAppPropertySpec::serialize(
            p.as_any_ref().downcast_ref::<VAppPropertySpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineCpuIdInfoSpec => VirtualMachineCpuIdInfoSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineCpuIdInfoSpec>().unwrap(),
            serializer,
        ),
        StructType::OptionType => OptionType::serialize(
            p.as_any_ref().downcast_ref::<OptionType>().unwrap(),
            serializer,
        ),
        StructType::BoolOption => BoolOption::serialize(
            p.as_any_ref().downcast_ref::<BoolOption>().unwrap(),
            serializer,
        ),
        StructType::ChoiceOption => ChoiceOption::serialize(
            p.as_any_ref().downcast_ref::<ChoiceOption>().unwrap(),
            serializer,
        ),
        StructType::FloatOption => FloatOption::serialize(
            p.as_any_ref().downcast_ref::<FloatOption>().unwrap(),
            serializer,
        ),
        StructType::IntOption => IntOption::serialize(
            p.as_any_ref().downcast_ref::<IntOption>().unwrap(),
            serializer,
        ),
        StructType::LongOption => LongOption::serialize(
            p.as_any_ref().downcast_ref::<LongOption>().unwrap(),
            serializer,
        ),
        StructType::StringOption => StringOption::serialize(
            p.as_any_ref().downcast_ref::<StringOption>().unwrap(),
            serializer,
        ),
        StructType::OptionValue => OptionValue::serialize(
            p.as_any_ref().downcast_ref::<OptionValue>().unwrap(),
            serializer,
        ),
        StructType::HostInternetScsiHbaParamValue => HostInternetScsiHbaParamValue::serialize(
            p.as_any_ref().downcast_ref::<HostInternetScsiHbaParamValue>().unwrap(),
            serializer,
        ),
        StructType::ApplyProfile => ApplyProfile::serialize(
            p.as_any_ref().downcast_ref::<ApplyProfile>().unwrap(),
            serializer,
        ),
        StructType::ProfileApplyProfileElement => ProfileApplyProfileElement::serialize(
            p.as_any_ref().downcast_ref::<ProfileApplyProfileElement>().unwrap(),
            serializer,
        ),
        StructType::ActiveDirectoryProfile => ActiveDirectoryProfile::serialize(
            p.as_any_ref().downcast_ref::<ActiveDirectoryProfile>().unwrap(),
            serializer,
        ),
        StructType::AuthenticationProfile => AuthenticationProfile::serialize(
            p.as_any_ref().downcast_ref::<AuthenticationProfile>().unwrap(),
            serializer,
        ),
        StructType::DateTimeProfile => DateTimeProfile::serialize(
            p.as_any_ref().downcast_ref::<DateTimeProfile>().unwrap(),
            serializer,
        ),
        StructType::DvsProfile => DvsProfile::serialize(
            p.as_any_ref().downcast_ref::<DvsProfile>().unwrap(),
            serializer,
        ),
        StructType::DvsVNicProfile => DvsVNicProfile::serialize(
            p.as_any_ref().downcast_ref::<DvsVNicProfile>().unwrap(),
            serializer,
        ),
        StructType::DvsHostVNicProfile => DvsHostVNicProfile::serialize(
            p.as_any_ref().downcast_ref::<DvsHostVNicProfile>().unwrap(),
            serializer,
        ),
        StructType::DvsServiceConsoleVNicProfile => DvsServiceConsoleVNicProfile::serialize(
            p.as_any_ref().downcast_ref::<DvsServiceConsoleVNicProfile>().unwrap(),
            serializer,
        ),
        StructType::FirewallProfile => FirewallProfile::serialize(
            p.as_any_ref().downcast_ref::<FirewallProfile>().unwrap(),
            serializer,
        ),
        StructType::FirewallProfileRulesetProfile => FirewallProfileRulesetProfile::serialize(
            p.as_any_ref().downcast_ref::<FirewallProfileRulesetProfile>().unwrap(),
            serializer,
        ),
        StructType::HostApplyProfile => HostApplyProfile::serialize(
            p.as_any_ref().downcast_ref::<HostApplyProfile>().unwrap(),
            serializer,
        ),
        StructType::HostMemoryProfile => HostMemoryProfile::serialize(
            p.as_any_ref().downcast_ref::<HostMemoryProfile>().unwrap(),
            serializer,
        ),
        StructType::IpAddressProfile => IpAddressProfile::serialize(
            p.as_any_ref().downcast_ref::<IpAddressProfile>().unwrap(),
            serializer,
        ),
        StructType::IpRouteProfile => IpRouteProfile::serialize(
            p.as_any_ref().downcast_ref::<IpRouteProfile>().unwrap(),
            serializer,
        ),
        StructType::NasStorageProfile => NasStorageProfile::serialize(
            p.as_any_ref().downcast_ref::<NasStorageProfile>().unwrap(),
            serializer,
        ),
        StructType::NetStackInstanceProfile => NetStackInstanceProfile::serialize(
            p.as_any_ref().downcast_ref::<NetStackInstanceProfile>().unwrap(),
            serializer,
        ),
        StructType::NetworkPolicyProfile => NetworkPolicyProfile::serialize(
            p.as_any_ref().downcast_ref::<NetworkPolicyProfile>().unwrap(),
            serializer,
        ),
        StructType::NetworkProfile => NetworkProfile::serialize(
            p.as_any_ref().downcast_ref::<NetworkProfile>().unwrap(),
            serializer,
        ),
        StructType::NetworkProfileDnsConfigProfile => NetworkProfileDnsConfigProfile::serialize(
            p.as_any_ref().downcast_ref::<NetworkProfileDnsConfigProfile>().unwrap(),
            serializer,
        ),
        StructType::NsxHostVNicProfile => NsxHostVNicProfile::serialize(
            p.as_any_ref().downcast_ref::<NsxHostVNicProfile>().unwrap(),
            serializer,
        ),
        StructType::OpaqueSwitchProfile => OpaqueSwitchProfile::serialize(
            p.as_any_ref().downcast_ref::<OpaqueSwitchProfile>().unwrap(),
            serializer,
        ),
        StructType::OptionProfile => OptionProfile::serialize(
            p.as_any_ref().downcast_ref::<OptionProfile>().unwrap(),
            serializer,
        ),
        StructType::PermissionProfile => PermissionProfile::serialize(
            p.as_any_ref().downcast_ref::<PermissionProfile>().unwrap(),
            serializer,
        ),
        StructType::PhysicalNicProfile => PhysicalNicProfile::serialize(
            p.as_any_ref().downcast_ref::<PhysicalNicProfile>().unwrap(),
            serializer,
        ),
        StructType::PnicUplinkProfile => PnicUplinkProfile::serialize(
            p.as_any_ref().downcast_ref::<PnicUplinkProfile>().unwrap(),
            serializer,
        ),
        StructType::PortGroupProfile => PortGroupProfile::serialize(
            p.as_any_ref().downcast_ref::<PortGroupProfile>().unwrap(),
            serializer,
        ),
        StructType::HostPortGroupProfile => HostPortGroupProfile::serialize(
            p.as_any_ref().downcast_ref::<HostPortGroupProfile>().unwrap(),
            serializer,
        ),
        StructType::ServiceConsolePortGroupProfile => ServiceConsolePortGroupProfile::serialize(
            p.as_any_ref().downcast_ref::<ServiceConsolePortGroupProfile>().unwrap(),
            serializer,
        ),
        StructType::VmPortGroupProfile => VmPortGroupProfile::serialize(
            p.as_any_ref().downcast_ref::<VmPortGroupProfile>().unwrap(),
            serializer,
        ),
        StructType::VirtualSwitchSelectionProfile => VirtualSwitchSelectionProfile::serialize(
            p.as_any_ref().downcast_ref::<VirtualSwitchSelectionProfile>().unwrap(),
            serializer,
        ),
        StructType::VlanProfile => VlanProfile::serialize(
            p.as_any_ref().downcast_ref::<VlanProfile>().unwrap(),
            serializer,
        ),
        StructType::SecurityProfile => SecurityProfile::serialize(
            p.as_any_ref().downcast_ref::<SecurityProfile>().unwrap(),
            serializer,
        ),
        StructType::ServiceProfile => ServiceProfile::serialize(
            p.as_any_ref().downcast_ref::<ServiceProfile>().unwrap(),
            serializer,
        ),
        StructType::StaticRouteProfile => StaticRouteProfile::serialize(
            p.as_any_ref().downcast_ref::<StaticRouteProfile>().unwrap(),
            serializer,
        ),
        StructType::StorageProfile => StorageProfile::serialize(
            p.as_any_ref().downcast_ref::<StorageProfile>().unwrap(),
            serializer,
        ),
        StructType::UserGroupProfile => UserGroupProfile::serialize(
            p.as_any_ref().downcast_ref::<UserGroupProfile>().unwrap(),
            serializer,
        ),
        StructType::UserProfile => UserProfile::serialize(
            p.as_any_ref().downcast_ref::<UserProfile>().unwrap(),
            serializer,
        ),
        StructType::VirtualSwitchProfile => VirtualSwitchProfile::serialize(
            p.as_any_ref().downcast_ref::<VirtualSwitchProfile>().unwrap(),
            serializer,
        ),
        StructType::LinkProfile => LinkProfile::serialize(
            p.as_any_ref().downcast_ref::<LinkProfile>().unwrap(),
            serializer,
        ),
        StructType::NumPortsProfile => NumPortsProfile::serialize(
            p.as_any_ref().downcast_ref::<NumPortsProfile>().unwrap(),
            serializer,
        ),
        StructType::ProfileApplyProfileProperty => ProfileApplyProfileProperty::serialize(
            p.as_any_ref().downcast_ref::<ProfileApplyProfileProperty>().unwrap(),
            serializer,
        ),
        StructType::ComplianceLocator => ComplianceLocator::serialize(
            p.as_any_ref().downcast_ref::<ComplianceLocator>().unwrap(),
            serializer,
        ),
        StructType::ComplianceProfile => ComplianceProfile::serialize(
            p.as_any_ref().downcast_ref::<ComplianceProfile>().unwrap(),
            serializer,
        ),
        StructType::ComplianceResult => ComplianceResult::serialize(
            p.as_any_ref().downcast_ref::<ComplianceResult>().unwrap(),
            serializer,
        ),
        StructType::ComplianceFailure => ComplianceFailure::serialize(
            p.as_any_ref().downcast_ref::<ComplianceFailure>().unwrap(),
            serializer,
        ),
        StructType::ComplianceFailureComplianceFailureValues => ComplianceFailureComplianceFailureValues::serialize(
            p.as_any_ref().downcast_ref::<ComplianceFailureComplianceFailureValues>().unwrap(),
            serializer,
        ),
        StructType::ProfileDeferredPolicyOptionParameter => ProfileDeferredPolicyOptionParameter::serialize(
            p.as_any_ref().downcast_ref::<ProfileDeferredPolicyOptionParameter>().unwrap(),
            serializer,
        ),
        StructType::ProfileExpression => ProfileExpression::serialize(
            p.as_any_ref().downcast_ref::<ProfileExpression>().unwrap(),
            serializer,
        ),
        StructType::ProfileCompositeExpression => ProfileCompositeExpression::serialize(
            p.as_any_ref().downcast_ref::<ProfileCompositeExpression>().unwrap(),
            serializer,
        ),
        StructType::ProfileSimpleExpression => ProfileSimpleExpression::serialize(
            p.as_any_ref().downcast_ref::<ProfileSimpleExpression>().unwrap(),
            serializer,
        ),
        StructType::ProfileExpressionMetadata => ProfileExpressionMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfileExpressionMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfileParameterMetadata => ProfileParameterMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfileParameterMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfileParameterMetadataParameterRelationMetadata => ProfileParameterMetadataParameterRelationMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfileParameterMetadataParameterRelationMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfilePolicy => ProfilePolicy::serialize(
            p.as_any_ref().downcast_ref::<ProfilePolicy>().unwrap(),
            serializer,
        ),
        StructType::ProfilePolicyMetadata => ProfilePolicyMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfilePolicyMetadata>().unwrap(),
            serializer,
        ),
        StructType::PolicyOption => PolicyOption::serialize(
            p.as_any_ref().downcast_ref::<PolicyOption>().unwrap(),
            serializer,
        ),
        StructType::CompositePolicyOption => CompositePolicyOption::serialize(
            p.as_any_ref().downcast_ref::<CompositePolicyOption>().unwrap(),
            serializer,
        ),
        StructType::ProfilePolicyOptionMetadata => ProfilePolicyOptionMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfilePolicyOptionMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfileCompositePolicyOptionMetadata => ProfileCompositePolicyOptionMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfileCompositePolicyOptionMetadata>().unwrap(),
            serializer,
        ),
        StructType::UserInputRequiredParameterMetadata => UserInputRequiredParameterMetadata::serialize(
            p.as_any_ref().downcast_ref::<UserInputRequiredParameterMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfileConfigInfo => ProfileConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ProfileConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ClusterProfileConfigInfo => ClusterProfileConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ClusterProfileConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::HostProfileConfigInfo => HostProfileConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<HostProfileConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ProfileCreateSpec => ProfileCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<ProfileCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::ProfileSerializedCreateSpec => ProfileSerializedCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<ProfileSerializedCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::HostProfileSerializedHostProfileSpec => HostProfileSerializedHostProfileSpec::serialize(
            p.as_any_ref().downcast_ref::<HostProfileSerializedHostProfileSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterProfileCreateSpec => ClusterProfileCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterProfileCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterProfileConfigSpec => ClusterProfileConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterProfileConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterProfileCompleteConfigSpec => ClusterProfileCompleteConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterProfileCompleteConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterProfileConfigServiceCreateSpec => ClusterProfileConfigServiceCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterProfileConfigServiceCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::HostProfileConfigSpec => HostProfileConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostProfileConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostProfileCompleteConfigSpec => HostProfileCompleteConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostProfileCompleteConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::HostProfileHostBasedConfigSpec => HostProfileHostBasedConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<HostProfileHostBasedConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ProfileDescription => ProfileDescription::serialize(
            p.as_any_ref().downcast_ref::<ProfileDescription>().unwrap(),
            serializer,
        ),
        StructType::ProfileDescriptionSection => ProfileDescriptionSection::serialize(
            p.as_any_ref().downcast_ref::<ProfileDescriptionSection>().unwrap(),
            serializer,
        ),
        StructType::ProfileMetadata => ProfileMetadata::serialize(
            p.as_any_ref().downcast_ref::<ProfileMetadata>().unwrap(),
            serializer,
        ),
        StructType::ProfileMetadataProfileOperationMessage => ProfileMetadataProfileOperationMessage::serialize(
            p.as_any_ref().downcast_ref::<ProfileMetadataProfileOperationMessage>().unwrap(),
            serializer,
        ),
        StructType::ProfileMetadataProfileSortSpec => ProfileMetadataProfileSortSpec::serialize(
            p.as_any_ref().downcast_ref::<ProfileMetadataProfileSortSpec>().unwrap(),
            serializer,
        ),
        StructType::ProfilePropertyPath => ProfilePropertyPath::serialize(
            p.as_any_ref().downcast_ref::<ProfilePropertyPath>().unwrap(),
            serializer,
        ),
        StructType::ProfileProfileStructure => ProfileProfileStructure::serialize(
            p.as_any_ref().downcast_ref::<ProfileProfileStructure>().unwrap(),
            serializer,
        ),
        StructType::ProfileProfileStructureProperty => ProfileProfileStructureProperty::serialize(
            p.as_any_ref().downcast_ref::<ProfileProfileStructureProperty>().unwrap(),
            serializer,
        ),
        StructType::AnswerFile => AnswerFile::serialize(
            p.as_any_ref().downcast_ref::<AnswerFile>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileStatusResult => AnswerFileStatusResult::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileStatusResult>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileStatusError => AnswerFileStatusError::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileStatusError>().unwrap(),
            serializer,
        ),
        StructType::ProfileExecuteResult => ProfileExecuteResult::serialize(
            p.as_any_ref().downcast_ref::<ProfileExecuteResult>().unwrap(),
            serializer,
        ),
        StructType::ApplyHostProfileConfigurationSpec => ApplyHostProfileConfigurationSpec::serialize(
            p.as_any_ref().downcast_ref::<ApplyHostProfileConfigurationSpec>().unwrap(),
            serializer,
        ),
        StructType::ProfileExecuteError => ProfileExecuteError::serialize(
            p.as_any_ref().downcast_ref::<ProfileExecuteError>().unwrap(),
            serializer,
        ),
        StructType::HostProfileValidationFailureInfo => HostProfileValidationFailureInfo::serialize(
            p.as_any_ref().downcast_ref::<HostProfileValidationFailureInfo>().unwrap(),
            serializer,
        ),
        StructType::HostSpecification => HostSpecification::serialize(
            p.as_any_ref().downcast_ref::<HostSpecification>().unwrap(),
            serializer,
        ),
        StructType::HostSubSpecification => HostSubSpecification::serialize(
            p.as_any_ref().downcast_ref::<HostSubSpecification>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileCreateSpec => AnswerFileCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileOptionsCreateSpec => AnswerFileOptionsCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileOptionsCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::AnswerFileSerializedCreateSpec => AnswerFileSerializedCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<AnswerFileSerializedCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::ApplyHostProfileConfigurationResult => ApplyHostProfileConfigurationResult::serialize(
            p.as_any_ref().downcast_ref::<ApplyHostProfileConfigurationResult>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerCompositionResult => HostProfileManagerCompositionResult::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerCompositionResult>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerCompositionResultResultElement => HostProfileManagerCompositionResultResultElement::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerCompositionResultResultElement>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerCompositionValidationResult => HostProfileManagerCompositionValidationResult::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerCompositionValidationResult>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerCompositionValidationResultResultElement => HostProfileManagerCompositionValidationResultResultElement::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerCompositionValidationResultResultElement>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerConfigTaskList => HostProfileManagerConfigTaskList::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerConfigTaskList>().unwrap(),
            serializer,
        ),
        StructType::HostProfilesEntityCustomizations => HostProfilesEntityCustomizations::serialize(
            p.as_any_ref().downcast_ref::<HostProfilesEntityCustomizations>().unwrap(),
            serializer,
        ),
        StructType::StructuredCustomizations => StructuredCustomizations::serialize(
            p.as_any_ref().downcast_ref::<StructuredCustomizations>().unwrap(),
            serializer,
        ),
        StructType::HostProfileManagerHostToConfigSpecMap => HostProfileManagerHostToConfigSpecMap::serialize(
            p.as_any_ref().downcast_ref::<HostProfileManagerHostToConfigSpecMap>().unwrap(),
            serializer,
        ),
        StructType::ScheduledTaskDescription => ScheduledTaskDescription::serialize(
            p.as_any_ref().downcast_ref::<ScheduledTaskDescription>().unwrap(),
            serializer,
        ),
        StructType::ScheduledTaskSpec => ScheduledTaskSpec::serialize(
            p.as_any_ref().downcast_ref::<ScheduledTaskSpec>().unwrap(),
            serializer,
        ),
        StructType::ScheduledTaskInfo => ScheduledTaskInfo::serialize(
            p.as_any_ref().downcast_ref::<ScheduledTaskInfo>().unwrap(),
            serializer,
        ),
        StructType::TaskScheduler => TaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<TaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::AfterStartupTaskScheduler => AfterStartupTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<AfterStartupTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::OnceTaskScheduler => OnceTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<OnceTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::RecurrentTaskScheduler => RecurrentTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<RecurrentTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::HourlyTaskScheduler => HourlyTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<HourlyTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::DailyTaskScheduler => DailyTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<DailyTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::MonthlyTaskScheduler => MonthlyTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<MonthlyTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::MonthlyByDayTaskScheduler => MonthlyByDayTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<MonthlyByDayTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::MonthlyByWeekdayTaskScheduler => MonthlyByWeekdayTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<MonthlyByWeekdayTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::WeeklyTaskScheduler => WeeklyTaskScheduler::serialize(
            p.as_any_ref().downcast_ref::<WeeklyTaskScheduler>().unwrap(),
            serializer,
        ),
        StructType::ApplyStorageRecommendationResult => ApplyStorageRecommendationResult::serialize(
            p.as_any_ref().downcast_ref::<ApplyStorageRecommendationResult>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsAutomationConfig => StorageDrsAutomationConfig::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsAutomationConfig>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsConfigInfo => StorageDrsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsConfigSpec => StorageDrsConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsIoLoadBalanceConfig => StorageDrsIoLoadBalanceConfig::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsIoLoadBalanceConfig>().unwrap(),
            serializer,
        ),
        StructType::PlacementAffinityRule => PlacementAffinityRule::serialize(
            p.as_any_ref().downcast_ref::<PlacementAffinityRule>().unwrap(),
            serializer,
        ),
        StructType::PlacementRankResult => PlacementRankResult::serialize(
            p.as_any_ref().downcast_ref::<PlacementRankResult>().unwrap(),
            serializer,
        ),
        StructType::PlacementRankSpec => PlacementRankSpec::serialize(
            p.as_any_ref().downcast_ref::<PlacementRankSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsPlacementRankVmSpec => StorageDrsPlacementRankVmSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsPlacementRankVmSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsPodConfigInfo => StorageDrsPodConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsPodConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsPodConfigSpec => StorageDrsPodConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsPodConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsPodSelectionSpec => StorageDrsPodSelectionSpec::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsPodSelectionSpec>().unwrap(),
            serializer,
        ),
        StructType::PodDiskLocator => PodDiskLocator::serialize(
            p.as_any_ref().downcast_ref::<PodDiskLocator>().unwrap(),
            serializer,
        ),
        StructType::VmPodConfigForPlacement => VmPodConfigForPlacement::serialize(
            p.as_any_ref().downcast_ref::<VmPodConfigForPlacement>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsSpaceLoadBalanceConfig => StorageDrsSpaceLoadBalanceConfig::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsSpaceLoadBalanceConfig>().unwrap(),
            serializer,
        ),
        StructType::StoragePlacementResult => StoragePlacementResult::serialize(
            p.as_any_ref().downcast_ref::<StoragePlacementResult>().unwrap(),
            serializer,
        ),
        StructType::StoragePlacementSpec => StoragePlacementSpec::serialize(
            p.as_any_ref().downcast_ref::<StoragePlacementSpec>().unwrap(),
            serializer,
        ),
        StructType::StorageDrsVmConfigInfo => StorageDrsVmConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<StorageDrsVmConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppCloneSpec => VAppCloneSpec::serialize(
            p.as_any_ref().downcast_ref::<VAppCloneSpec>().unwrap(),
            serializer,
        ),
        StructType::VAppCloneSpecNetworkMappingPair => VAppCloneSpecNetworkMappingPair::serialize(
            p.as_any_ref().downcast_ref::<VAppCloneSpecNetworkMappingPair>().unwrap(),
            serializer,
        ),
        StructType::VAppCloneSpecResourceMap => VAppCloneSpecResourceMap::serialize(
            p.as_any_ref().downcast_ref::<VAppCloneSpecResourceMap>().unwrap(),
            serializer,
        ),
        StructType::VAppEntityConfigInfo => VAppEntityConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppEntityConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppIpAssignmentInfo => VAppIpAssignmentInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppIpAssignmentInfo>().unwrap(),
            serializer,
        ),
        StructType::IpPool => IpPool::serialize(
            p.as_any_ref().downcast_ref::<IpPool>().unwrap(),
            serializer,
        ),
        StructType::IpPoolAssociation => IpPoolAssociation::serialize(
            p.as_any_ref().downcast_ref::<IpPoolAssociation>().unwrap(),
            serializer,
        ),
        StructType::IpPoolIpPoolConfigInfo => IpPoolIpPoolConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<IpPoolIpPoolConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppOvfSectionInfo => VAppOvfSectionInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppOvfSectionInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppProductInfo => VAppProductInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppProductInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppPropertyInfo => VAppPropertyInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppPropertyInfo>().unwrap(),
            serializer,
        ),
        StructType::VmConfigInfo => VmConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VmConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VAppConfigInfo => VAppConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VAppConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VmConfigSpec => VmConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VmConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VAppConfigSpec => VAppConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VAppConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ClusterNetworkConfigSpec => ClusterNetworkConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ClusterNetworkConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::FailoverNodeInfo => FailoverNodeInfo::serialize(
            p.as_any_ref().downcast_ref::<FailoverNodeInfo>().unwrap(),
            serializer,
        ),
        StructType::NodeDeploymentSpec => NodeDeploymentSpec::serialize(
            p.as_any_ref().downcast_ref::<NodeDeploymentSpec>().unwrap(),
            serializer,
        ),
        StructType::PassiveNodeDeploymentSpec => PassiveNodeDeploymentSpec::serialize(
            p.as_any_ref().downcast_ref::<PassiveNodeDeploymentSpec>().unwrap(),
            serializer,
        ),
        StructType::NodeNetworkSpec => NodeNetworkSpec::serialize(
            p.as_any_ref().downcast_ref::<NodeNetworkSpec>().unwrap(),
            serializer,
        ),
        StructType::PassiveNodeNetworkSpec => PassiveNodeNetworkSpec::serialize(
            p.as_any_ref().downcast_ref::<PassiveNodeNetworkSpec>().unwrap(),
            serializer,
        ),
        StructType::SourceNodeSpec => SourceNodeSpec::serialize(
            p.as_any_ref().downcast_ref::<SourceNodeSpec>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterConfigInfo => VchaClusterConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterConfigSpec => VchaClusterConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterDeploymentSpec => VchaClusterDeploymentSpec::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterDeploymentSpec>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterNetworkSpec => VchaClusterNetworkSpec::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterNetworkSpec>().unwrap(),
            serializer,
        ),
        StructType::WitnessNodeInfo => WitnessNodeInfo::serialize(
            p.as_any_ref().downcast_ref::<WitnessNodeInfo>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterHealth => VchaClusterHealth::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterHealth>().unwrap(),
            serializer,
        ),
        StructType::VchaClusterRuntimeInfo => VchaClusterRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VchaClusterRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VchaNodeRuntimeInfo => VchaNodeRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VchaNodeRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineAffinityInfo => VirtualMachineAffinityInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineAffinityInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBaseIndependentFilterSpec => VirtualMachineBaseIndependentFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBaseIndependentFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineEmptyIndependentFilterSpec => VirtualMachineEmptyIndependentFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineEmptyIndependentFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineIndependentFilterSpec => VirtualMachineIndependentFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineIndependentFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptions => VirtualMachineBootOptions::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptions>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptionsBootableDevice => VirtualMachineBootOptionsBootableDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptionsBootableCdromDevice => VirtualMachineBootOptionsBootableCdromDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableCdromDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptionsBootableDiskDevice => VirtualMachineBootOptionsBootableDiskDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableDiskDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptionsBootableEthernetDevice => VirtualMachineBootOptionsBootableEthernetDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableEthernetDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineBootOptionsBootableFloppyDevice => VirtualMachineBootOptionsBootableFloppyDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineBootOptionsBootableFloppyDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineCapability => VirtualMachineCapability::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineCapability>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineCertThumbprint => VirtualMachineCertThumbprint::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineCertThumbprint>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineCloneSpec => VirtualMachineCloneSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineCloneSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigInfo => VirtualMachineConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigInfoDatastoreUrlPair => VirtualMachineConfigInfoDatastoreUrlPair::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigInfoDatastoreUrlPair>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigInfoOverheadInfo => VirtualMachineConfigInfoOverheadInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigInfoOverheadInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigOption => VirtualMachineConfigOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigOptionDescriptor => VirtualMachineConfigOptionDescriptor::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigOptionDescriptor>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigSpec => VirtualMachineConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ConfigTarget => ConfigTarget::serialize(
            p.as_any_ref().downcast_ref::<ConfigTarget>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConsolePreferences => VirtualMachineConsolePreferences::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConsolePreferences>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineContentLibraryItemInfo => VirtualMachineContentLibraryItemInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineContentLibraryItemInfo>().unwrap(),
            serializer,
        ),
        StructType::DatastoreOption => DatastoreOption::serialize(
            p.as_any_ref().downcast_ref::<DatastoreOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDatastoreVolumeOption => VirtualMachineDatastoreVolumeOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDatastoreVolumeOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDefaultPowerOpInfo => VirtualMachineDefaultPowerOpInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDefaultPowerOpInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDeviceRuntimeInfo => VirtualMachineDeviceRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState => VirtualMachineDeviceRuntimeInfoDeviceRuntimeState::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoDeviceRuntimeState>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState => VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDvxClassInfo => VirtualMachineDvxClassInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDvxClassInfo>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceConfigInfo => FaultToleranceConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::FaultTolerancePrimaryConfigInfo => FaultTolerancePrimaryConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<FaultTolerancePrimaryConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceSecondaryConfigInfo => FaultToleranceSecondaryConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceSecondaryConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceConfigSpec => FaultToleranceConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceMetaSpec => FaultToleranceMetaSpec::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceMetaSpec>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceSecondaryOpResult => FaultToleranceSecondaryOpResult::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceSecondaryOpResult>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceVmConfigSpec => FaultToleranceVmConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceVmConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::FaultToleranceDiskSpec => FaultToleranceDiskSpec::serialize(
            p.as_any_ref().downcast_ref::<FaultToleranceDiskSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFeatureRequirement => VirtualMachineFeatureRequirement::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFeatureRequirement>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileInfo => VirtualMachineFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayout => VirtualMachineFileLayout::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayout>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutDiskLayout => VirtualMachineFileLayoutDiskLayout::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutDiskLayout>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutSnapshotLayout => VirtualMachineFileLayoutSnapshotLayout::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutSnapshotLayout>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutEx => VirtualMachineFileLayoutEx::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutEx>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutExDiskLayout => VirtualMachineFileLayoutExDiskLayout::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExDiskLayout>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutExDiskUnit => VirtualMachineFileLayoutExDiskUnit::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExDiskUnit>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutExFileInfo => VirtualMachineFileLayoutExFileInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExFileInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFileLayoutExSnapshotLayout => VirtualMachineFileLayoutExSnapshotLayout::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFileLayoutExSnapshotLayout>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFlagInfo => VirtualMachineFlagInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFlagInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineForkConfigInfo => VirtualMachineForkConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineForkConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestInfo => GuestInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestInfoCustomizationInfo => GuestInfoCustomizationInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestInfoCustomizationInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestDiskInfo => GuestDiskInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestDiskInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestInfoNamespaceGenerationInfo => GuestInfoNamespaceGenerationInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestInfoNamespaceGenerationInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestNicInfo => GuestNicInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestNicInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestScreenInfo => GuestScreenInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestScreenInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestStackInfo => GuestStackInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestStackInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestInfoVirtualDiskMapping => GuestInfoVirtualDiskMapping::serialize(
            p.as_any_ref().downcast_ref::<GuestInfoVirtualDiskMapping>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineGuestIntegrityInfo => VirtualMachineGuestIntegrityInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineGuestIntegrityInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineGuestMonitoringModeInfo => VirtualMachineGuestMonitoringModeInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineGuestMonitoringModeInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestOsDescriptor => GuestOsDescriptor::serialize(
            p.as_any_ref().downcast_ref::<GuestOsDescriptor>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineGuestQuiesceSpec => VirtualMachineGuestQuiesceSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineGuestQuiesceSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineWindowsQuiesceSpec => VirtualMachineWindowsQuiesceSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineWindowsQuiesceSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineIdeDiskDevicePartitionInfo => VirtualMachineIdeDiskDevicePartitionInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDevicePartitionInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineInstantCloneSpec => VirtualMachineInstantCloneSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineInstantCloneSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineLegacyNetworkSwitchInfo => VirtualMachineLegacyNetworkSwitchInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineLegacyNetworkSwitchInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMessage => VirtualMachineMessage::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMessage>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMetadataManagerVmMetadata => VirtualMachineMetadataManagerVmMetadata::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadata>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMetadataManagerVmMetadataInput => VirtualMachineMetadataManagerVmMetadataInput::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataInput>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMetadataManagerVmMetadataOwner => VirtualMachineMetadataManagerVmMetadataOwner::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataOwner>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineMetadataManagerVmMetadataResult => VirtualMachineMetadataManagerVmMetadataResult::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineMetadataManagerVmMetadataResult>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineNetworkShaperInfo => VirtualMachineNetworkShaperInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineNetworkShaperInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineProfileDetails => VirtualMachineProfileDetails::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineProfileDetails>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineProfileDetailsDiskProfileDetails => VirtualMachineProfileDetailsDiskProfileDetails::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineProfileDetailsDiskProfileDetails>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineProfileRawData => VirtualMachineProfileRawData::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineProfileRawData>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineProfileSpec => VirtualMachineProfileSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineProfileSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDefaultProfileSpec => VirtualMachineDefaultProfileSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDefaultProfileSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDefinedProfileSpec => VirtualMachineDefinedProfileSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDefinedProfileSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineEmptyProfileSpec => VirtualMachineEmptyProfileSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineEmptyProfileSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachinePropertyRelation => VirtualMachinePropertyRelation::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachinePropertyRelation>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineQuestionInfo => VirtualMachineQuestionInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineQuestionInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineRelocateSpec => VirtualMachineRelocateSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineRelocateSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineRelocateSpecDiskLocator => VirtualMachineRelocateSpecDiskLocator::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineRelocateSpecDiskLocator>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineRelocateSpecDiskLocatorBackingSpec => VirtualMachineRelocateSpecDiskLocatorBackingSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineRelocateSpecDiskLocatorBackingSpec>().unwrap(),
            serializer,
        ),
        StructType::ReplicationConfigSpec => ReplicationConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<ReplicationConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::ReplicationInfoDiskSettings => ReplicationInfoDiskSettings::serialize(
            p.as_any_ref().downcast_ref::<ReplicationInfoDiskSettings>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineRuntimeInfo => VirtualMachineRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineRuntimeInfoDasProtectionState => VirtualMachineRuntimeInfoDasProtectionState::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineRuntimeInfoDasProtectionState>().unwrap(),
            serializer,
        ),
        StructType::ScheduledHardwareUpgradeInfo => ScheduledHardwareUpgradeInfo::serialize(
            p.as_any_ref().downcast_ref::<ScheduledHardwareUpgradeInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSgxInfo => VirtualMachineSgxInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSgxInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSnapshotInfo => VirtualMachineSnapshotInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSnapshotInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSnapshotTree => VirtualMachineSnapshotTree::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSnapshotTree>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSriovDevicePoolInfo => VirtualMachineSriovDevicePoolInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSriovDevicePoolInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSriovNetworkDevicePoolInfo => VirtualMachineSriovNetworkDevicePoolInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSriovNetworkDevicePoolInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineStorageInfo => VirtualMachineStorageInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineStorageInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineUsageOnDatastore => VirtualMachineUsageOnDatastore::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineUsageOnDatastore>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSummary => VirtualMachineSummary::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSummary>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineConfigSummary => VirtualMachineConfigSummary::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineConfigSummary>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineGuestSummary => VirtualMachineGuestSummary::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineGuestSummary>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineQuickStats => VirtualMachineQuickStats::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineQuickStats>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineQuickStatsMemoryTierStats => VirtualMachineQuickStatsMemoryTierStats::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineQuickStatsMemoryTierStats>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineStorageSummary => VirtualMachineStorageSummary::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineStorageSummary>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineTargetInfo => VirtualMachineTargetInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineTargetInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineCdromInfo => VirtualMachineCdromInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineCdromInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDatastoreInfo => VirtualMachineDatastoreInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDatastoreInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDiskDeviceInfo => VirtualMachineDiskDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDiskDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineIdeDiskDeviceInfo => VirtualMachineIdeDiskDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineIdeDiskDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineScsiDiskDeviceInfo => VirtualMachineScsiDiskDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineScsiDiskDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineDynamicPassthroughInfo => VirtualMachineDynamicPassthroughInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineDynamicPassthroughInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineFloppyInfo => VirtualMachineFloppyInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineFloppyInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineNetworkInfo => VirtualMachineNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::OpaqueNetworkTargetInfo => OpaqueNetworkTargetInfo::serialize(
            p.as_any_ref().downcast_ref::<OpaqueNetworkTargetInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineParallelInfo => VirtualMachineParallelInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineParallelInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachinePciPassthroughInfo => VirtualMachinePciPassthroughInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachinePciPassthroughInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSriovInfo => VirtualMachineSriovInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSriovInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachinePciSharedGpuPassthroughInfo => VirtualMachinePciSharedGpuPassthroughInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachinePciSharedGpuPassthroughInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachinePrecisionClockInfo => VirtualMachinePrecisionClockInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachinePrecisionClockInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineScsiPassthroughInfo => VirtualMachineScsiPassthroughInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineScsiPassthroughInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSerialInfo => VirtualMachineSerialInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSerialInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSgxTargetInfo => VirtualMachineSgxTargetInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSgxTargetInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineSoundInfo => VirtualMachineSoundInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineSoundInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineUsbInfo => VirtualMachineUsbInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineUsbInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVFlashModuleInfo => VirtualMachineVFlashModuleInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVFlashModuleInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVMotionStunTimeInfo => VirtualMachineVMotionStunTimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVMotionStunTimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVendorDeviceGroupInfo => VirtualMachineVendorDeviceGroupInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVendorDeviceGroupInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVgpuDeviceInfo => VirtualMachineVgpuDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVgpuDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVgpuProfileInfo => VirtualMachineVgpuProfileInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVgpuProfileInfo>().unwrap(),
            serializer,
        ),
        StructType::ToolsConfigInfo => ToolsConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<ToolsConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::ToolsConfigInfoToolsLastInstallInfo => ToolsConfigInfoToolsLastInstallInfo::serialize(
            p.as_any_ref().downcast_ref::<ToolsConfigInfoToolsLastInstallInfo>().unwrap(),
            serializer,
        ),
        StructType::UsbScanCodeSpec => UsbScanCodeSpec::serialize(
            p.as_any_ref().downcast_ref::<UsbScanCodeSpec>().unwrap(),
            serializer,
        ),
        StructType::UsbScanCodeSpecKeyEvent => UsbScanCodeSpecKeyEvent::serialize(
            p.as_any_ref().downcast_ref::<UsbScanCodeSpecKeyEvent>().unwrap(),
            serializer,
        ),
        StructType::UsbScanCodeSpecModifierType => UsbScanCodeSpecModifierType::serialize(
            p.as_any_ref().downcast_ref::<UsbScanCodeSpecModifierType>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVcpuConfig => VirtualMachineVcpuConfig::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVcpuConfig>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo => VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualDeviceGroups => VirtualMachineVirtualDeviceGroups::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroups>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup => VirtualMachineVirtualDeviceGroupsDeviceGroup::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsDeviceGroup>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup => VirtualMachineVirtualDeviceGroupsVendorDeviceGroup::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualDeviceSwap => VirtualMachineVirtualDeviceSwap::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceSwap>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualDeviceSwapDeviceSwapInfo => VirtualMachineVirtualDeviceSwapDeviceSwapInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualDeviceSwapDeviceSwapInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualHardware => VirtualHardware::serialize(
            p.as_any_ref().downcast_ref::<VirtualHardware>().unwrap(),
            serializer,
        ),
        StructType::VirtualHardwareOption => VirtualHardwareOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualHardwareOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualNuma => VirtualMachineVirtualNuma::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualNuma>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualNumaInfo => VirtualMachineVirtualNumaInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualNumaInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVirtualPMem => VirtualMachineVirtualPMem::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVirtualPMem>().unwrap(),
            serializer,
        ),
        StructType::CheckResult => CheckResult::serialize(
            p.as_any_ref().downcast_ref::<CheckResult>().unwrap(),
            serializer,
        ),
        StructType::CustomizationAdapterMapping => CustomizationAdapterMapping::serialize(
            p.as_any_ref().downcast_ref::<CustomizationAdapterMapping>().unwrap(),
            serializer,
        ),
        StructType::CustomizationGlobalIpSettings => CustomizationGlobalIpSettings::serialize(
            p.as_any_ref().downcast_ref::<CustomizationGlobalIpSettings>().unwrap(),
            serializer,
        ),
        StructType::CustomizationGuiRunOnce => CustomizationGuiRunOnce::serialize(
            p.as_any_ref().downcast_ref::<CustomizationGuiRunOnce>().unwrap(),
            serializer,
        ),
        StructType::CustomizationGuiUnattended => CustomizationGuiUnattended::serialize(
            p.as_any_ref().downcast_ref::<CustomizationGuiUnattended>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIpSettings => CustomizationIpSettings::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIpSettings>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIpSettingsIpV6AddressSpec => CustomizationIpSettingsIpV6AddressSpec::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIpSettingsIpV6AddressSpec>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIdentification => CustomizationIdentification::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIdentification>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIdentitySettings => CustomizationIdentitySettings::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIdentitySettings>().unwrap(),
            serializer,
        ),
        StructType::CustomizationCloudinitPrep => CustomizationCloudinitPrep::serialize(
            p.as_any_ref().downcast_ref::<CustomizationCloudinitPrep>().unwrap(),
            serializer,
        ),
        StructType::CustomizationLinuxPrep => CustomizationLinuxPrep::serialize(
            p.as_any_ref().downcast_ref::<CustomizationLinuxPrep>().unwrap(),
            serializer,
        ),
        StructType::CustomizationSysprep => CustomizationSysprep::serialize(
            p.as_any_ref().downcast_ref::<CustomizationSysprep>().unwrap(),
            serializer,
        ),
        StructType::CustomizationSysprepText => CustomizationSysprepText::serialize(
            p.as_any_ref().downcast_ref::<CustomizationSysprepText>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIpGenerator => CustomizationIpGenerator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIpGenerator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationCustomIpGenerator => CustomizationCustomIpGenerator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationCustomIpGenerator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationDhcpIpGenerator => CustomizationDhcpIpGenerator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationDhcpIpGenerator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationFixedIp => CustomizationFixedIp::serialize(
            p.as_any_ref().downcast_ref::<CustomizationFixedIp>().unwrap(),
            serializer,
        ),
        StructType::CustomizationUnknownIpGenerator => CustomizationUnknownIpGenerator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationUnknownIpGenerator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationIpV6Generator => CustomizationIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationAutoIpV6Generator => CustomizationAutoIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationAutoIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationCustomIpV6Generator => CustomizationCustomIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationCustomIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationDhcpIpV6Generator => CustomizationDhcpIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationDhcpIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationFixedIpV6 => CustomizationFixedIpV6::serialize(
            p.as_any_ref().downcast_ref::<CustomizationFixedIpV6>().unwrap(),
            serializer,
        ),
        StructType::CustomizationStatelessIpV6Generator => CustomizationStatelessIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationStatelessIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationUnknownIpV6Generator => CustomizationUnknownIpV6Generator::serialize(
            p.as_any_ref().downcast_ref::<CustomizationUnknownIpV6Generator>().unwrap(),
            serializer,
        ),
        StructType::CustomizationLicenseFilePrintData => CustomizationLicenseFilePrintData::serialize(
            p.as_any_ref().downcast_ref::<CustomizationLicenseFilePrintData>().unwrap(),
            serializer,
        ),
        StructType::CustomizationName => CustomizationName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationCustomName => CustomizationCustomName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationCustomName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationFixedName => CustomizationFixedName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationFixedName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationPrefixName => CustomizationPrefixName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationPrefixName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationUnknownName => CustomizationUnknownName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationUnknownName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationVirtualMachineName => CustomizationVirtualMachineName::serialize(
            p.as_any_ref().downcast_ref::<CustomizationVirtualMachineName>().unwrap(),
            serializer,
        ),
        StructType::CustomizationOptions => CustomizationOptions::serialize(
            p.as_any_ref().downcast_ref::<CustomizationOptions>().unwrap(),
            serializer,
        ),
        StructType::CustomizationLinuxOptions => CustomizationLinuxOptions::serialize(
            p.as_any_ref().downcast_ref::<CustomizationLinuxOptions>().unwrap(),
            serializer,
        ),
        StructType::CustomizationWinOptions => CustomizationWinOptions::serialize(
            p.as_any_ref().downcast_ref::<CustomizationWinOptions>().unwrap(),
            serializer,
        ),
        StructType::CustomizationPassword => CustomizationPassword::serialize(
            p.as_any_ref().downcast_ref::<CustomizationPassword>().unwrap(),
            serializer,
        ),
        StructType::CustomizationSpec => CustomizationSpec::serialize(
            p.as_any_ref().downcast_ref::<CustomizationSpec>().unwrap(),
            serializer,
        ),
        StructType::CustomizationUserData => CustomizationUserData::serialize(
            p.as_any_ref().downcast_ref::<CustomizationUserData>().unwrap(),
            serializer,
        ),
        StructType::HostDiskMappingInfo => HostDiskMappingInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDiskMappingInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDiskMappingPartitionInfo => HostDiskMappingPartitionInfo::serialize(
            p.as_any_ref().downcast_ref::<HostDiskMappingPartitionInfo>().unwrap(),
            serializer,
        ),
        StructType::HostDiskMappingOption => HostDiskMappingOption::serialize(
            p.as_any_ref().downcast_ref::<HostDiskMappingOption>().unwrap(),
            serializer,
        ),
        StructType::HostDiskMappingPartitionOption => HostDiskMappingPartitionOption::serialize(
            p.as_any_ref().downcast_ref::<HostDiskMappingPartitionOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDevice => VirtualDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdrom => VirtualCdrom::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdrom>().unwrap(),
            serializer,
        ),
        StructType::VirtualController => VirtualController::serialize(
            p.as_any_ref().downcast_ref::<VirtualController>().unwrap(),
            serializer,
        ),
        StructType::VirtualIdeController => VirtualIdeController::serialize(
            p.as_any_ref().downcast_ref::<VirtualIdeController>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvdimmController => VirtualNvdimmController::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvdimmController>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvmeController => VirtualNvmeController::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvmeController>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciController => VirtualPciController::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciController>().unwrap(),
            serializer,
        ),
        StructType::VirtualPs2Controller => VirtualPs2Controller::serialize(
            p.as_any_ref().downcast_ref::<VirtualPs2Controller>().unwrap(),
            serializer,
        ),
        StructType::VirtualSataController => VirtualSataController::serialize(
            p.as_any_ref().downcast_ref::<VirtualSataController>().unwrap(),
            serializer,
        ),
        StructType::VirtualAhciController => VirtualAhciController::serialize(
            p.as_any_ref().downcast_ref::<VirtualAhciController>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiController => VirtualScsiController::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiController>().unwrap(),
            serializer,
        ),
        StructType::ParaVirtualScsiController => ParaVirtualScsiController::serialize(
            p.as_any_ref().downcast_ref::<ParaVirtualScsiController>().unwrap(),
            serializer,
        ),
        StructType::VirtualBusLogicController => VirtualBusLogicController::serialize(
            p.as_any_ref().downcast_ref::<VirtualBusLogicController>().unwrap(),
            serializer,
        ),
        StructType::VirtualLsiLogicController => VirtualLsiLogicController::serialize(
            p.as_any_ref().downcast_ref::<VirtualLsiLogicController>().unwrap(),
            serializer,
        ),
        StructType::VirtualLsiLogicSasController => VirtualLsiLogicSasController::serialize(
            p.as_any_ref().downcast_ref::<VirtualLsiLogicSasController>().unwrap(),
            serializer,
        ),
        StructType::VirtualSioController => VirtualSioController::serialize(
            p.as_any_ref().downcast_ref::<VirtualSioController>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbController => VirtualUsbController::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbController>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbxhciController => VirtualUsbxhciController::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbxhciController>().unwrap(),
            serializer,
        ),
        StructType::VirtualDisk => VirtualDisk::serialize(
            p.as_any_ref().downcast_ref::<VirtualDisk>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCard => VirtualEthernetCard::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCard>().unwrap(),
            serializer,
        ),
        StructType::VirtualE1000 => VirtualE1000::serialize(
            p.as_any_ref().downcast_ref::<VirtualE1000>().unwrap(),
            serializer,
        ),
        StructType::VirtualE1000E => VirtualE1000E::serialize(
            p.as_any_ref().downcast_ref::<VirtualE1000E>().unwrap(),
            serializer,
        ),
        StructType::VirtualPcNet32 => VirtualPcNet32::serialize(
            p.as_any_ref().downcast_ref::<VirtualPcNet32>().unwrap(),
            serializer,
        ),
        StructType::VirtualSriovEthernetCard => VirtualSriovEthernetCard::serialize(
            p.as_any_ref().downcast_ref::<VirtualSriovEthernetCard>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet => VirtualVmxnet::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet2 => VirtualVmxnet2::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet2>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet3 => VirtualVmxnet3::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet3>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet3Vrdma => VirtualVmxnet3Vrdma::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppy => VirtualFloppy::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppy>().unwrap(),
            serializer,
        ),
        StructType::VirtualKeyboard => VirtualKeyboard::serialize(
            p.as_any_ref().downcast_ref::<VirtualKeyboard>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvdimm => VirtualNvdimm::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvdimm>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthrough => VirtualPciPassthrough::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthrough>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPort => VirtualParallelPort::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPort>().unwrap(),
            serializer,
        ),
        StructType::VirtualPointingDevice => VirtualPointingDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualPointingDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualPrecisionClock => VirtualPrecisionClock::serialize(
            p.as_any_ref().downcast_ref::<VirtualPrecisionClock>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiPassthrough => VirtualScsiPassthrough::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiPassthrough>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPort => VirtualSerialPort::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPort>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundCard => VirtualSoundCard::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundCard>().unwrap(),
            serializer,
        ),
        StructType::VirtualEnsoniq1371 => VirtualEnsoniq1371::serialize(
            p.as_any_ref().downcast_ref::<VirtualEnsoniq1371>().unwrap(),
            serializer,
        ),
        StructType::VirtualHdAudioCard => VirtualHdAudioCard::serialize(
            p.as_any_ref().downcast_ref::<VirtualHdAudioCard>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundBlaster16 => VirtualSoundBlaster16::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundBlaster16>().unwrap(),
            serializer,
        ),
        StructType::VirtualTpm => VirtualTpm::serialize(
            p.as_any_ref().downcast_ref::<VirtualTpm>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsb => VirtualUsb::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsb>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmciDevice => VirtualMachineVmciDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmciDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmirom => VirtualMachineVmirom::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmirom>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVideoCard => VirtualMachineVideoCard::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVideoCard>().unwrap(),
            serializer,
        ),
        StructType::VirtualWdt => VirtualWdt::serialize(
            p.as_any_ref().downcast_ref::<VirtualWdt>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceBackingInfo => VirtualDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceDeviceBackingInfo => VirtualDeviceDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromAtapiBackingInfo => VirtualCdromAtapiBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromPassthroughBackingInfo => VirtualCdromPassthroughBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskRawDiskVer2BackingInfo => VirtualDiskRawDiskVer2BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo => VirtualDiskPartitionedRawDiskVer2BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardLegacyNetworkBackingInfo => VirtualEthernetCardLegacyNetworkBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardNetworkBackingInfo => VirtualEthernetCardNetworkBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyDeviceBackingInfo => VirtualFloppyDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDeviceBackingInfo => VirtualPciPassthroughDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDynamicBackingInfo => VirtualPciPassthroughDynamicBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPortDeviceBackingInfo => VirtualParallelPortDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPointingDeviceDeviceBackingInfo => VirtualPointingDeviceDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPointingDeviceDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiPassthroughDeviceBackingInfo => VirtualScsiPassthroughDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortDeviceBackingInfo => VirtualSerialPortDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundCardDeviceBackingInfo => VirtualSoundCardDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbRemoteHostBackingInfo => VirtualUsbRemoteHostBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbusbBackingInfo => VirtualUsbusbBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbusbBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceFileBackingInfo => VirtualDeviceFileBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceFileBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromIsoBackingInfo => VirtualCdromIsoBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromIsoBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskFlatVer1BackingInfo => VirtualDiskFlatVer1BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskFlatVer2BackingInfo => VirtualDiskFlatVer2BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskLocalPMemBackingInfo => VirtualDiskLocalPMemBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskRawDiskMappingVer1BackingInfo => VirtualDiskRawDiskMappingVer1BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSeSparseBackingInfo => VirtualDiskSeSparseBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSparseVer1BackingInfo => VirtualDiskSparseVer1BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSparseVer2BackingInfo => VirtualDiskSparseVer2BackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyImageBackingInfo => VirtualFloppyImageBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyImageBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvdimmBackingInfo => VirtualNvdimmBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvdimmBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPortFileBackingInfo => VirtualParallelPortFileBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortFileBackingInfo => VirtualSerialPortFileBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDevicePipeBackingInfo => VirtualDevicePipeBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDevicePipeBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortPipeBackingInfo => VirtualSerialPortPipeBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceRemoteDeviceBackingInfo => VirtualDeviceRemoteDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromRemoteAtapiBackingInfo => VirtualCdromRemoteAtapiBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromRemotePassthroughBackingInfo => VirtualCdromRemotePassthroughBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyRemoteDeviceBackingInfo => VirtualFloppyRemoteDeviceBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbRemoteClientBackingInfo => VirtualUsbRemoteClientBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceUriBackingInfo => VirtualDeviceUriBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceUriBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortUriBackingInfo => VirtualSerialPortUriBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo => VirtualEthernetCardDistributedVirtualPortBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardDistributedVirtualPortBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardOpaqueNetworkBackingInfo => VirtualEthernetCardOpaqueNetworkBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDvxBackingInfo => VirtualPciPassthroughDvxBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughPluginBackingInfo => VirtualPciPassthroughPluginBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughVmiopBackingInfo => VirtualPciPassthroughVmiopBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualPrecisionClockSystemClockBackingInfo => VirtualPrecisionClockSystemClockBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortThinPrintBackingInfo => VirtualSerialPortThinPrintBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualSriovEthernetCardSriovBackingInfo => VirtualSriovEthernetCardSriovBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceBusSlotInfo => VirtualDeviceBusSlotInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceBusSlotInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDevicePciBusSlotInfo => VirtualDevicePciBusSlotInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDevicePciBusSlotInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbControllerPciBusSlotInfo => VirtualUsbControllerPciBusSlotInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbControllerPciBusSlotInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceConnectInfo => VirtualDeviceConnectInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceConnectInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceDeviceGroupInfo => VirtualDeviceDeviceGroupInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceDeviceGroupInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceOption => VirtualDeviceOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromOption => VirtualCdromOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualControllerOption => VirtualControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualIdeControllerOption => VirtualIdeControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualIdeControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvdimmControllerOption => VirtualNvdimmControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvdimmControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvmeControllerOption => VirtualNvmeControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvmeControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciControllerOption => VirtualPciControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPs2ControllerOption => VirtualPs2ControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPs2ControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSataControllerOption => VirtualSataControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSataControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualAhciControllerOption => VirtualAhciControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualAhciControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiControllerOption => VirtualScsiControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiControllerOption>().unwrap(),
            serializer,
        ),
        StructType::ParaVirtualScsiControllerOption => ParaVirtualScsiControllerOption::serialize(
            p.as_any_ref().downcast_ref::<ParaVirtualScsiControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualBusLogicControllerOption => VirtualBusLogicControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualBusLogicControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualLsiLogicControllerOption => VirtualLsiLogicControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualLsiLogicControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualLsiLogicSasControllerOption => VirtualLsiLogicSasControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualLsiLogicSasControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSioControllerOption => VirtualSioControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSioControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbControllerOption => VirtualUsbControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbxhciControllerOption => VirtualUsbxhciControllerOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbxhciControllerOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskOption => VirtualDiskOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardOption => VirtualEthernetCardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualE1000Option => VirtualE1000Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualE1000Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualE1000EOption => VirtualE1000EOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualE1000EOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPcNet32Option => VirtualPcNet32Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualPcNet32Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualSriovEthernetCardOption => VirtualSriovEthernetCardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSriovEthernetCardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnetOption => VirtualVmxnetOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnetOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet2Option => VirtualVmxnet2Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet2Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet3Option => VirtualVmxnet3Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet3Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmxnet3VrdmaOption => VirtualVmxnet3VrdmaOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyOption => VirtualFloppyOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualKeyboardOption => VirtualKeyboardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualKeyboardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualNvdimmOption => VirtualNvdimmOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualNvdimmOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughOption => VirtualPciPassthroughOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPortOption => VirtualParallelPortOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPortOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPointingDeviceOption => VirtualPointingDeviceOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPointingDeviceOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPrecisionClockOption => VirtualPrecisionClockOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPrecisionClockOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiPassthroughOption => VirtualScsiPassthroughOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiPassthroughOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortOption => VirtualSerialPortOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundCardOption => VirtualSoundCardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundCardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEnsoniq1371Option => VirtualEnsoniq1371Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualEnsoniq1371Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualHdAudioCardOption => VirtualHdAudioCardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualHdAudioCardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundBlaster16Option => VirtualSoundBlaster16Option::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundBlaster16Option>().unwrap(),
            serializer,
        ),
        StructType::VirtualTpmOption => VirtualTpmOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualTpmOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbOption => VirtualUsbOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmciDeviceOption => VirtualMachineVmciDeviceOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualVmiromOption => VirtualVmiromOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualVmiromOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualVideoCardOption => VirtualVideoCardOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualVideoCardOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualWdtOption => VirtualWdtOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualWdtOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceBackingOption => VirtualDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceDeviceBackingOption => VirtualDeviceDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromAtapiBackingOption => VirtualCdromAtapiBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromAtapiBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromPassthroughBackingOption => VirtualCdromPassthroughBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromPassthroughBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromRemoteAtapiBackingOption => VirtualCdromRemoteAtapiBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromRemoteAtapiBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskRawDiskMappingVer1BackingOption => VirtualDiskRawDiskMappingVer1BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskRawDiskMappingVer1BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskRawDiskVer2BackingOption => VirtualDiskRawDiskVer2BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskRawDiskVer2BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskPartitionedRawDiskVer2BackingOption => VirtualDiskPartitionedRawDiskVer2BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskPartitionedRawDiskVer2BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardLegacyNetworkBackingOption => VirtualEthernetCardLegacyNetworkBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardLegacyNetworkBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardNetworkBackingOption => VirtualEthernetCardNetworkBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardNetworkBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyDeviceBackingOption => VirtualFloppyDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDeviceBackingOption => VirtualPciPassthroughDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDynamicBackingOption => VirtualPciPassthroughDynamicBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDynamicBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPortDeviceBackingOption => VirtualParallelPortDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPortDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPointingDeviceBackingOption => VirtualPointingDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPointingDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualScsiPassthroughDeviceBackingOption => VirtualScsiPassthroughDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualScsiPassthroughDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortDeviceBackingOption => VirtualSerialPortDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSoundCardDeviceBackingOption => VirtualSoundCardDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSoundCardDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbRemoteHostBackingOption => VirtualUsbRemoteHostBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbRemoteHostBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbusbBackingOption => VirtualUsbusbBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbusbBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceFileBackingOption => VirtualDeviceFileBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceFileBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromIsoBackingOption => VirtualCdromIsoBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromIsoBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskFlatVer1BackingOption => VirtualDiskFlatVer1BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskFlatVer1BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskFlatVer2BackingOption => VirtualDiskFlatVer2BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskFlatVer2BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskLocalPMemBackingOption => VirtualDiskLocalPMemBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskLocalPMemBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSeSparseBackingOption => VirtualDiskSeSparseBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSeSparseBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSparseVer1BackingOption => VirtualDiskSparseVer1BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSparseVer1BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskSparseVer2BackingOption => VirtualDiskSparseVer2BackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskSparseVer2BackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyImageBackingOption => VirtualFloppyImageBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyImageBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualParallelPortFileBackingOption => VirtualParallelPortFileBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualParallelPortFileBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortFileBackingOption => VirtualSerialPortFileBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortFileBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDevicePipeBackingOption => VirtualDevicePipeBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDevicePipeBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortPipeBackingOption => VirtualSerialPortPipeBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortPipeBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceRemoteDeviceBackingOption => VirtualDeviceRemoteDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceRemoteDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualCdromRemotePassthroughBackingOption => VirtualCdromRemotePassthroughBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualCdromRemotePassthroughBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualFloppyRemoteDeviceBackingOption => VirtualFloppyRemoteDeviceBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualFloppyRemoteDeviceBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualUsbRemoteClientBackingOption => VirtualUsbRemoteClientBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualUsbRemoteClientBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceUriBackingOption => VirtualDeviceUriBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceUriBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortUriBackingOption => VirtualSerialPortUriBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortUriBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardDvPortBackingOption => VirtualEthernetCardDvPortBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardDvPortBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardOpaqueNetworkBackingOption => VirtualEthernetCardOpaqueNetworkBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardOpaqueNetworkBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughDvxBackingOption => VirtualPciPassthroughDvxBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughDvxBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughPluginBackingOption => VirtualPciPassthroughPluginBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughVmiopBackingOption => VirtualPciPassthroughVmiopBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualPrecisionClockSystemClockBackingOption => VirtualPrecisionClockSystemClockBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualPrecisionClockSystemClockBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSerialPortThinPrintBackingOption => VirtualSerialPortThinPrintBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSerialPortThinPrintBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualSriovEthernetCardSriovBackingOption => VirtualSriovEthernetCardSriovBackingOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualSriovEthernetCardSriovBackingOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceBusSlotOption => VirtualDeviceBusSlotOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceBusSlotOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceConnectOption => VirtualDeviceConnectOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceConnectOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceConfigSpec => VirtualDeviceConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskConfigSpec => VirtualDiskConfigSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskConfigSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualDeviceConfigSpecBackingSpec => VirtualDeviceConfigSpecBackingSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualDeviceConfigSpecBackingSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskVFlashCacheConfigInfo => VirtualDiskVFlashCacheConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskVFlashCacheConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskId => VirtualDiskId::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskId>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskDeltaDiskFormatsSupported => VirtualDiskDeltaDiskFormatsSupported::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskDeltaDiskFormatsSupported>().unwrap(),
            serializer,
        ),
        StructType::VirtualDiskOptionVFlashCacheConfigOption => VirtualDiskOptionVFlashCacheConfigOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualDiskOptionVFlashCacheConfigOption>().unwrap(),
            serializer,
        ),
        StructType::VirtualEthernetCardResourceAllocation => VirtualEthernetCardResourceAllocation::serialize(
            p.as_any_ref().downcast_ref::<VirtualEthernetCardResourceAllocation>().unwrap(),
            serializer,
        ),
        StructType::VirtualPciPassthroughAllowedDevice => VirtualPciPassthroughAllowedDevice::serialize(
            p.as_any_ref().downcast_ref::<VirtualPciPassthroughAllowedDevice>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmciDeviceFilterInfo => VirtualMachineVmciDeviceFilterInfo::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceFilterInfo>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmciDeviceFilterSpec => VirtualMachineVmciDeviceFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::VirtualMachineVmciDeviceOptionFilterSpecOption => VirtualMachineVmciDeviceOptionFilterSpecOption::serialize(
            p.as_any_ref().downcast_ref::<VirtualMachineVmciDeviceOptionFilterSpecOption>().unwrap(),
            serializer,
        ),
        StructType::GuestAliases => GuestAliases::serialize(
            p.as_any_ref().downcast_ref::<GuestAliases>().unwrap(),
            serializer,
        ),
        StructType::GuestAuthAliasInfo => GuestAuthAliasInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestAuthAliasInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestAuthSubject => GuestAuthSubject::serialize(
            p.as_any_ref().downcast_ref::<GuestAuthSubject>().unwrap(),
            serializer,
        ),
        StructType::GuestAuthAnySubject => GuestAuthAnySubject::serialize(
            p.as_any_ref().downcast_ref::<GuestAuthAnySubject>().unwrap(),
            serializer,
        ),
        StructType::GuestAuthNamedSubject => GuestAuthNamedSubject::serialize(
            p.as_any_ref().downcast_ref::<GuestAuthNamedSubject>().unwrap(),
            serializer,
        ),
        StructType::GuestMappedAliases => GuestMappedAliases::serialize(
            p.as_any_ref().downcast_ref::<GuestMappedAliases>().unwrap(),
            serializer,
        ),
        StructType::GuestFileAttributes => GuestFileAttributes::serialize(
            p.as_any_ref().downcast_ref::<GuestFileAttributes>().unwrap(),
            serializer,
        ),
        StructType::GuestPosixFileAttributes => GuestPosixFileAttributes::serialize(
            p.as_any_ref().downcast_ref::<GuestPosixFileAttributes>().unwrap(),
            serializer,
        ),
        StructType::GuestWindowsFileAttributes => GuestWindowsFileAttributes::serialize(
            p.as_any_ref().downcast_ref::<GuestWindowsFileAttributes>().unwrap(),
            serializer,
        ),
        StructType::GuestFileInfo => GuestFileInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestFileInfo>().unwrap(),
            serializer,
        ),
        StructType::FileTransferInformation => FileTransferInformation::serialize(
            p.as_any_ref().downcast_ref::<FileTransferInformation>().unwrap(),
            serializer,
        ),
        StructType::GuestListFileInfo => GuestListFileInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestListFileInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestAuthentication => GuestAuthentication::serialize(
            p.as_any_ref().downcast_ref::<GuestAuthentication>().unwrap(),
            serializer,
        ),
        StructType::NamePasswordAuthentication => NamePasswordAuthentication::serialize(
            p.as_any_ref().downcast_ref::<NamePasswordAuthentication>().unwrap(),
            serializer,
        ),
        StructType::SamlTokenAuthentication => SamlTokenAuthentication::serialize(
            p.as_any_ref().downcast_ref::<SamlTokenAuthentication>().unwrap(),
            serializer,
        ),
        StructType::SspiAuthentication => SspiAuthentication::serialize(
            p.as_any_ref().downcast_ref::<SspiAuthentication>().unwrap(),
            serializer,
        ),
        StructType::TicketedSessionAuthentication => TicketedSessionAuthentication::serialize(
            p.as_any_ref().downcast_ref::<TicketedSessionAuthentication>().unwrap(),
            serializer,
        ),
        StructType::GuestProcessInfo => GuestProcessInfo::serialize(
            p.as_any_ref().downcast_ref::<GuestProcessInfo>().unwrap(),
            serializer,
        ),
        StructType::GuestProgramSpec => GuestProgramSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestProgramSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestWindowsProgramSpec => GuestWindowsProgramSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestWindowsProgramSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegKeySpec => GuestRegKeySpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegKeySpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegKeyNameSpec => GuestRegKeyNameSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegKeyNameSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegKeyRecordSpec => GuestRegKeyRecordSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegKeyRecordSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueSpec => GuestRegValueSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueDataSpec => GuestRegValueDataSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueDataSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueBinarySpec => GuestRegValueBinarySpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueBinarySpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueDwordSpec => GuestRegValueDwordSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueDwordSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueExpandStringSpec => GuestRegValueExpandStringSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueExpandStringSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueMultiStringSpec => GuestRegValueMultiStringSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueMultiStringSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueQwordSpec => GuestRegValueQwordSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueQwordSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueStringSpec => GuestRegValueStringSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueStringSpec>().unwrap(),
            serializer,
        ),
        StructType::GuestRegValueNameSpec => GuestRegValueNameSpec::serialize(
            p.as_any_ref().downcast_ref::<GuestRegValueNameSpec>().unwrap(),
            serializer,
        ),
        StructType::DeviceGroupId => DeviceGroupId::serialize(
            p.as_any_ref().downcast_ref::<DeviceGroupId>().unwrap(),
            serializer,
        ),
        StructType::FaultDomainId => FaultDomainId::serialize(
            p.as_any_ref().downcast_ref::<FaultDomainId>().unwrap(),
            serializer,
        ),
        StructType::ReplicationGroupId => ReplicationGroupId::serialize(
            p.as_any_ref().downcast_ref::<ReplicationGroupId>().unwrap(),
            serializer,
        ),
        StructType::ReplicationSpec => ReplicationSpec::serialize(
            p.as_any_ref().downcast_ref::<ReplicationSpec>().unwrap(),
            serializer,
        ),
        StructType::VsanClusterConfigInfo => VsanClusterConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanClusterConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanClusterConfigInfoHostDefaultInfo => VsanClusterConfigInfoHostDefaultInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanClusterConfigInfoHostDefaultInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostClusterStatus => VsanHostClusterStatus::serialize(
            p.as_any_ref().downcast_ref::<VsanHostClusterStatus>().unwrap(),
            serializer,
        ),
        StructType::VsanHostClusterStatusState => VsanHostClusterStatusState::serialize(
            p.as_any_ref().downcast_ref::<VsanHostClusterStatusState>().unwrap(),
            serializer,
        ),
        StructType::VsanHostClusterStatusStateCompletionEstimate => VsanHostClusterStatusStateCompletionEstimate::serialize(
            p.as_any_ref().downcast_ref::<VsanHostClusterStatusStateCompletionEstimate>().unwrap(),
            serializer,
        ),
        StructType::VsanHostConfigInfo => VsanHostConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostConfigInfoClusterInfo => VsanHostConfigInfoClusterInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostConfigInfoClusterInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostFaultDomainInfo => VsanHostFaultDomainInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostFaultDomainInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostConfigInfoNetworkInfo => VsanHostConfigInfoNetworkInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostConfigInfoNetworkInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostConfigInfoNetworkInfoPortConfig => VsanHostConfigInfoNetworkInfoPortConfig::serialize(
            p.as_any_ref().downcast_ref::<VsanHostConfigInfoNetworkInfoPortConfig>().unwrap(),
            serializer,
        ),
        StructType::VsanHostConfigInfoStorageInfo => VsanHostConfigInfoStorageInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostConfigInfoStorageInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostDecommissionMode => VsanHostDecommissionMode::serialize(
            p.as_any_ref().downcast_ref::<VsanHostDecommissionMode>().unwrap(),
            serializer,
        ),
        StructType::VsanHostDiskMapInfo => VsanHostDiskMapInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostDiskMapInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostDiskMapResult => VsanHostDiskMapResult::serialize(
            p.as_any_ref().downcast_ref::<VsanHostDiskMapResult>().unwrap(),
            serializer,
        ),
        StructType::VsanHostDiskMapping => VsanHostDiskMapping::serialize(
            p.as_any_ref().downcast_ref::<VsanHostDiskMapping>().unwrap(),
            serializer,
        ),
        StructType::VsanHostDiskResult => VsanHostDiskResult::serialize(
            p.as_any_ref().downcast_ref::<VsanHostDiskResult>().unwrap(),
            serializer,
        ),
        StructType::VsanHostIpConfig => VsanHostIpConfig::serialize(
            p.as_any_ref().downcast_ref::<VsanHostIpConfig>().unwrap(),
            serializer,
        ),
        StructType::VsanHostMembershipInfo => VsanHostMembershipInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostMembershipInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostVsanDiskInfo => VsanHostVsanDiskInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostVsanDiskInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostRuntimeInfo => VsanHostRuntimeInfo::serialize(
            p.as_any_ref().downcast_ref::<VsanHostRuntimeInfo>().unwrap(),
            serializer,
        ),
        StructType::VsanHostRuntimeInfoDiskIssue => VsanHostRuntimeInfoDiskIssue::serialize(
            p.as_any_ref().downcast_ref::<VsanHostRuntimeInfoDiskIssue>().unwrap(),
            serializer,
        ),
        StructType::BaseConfigInfo => BaseConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<BaseConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectConfigInfo => VStorageObjectConfigInfo::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectConfigInfo>().unwrap(),
            serializer,
        ),
        StructType::BaseConfigInfoBackingInfo => BaseConfigInfoBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<BaseConfigInfoBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::BaseConfigInfoFileBackingInfo => BaseConfigInfoFileBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<BaseConfigInfoFileBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::BaseConfigInfoDiskFileBackingInfo => BaseConfigInfoDiskFileBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<BaseConfigInfoDiskFileBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::BaseConfigInfoRawDiskMappingBackingInfo => BaseConfigInfoRawDiskMappingBackingInfo::serialize(
            p.as_any_ref().downcast_ref::<BaseConfigInfoRawDiskMappingBackingInfo>().unwrap(),
            serializer,
        ),
        StructType::VslmCreateSpec => VslmCreateSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmCreateSpec>().unwrap(),
            serializer,
        ),
        StructType::VslmCreateSpecBackingSpec => VslmCreateSpecBackingSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmCreateSpecBackingSpec>().unwrap(),
            serializer,
        ),
        StructType::VslmCreateSpecDiskFileBackingSpec => VslmCreateSpecDiskFileBackingSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmCreateSpecDiskFileBackingSpec>().unwrap(),
            serializer,
        ),
        StructType::VslmCreateSpecRawDiskMappingBackingSpec => VslmCreateSpecRawDiskMappingBackingSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmCreateSpecRawDiskMappingBackingSpec>().unwrap(),
            serializer,
        ),
        StructType::DiskCryptoSpec => DiskCryptoSpec::serialize(
            p.as_any_ref().downcast_ref::<DiskCryptoSpec>().unwrap(),
            serializer,
        ),
        StructType::Id => Id::serialize(
            p.as_any_ref().downcast_ref::<Id>().unwrap(),
            serializer,
        ),
        StructType::VslmInfrastructureObjectPolicy => VslmInfrastructureObjectPolicy::serialize(
            p.as_any_ref().downcast_ref::<VslmInfrastructureObjectPolicy>().unwrap(),
            serializer,
        ),
        StructType::VslmInfrastructureObjectPolicySpec => VslmInfrastructureObjectPolicySpec::serialize(
            p.as_any_ref().downcast_ref::<VslmInfrastructureObjectPolicySpec>().unwrap(),
            serializer,
        ),
        StructType::VslmMigrateSpec => VslmMigrateSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmMigrateSpec>().unwrap(),
            serializer,
        ),
        StructType::VslmCloneSpec => VslmCloneSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmCloneSpec>().unwrap(),
            serializer,
        ),
        StructType::VslmRelocateSpec => VslmRelocateSpec::serialize(
            p.as_any_ref().downcast_ref::<VslmRelocateSpec>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectStateInfo => VStorageObjectStateInfo::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectStateInfo>().unwrap(),
            serializer,
        ),
        StructType::VslmTagEntry => VslmTagEntry::serialize(
            p.as_any_ref().downcast_ref::<VslmTagEntry>().unwrap(),
            serializer,
        ),
        StructType::VslmVClockInfo => VslmVClockInfo::serialize(
            p.as_any_ref().downcast_ref::<VslmVClockInfo>().unwrap(),
            serializer,
        ),
        StructType::VStorageObject => VStorageObject::serialize(
            p.as_any_ref().downcast_ref::<VStorageObject>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectSnapshot => VStorageObjectSnapshot::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectSnapshot>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectSnapshotDetails => VStorageObjectSnapshotDetails::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectSnapshotDetails>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectSnapshotInfo => VStorageObjectSnapshotInfo::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectSnapshotInfo>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectSnapshotInfoVStorageObjectSnapshot => VStorageObjectSnapshotInfoVStorageObjectSnapshot::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectSnapshotInfoVStorageObjectSnapshot>().unwrap(),
            serializer,
        ),
        StructType::RetrieveVStorageObjSpec => RetrieveVStorageObjSpec::serialize(
            p.as_any_ref().downcast_ref::<RetrieveVStorageObjSpec>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectAssociations => VStorageObjectAssociations::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectAssociations>().unwrap(),
            serializer,
        ),
        StructType::VStorageObjectAssociationsVmDiskAssociations => VStorageObjectAssociationsVmDiskAssociations::serialize(
            p.as_any_ref().downcast_ref::<VStorageObjectAssociationsVmDiskAssociations>().unwrap(),
            serializer,
        ),
        StructType::DynamicArray => DynamicArray::serialize(
            p.as_any_ref().downcast_ref::<DynamicArray>().unwrap(),
            serializer,
        ),
        StructType::DynamicProperty => DynamicProperty::serialize(
            p.as_any_ref().downcast_ref::<DynamicProperty>().unwrap(),
            serializer,
        ),
        StructType::KeyAnyValue => KeyAnyValue::serialize(
            p.as_any_ref().downcast_ref::<KeyAnyValue>().unwrap(),
            serializer,
        ),
        StructType::LocalizableMessage => LocalizableMessage::serialize(
            p.as_any_ref().downcast_ref::<LocalizableMessage>().unwrap(),
            serializer,
        ),
        StructType::LocalizedMethodFault => LocalizedMethodFault::serialize(
            p.as_any_ref().downcast_ref::<LocalizedMethodFault>().unwrap(),
            serializer,
        ),
        StructType::PropertyChange => PropertyChange::serialize(
            p.as_any_ref().downcast_ref::<PropertyChange>().unwrap(),
            serializer,
        ),
        StructType::PropertyFilterSpec => PropertyFilterSpec::serialize(
            p.as_any_ref().downcast_ref::<PropertyFilterSpec>().unwrap(),
            serializer,
        ),
        StructType::PropertyFilterUpdate => PropertyFilterUpdate::serialize(
            p.as_any_ref().downcast_ref::<PropertyFilterUpdate>().unwrap(),
            serializer,
        ),
        StructType::MissingObject => MissingObject::serialize(
            p.as_any_ref().downcast_ref::<MissingObject>().unwrap(),
            serializer,
        ),
        StructType::MissingProperty => MissingProperty::serialize(
            p.as_any_ref().downcast_ref::<MissingProperty>().unwrap(),
            serializer,
        ),
        StructType::ObjectContent => ObjectContent::serialize(
            p.as_any_ref().downcast_ref::<ObjectContent>().unwrap(),
            serializer,
        ),
        StructType::ObjectSpec => ObjectSpec::serialize(
            p.as_any_ref().downcast_ref::<ObjectSpec>().unwrap(),
            serializer,
        ),
        StructType::ObjectUpdate => ObjectUpdate::serialize(
            p.as_any_ref().downcast_ref::<ObjectUpdate>().unwrap(),
            serializer,
        ),
        StructType::PropertySpec => PropertySpec::serialize(
            p.as_any_ref().downcast_ref::<PropertySpec>().unwrap(),
            serializer,
        ),
        StructType::RetrieveOptions => RetrieveOptions::serialize(
            p.as_any_ref().downcast_ref::<RetrieveOptions>().unwrap(),
            serializer,
        ),
        StructType::RetrieveResult => RetrieveResult::serialize(
            p.as_any_ref().downcast_ref::<RetrieveResult>().unwrap(),
            serializer,
        ),
        StructType::SelectionSpec => SelectionSpec::serialize(
            p.as_any_ref().downcast_ref::<SelectionSpec>().unwrap(),
            serializer,
        ),
        StructType::TraversalSpec => TraversalSpec::serialize(
            p.as_any_ref().downcast_ref::<TraversalSpec>().unwrap(),
            serializer,
        ),
        StructType::UpdateSet => UpdateSet::serialize(
            p.as_any_ref().downcast_ref::<UpdateSet>().unwrap(),
            serializer,
        ),
        StructType::WaitOptions => WaitOptions::serialize(
            p.as_any_ref().downcast_ref::<WaitOptions>().unwrap(),
            serializer,
        ),
        StructType::MethodFault => MethodFault::serialize(
            p.as_any_ref().downcast_ref::<MethodFault>().unwrap(),
            serializer,
        ),
        _ => Err(serde::ser::Error::custom("Unknown VIM data type")),
    }
}
