use super::vim_any::VimAny;
use super::structs::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(tag = "_typeName", content = "_value")]
pub enum ValueElements {
    /// A boxed array of *Any*. To be used in *Any* placeholders.
    ArrayOfAnyType(Vec<VimAny>),
    /// A boxed array of *ManagedObjectReference*. To be used in *Any* placeholders.
    ArrayOfManagedObjectReference(Vec<ManagedObjectReference>),
    /// A boxed Boolean primitive. To be used in *Any* placeholders.
    #[serde(rename = "boolean")]
    PrimitiveBoolean(bool),
    /// A boxed array of *PrimitiveBoolean*. To be used in *Any* placeholders.
    ArrayOfBoolean(Vec<bool>),
    /// A boxed Byte primitive. To be used in *Any* placeholders.
    #[serde(rename = "byte")]
    PrimitiveByte(i8),
    /// A boxed array of *PrimitiveByte*. To be used in *Any* placeholders.
    ArrayOfByte(Vec<i8>),
    /// A boxed Short primitive. To be used in *Any* placeholders.
    #[serde(rename = "short")]
    PrimitiveShort(i16),
    /// A boxed array of *PrimitiveShort*. To be used in *Any* placeholders.
    ArrayOfShort(Vec<i16>),
    /// A boxed Int primitive. To be used in *Any* placeholders.
    #[serde(rename = "int")]
    PrimitiveInt(i32),
    /// A boxed array of *PrimitiveInt*. To be used in *Any* placeholders.
    ArrayOfInt(Vec<i32>),
    /// A boxed Long primitive. To be used in *Any* placeholders.
    #[serde(rename = "long")]
    PrimitiveLong(i64),
    /// A boxed array of *PrimitiveLong*. To be used in *Any* placeholders.
    ArrayOfLong(Vec<i64>),
    /// A boxed Float primitive. To be used in *Any* placeholders.
    #[serde(rename = "float")]
    PrimitiveFloat(f32),
    /// A boxed array of *PrimitiveFloat*. To be used in *Any* placeholders.
    ArrayOfFloat(Vec<f32>),
    /// A boxed Double primitive. To be used in *Any* placeholders.
    #[serde(rename = "double")]
    PrimitiveDouble(f64),
    /// A boxed array of *PrimitiveDouble*. To be used in *Any* placeholders.
    ArrayOfDouble(Vec<f64>),
    /// A boxed String primitive. To be used in *Any* placeholders.
    #[serde(rename = "string")]
    PrimitiveString(String),
    /// A boxed array of *PrimitiveString*. To be used in *Any* placeholders.
    ArrayOfString(Vec<String>),
    /// A boxed DateTime primitive. To be used in *Any* placeholders.
    #[serde(rename = "dateTime")]
    PrimitiveDateTime(String),
    /// A boxed array of *PrimitiveDateTime*. To be used in *Any* placeholders.
    ArrayOfDateTime(Vec<String>),
    /// A boxed URI primitive. To be used in *Any* placeholders.
    #[serde(rename = "anyURI")]
    PrimitiveUri(String),
    /// A boxed array of *PrimitiveURI*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfAnyURI")]
    ArrayOfUri(Vec<String>),
    /// A boxed Binary primitive. To be used in *Any* placeholders.
    #[serde(rename = "base64Binary")]
    PrimitiveBinary(Vec<u8>),
    /// A boxed array of *PrimitiveBinary*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfBase64Binary")]
    ArrayOfBinary(Vec<Vec<u8>>),
    /// A boxed TypeName primitive. To be used in *Any* placeholders.
    #[serde(rename = "TypeName")]
    PrimitiveTypeName(String),
    /// A boxed array of *PrimitiveTypeName*. To be used in *Any* placeholders.
    ArrayOfTypeName(Vec<String>),
    /// A boxed PropPath primitive. To be used in *Any* placeholders.
    #[serde(rename = "PropertyPath")]
    PrimitivePropPath(String),
    /// A boxed array of *PrimitivePropPath*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfPropertyPath")]
    ArrayOfPropPath(Vec<String>),
    /// A boxed MethodName primitive. To be used in *Any* placeholders.
    #[serde(rename = "MethodName")]
    PrimitiveMethodName(String),
    /// A boxed array of *PrimitiveMethodName*. To be used in *Any* placeholders.
    ArrayOfMethodName(Vec<String>),
    /// A boxed array of *AboutInfo*. To be used in *Any* placeholders.
    ArrayOfAboutInfo(Vec<AboutInfo>),
    /// A boxed array of *AuthorizationDescription*. To be used in *Any* placeholders.
    ArrayOfAuthorizationDescription(Vec<AuthorizationDescription>),
    /// A boxed array of *EntityPrivilege*. To be used in *Any* placeholders.
    ArrayOfEntityPrivilege(Vec<EntityPrivilege>),
    /// A boxed array of *Permission*. To be used in *Any* placeholders.
    ArrayOfPermission(Vec<Permission>),
    /// A boxed array of *AuthorizationPrivilege*. To be used in *Any* placeholders.
    ArrayOfAuthorizationPrivilege(Vec<AuthorizationPrivilege>),
    /// A boxed array of *PrivilegeAvailability*. To be used in *Any* placeholders.
    ArrayOfPrivilegeAvailability(Vec<PrivilegeAvailability>),
    /// A boxed array of *AuthorizationRole*. To be used in *Any* placeholders.
    ArrayOfAuthorizationRole(Vec<AuthorizationRole>),
    /// A boxed array of *UserPrivilegeResult*. To be used in *Any* placeholders.
    ArrayOfUserPrivilegeResult(Vec<UserPrivilegeResult>),
    /// A boxed array of *BatchResult*. To be used in *Any* placeholders.
    ArrayOfBatchResult(Vec<BatchResult>),
    /// A boxed array of *BoolPolicy*. To be used in *Any* placeholders.
    ArrayOfBoolPolicy(Vec<BoolPolicy>),
    /// A boxed array of *Capability*. To be used in *Any* placeholders.
    ArrayOfCapability(Vec<Capability>),
    /// A boxed array of *ClusterComputeResourceClusterConfigResult*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceClusterConfigResult(Vec<ClusterComputeResourceClusterConfigResult>),
    /// A boxed array of *ClusterComputeResourceDVSConfigurationValidation*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceDVSConfigurationValidation")]
    ArrayOfClusterComputeResourceDvsConfigurationValidation(Vec<ClusterComputeResourceDvsConfigurationValidation>),
    /// A boxed array of *ClusterComputeResourceDVSSetting*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceDVSSetting")]
    ArrayOfClusterComputeResourceDvsSetting(Vec<ClusterComputeResourceDvsSetting>),
    /// A boxed array of *ClusterComputeResourceDVSSettingDVPortgroupToServiceMapping*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceDVSSettingDVPortgroupToServiceMapping")]
    ArrayOfClusterComputeResourceDvsSettingDvPortgroupToServiceMapping(Vec<ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping>),
    /// A boxed array of *ClusterComputeResourceDvsProfile*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceDvsProfile(Vec<ClusterComputeResourceDvsProfile>),
    /// A boxed array of *ClusterComputeResourceDvsProfileDVPortgroupSpecToServiceMapping*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceDvsProfileDVPortgroupSpecToServiceMapping")]
    ArrayOfClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping(Vec<ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping>),
    /// A boxed array of *ClusterComputeResourceHCIConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceHCIConfigInfo")]
    ArrayOfClusterComputeResourceHciConfigInfo(Vec<ClusterComputeResourceHciConfigInfo>),
    /// A boxed array of *ClusterComputeResourceHCIConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceHCIConfigSpec")]
    ArrayOfClusterComputeResourceHciConfigSpec(Vec<ClusterComputeResourceHciConfigSpec>),
    /// A boxed array of *ClusterComputeResourceHostConfigurationInput*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceHostConfigurationInput(Vec<ClusterComputeResourceHostConfigurationInput>),
    /// A boxed array of *ClusterComputeResourceHostConfigurationProfile*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceHostConfigurationProfile(Vec<ClusterComputeResourceHostConfigurationProfile>),
    /// A boxed array of *ClusterComputeResourceHostConfigurationValidation*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceHostConfigurationValidation(Vec<ClusterComputeResourceHostConfigurationValidation>),
    /// A boxed array of *ClusterComputeResourceHostVmkNicInfo*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceHostVmkNicInfo(Vec<ClusterComputeResourceHostVmkNicInfo>),
    /// A boxed array of *ClusterComputeResourceSummary*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceSummary(Vec<ClusterComputeResourceSummary>),
    /// A boxed array of *ClusterComputeResourceVCProfile*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterComputeResourceVCProfile")]
    ArrayOfClusterComputeResourceVcProfile(Vec<ClusterComputeResourceVcProfile>),
    /// A boxed array of *ClusterComputeResourceValidationResultBase*. To be used in *Any* placeholders.
    ArrayOfClusterComputeResourceValidationResultBase(Vec<Box<dyn super::traits::ClusterComputeResourceValidationResultBaseTrait>>),
    /// A boxed array of *ClusterComputeResourceVcsSlots*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.1
    ArrayOfClusterComputeResourceVcsSlots(Vec<ClusterComputeResourceVcsSlots>),
    /// A boxed array of *ComputeResourceConfigInfo*. To be used in *Any* placeholders.
    ArrayOfComputeResourceConfigInfo(Vec<Box<dyn super::traits::ComputeResourceConfigInfoTrait>>),
    /// A boxed array of *ComputeResourceConfigSpec*. To be used in *Any* placeholders.
    ArrayOfComputeResourceConfigSpec(Vec<Box<dyn super::traits::ComputeResourceConfigSpecTrait>>),
    /// A boxed array of *ComputeResourceHostSPBMLicenseInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfComputeResourceHostSPBMLicenseInfo")]
    ArrayOfComputeResourceHostSpbmLicenseInfo(Vec<ComputeResourceHostSpbmLicenseInfo>),
    /// A boxed array of *ComputeResourceSummary*. To be used in *Any* placeholders.
    ArrayOfComputeResourceSummary(Vec<Box<dyn super::traits::ComputeResourceSummaryTrait>>),
    /// A boxed array of *CustomFieldDef*. To be used in *Any* placeholders.
    ArrayOfCustomFieldDef(Vec<CustomFieldDef>),
    /// A boxed array of *CustomFieldStringValue*. To be used in *Any* placeholders.
    ArrayOfCustomFieldStringValue(Vec<CustomFieldStringValue>),
    /// A boxed array of *CustomFieldValue*. To be used in *Any* placeholders.
    ArrayOfCustomFieldValue(Vec<Box<dyn super::traits::CustomFieldValueTrait>>),
    /// A boxed array of *CustomizationSpecInfo*. To be used in *Any* placeholders.
    ArrayOfCustomizationSpecInfo(Vec<CustomizationSpecInfo>),
    /// A boxed array of *CustomizationSpecItem*. To be used in *Any* placeholders.
    ArrayOfCustomizationSpecItem(Vec<CustomizationSpecItem>),
    /// A boxed array of *DatacenterBasicConnectInfo*. To be used in *Any* placeholders.
    ArrayOfDatacenterBasicConnectInfo(Vec<DatacenterBasicConnectInfo>),
    /// A boxed array of *DatacenterConfigInfo*. To be used in *Any* placeholders.
    ArrayOfDatacenterConfigInfo(Vec<DatacenterConfigInfo>),
    /// A boxed array of *DatacenterConfigSpec*. To be used in *Any* placeholders.
    ArrayOfDatacenterConfigSpec(Vec<DatacenterConfigSpec>),
    /// A boxed array of *DatastoreCapability*. To be used in *Any* placeholders.
    ArrayOfDatastoreCapability(Vec<DatastoreCapability>),
    /// A boxed array of *DatastoreHostMount*. To be used in *Any* placeholders.
    ArrayOfDatastoreHostMount(Vec<DatastoreHostMount>),
    /// A boxed array of *DatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfDatastoreInfo(Vec<Box<dyn super::traits::DatastoreInfoTrait>>),
    /// A boxed array of *DatastoreMountPathDatastorePair*. To be used in *Any* placeholders.
    ArrayOfDatastoreMountPathDatastorePair(Vec<DatastoreMountPathDatastorePair>),
    /// A boxed array of *DatastoreSummary*. To be used in *Any* placeholders.
    ArrayOfDatastoreSummary(Vec<DatastoreSummary>),
    /// A boxed array of *DatastoreVVolContainerFailoverPair*. To be used in *Any* placeholders.
    ArrayOfDatastoreVVolContainerFailoverPair(Vec<DatastoreVVolContainerFailoverPair>),
    /// A boxed array of *DatastoreNamespaceManagerDirectoryInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfDatastoreNamespaceManagerDirectoryInfo(Vec<DatastoreNamespaceManagerDirectoryInfo>),
    /// A boxed array of *Description*. To be used in *Any* placeholders.
    ArrayOfDescription(Vec<Box<dyn super::traits::DescriptionTrait>>),
    /// A boxed array of *DesiredSoftwareSpec*. To be used in *Any* placeholders.
    ArrayOfDesiredSoftwareSpec(Vec<DesiredSoftwareSpec>),
    /// A boxed array of *DesiredSoftwareSpecBaseImageSpec*. To be used in *Any* placeholders.
    ArrayOfDesiredSoftwareSpecBaseImageSpec(Vec<DesiredSoftwareSpecBaseImageSpec>),
    /// A boxed array of *DesiredSoftwareSpecComponentSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ArrayOfDesiredSoftwareSpecComponentSpec(Vec<DesiredSoftwareSpecComponentSpec>),
    /// A boxed array of *DesiredSoftwareSpecVendorAddOnSpec*. To be used in *Any* placeholders.
    ArrayOfDesiredSoftwareSpecVendorAddOnSpec(Vec<DesiredSoftwareSpecVendorAddOnSpec>),
    /// A boxed array of *DiagnosticManagerAuditRecordResult*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfDiagnosticManagerAuditRecordResult(Vec<DiagnosticManagerAuditRecordResult>),
    /// A boxed array of *DiagnosticManagerBundleInfo*. To be used in *Any* placeholders.
    ArrayOfDiagnosticManagerBundleInfo(Vec<DiagnosticManagerBundleInfo>),
    /// A boxed array of *DiagnosticManagerLogDescriptor*. To be used in *Any* placeholders.
    ArrayOfDiagnosticManagerLogDescriptor(Vec<DiagnosticManagerLogDescriptor>),
    /// A boxed array of *DiagnosticManagerLogHeader*. To be used in *Any* placeholders.
    ArrayOfDiagnosticManagerLogHeader(Vec<DiagnosticManagerLogHeader>),
    /// A boxed array of *DVSBackupRestoreCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSBackupRestoreCapability")]
    ArrayOfDvsBackupRestoreCapability(Vec<DvsBackupRestoreCapability>),
    /// A boxed array of *DVSCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSCapability")]
    ArrayOfDvsCapability(Vec<DvsCapability>),
    /// A boxed array of *DVSConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSConfigInfo")]
    ArrayOfDvsConfigInfo(Vec<Box<dyn super::traits::DvsConfigInfoTrait>>),
    /// A boxed array of *DVSConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSConfigSpec")]
    ArrayOfDvsConfigSpec(Vec<Box<dyn super::traits::DvsConfigSpecTrait>>),
    /// A boxed array of *DVSContactInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSContactInfo")]
    ArrayOfDvsContactInfo(Vec<DvsContactInfo>),
    /// A boxed array of *DVSCreateSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSCreateSpec")]
    ArrayOfDvsCreateSpec(Vec<DvsCreateSpec>),
    /// A boxed array of *DVSFeatureCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSFeatureCapability")]
    ArrayOfDvsFeatureCapability(Vec<Box<dyn super::traits::DvsFeatureCapabilityTrait>>),
    /// A boxed array of *DVSHealthCheckConfig*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSHealthCheckConfig")]
    ArrayOfDvsHealthCheckConfig(Vec<Box<dyn super::traits::DvsHealthCheckConfigTrait>>),
    /// A boxed array of *DVSHealthCheckCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSHealthCheckCapability")]
    ArrayOfDvsHealthCheckCapability(Vec<Box<dyn super::traits::DvsHealthCheckCapabilityTrait>>),
    /// A boxed array of *DvsHostInfrastructureTrafficResource*. To be used in *Any* placeholders.
    ArrayOfDvsHostInfrastructureTrafficResource(Vec<DvsHostInfrastructureTrafficResource>),
    /// A boxed array of *DvsHostInfrastructureTrafficResourceAllocation*. To be used in *Any* placeholders.
    ArrayOfDvsHostInfrastructureTrafficResourceAllocation(Vec<DvsHostInfrastructureTrafficResourceAllocation>),
    /// A boxed array of *DVSNameArrayUplinkPortPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSNameArrayUplinkPortPolicy")]
    ArrayOfDvsNameArrayUplinkPortPolicy(Vec<DvsNameArrayUplinkPortPolicy>),
    /// A boxed array of *DVSNetworkResourceManagementCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSNetworkResourceManagementCapability")]
    ArrayOfDvsNetworkResourceManagementCapability(Vec<DvsNetworkResourceManagementCapability>),
    /// A boxed array of *DvsResourceRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfDvsResourceRuntimeInfo(Vec<DvsResourceRuntimeInfo>),
    /// A boxed array of *DVSRollbackCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSRollbackCapability")]
    ArrayOfDvsRollbackCapability(Vec<DvsRollbackCapability>),
    /// A boxed array of *DVSRuntimeInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSRuntimeInfo")]
    ArrayOfDvsRuntimeInfo(Vec<DvsRuntimeInfo>),
    /// A boxed array of *DVSSummary*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSSummary")]
    ArrayOfDvsSummary(Vec<DvsSummary>),
    /// A boxed array of *DVSPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSPolicy")]
    ArrayOfDvsPolicy(Vec<DvsPolicy>),
    /// A boxed array of *DVSUplinkPortPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSUplinkPortPolicy")]
    ArrayOfDvsUplinkPortPolicy(Vec<Box<dyn super::traits::DvsUplinkPortPolicyTrait>>),
    /// A boxed array of *EVCMode*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCMode")]
    ArrayOfEvcMode(Vec<EvcMode>),
    /// A boxed array of *ElementDescription*. To be used in *Any* placeholders.
    ArrayOfElementDescription(Vec<Box<dyn super::traits::ElementDescriptionTrait>>),
    /// A boxed array of *EnumDescription*. To be used in *Any* placeholders.
    ArrayOfEnumDescription(Vec<EnumDescription>),
    /// A boxed array of *EnvironmentBrowserConfigOptionQuerySpec*. To be used in *Any* placeholders.
    ArrayOfEnvironmentBrowserConfigOptionQuerySpec(Vec<EnvironmentBrowserConfigOptionQuerySpec>),
    /// A boxed array of *ExtendedDescription*. To be used in *Any* placeholders.
    ArrayOfExtendedDescription(Vec<ExtendedDescription>),
    /// A boxed array of *ExtendedElementDescription*. To be used in *Any* placeholders.
    ArrayOfExtendedElementDescription(Vec<ExtendedElementDescription>),
    /// A boxed array of *Extension*. To be used in *Any* placeholders.
    ArrayOfExtension(Vec<Extension>),
    /// A boxed array of *ExtensionClientInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionClientInfo(Vec<ExtensionClientInfo>),
    /// A boxed array of *ExtensionEventTypeInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionEventTypeInfo(Vec<ExtensionEventTypeInfo>),
    /// A boxed array of *ExtensionFaultTypeInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionFaultTypeInfo(Vec<ExtensionFaultTypeInfo>),
    /// A boxed array of *ExtensionHealthInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionHealthInfo(Vec<ExtensionHealthInfo>),
    /// A boxed array of *ExtensionOvfConsumerInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionOvfConsumerInfo(Vec<ExtensionOvfConsumerInfo>),
    /// A boxed array of *ExtensionPrivilegeInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionPrivilegeInfo(Vec<ExtensionPrivilegeInfo>),
    /// A boxed array of *ExtensionResourceInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionResourceInfo(Vec<ExtensionResourceInfo>),
    /// A boxed array of *ExtensionServerInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionServerInfo(Vec<ExtensionServerInfo>),
    /// A boxed array of *ExtensionTaskTypeInfo*. To be used in *Any* placeholders.
    ArrayOfExtensionTaskTypeInfo(Vec<ExtensionTaskTypeInfo>),
    /// A boxed array of *ExtensionManagerIpAllocationUsage*. To be used in *Any* placeholders.
    ArrayOfExtensionManagerIpAllocationUsage(Vec<ExtensionManagerIpAllocationUsage>),
    /// A boxed array of *FaultsByHost*. To be used in *Any* placeholders.
    ArrayOfFaultsByHost(Vec<FaultsByHost>),
    /// A boxed array of *FaultsByVM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfFaultsByVM")]
    ArrayOfFaultsByVm(Vec<FaultsByVm>),
    /// A boxed array of *FeatureEVCMode*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    #[serde(rename = "ArrayOfFeatureEVCMode")]
    ArrayOfFeatureEvcMode(Vec<FeatureEvcMode>),
    /// A boxed array of *FileLockInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    ArrayOfFileLockInfo(Vec<FileLockInfo>),
    /// A boxed array of *FileLockInfoResult*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    ArrayOfFileLockInfoResult(Vec<FileLockInfoResult>),
    /// A boxed array of *FolderBatchAddHostsToClusterResult*. To be used in *Any* placeholders.
    ArrayOfFolderBatchAddHostsToClusterResult(Vec<FolderBatchAddHostsToClusterResult>),
    /// A boxed array of *FolderBatchAddStandaloneHostsResult*. To be used in *Any* placeholders.
    ArrayOfFolderBatchAddStandaloneHostsResult(Vec<FolderBatchAddStandaloneHostsResult>),
    /// A boxed array of *FolderFailedHostResult*. To be used in *Any* placeholders.
    ArrayOfFolderFailedHostResult(Vec<FolderFailedHostResult>),
    /// A boxed array of *FolderNewHostSpec*. To be used in *Any* placeholders.
    ArrayOfFolderNewHostSpec(Vec<FolderNewHostSpec>),
    /// A boxed array of *HbrManagerReplicationVmInfo*. To be used in *Any* placeholders.
    ArrayOfHbrManagerReplicationVmInfo(Vec<HbrManagerReplicationVmInfo>),
    /// A boxed array of *ReplicationVmProgressInfo*. To be used in *Any* placeholders.
    ArrayOfReplicationVmProgressInfo(Vec<ReplicationVmProgressInfo>),
    /// A boxed array of *HbrManagerVmReplicationCapability*. To be used in *Any* placeholders.
    ArrayOfHbrManagerVmReplicationCapability(Vec<HbrManagerVmReplicationCapability>),
    /// A boxed array of *HealthUpdate*. To be used in *Any* placeholders.
    ArrayOfHealthUpdate(Vec<HealthUpdate>),
    /// A boxed array of *HealthUpdateInfo*. To be used in *Any* placeholders.
    ArrayOfHealthUpdateInfo(Vec<HealthUpdateInfo>),
    /// A boxed array of *PerfInterval*. To be used in *Any* placeholders.
    ArrayOfPerfInterval(Vec<PerfInterval>),
    /// A boxed array of *HostServiceTicket*. To be used in *Any* placeholders.
    ArrayOfHostServiceTicket(Vec<HostServiceTicket>),
    /// A boxed array of *HostSystemComplianceCheckState*. To be used in *Any* placeholders.
    ArrayOfHostSystemComplianceCheckState(Vec<HostSystemComplianceCheckState>),
    /// A boxed array of *HostSystemReconnectSpec*. To be used in *Any* placeholders.
    ArrayOfHostSystemReconnectSpec(Vec<HostSystemReconnectSpec>),
    /// A boxed array of *HostSystemRemediationState*. To be used in *Any* placeholders.
    ArrayOfHostSystemRemediationState(Vec<HostSystemRemediationState>),
    /// A boxed array of *HttpNfcLeaseCapabilities*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseCapabilities(Vec<HttpNfcLeaseCapabilities>),
    /// A boxed array of *HttpNfcLeaseDatastoreLeaseInfo*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseDatastoreLeaseInfo(Vec<HttpNfcLeaseDatastoreLeaseInfo>),
    /// A boxed array of *HttpNfcLeaseDeviceUrl*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseDeviceUrl(Vec<HttpNfcLeaseDeviceUrl>),
    /// A boxed array of *HttpNfcLeaseHostInfo*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseHostInfo(Vec<HttpNfcLeaseHostInfo>),
    /// A boxed array of *HttpNfcLeaseInfo*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseInfo(Vec<HttpNfcLeaseInfo>),
    /// A boxed array of *HttpNfcLeaseManifestEntry*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseManifestEntry(Vec<HttpNfcLeaseManifestEntry>),
    /// A boxed array of *HttpNfcLeaseProbeResult*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ArrayOfHttpNfcLeaseProbeResult(Vec<HttpNfcLeaseProbeResult>),
    /// A boxed array of *HttpNfcLeaseSourceFile*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseSourceFile(Vec<HttpNfcLeaseSourceFile>),
    /// A boxed array of *ImportSpec*. To be used in *Any* placeholders.
    ArrayOfImportSpec(Vec<Box<dyn super::traits::ImportSpecTrait>>),
    /// A boxed array of *InheritablePolicy*. To be used in *Any* placeholders.
    ArrayOfInheritablePolicy(Vec<Box<dyn super::traits::InheritablePolicyTrait>>),
    /// A boxed array of *IntExpression*. To be used in *Any* placeholders.
    ArrayOfIntExpression(Vec<IntExpression>),
    /// A boxed array of *IntPolicy*. To be used in *Any* placeholders.
    ArrayOfIntPolicy(Vec<IntPolicy>),
    /// A boxed array of *ClusterIoFilterInfo*. To be used in *Any* placeholders.
    ArrayOfClusterIoFilterInfo(Vec<ClusterIoFilterInfo>),
    /// A boxed array of *HostIoFilterInfo*. To be used in *Any* placeholders.
    ArrayOfHostIoFilterInfo(Vec<HostIoFilterInfo>),
    /// A boxed array of *IoFilterInfo*. To be used in *Any* placeholders.
    ArrayOfIoFilterInfo(Vec<Box<dyn super::traits::IoFilterInfoTrait>>),
    /// A boxed array of *IoFilterQueryIssueResult*. To be used in *Any* placeholders.
    ArrayOfIoFilterQueryIssueResult(Vec<IoFilterQueryIssueResult>),
    /// A boxed array of *IoFilterHostIssue*. To be used in *Any* placeholders.
    ArrayOfIoFilterHostIssue(Vec<IoFilterHostIssue>),
    /// A boxed array of *IpAddress*. To be used in *Any* placeholders.
    ArrayOfIpAddress(Vec<Box<dyn super::traits::IpAddressTrait>>),
    /// A boxed array of *IpPoolManagerIpAllocation*. To be used in *Any* placeholders.
    ArrayOfIpPoolManagerIpAllocation(Vec<IpPoolManagerIpAllocation>),
    /// A boxed array of *IpRange*. To be used in *Any* placeholders.
    ArrayOfIpRange(Vec<IpRange>),
    /// A boxed array of *KeyValue*. To be used in *Any* placeholders.
    ArrayOfKeyValue(Vec<KeyValue>),
    /// A boxed array of *LatencySensitivity*. To be used in *Any* placeholders.
    ArrayOfLatencySensitivity(Vec<LatencySensitivity>),
    /// A boxed array of *LicenseAssignmentManagerLicenseAssignment*. To be used in *Any* placeholders.
    ArrayOfLicenseAssignmentManagerLicenseAssignment(Vec<LicenseAssignmentManagerLicenseAssignment>),
    /// A boxed array of *LicenseAvailabilityInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseAvailabilityInfo(Vec<LicenseAvailabilityInfo>),
    /// A boxed array of *LicenseDiagnostics*. To be used in *Any* placeholders.
    ArrayOfLicenseDiagnostics(Vec<LicenseDiagnostics>),
    /// A boxed array of *LicenseManagerEvaluationInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseManagerEvaluationInfo(Vec<LicenseManagerEvaluationInfo>),
    /// A boxed array of *EvaluationLicenseSource*. To be used in *Any* placeholders.
    ArrayOfEvaluationLicenseSource(Vec<EvaluationLicenseSource>),
    /// A boxed array of *LicenseFeatureInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseFeatureInfo(Vec<LicenseFeatureInfo>),
    /// A boxed array of *HostLicensableResourceInfo*. To be used in *Any* placeholders.
    ArrayOfHostLicensableResourceInfo(Vec<HostLicensableResourceInfo>),
    /// A boxed array of *LicenseManagerLicenseInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseManagerLicenseInfo(Vec<LicenseManagerLicenseInfo>),
    /// A boxed array of *LicenseServerSource*. To be used in *Any* placeholders.
    ArrayOfLicenseServerSource(Vec<LicenseServerSource>),
    /// A boxed array of *LicenseSource*. To be used in *Any* placeholders.
    ArrayOfLicenseSource(Vec<Box<dyn super::traits::LicenseSourceTrait>>),
    /// A boxed array of *LicenseUsageInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseUsageInfo(Vec<LicenseUsageInfo>),
    /// A boxed array of *LocalLicenseSource*. To be used in *Any* placeholders.
    ArrayOfLocalLicenseSource(Vec<LocalLicenseSource>),
    /// A boxed array of *LicenseReservationInfo*. To be used in *Any* placeholders.
    ArrayOfLicenseReservationInfo(Vec<LicenseReservationInfo>),
    /// A boxed array of *LocalizationManagerMessageCatalog*. To be used in *Any* placeholders.
    ArrayOfLocalizationManagerMessageCatalog(Vec<LocalizationManagerMessageCatalog>),
    /// A boxed array of *LongPolicy*. To be used in *Any* placeholders.
    ArrayOfLongPolicy(Vec<LongPolicy>),
    /// A boxed array of *MacAddress*. To be used in *Any* placeholders.
    ArrayOfMacAddress(Vec<Box<dyn super::traits::MacAddressTrait>>),
    /// A boxed array of *MacRange*. To be used in *Any* placeholders.
    ArrayOfMacRange(Vec<MacRange>),
    /// A boxed array of *MethodDescription*. To be used in *Any* placeholders.
    ArrayOfMethodDescription(Vec<MethodDescription>),
    /// A boxed array of *NegatableExpression*. To be used in *Any* placeholders.
    ArrayOfNegatableExpression(Vec<Box<dyn super::traits::NegatableExpressionTrait>>),
    /// A boxed array of *NetworkSummary*. To be used in *Any* placeholders.
    ArrayOfNetworkSummary(Vec<Box<dyn super::traits::NetworkSummaryTrait>>),
    /// A boxed array of *NumericRange*. To be used in *Any* placeholders.
    ArrayOfNumericRange(Vec<NumericRange>),
    /// A boxed array of *OpaqueNetworkCapability*. To be used in *Any* placeholders.
    ArrayOfOpaqueNetworkCapability(Vec<OpaqueNetworkCapability>),
    /// A boxed array of *OpaqueNetworkSummary*. To be used in *Any* placeholders.
    ArrayOfOpaqueNetworkSummary(Vec<OpaqueNetworkSummary>),
    /// A boxed array of *OvfConsumerOstNode*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerOstNode(Vec<OvfConsumerOstNode>),
    /// A boxed array of *OvfConsumerOvfSection*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerOvfSection(Vec<OvfConsumerOvfSection>),
    /// A boxed array of *OvfManagerCommonParams*. To be used in *Any* placeholders.
    ArrayOfOvfManagerCommonParams(Vec<Box<dyn super::traits::OvfManagerCommonParamsTrait>>),
    /// A boxed array of *OvfCreateDescriptorParams*. To be used in *Any* placeholders.
    ArrayOfOvfCreateDescriptorParams(Vec<OvfCreateDescriptorParams>),
    /// A boxed array of *OvfCreateDescriptorResult*. To be used in *Any* placeholders.
    ArrayOfOvfCreateDescriptorResult(Vec<OvfCreateDescriptorResult>),
    /// A boxed array of *OvfCreateImportSpecParams*. To be used in *Any* placeholders.
    ArrayOfOvfCreateImportSpecParams(Vec<OvfCreateImportSpecParams>),
    /// A boxed array of *OvfCreateImportSpecResult*. To be used in *Any* placeholders.
    ArrayOfOvfCreateImportSpecResult(Vec<OvfCreateImportSpecResult>),
    /// A boxed array of *OvfDeploymentOption*. To be used in *Any* placeholders.
    ArrayOfOvfDeploymentOption(Vec<OvfDeploymentOption>),
    /// A boxed array of *OvfFileItem*. To be used in *Any* placeholders.
    ArrayOfOvfFileItem(Vec<OvfFileItem>),
    /// A boxed array of *OvfNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfOvfNetworkInfo(Vec<OvfNetworkInfo>),
    /// A boxed array of *OvfNetworkMapping*. To be used in *Any* placeholders.
    ArrayOfOvfNetworkMapping(Vec<OvfNetworkMapping>),
    /// A boxed array of *OvfFile*. To be used in *Any* placeholders.
    ArrayOfOvfFile(Vec<OvfFile>),
    /// A boxed array of *OvfOptionInfo*. To be used in *Any* placeholders.
    ArrayOfOvfOptionInfo(Vec<OvfOptionInfo>),
    /// A boxed array of *OvfParseDescriptorParams*. To be used in *Any* placeholders.
    ArrayOfOvfParseDescriptorParams(Vec<OvfParseDescriptorParams>),
    /// A boxed array of *OvfParseDescriptorResult*. To be used in *Any* placeholders.
    ArrayOfOvfParseDescriptorResult(Vec<OvfParseDescriptorResult>),
    /// A boxed array of *OvfResourceMap*. To be used in *Any* placeholders.
    ArrayOfOvfResourceMap(Vec<OvfResourceMap>),
    /// A boxed array of *OvfValidateHostParams*. To be used in *Any* placeholders.
    ArrayOfOvfValidateHostParams(Vec<OvfValidateHostParams>),
    /// A boxed array of *OvfValidateHostResult*. To be used in *Any* placeholders.
    ArrayOfOvfValidateHostResult(Vec<OvfValidateHostResult>),
    /// A boxed array of *PasswordField*. To be used in *Any* placeholders.
    ArrayOfPasswordField(Vec<PasswordField>),
    /// A boxed array of *PerformanceDescription*. To be used in *Any* placeholders.
    ArrayOfPerformanceDescription(Vec<PerformanceDescription>),
    /// A boxed array of *PerfCompositeMetric*. To be used in *Any* placeholders.
    ArrayOfPerfCompositeMetric(Vec<PerfCompositeMetric>),
    /// A boxed array of *PerfCounterInfo*. To be used in *Any* placeholders.
    ArrayOfPerfCounterInfo(Vec<PerfCounterInfo>),
    /// A boxed array of *PerformanceManagerCounterLevelMapping*. To be used in *Any* placeholders.
    ArrayOfPerformanceManagerCounterLevelMapping(Vec<PerformanceManagerCounterLevelMapping>),
    /// A boxed array of *PerfEntityMetric*. To be used in *Any* placeholders.
    ArrayOfPerfEntityMetric(Vec<PerfEntityMetric>),
    /// A boxed array of *PerfEntityMetricBase*. To be used in *Any* placeholders.
    ArrayOfPerfEntityMetricBase(Vec<Box<dyn super::traits::PerfEntityMetricBaseTrait>>),
    /// A boxed array of *PerfEntityMetricCSV*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfPerfEntityMetricCSV")]
    ArrayOfPerfEntityMetricCsv(Vec<PerfEntityMetricCsv>),
    /// A boxed array of *PerfMetricIntSeries*. To be used in *Any* placeholders.
    ArrayOfPerfMetricIntSeries(Vec<PerfMetricIntSeries>),
    /// A boxed array of *PerfMetricId*. To be used in *Any* placeholders.
    ArrayOfPerfMetricId(Vec<PerfMetricId>),
    /// A boxed array of *PerfMetricSeries*. To be used in *Any* placeholders.
    ArrayOfPerfMetricSeries(Vec<Box<dyn super::traits::PerfMetricSeriesTrait>>),
    /// A boxed array of *PerfMetricSeriesCSV*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfPerfMetricSeriesCSV")]
    ArrayOfPerfMetricSeriesCsv(Vec<PerfMetricSeriesCsv>),
    /// A boxed array of *PerfProviderSummary*. To be used in *Any* placeholders.
    ArrayOfPerfProviderSummary(Vec<PerfProviderSummary>),
    /// A boxed array of *PerfQuerySpec*. To be used in *Any* placeholders.
    ArrayOfPerfQuerySpec(Vec<PerfQuerySpec>),
    /// A boxed array of *PerfSampleInfo*. To be used in *Any* placeholders.
    ArrayOfPerfSampleInfo(Vec<PerfSampleInfo>),
    /// A boxed array of *PosixUserSearchResult*. To be used in *Any* placeholders.
    ArrayOfPosixUserSearchResult(Vec<PosixUserSearchResult>),
    /// A boxed array of *PrivilegePolicyDef*. To be used in *Any* placeholders.
    ArrayOfPrivilegePolicyDef(Vec<PrivilegePolicyDef>),
    /// A boxed array of *ResourceAllocationInfo*. To be used in *Any* placeholders.
    ArrayOfResourceAllocationInfo(Vec<ResourceAllocationInfo>),
    /// A boxed array of *ResourceAllocationOption*. To be used in *Any* placeholders.
    ArrayOfResourceAllocationOption(Vec<ResourceAllocationOption>),
    /// A boxed array of *ResourceConfigOption*. To be used in *Any* placeholders.
    ArrayOfResourceConfigOption(Vec<ResourceConfigOption>),
    /// A boxed array of *ResourceConfigSpec*. To be used in *Any* placeholders.
    ArrayOfResourceConfigSpec(Vec<ResourceConfigSpec>),
    /// A boxed array of *DatabaseSizeEstimate*. To be used in *Any* placeholders.
    ArrayOfDatabaseSizeEstimate(Vec<DatabaseSizeEstimate>),
    /// A boxed array of *DatabaseSizeParam*. To be used in *Any* placeholders.
    ArrayOfDatabaseSizeParam(Vec<DatabaseSizeParam>),
    /// A boxed array of *InventoryDescription*. To be used in *Any* placeholders.
    ArrayOfInventoryDescription(Vec<InventoryDescription>),
    /// A boxed array of *PerformanceStatisticsDescription*. To be used in *Any* placeholders.
    ArrayOfPerformanceStatisticsDescription(Vec<PerformanceStatisticsDescription>),
    /// A boxed array of *ResourcePoolResourceUsage*. To be used in *Any* placeholders.
    ArrayOfResourcePoolResourceUsage(Vec<ResourcePoolResourceUsage>),
    /// A boxed array of *ResourcePoolRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfResourcePoolRuntimeInfo(Vec<ResourcePoolRuntimeInfo>),
    /// A boxed array of *ResourcePoolSummary*. To be used in *Any* placeholders.
    ArrayOfResourcePoolSummary(Vec<Box<dyn super::traits::ResourcePoolSummaryTrait>>),
    /// A boxed array of *ResourcePoolQuickStats*. To be used in *Any* placeholders.
    ArrayOfResourcePoolQuickStats(Vec<ResourcePoolQuickStats>),
    /// A boxed array of *SDDCBase*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSDDCBase")]
    ArrayOfSddcBase(Vec<SddcBase>),
    /// A boxed array of *SelectionSet*. To be used in *Any* placeholders.
    ArrayOfSelectionSet(Vec<Box<dyn super::traits::SelectionSetTrait>>),
    /// A boxed array of *HostVMotionCompatibility*. To be used in *Any* placeholders.
    ArrayOfHostVMotionCompatibility(Vec<HostVMotionCompatibility>),
    /// A boxed array of *ProductComponentInfo*. To be used in *Any* placeholders.
    ArrayOfProductComponentInfo(Vec<ProductComponentInfo>),
    /// A boxed array of *ServiceContent*. To be used in *Any* placeholders.
    ArrayOfServiceContent(Vec<ServiceContent>),
    /// A boxed array of *ServiceLocator*. To be used in *Any* placeholders.
    ArrayOfServiceLocator(Vec<ServiceLocator>),
    /// A boxed array of *ServiceLocatorCredential*. To be used in *Any* placeholders.
    ArrayOfServiceLocatorCredential(Vec<Box<dyn super::traits::ServiceLocatorCredentialTrait>>),
    /// A boxed array of *ServiceLocatorNamePassword*. To be used in *Any* placeholders.
    ArrayOfServiceLocatorNamePassword(Vec<ServiceLocatorNamePassword>),
    /// A boxed array of *ServiceLocatorSAMLCredential*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfServiceLocatorSAMLCredential")]
    ArrayOfServiceLocatorSamlCredential(Vec<ServiceLocatorSamlCredential>),
    /// A boxed array of *ServiceManagerServiceInfo*. To be used in *Any* placeholders.
    ArrayOfServiceManagerServiceInfo(Vec<ServiceManagerServiceInfo>),
    /// A boxed array of *SessionManagerGenericServiceTicket*. To be used in *Any* placeholders.
    ArrayOfSessionManagerGenericServiceTicket(Vec<SessionManagerGenericServiceTicket>),
    /// A boxed array of *SessionManagerHttpServiceRequestSpec*. To be used in *Any* placeholders.
    ArrayOfSessionManagerHttpServiceRequestSpec(Vec<SessionManagerHttpServiceRequestSpec>),
    /// A boxed array of *SessionManagerLocalTicket*. To be used in *Any* placeholders.
    ArrayOfSessionManagerLocalTicket(Vec<SessionManagerLocalTicket>),
    /// A boxed array of *SessionManagerServiceRequestSpec*. To be used in *Any* placeholders.
    ArrayOfSessionManagerServiceRequestSpec(Vec<Box<dyn super::traits::SessionManagerServiceRequestSpecTrait>>),
    /// A boxed array of *SessionManagerVmomiServiceRequestSpec*. To be used in *Any* placeholders.
    ArrayOfSessionManagerVmomiServiceRequestSpec(Vec<SessionManagerVmomiServiceRequestSpec>),
    /// A boxed array of *SharesInfo*. To be used in *Any* placeholders.
    ArrayOfSharesInfo(Vec<SharesInfo>),
    /// A boxed array of *SharesOption*. To be used in *Any* placeholders.
    ArrayOfSharesOption(Vec<SharesOption>),
    /// A boxed array of *SingleIp*. To be used in *Any* placeholders.
    ArrayOfSingleIp(Vec<SingleIp>),
    /// A boxed array of *SingleMac*. To be used in *Any* placeholders.
    ArrayOfSingleMac(Vec<SingleMac>),
    /// A boxed array of *SiteInfo*. To be used in *Any* placeholders.
    ArrayOfSiteInfo(Vec<SiteInfo>),
    /// A boxed array of *StoragePodSummary*. To be used in *Any* placeholders.
    ArrayOfStoragePodSummary(Vec<StoragePodSummary>),
    /// A boxed array of *StorageIOAllocationInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageIOAllocationInfo")]
    ArrayOfStorageIoAllocationInfo(Vec<StorageIoAllocationInfo>),
    /// A boxed array of *StorageIOAllocationOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageIOAllocationOption")]
    ArrayOfStorageIoAllocationOption(Vec<StorageIoAllocationOption>),
    /// A boxed array of *StorageIORMInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageIORMInfo")]
    ArrayOfStorageIormInfo(Vec<StorageIormInfo>),
    /// A boxed array of *StorageIORMConfigOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageIORMConfigOption")]
    ArrayOfStorageIormConfigOption(Vec<StorageIormConfigOption>),
    /// A boxed array of *StorageIORMConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageIORMConfigSpec")]
    ArrayOfStorageIormConfigSpec(Vec<StorageIormConfigSpec>),
    /// A boxed array of *PodStorageDrsEntry*. To be used in *Any* placeholders.
    ArrayOfPodStorageDrsEntry(Vec<PodStorageDrsEntry>),
    /// A boxed array of *StoragePerformanceSummary*. To be used in *Any* placeholders.
    ArrayOfStoragePerformanceSummary(Vec<StoragePerformanceSummary>),
    /// A boxed array of *StorageResourceManagerStorageProfileStatistics*. To be used in *Any* placeholders.
    ArrayOfStorageResourceManagerStorageProfileStatistics(Vec<StorageResourceManagerStorageProfileStatistics>),
    /// A boxed array of *StringExpression*. To be used in *Any* placeholders.
    ArrayOfStringExpression(Vec<StringExpression>),
    /// A boxed array of *StringPolicy*. To be used in *Any* placeholders.
    ArrayOfStringPolicy(Vec<StringPolicy>),
    /// A boxed array of *Tag*. To be used in *Any* placeholders.
    ArrayOfTag(Vec<Tag>),
    /// A boxed array of *TaskDescription*. To be used in *Any* placeholders.
    ArrayOfTaskDescription(Vec<TaskDescription>),
    /// A boxed array of *TaskFilterSpec*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpec(Vec<TaskFilterSpec>),
    /// A boxed array of *TaskFilterSpecByEntity*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpecByEntity(Vec<TaskFilterSpecByEntity>),
    /// A boxed array of *TaskFilterSpecByTime*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpecByTime(Vec<TaskFilterSpecByTime>),
    /// A boxed array of *TaskFilterSpecByUsername*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpecByUsername(Vec<TaskFilterSpecByUsername>),
    /// A boxed array of *TaskInfo*. To be used in *Any* placeholders.
    ArrayOfTaskInfo(Vec<TaskInfo>),
    /// A boxed array of *TaskReason*. To be used in *Any* placeholders.
    ArrayOfTaskReason(Vec<Box<dyn super::traits::TaskReasonTrait>>),
    /// A boxed array of *TaskReasonAlarm*. To be used in *Any* placeholders.
    ArrayOfTaskReasonAlarm(Vec<TaskReasonAlarm>),
    /// A boxed array of *TaskReasonSchedule*. To be used in *Any* placeholders.
    ArrayOfTaskReasonSchedule(Vec<TaskReasonSchedule>),
    /// A boxed array of *TaskReasonSystem*. To be used in *Any* placeholders.
    ArrayOfTaskReasonSystem(Vec<TaskReasonSystem>),
    /// A boxed array of *TaskReasonUser*. To be used in *Any* placeholders.
    ArrayOfTaskReasonUser(Vec<TaskReasonUser>),
    /// A boxed array of *TypeDescription*. To be used in *Any* placeholders.
    ArrayOfTypeDescription(Vec<Box<dyn super::traits::TypeDescriptionTrait>>),
    /// A boxed array of *UpdateVirtualMachineFilesResult*. To be used in *Any* placeholders.
    ArrayOfUpdateVirtualMachineFilesResult(Vec<UpdateVirtualMachineFilesResult>),
    /// A boxed array of *UpdateVirtualMachineFilesResultFailedVmFileInfo*. To be used in *Any* placeholders.
    ArrayOfUpdateVirtualMachineFilesResultFailedVmFileInfo(Vec<UpdateVirtualMachineFilesResultFailedVmFileInfo>),
    /// A boxed array of *UserSearchResult*. To be used in *Any* placeholders.
    ArrayOfUserSearchResult(Vec<Box<dyn super::traits::UserSearchResultTrait>>),
    /// A boxed array of *UserSession*. To be used in *Any* placeholders.
    ArrayOfUserSession(Vec<UserSession>),
    /// A boxed array of *VVolVmConfigFileUpdateResult*. To be used in *Any* placeholders.
    ArrayOfVVolVmConfigFileUpdateResult(Vec<VVolVmConfigFileUpdateResult>),
    /// A boxed array of *VVolVmConfigFileUpdateResultFailedVmConfigFileInfo*. To be used in *Any* placeholders.
    ArrayOfVVolVmConfigFileUpdateResultFailedVmConfigFileInfo(Vec<VVolVmConfigFileUpdateResultFailedVmConfigFileInfo>),
    /// A boxed array of *VASAStorageArray*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVASAStorageArray")]
    ArrayOfVasaStorageArray(Vec<VasaStorageArray>),
    /// A boxed array of *VASAStorageArrayDiscoveryFcTransport*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.0
    #[serde(rename = "ArrayOfVASAStorageArrayDiscoveryFcTransport")]
    ArrayOfVasaStorageArrayDiscoveryFcTransport(Vec<VasaStorageArrayDiscoveryFcTransport>),
    /// A boxed array of *VASAStorageArrayDiscoveryIpTransport*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.0
    #[serde(rename = "ArrayOfVASAStorageArrayDiscoveryIpTransport")]
    ArrayOfVasaStorageArrayDiscoveryIpTransport(Vec<VasaStorageArrayDiscoveryIpTransport>),
    /// A boxed array of *VASAStorageArrayDiscoverySvcInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.0
    #[serde(rename = "ArrayOfVASAStorageArrayDiscoverySvcInfo")]
    ArrayOfVasaStorageArrayDiscoverySvcInfo(Vec<VasaStorageArrayDiscoverySvcInfo>),
    /// A boxed array of *VasaProviderContainerSpec*. To be used in *Any* placeholders.
    ArrayOfVasaProviderContainerSpec(Vec<VasaProviderContainerSpec>),
    /// A boxed array of *VimVasaProvider*. To be used in *Any* placeholders.
    ArrayOfVimVasaProvider(Vec<VimVasaProvider>),
    /// A boxed array of *VimVasaProviderStatePerArray*. To be used in *Any* placeholders.
    ArrayOfVimVasaProviderStatePerArray(Vec<VimVasaProviderStatePerArray>),
    /// A boxed array of *VimVasaProviderVirtualHostConfig*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfVimVasaProviderVirtualHostConfig(Vec<VimVasaProviderVirtualHostConfig>),
    /// A boxed array of *VimVasaProviderInfo*. To be used in *Any* placeholders.
    ArrayOfVimVasaProviderInfo(Vec<VimVasaProviderInfo>),
    /// A boxed array of *VirtualAppLinkInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualAppLinkInfo(Vec<VirtualAppLinkInfo>),
    /// A boxed array of *VirtualAppSummary*. To be used in *Any* placeholders.
    ArrayOfVirtualAppSummary(Vec<VirtualAppSummary>),
    /// A boxed array of *DeviceBackedVirtualDiskSpec*. To be used in *Any* placeholders.
    ArrayOfDeviceBackedVirtualDiskSpec(Vec<DeviceBackedVirtualDiskSpec>),
    /// A boxed array of *FileBackedVirtualDiskSpec*. To be used in *Any* placeholders.
    ArrayOfFileBackedVirtualDiskSpec(Vec<Box<dyn super::traits::FileBackedVirtualDiskSpecTrait>>),
    /// A boxed array of *SeSparseVirtualDiskSpec*. To be used in *Any* placeholders.
    ArrayOfSeSparseVirtualDiskSpec(Vec<SeSparseVirtualDiskSpec>),
    /// A boxed array of *VirtualDiskSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSpec(Vec<Box<dyn super::traits::VirtualDiskSpecTrait>>),
    /// A boxed array of *VirtualMachineConnection*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ArrayOfVirtualMachineConnection(Vec<Box<dyn super::traits::VirtualMachineConnectionTrait>>),
    /// A boxed array of *DiskChangeInfo*. To be used in *Any* placeholders.
    ArrayOfDiskChangeInfo(Vec<DiskChangeInfo>),
    /// A boxed array of *DiskChangeExtent*. To be used in *Any* placeholders.
    ArrayOfDiskChangeExtent(Vec<DiskChangeExtent>),
    /// A boxed array of *VirtualMachineDisplayTopology*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDisplayTopology(Vec<VirtualMachineDisplayTopology>),
    /// A boxed array of *VirtualMachineMksConnection*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ArrayOfVirtualMachineMksConnection(Vec<VirtualMachineMksConnection>),
    /// A boxed array of *VirtualMachineMksTicket*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMksTicket(Vec<VirtualMachineMksTicket>),
    /// A boxed array of *StorageRequirement*. To be used in *Any* placeholders.
    ArrayOfStorageRequirement(Vec<StorageRequirement>),
    /// A boxed array of *VirtualMachineTicket*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineTicket(Vec<VirtualMachineTicket>),
    /// A boxed array of *VirtualMachineWipeResult*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineWipeResult(Vec<VirtualMachineWipeResult>),
    /// A boxed array of *VsanUpgradeSystemAPIBrokenIssue*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVsanUpgradeSystemAPIBrokenIssue")]
    ArrayOfVsanUpgradeSystemApiBrokenIssue(Vec<VsanUpgradeSystemApiBrokenIssue>),
    /// A boxed array of *VsanUpgradeSystemAutoClaimEnabledOnHostsIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemAutoClaimEnabledOnHostsIssue(Vec<VsanUpgradeSystemAutoClaimEnabledOnHostsIssue>),
    /// A boxed array of *VsanUpgradeSystemHostsDisconnectedIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemHostsDisconnectedIssue(Vec<VsanUpgradeSystemHostsDisconnectedIssue>),
    /// A boxed array of *VsanUpgradeSystemMissingHostsInClusterIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemMissingHostsInClusterIssue(Vec<VsanUpgradeSystemMissingHostsInClusterIssue>),
    /// A boxed array of *VsanUpgradeSystemNetworkPartitionInfo*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemNetworkPartitionInfo(Vec<VsanUpgradeSystemNetworkPartitionInfo>),
    /// A boxed array of *VsanUpgradeSystemNetworkPartitionIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemNetworkPartitionIssue(Vec<VsanUpgradeSystemNetworkPartitionIssue>),
    /// A boxed array of *VsanUpgradeSystemNotEnoughFreeCapacityIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemNotEnoughFreeCapacityIssue(Vec<VsanUpgradeSystemNotEnoughFreeCapacityIssue>),
    /// A boxed array of *VsanUpgradeSystemPreflightCheckIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemPreflightCheckIssue(Vec<Box<dyn super::traits::VsanUpgradeSystemPreflightCheckIssueTrait>>),
    /// A boxed array of *VsanUpgradeSystemPreflightCheckResult*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemPreflightCheckResult(Vec<VsanUpgradeSystemPreflightCheckResult>),
    /// A boxed array of *VsanUpgradeSystemRogueHostsInClusterIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemRogueHostsInClusterIssue(Vec<VsanUpgradeSystemRogueHostsInClusterIssue>),
    /// A boxed array of *VsanUpgradeSystemUpgradeHistoryDiskGroupOp*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemUpgradeHistoryDiskGroupOp(Vec<VsanUpgradeSystemUpgradeHistoryDiskGroupOp>),
    /// A boxed array of *VsanUpgradeSystemUpgradeHistoryItem*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemUpgradeHistoryItem(Vec<Box<dyn super::traits::VsanUpgradeSystemUpgradeHistoryItemTrait>>),
    /// A boxed array of *VsanUpgradeSystemUpgradeHistoryPreflightFail*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemUpgradeHistoryPreflightFail(Vec<VsanUpgradeSystemUpgradeHistoryPreflightFail>),
    /// A boxed array of *VsanUpgradeSystemUpgradeStatus*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemUpgradeStatus(Vec<VsanUpgradeSystemUpgradeStatus>),
    /// A boxed array of *VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue(Vec<VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue>),
    /// A boxed array of *VsanUpgradeSystemWrongEsxVersionIssue*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemWrongEsxVersionIssue(Vec<VsanUpgradeSystemWrongEsxVersionIssue>),
    /// A boxed array of *Action*. To be used in *Any* placeholders.
    ArrayOfAction(Vec<Box<dyn super::traits::ActionTrait>>),
    /// A boxed array of *CreateTaskAction*. To be used in *Any* placeholders.
    ArrayOfCreateTaskAction(Vec<CreateTaskAction>),
    /// A boxed array of *MethodAction*. To be used in *Any* placeholders.
    ArrayOfMethodAction(Vec<MethodAction>),
    /// A boxed array of *MethodActionArgument*. To be used in *Any* placeholders.
    ArrayOfMethodActionArgument(Vec<MethodActionArgument>),
    /// A boxed array of *RunScriptAction*. To be used in *Any* placeholders.
    ArrayOfRunScriptAction(Vec<RunScriptAction>),
    /// A boxed array of *SendEmailAction*. To be used in *Any* placeholders.
    ArrayOfSendEmailAction(Vec<SendEmailAction>),
    /// A boxed array of *SendSNMPAction*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSendSNMPAction")]
    ArrayOfSendSnmpAction(Vec<SendSnmpAction>),
    /// A boxed array of *AlarmAction*. To be used in *Any* placeholders.
    ArrayOfAlarmAction(Vec<Box<dyn super::traits::AlarmActionTrait>>),
    /// A boxed array of *AlarmDescription*. To be used in *Any* placeholders.
    ArrayOfAlarmDescription(Vec<AlarmDescription>),
    /// A boxed array of *AlarmExpression*. To be used in *Any* placeholders.
    ArrayOfAlarmExpression(Vec<Box<dyn super::traits::AlarmExpressionTrait>>),
    /// A boxed array of *AlarmFilterSpec*. To be used in *Any* placeholders.
    ArrayOfAlarmFilterSpec(Vec<AlarmFilterSpec>),
    /// A boxed array of *AlarmInfo*. To be used in *Any* placeholders.
    ArrayOfAlarmInfo(Vec<AlarmInfo>),
    /// A boxed array of *AlarmSetting*. To be used in *Any* placeholders.
    ArrayOfAlarmSetting(Vec<AlarmSetting>),
    /// A boxed array of *AlarmSpec*. To be used in *Any* placeholders.
    ArrayOfAlarmSpec(Vec<Box<dyn super::traits::AlarmSpecTrait>>),
    /// A boxed array of *AlarmState*. To be used in *Any* placeholders.
    ArrayOfAlarmState(Vec<AlarmState>),
    /// A boxed array of *AlarmTriggeringAction*. To be used in *Any* placeholders.
    ArrayOfAlarmTriggeringAction(Vec<AlarmTriggeringAction>),
    /// A boxed array of *AlarmTriggeringActionTransitionSpec*. To be used in *Any* placeholders.
    ArrayOfAlarmTriggeringActionTransitionSpec(Vec<AlarmTriggeringActionTransitionSpec>),
    /// A boxed array of *AndAlarmExpression*. To be used in *Any* placeholders.
    ArrayOfAndAlarmExpression(Vec<AndAlarmExpression>),
    /// A boxed array of *EventAlarmExpression*. To be used in *Any* placeholders.
    ArrayOfEventAlarmExpression(Vec<EventAlarmExpression>),
    /// A boxed array of *EventAlarmExpressionComparison*. To be used in *Any* placeholders.
    ArrayOfEventAlarmExpressionComparison(Vec<EventAlarmExpressionComparison>),
    /// A boxed array of *GroupAlarmAction*. To be used in *Any* placeholders.
    ArrayOfGroupAlarmAction(Vec<GroupAlarmAction>),
    /// A boxed array of *MetricAlarmExpression*. To be used in *Any* placeholders.
    ArrayOfMetricAlarmExpression(Vec<MetricAlarmExpression>),
    /// A boxed array of *OrAlarmExpression*. To be used in *Any* placeholders.
    ArrayOfOrAlarmExpression(Vec<OrAlarmExpression>),
    /// A boxed array of *StateAlarmExpression*. To be used in *Any* placeholders.
    ArrayOfStateAlarmExpression(Vec<StateAlarmExpression>),
    /// A boxed array of *ClusterAction*. To be used in *Any* placeholders.
    ArrayOfClusterAction(Vec<Box<dyn super::traits::ClusterActionTrait>>),
    /// A boxed array of *ClusterActionHistory*. To be used in *Any* placeholders.
    ArrayOfClusterActionHistory(Vec<ClusterActionHistory>),
    /// A boxed array of *ClusterAffinityRuleSpec*. To be used in *Any* placeholders.
    ArrayOfClusterAffinityRuleSpec(Vec<ClusterAffinityRuleSpec>),
    /// A boxed array of *ClusterAntiAffinityRuleSpec*. To be used in *Any* placeholders.
    ArrayOfClusterAntiAffinityRuleSpec(Vec<ClusterAntiAffinityRuleSpec>),
    /// A boxed array of *ClusterAttemptedVmInfo*. To be used in *Any* placeholders.
    ArrayOfClusterAttemptedVmInfo(Vec<ClusterAttemptedVmInfo>),
    /// A boxed array of *ClusterClusterInitialPlacementAction*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfClusterClusterInitialPlacementAction(Vec<ClusterClusterInitialPlacementAction>),
    /// A boxed array of *ClusterConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterConfigInfo(Vec<ClusterConfigInfo>),
    /// A boxed array of *ClusterConfigInfoEx*. To be used in *Any* placeholders.
    ArrayOfClusterConfigInfoEx(Vec<ClusterConfigInfoEx>),
    /// A boxed array of *ClusterConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterConfigSpec(Vec<ClusterConfigSpec>),
    /// A boxed array of *ClusterConfigSpecEx*. To be used in *Any* placeholders.
    ArrayOfClusterConfigSpecEx(Vec<ClusterConfigSpecEx>),
    /// A boxed array of *ClusterCryptoConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterCryptoConfigInfo(Vec<ClusterCryptoConfigInfo>),
    /// A boxed array of *ClusterDasAamHostInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasAamHostInfo(Vec<ClusterDasAamHostInfo>),
    /// A boxed array of *ClusterDasAamNodeState*. To be used in *Any* placeholders.
    ArrayOfClusterDasAamNodeState(Vec<ClusterDasAamNodeState>),
    /// A boxed array of *ClusterDasAdmissionControlInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasAdmissionControlInfo(Vec<Box<dyn super::traits::ClusterDasAdmissionControlInfoTrait>>),
    /// A boxed array of *ClusterDasAdmissionControlPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterDasAdmissionControlPolicy(Vec<Box<dyn super::traits::ClusterDasAdmissionControlPolicyTrait>>),
    /// A boxed array of *ClusterDasAdvancedRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasAdvancedRuntimeInfo(Vec<Box<dyn super::traits::ClusterDasAdvancedRuntimeInfoTrait>>),
    /// A boxed array of *DasHeartbeatDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfDasHeartbeatDatastoreInfo(Vec<DasHeartbeatDatastoreInfo>),
    /// A boxed array of *ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo(Vec<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo>),
    /// A boxed array of *ClusterDasConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasConfigInfo(Vec<ClusterDasConfigInfo>),
    /// A boxed array of *ClusterDasData*. To be used in *Any* placeholders.
    ArrayOfClusterDasData(Vec<Box<dyn super::traits::ClusterDasDataTrait>>),
    /// A boxed array of *ClusterDasDataSummary*. To be used in *Any* placeholders.
    ArrayOfClusterDasDataSummary(Vec<ClusterDasDataSummary>),
    /// A boxed array of *ClusterDasFailoverLevelAdvancedRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasFailoverLevelAdvancedRuntimeInfo(Vec<ClusterDasFailoverLevelAdvancedRuntimeInfo>),
    /// A boxed array of *ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots*. To be used in *Any* placeholders.
    ArrayOfClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots(Vec<ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots>),
    /// A boxed array of *ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo(Vec<ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo>),
    /// A boxed array of *ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots*. To be used in *Any* placeholders.
    ArrayOfClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots(Vec<ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots>),
    /// A boxed array of *ClusterDasFdmHostState*. To be used in *Any* placeholders.
    ArrayOfClusterDasFdmHostState(Vec<ClusterDasFdmHostState>),
    /// A boxed array of *ClusterDasHostInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasHostInfo(Vec<Box<dyn super::traits::ClusterDasHostInfoTrait>>),
    /// A boxed array of *ClusterDasHostRecommendation*. To be used in *Any* placeholders.
    ArrayOfClusterDasHostRecommendation(Vec<ClusterDasHostRecommendation>),
    /// A boxed array of *ClusterDasVmConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDasVmConfigInfo(Vec<ClusterDasVmConfigInfo>),
    /// A boxed array of *ClusterDasVmConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterDasVmConfigSpec(Vec<ClusterDasVmConfigSpec>),
    /// A boxed array of *ClusterDasVmSettings*. To be used in *Any* placeholders.
    ArrayOfClusterDasVmSettings(Vec<ClusterDasVmSettings>),
    /// A boxed array of *ClusterDatastoreUpdateSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfClusterDatastoreUpdateSpec(Vec<ClusterDatastoreUpdateSpec>),
    /// A boxed array of *ClusterDependencyRuleInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDependencyRuleInfo(Vec<ClusterDependencyRuleInfo>),
    /// A boxed array of *ClusterDpmConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDpmConfigInfo(Vec<ClusterDpmConfigInfo>),
    /// A boxed array of *ClusterDpmHostConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDpmHostConfigInfo(Vec<ClusterDpmHostConfigInfo>),
    /// A boxed array of *ClusterDpmHostConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterDpmHostConfigSpec(Vec<ClusterDpmHostConfigSpec>),
    /// A boxed array of *ClusterDrsConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDrsConfigInfo(Vec<ClusterDrsConfigInfo>),
    /// A boxed array of *ClusterDrsFaults*. To be used in *Any* placeholders.
    ArrayOfClusterDrsFaults(Vec<ClusterDrsFaults>),
    /// A boxed array of *ClusterDrsFaultsFaultsByVirtualDisk*. To be used in *Any* placeholders.
    ArrayOfClusterDrsFaultsFaultsByVirtualDisk(Vec<ClusterDrsFaultsFaultsByVirtualDisk>),
    /// A boxed array of *ClusterDrsFaultsFaultsByVm*. To be used in *Any* placeholders.
    ArrayOfClusterDrsFaultsFaultsByVm(Vec<Box<dyn super::traits::ClusterDrsFaultsFaultsByVmTrait>>),
    /// A boxed array of *ClusterDrsMigration*. To be used in *Any* placeholders.
    ArrayOfClusterDrsMigration(Vec<ClusterDrsMigration>),
    /// A boxed array of *ClusterDrsRecommendation*. To be used in *Any* placeholders.
    ArrayOfClusterDrsRecommendation(Vec<ClusterDrsRecommendation>),
    /// A boxed array of *ClusterDrsVmConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterDrsVmConfigInfo(Vec<ClusterDrsVmConfigInfo>),
    /// A boxed array of *ClusterDrsVmConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterDrsVmConfigSpec(Vec<ClusterDrsVmConfigSpec>),
    /// A boxed array of *ClusterEVCManagerCheckResult*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterEVCManagerCheckResult")]
    ArrayOfClusterEvcManagerCheckResult(Vec<ClusterEvcManagerCheckResult>),
    /// A boxed array of *ClusterEVCManagerEVCState*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfClusterEVCManagerEVCState")]
    ArrayOfClusterEvcManagerEvcState(Vec<ClusterEvcManagerEvcState>),
    /// A boxed array of *ClusterEnterMaintenanceResult*. To be used in *Any* placeholders.
    ArrayOfClusterEnterMaintenanceResult(Vec<ClusterEnterMaintenanceResult>),
    /// A boxed array of *ClusterFailoverHostAdmissionControlInfo*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverHostAdmissionControlInfo(Vec<ClusterFailoverHostAdmissionControlInfo>),
    /// A boxed array of *ClusterFailoverHostAdmissionControlInfoHostStatus*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverHostAdmissionControlInfoHostStatus(Vec<ClusterFailoverHostAdmissionControlInfoHostStatus>),
    /// A boxed array of *ClusterFailoverHostAdmissionControlPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverHostAdmissionControlPolicy(Vec<ClusterFailoverHostAdmissionControlPolicy>),
    /// A boxed array of *ClusterFailoverLevelAdmissionControlInfo*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverLevelAdmissionControlInfo(Vec<ClusterFailoverLevelAdmissionControlInfo>),
    /// A boxed array of *ClusterFailoverLevelAdmissionControlPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverLevelAdmissionControlPolicy(Vec<ClusterFailoverLevelAdmissionControlPolicy>),
    /// A boxed array of *ClusterFailoverResourcesAdmissionControlInfo*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverResourcesAdmissionControlInfo(Vec<ClusterFailoverResourcesAdmissionControlInfo>),
    /// A boxed array of *ClusterFailoverResourcesAdmissionControlPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterFailoverResourcesAdmissionControlPolicy(Vec<ClusterFailoverResourcesAdmissionControlPolicy>),
    /// A boxed array of *ClusterFixedSizeSlotPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterFixedSizeSlotPolicy(Vec<ClusterFixedSizeSlotPolicy>),
    /// A boxed array of *ClusterGroupInfo*. To be used in *Any* placeholders.
    ArrayOfClusterGroupInfo(Vec<Box<dyn super::traits::ClusterGroupInfoTrait>>),
    /// A boxed array of *ClusterGroupSpec*. To be used in *Any* placeholders.
    ArrayOfClusterGroupSpec(Vec<ClusterGroupSpec>),
    /// A boxed array of *ClusterHostGroup*. To be used in *Any* placeholders.
    ArrayOfClusterHostGroup(Vec<ClusterHostGroup>),
    /// A boxed array of *ClusterHostInfraUpdateHaModeAction*. To be used in *Any* placeholders.
    ArrayOfClusterHostInfraUpdateHaModeAction(Vec<ClusterHostInfraUpdateHaModeAction>),
    /// A boxed array of *ClusterHostPowerAction*. To be used in *Any* placeholders.
    ArrayOfClusterHostPowerAction(Vec<ClusterHostPowerAction>),
    /// A boxed array of *ClusterHostRecommendation*. To be used in *Any* placeholders.
    ArrayOfClusterHostRecommendation(Vec<ClusterHostRecommendation>),
    /// A boxed array of *ClusterInfraUpdateHaConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterInfraUpdateHaConfigInfo(Vec<ClusterInfraUpdateHaConfigInfo>),
    /// A boxed array of *ClusterInitialPlacementAction*. To be used in *Any* placeholders.
    ArrayOfClusterInitialPlacementAction(Vec<ClusterInitialPlacementAction>),
    /// A boxed array of *ClusterMigrationAction*. To be used in *Any* placeholders.
    ArrayOfClusterMigrationAction(Vec<ClusterMigrationAction>),
    /// A boxed array of *ClusterNotAttemptedVmInfo*. To be used in *Any* placeholders.
    ArrayOfClusterNotAttemptedVmInfo(Vec<ClusterNotAttemptedVmInfo>),
    /// A boxed array of *ClusterOrchestrationInfo*. To be used in *Any* placeholders.
    ArrayOfClusterOrchestrationInfo(Vec<ClusterOrchestrationInfo>),
    /// A boxed array of *PlacementAction*. To be used in *Any* placeholders.
    ArrayOfPlacementAction(Vec<PlacementAction>),
    /// A boxed array of *PlacementResult*. To be used in *Any* placeholders.
    ArrayOfPlacementResult(Vec<PlacementResult>),
    /// A boxed array of *PlacementSpec*. To be used in *Any* placeholders.
    ArrayOfPlacementSpec(Vec<PlacementSpec>),
    /// A boxed array of *ClusterPowerOnVmResult*. To be used in *Any* placeholders.
    ArrayOfClusterPowerOnVmResult(Vec<ClusterPowerOnVmResult>),
    /// A boxed array of *ClusterPreemptibleVmPairInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfClusterPreemptibleVmPairInfo(Vec<ClusterPreemptibleVmPairInfo>),
    /// A boxed array of *ClusterPreemptibleVmPairSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfClusterPreemptibleVmPairSpec(Vec<ClusterPreemptibleVmPairSpec>),
    /// A boxed array of *ClusterProactiveDrsConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterProactiveDrsConfigInfo(Vec<ClusterProactiveDrsConfigInfo>),
    /// A boxed array of *ClusterRecommendation*. To be used in *Any* placeholders.
    ArrayOfClusterRecommendation(Vec<ClusterRecommendation>),
    /// A boxed array of *ClusterResourceUsageSummary*. To be used in *Any* placeholders.
    ArrayOfClusterResourceUsageSummary(Vec<ClusterResourceUsageSummary>),
    /// A boxed array of *ClusterRuleInfo*. To be used in *Any* placeholders.
    ArrayOfClusterRuleInfo(Vec<Box<dyn super::traits::ClusterRuleInfoTrait>>),
    /// A boxed array of *ClusterRuleSpec*. To be used in *Any* placeholders.
    ArrayOfClusterRuleSpec(Vec<ClusterRuleSpec>),
    /// A boxed array of *ClusterSlotPolicy*. To be used in *Any* placeholders.
    ArrayOfClusterSlotPolicy(Vec<Box<dyn super::traits::ClusterSlotPolicyTrait>>),
    /// A boxed array of *ClusterSystemVMsConfigInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfClusterSystemVMsConfigInfo(Vec<ClusterSystemVMsConfigInfo>),
    /// A boxed array of *ClusterSystemVMsConfigSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfClusterSystemVMsConfigSpec(Vec<ClusterSystemVMsConfigSpec>),
    /// A boxed array of *ClusterTagCategoryUpdateSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfClusterTagCategoryUpdateSpec(Vec<ClusterTagCategoryUpdateSpec>),
    /// A boxed array of *ClusterUsageSummary*. To be used in *Any* placeholders.
    ArrayOfClusterUsageSummary(Vec<ClusterUsageSummary>),
    /// A boxed array of *ClusterVmComponentProtectionSettings*. To be used in *Any* placeholders.
    ArrayOfClusterVmComponentProtectionSettings(Vec<ClusterVmComponentProtectionSettings>),
    /// A boxed array of *ClusterVmGroup*. To be used in *Any* placeholders.
    ArrayOfClusterVmGroup(Vec<ClusterVmGroup>),
    /// A boxed array of *ClusterVmHostRuleInfo*. To be used in *Any* placeholders.
    ArrayOfClusterVmHostRuleInfo(Vec<ClusterVmHostRuleInfo>),
    /// A boxed array of *ClusterVmOrchestrationInfo*. To be used in *Any* placeholders.
    ArrayOfClusterVmOrchestrationInfo(Vec<ClusterVmOrchestrationInfo>),
    /// A boxed array of *ClusterVmOrchestrationSpec*. To be used in *Any* placeholders.
    ArrayOfClusterVmOrchestrationSpec(Vec<ClusterVmOrchestrationSpec>),
    /// A boxed array of *ClusterVmReadiness*. To be used in *Any* placeholders.
    ArrayOfClusterVmReadiness(Vec<ClusterVmReadiness>),
    /// A boxed array of *ClusterVmToolsMonitoringSettings*. To be used in *Any* placeholders.
    ArrayOfClusterVmToolsMonitoringSettings(Vec<ClusterVmToolsMonitoringSettings>),
    /// A boxed array of *DistributedVirtualPort*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualPort(Vec<DistributedVirtualPort>),
    /// A boxed array of *DVPortConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortConfigInfo")]
    ArrayOfDvPortConfigInfo(Vec<DvPortConfigInfo>),
    /// A boxed array of *DVPortConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortConfigSpec")]
    ArrayOfDvPortConfigSpec(Vec<DvPortConfigSpec>),
    /// A boxed array of *DvsFilterConfig*. To be used in *Any* placeholders.
    ArrayOfDvsFilterConfig(Vec<Box<dyn super::traits::DvsFilterConfigTrait>>),
    /// A boxed array of *DvsFilterConfigSpec*. To be used in *Any* placeholders.
    ArrayOfDvsFilterConfigSpec(Vec<DvsFilterConfigSpec>),
    /// A boxed array of *DvsFilterParameter*. To be used in *Any* placeholders.
    ArrayOfDvsFilterParameter(Vec<DvsFilterParameter>),
    /// A boxed array of *DvsFilterPolicy*. To be used in *Any* placeholders.
    ArrayOfDvsFilterPolicy(Vec<DvsFilterPolicy>),
    /// A boxed array of *DVSHostLocalPortInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSHostLocalPortInfo")]
    ArrayOfDvsHostLocalPortInfo(Vec<DvsHostLocalPortInfo>),
    /// A boxed array of *DVPortStatus*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortStatus")]
    ArrayOfDvPortStatus(Vec<DvPortStatus>),
    /// A boxed array of *DVPortSetting*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortSetting")]
    ArrayOfDvPortSetting(Vec<Box<dyn super::traits::DvPortSettingTrait>>),
    /// A boxed array of *DVPortState*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortState")]
    ArrayOfDvPortState(Vec<DvPortState>),
    /// A boxed array of *DvsTrafficFilterConfig*. To be used in *Any* placeholders.
    ArrayOfDvsTrafficFilterConfig(Vec<Box<dyn super::traits::DvsTrafficFilterConfigTrait>>),
    /// A boxed array of *DvsTrafficFilterConfigSpec*. To be used in *Any* placeholders.
    ArrayOfDvsTrafficFilterConfigSpec(Vec<DvsTrafficFilterConfigSpec>),
    /// A boxed array of *DVSTrafficShapingPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSTrafficShapingPolicy")]
    ArrayOfDvsTrafficShapingPolicy(Vec<DvsTrafficShapingPolicy>),
    /// A boxed array of *DVSVendorSpecificConfig*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSVendorSpecificConfig")]
    ArrayOfDvsVendorSpecificConfig(Vec<DvsVendorSpecificConfig>),
    /// A boxed array of *DVPortgroupConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupConfigInfo")]
    ArrayOfDvPortgroupConfigInfo(Vec<DvPortgroupConfigInfo>),
    /// A boxed array of *DVPortgroupConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupConfigSpec")]
    ArrayOfDvPortgroupConfigSpec(Vec<DvPortgroupConfigSpec>),
    /// A boxed array of *DistributedVirtualPortgroupNsxPortgroupOperationResult*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualPortgroupNsxPortgroupOperationResult(Vec<DistributedVirtualPortgroupNsxPortgroupOperationResult>),
    /// A boxed array of *DVPortgroupPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupPolicy")]
    ArrayOfDvPortgroupPolicy(Vec<Box<dyn super::traits::DvPortgroupPolicyTrait>>),
    /// A boxed array of *DistributedVirtualPortgroupProblem*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualPortgroupProblem(Vec<DistributedVirtualPortgroupProblem>),
    /// A boxed array of *DistributedVirtualPortgroupInfo*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualPortgroupInfo(Vec<DistributedVirtualPortgroupInfo>),
    /// A boxed array of *DVPortgroupSelection*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupSelection")]
    ArrayOfDvPortgroupSelection(Vec<DvPortgroupSelection>),
    /// A boxed array of *DistributedVirtualSwitchInfo*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchInfo(Vec<DistributedVirtualSwitchInfo>),
    /// A boxed array of *DistributedVirtualSwitchManagerCompatibilityResult*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerCompatibilityResult(Vec<DistributedVirtualSwitchManagerCompatibilityResult>),
    /// A boxed array of *DVSManagerDvsConfigTarget*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSManagerDvsConfigTarget")]
    ArrayOfDvsManagerDvsConfigTarget(Vec<DvsManagerDvsConfigTarget>),
    /// A boxed array of *DistributedVirtualSwitchManagerDvsProductSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerDvsProductSpec(Vec<DistributedVirtualSwitchManagerDvsProductSpec>),
    /// A boxed array of *DistributedVirtualSwitchManagerHostArrayFilter*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerHostArrayFilter(Vec<DistributedVirtualSwitchManagerHostArrayFilter>),
    /// A boxed array of *DistributedVirtualSwitchManagerHostContainer*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerHostContainer(Vec<DistributedVirtualSwitchManagerHostContainer>),
    /// A boxed array of *DistributedVirtualSwitchManagerHostContainerFilter*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerHostContainerFilter(Vec<DistributedVirtualSwitchManagerHostContainerFilter>),
    /// A boxed array of *DistributedVirtualSwitchManagerHostDvsFilterSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerHostDvsFilterSpec(Vec<Box<dyn super::traits::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait>>),
    /// A boxed array of *DistributedVirtualSwitchManagerHostDvsMembershipFilter*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerHostDvsMembershipFilter(Vec<DistributedVirtualSwitchManagerHostDvsMembershipFilter>),
    /// A boxed array of *DistributedVirtualSwitchManagerImportResult*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchManagerImportResult(Vec<DistributedVirtualSwitchManagerImportResult>),
    /// A boxed array of *DVSManagerPhysicalNicsList*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    #[serde(rename = "ArrayOfDVSManagerPhysicalNicsList")]
    ArrayOfDvsManagerPhysicalNicsList(Vec<DvsManagerPhysicalNicsList>),
    /// A boxed array of *DVSSelection*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSSelection")]
    ArrayOfDvsSelection(Vec<DvsSelection>),
    /// A boxed array of *EntityBackup*. To be used in *Any* placeholders.
    ArrayOfEntityBackup(Vec<EntityBackup>),
    /// A boxed array of *EntityBackupConfig*. To be used in *Any* placeholders.
    ArrayOfEntityBackupConfig(Vec<EntityBackupConfig>),
    /// A boxed array of *DistributedVirtualSwitchHostMember*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMember(Vec<DistributedVirtualSwitchHostMember>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberBacking*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberBacking(Vec<Box<dyn super::traits::DistributedVirtualSwitchHostMemberBackingTrait>>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberConfigInfo*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberConfigInfo(Vec<DistributedVirtualSwitchHostMemberConfigInfo>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberConfigSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberConfigSpec(Vec<DistributedVirtualSwitchHostMemberConfigSpec>),
    /// A boxed array of *HostMemberHealthCheckResult*. To be used in *Any* placeholders.
    ArrayOfHostMemberHealthCheckResult(Vec<Box<dyn super::traits::HostMemberHealthCheckResultTrait>>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberPnicBacking*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberPnicBacking(Vec<DistributedVirtualSwitchHostMemberPnicBacking>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberPnicSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberPnicSpec(Vec<DistributedVirtualSwitchHostMemberPnicSpec>),
    /// A boxed array of *HostMemberRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostMemberRuntimeInfo(Vec<HostMemberRuntimeInfo>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberRuntimeState*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberRuntimeState(Vec<DistributedVirtualSwitchHostMemberRuntimeState>),
    /// A boxed array of *DistributedVirtualSwitchHostMemberTransportZoneInfo*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostMemberTransportZoneInfo(Vec<DistributedVirtualSwitchHostMemberTransportZoneInfo>),
    /// A boxed array of *HostMemberUplinkHealthCheckResult*. To be used in *Any* placeholders.
    ArrayOfHostMemberUplinkHealthCheckResult(Vec<Box<dyn super::traits::HostMemberUplinkHealthCheckResultTrait>>),
    /// A boxed array of *DistributedVirtualSwitchHostProductSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchHostProductSpec(Vec<DistributedVirtualSwitchHostProductSpec>),
    /// A boxed array of *DistributedVirtualSwitchKeyedOpaqueBlob*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchKeyedOpaqueBlob(Vec<DistributedVirtualSwitchKeyedOpaqueBlob>),
    /// A boxed array of *DistributedVirtualSwitchNetworkOffloadSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfDistributedVirtualSwitchNetworkOffloadSpec(Vec<DistributedVirtualSwitchNetworkOffloadSpec>),
    /// A boxed array of *DVSNetworkResourcePool*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSNetworkResourcePool")]
    ArrayOfDvsNetworkResourcePool(Vec<DvsNetworkResourcePool>),
    /// A boxed array of *DVSNetworkResourcePoolAllocationInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSNetworkResourcePoolAllocationInfo")]
    ArrayOfDvsNetworkResourcePoolAllocationInfo(Vec<DvsNetworkResourcePoolAllocationInfo>),
    /// A boxed array of *DVSNetworkResourcePoolConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSNetworkResourcePoolConfigSpec")]
    ArrayOfDvsNetworkResourcePoolConfigSpec(Vec<DvsNetworkResourcePoolConfigSpec>),
    /// A boxed array of *DistributedVirtualSwitchPortConnectee*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchPortConnectee(Vec<DistributedVirtualSwitchPortConnectee>),
    /// A boxed array of *DistributedVirtualSwitchPortConnection*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchPortConnection(Vec<DistributedVirtualSwitchPortConnection>),
    /// A boxed array of *DistributedVirtualSwitchPortCriteria*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchPortCriteria(Vec<DistributedVirtualSwitchPortCriteria>),
    /// A boxed array of *DistributedVirtualSwitchPortStatistics*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchPortStatistics(Vec<DistributedVirtualSwitchPortStatistics>),
    /// A boxed array of *DistributedVirtualSwitchProductSpec*. To be used in *Any* placeholders.
    ArrayOfDistributedVirtualSwitchProductSpec(Vec<DistributedVirtualSwitchProductSpec>),
    /// A boxed array of *DvsTrafficRule*. To be used in *Any* placeholders.
    ArrayOfDvsTrafficRule(Vec<DvsTrafficRule>),
    /// A boxed array of *DvsAcceptNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsAcceptNetworkRuleAction(Vec<DvsAcceptNetworkRuleAction>),
    /// A boxed array of *DvsNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsNetworkRuleAction(Vec<Box<dyn super::traits::DvsNetworkRuleActionTrait>>),
    /// A boxed array of *DvsCopyNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsCopyNetworkRuleAction(Vec<DvsCopyNetworkRuleAction>),
    /// A boxed array of *DvsDropNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsDropNetworkRuleAction(Vec<DvsDropNetworkRuleAction>),
    /// A boxed array of *DvsGreEncapNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsGreEncapNetworkRuleAction(Vec<DvsGreEncapNetworkRuleAction>),
    /// A boxed array of *DvsIpPort*. To be used in *Any* placeholders.
    ArrayOfDvsIpPort(Vec<Box<dyn super::traits::DvsIpPortTrait>>),
    /// A boxed array of *DvsIpPortRange*. To be used in *Any* placeholders.
    ArrayOfDvsIpPortRange(Vec<DvsIpPortRange>),
    /// A boxed array of *DvsIpNetworkRuleQualifier*. To be used in *Any* placeholders.
    ArrayOfDvsIpNetworkRuleQualifier(Vec<DvsIpNetworkRuleQualifier>),
    /// A boxed array of *DvsLogNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsLogNetworkRuleAction(Vec<DvsLogNetworkRuleAction>),
    /// A boxed array of *DvsMacNetworkRuleQualifier*. To be used in *Any* placeholders.
    ArrayOfDvsMacNetworkRuleQualifier(Vec<DvsMacNetworkRuleQualifier>),
    /// A boxed array of *DvsMacRewriteNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsMacRewriteNetworkRuleAction(Vec<DvsMacRewriteNetworkRuleAction>),
    /// A boxed array of *DvsPuntNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsPuntNetworkRuleAction(Vec<DvsPuntNetworkRuleAction>),
    /// A boxed array of *DvsNetworkRuleQualifier*. To be used in *Any* placeholders.
    ArrayOfDvsNetworkRuleQualifier(Vec<Box<dyn super::traits::DvsNetworkRuleQualifierTrait>>),
    /// A boxed array of *DvsRateLimitNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsRateLimitNetworkRuleAction(Vec<DvsRateLimitNetworkRuleAction>),
    /// A boxed array of *DvsSingleIpPort*. To be used in *Any* placeholders.
    ArrayOfDvsSingleIpPort(Vec<DvsSingleIpPort>),
    /// A boxed array of *DvsSystemTrafficNetworkRuleQualifier*. To be used in *Any* placeholders.
    ArrayOfDvsSystemTrafficNetworkRuleQualifier(Vec<DvsSystemTrafficNetworkRuleQualifier>),
    /// A boxed array of *DvsUpdateTagNetworkRuleAction*. To be used in *Any* placeholders.
    ArrayOfDvsUpdateTagNetworkRuleAction(Vec<DvsUpdateTagNetworkRuleAction>),
    /// A boxed array of *DvsTrafficRuleset*. To be used in *Any* placeholders.
    ArrayOfDvsTrafficRuleset(Vec<DvsTrafficRuleset>),
    /// A boxed array of *DVSVmVnicNetworkResourcePool*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSVmVnicNetworkResourcePool")]
    ArrayOfDvsVmVnicNetworkResourcePool(Vec<DvsVmVnicNetworkResourcePool>),
    /// A boxed array of *DvsVmVnicResourcePoolConfigSpec*. To be used in *Any* placeholders.
    ArrayOfDvsVmVnicResourcePoolConfigSpec(Vec<DvsVmVnicResourcePoolConfigSpec>),
    /// A boxed array of *DvsVmVnicResourceAllocation*. To be used in *Any* placeholders.
    ArrayOfDvsVmVnicResourceAllocation(Vec<DvsVmVnicResourceAllocation>),
    /// A boxed array of *DvsVmVnicNetworkResourcePoolRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfDvsVmVnicNetworkResourcePoolRuntimeInfo(Vec<DvsVmVnicNetworkResourcePoolRuntimeInfo>),
    /// A boxed array of *DvsVnicAllocatedResource*. To be used in *Any* placeholders.
    ArrayOfDvsVnicAllocatedResource(Vec<DvsVnicAllocatedResource>),
    /// A boxed array of *VMwareDVSConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSConfigInfo")]
    ArrayOfVMwareDvsConfigInfo(Vec<VMwareDvsConfigInfo>),
    /// A boxed array of *VMwareDVSConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSConfigSpec")]
    ArrayOfVMwareDvsConfigSpec(Vec<VMwareDvsConfigSpec>),
    /// A boxed array of *VMwareDvsDpuCapability*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVMwareDvsDpuCapability(Vec<VMwareDvsDpuCapability>),
    /// A boxed array of *DVSFailureCriteria*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSFailureCriteria")]
    ArrayOfDvsFailureCriteria(Vec<DvsFailureCriteria>),
    /// A boxed array of *VMwareDVSFeatureCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSFeatureCapability")]
    ArrayOfVMwareDvsFeatureCapability(Vec<VMwareDvsFeatureCapability>),
    /// A boxed array of *VMwareIpfixConfig*. To be used in *Any* placeholders.
    ArrayOfVMwareIpfixConfig(Vec<VMwareIpfixConfig>),
    /// A boxed array of *VMwareDvsIpfixCapability*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsIpfixCapability(Vec<VMwareDvsIpfixCapability>),
    /// A boxed array of *VMwareDvsLacpCapability*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsLacpCapability(Vec<VMwareDvsLacpCapability>),
    /// A boxed array of *VMwareDvsLacpGroupConfig*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsLacpGroupConfig(Vec<VMwareDvsLacpGroupConfig>),
    /// A boxed array of *VMwareDvsLacpGroupSpec*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsLacpGroupSpec(Vec<VMwareDvsLacpGroupSpec>),
    /// A boxed array of *VMwareDvsLagIpfixConfig*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsLagIpfixConfig(Vec<VMwareDvsLagIpfixConfig>),
    /// A boxed array of *VMwareDvsLagVlanConfig*. To be used in *Any* placeholders.
    ArrayOfVMwareDvsLagVlanConfig(Vec<VMwareDvsLagVlanConfig>),
    /// A boxed array of *DVSMacLearningPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSMacLearningPolicy")]
    ArrayOfDvsMacLearningPolicy(Vec<DvsMacLearningPolicy>),
    /// A boxed array of *DVSMacManagementPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSMacManagementPolicy")]
    ArrayOfDvsMacManagementPolicy(Vec<DvsMacManagementPolicy>),
    /// A boxed array of *VMwareDvsMtuCapability*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ArrayOfVMwareDvsMtuCapability(Vec<VMwareDvsMtuCapability>),
    /// A boxed array of *VMwareDVSMtuHealthCheckResult*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSMtuHealthCheckResult")]
    ArrayOfVMwareDvsMtuHealthCheckResult(Vec<VMwareDvsMtuHealthCheckResult>),
    /// A boxed array of *VMwareDVSPvlanConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSPvlanConfigSpec")]
    ArrayOfVMwareDvsPvlanConfigSpec(Vec<VMwareDvsPvlanConfigSpec>),
    /// A boxed array of *VMwareDVSPvlanMapEntry*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSPvlanMapEntry")]
    ArrayOfVMwareDvsPvlanMapEntry(Vec<VMwareDvsPvlanMapEntry>),
    /// A boxed array of *VmwareDistributedVirtualSwitchPvlanSpec*. To be used in *Any* placeholders.
    ArrayOfVmwareDistributedVirtualSwitchPvlanSpec(Vec<VmwareDistributedVirtualSwitchPvlanSpec>),
    /// A boxed array of *DVSSecurityPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVSSecurityPolicy")]
    ArrayOfDvsSecurityPolicy(Vec<DvsSecurityPolicy>),
    /// A boxed array of *VMwareDVSTeamingHealthCheckConfig*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSTeamingHealthCheckConfig")]
    ArrayOfVMwareDvsTeamingHealthCheckConfig(Vec<VMwareDvsTeamingHealthCheckConfig>),
    /// A boxed array of *VMwareDVSTeamingHealthCheckResult*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSTeamingHealthCheckResult")]
    ArrayOfVMwareDvsTeamingHealthCheckResult(Vec<VMwareDvsTeamingHealthCheckResult>),
    /// A boxed array of *VmwareDistributedVirtualSwitchTrunkVlanSpec*. To be used in *Any* placeholders.
    ArrayOfVmwareDistributedVirtualSwitchTrunkVlanSpec(Vec<VmwareDistributedVirtualSwitchTrunkVlanSpec>),
    /// A boxed array of *VMwareUplinkLacpPolicy*. To be used in *Any* placeholders.
    ArrayOfVMwareUplinkLacpPolicy(Vec<VMwareUplinkLacpPolicy>),
    /// A boxed array of *VMwareUplinkPortOrderPolicy*. To be used in *Any* placeholders.
    ArrayOfVMwareUplinkPortOrderPolicy(Vec<VMwareUplinkPortOrderPolicy>),
    /// A boxed array of *VmwareUplinkPortTeamingPolicy*. To be used in *Any* placeholders.
    ArrayOfVmwareUplinkPortTeamingPolicy(Vec<VmwareUplinkPortTeamingPolicy>),
    /// A boxed array of *VMwareDVSPortgroupPolicy*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSPortgroupPolicy")]
    ArrayOfVMwareDvsPortgroupPolicy(Vec<VMwareDvsPortgroupPolicy>),
    /// A boxed array of *VMwareDVSVlanHealthCheckResult*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSVlanHealthCheckResult")]
    ArrayOfVMwareDvsVlanHealthCheckResult(Vec<VMwareDvsVlanHealthCheckResult>),
    /// A boxed array of *VmwareDistributedVirtualSwitchVlanIdSpec*. To be used in *Any* placeholders.
    ArrayOfVmwareDistributedVirtualSwitchVlanIdSpec(Vec<VmwareDistributedVirtualSwitchVlanIdSpec>),
    /// A boxed array of *VMwareDVSVlanMtuHealthCheckConfig*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSVlanMtuHealthCheckConfig")]
    ArrayOfVMwareDvsVlanMtuHealthCheckConfig(Vec<VMwareDvsVlanMtuHealthCheckConfig>),
    /// A boxed array of *VmwareDistributedVirtualSwitchVlanSpec*. To be used in *Any* placeholders.
    ArrayOfVmwareDistributedVirtualSwitchVlanSpec(Vec<Box<dyn super::traits::VmwareDistributedVirtualSwitchVlanSpecTrait>>),
    /// A boxed array of *VMwareDVSHealthCheckConfig*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSHealthCheckConfig")]
    ArrayOfVMwareDvsHealthCheckConfig(Vec<Box<dyn super::traits::VMwareDvsHealthCheckConfigTrait>>),
    /// A boxed array of *VMwareDVSHealthCheckCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSHealthCheckCapability")]
    ArrayOfVMwareDvsHealthCheckCapability(Vec<VMwareDvsHealthCheckCapability>),
    /// A boxed array of *VMwareDVSPortSetting*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSPortSetting")]
    ArrayOfVMwareDvsPortSetting(Vec<VMwareDvsPortSetting>),
    /// A boxed array of *VMwareDVSVspanConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSVspanConfigSpec")]
    ArrayOfVMwareDvsVspanConfigSpec(Vec<VMwareDvsVspanConfigSpec>),
    /// A boxed array of *VMwareDVSVspanCapability*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMwareDVSVspanCapability")]
    ArrayOfVMwareDvsVspanCapability(Vec<VMwareDvsVspanCapability>),
    /// A boxed array of *VMwareVspanPort*. To be used in *Any* placeholders.
    ArrayOfVMwareVspanPort(Vec<VMwareVspanPort>),
    /// A boxed array of *VMwareVspanSession*. To be used in *Any* placeholders.
    ArrayOfVMwareVspanSession(Vec<VMwareVspanSession>),
    /// A boxed array of *CryptoKeyId*. To be used in *Any* placeholders.
    ArrayOfCryptoKeyId(Vec<CryptoKeyId>),
    /// A boxed array of *CryptoKeyPlain*. To be used in *Any* placeholders.
    ArrayOfCryptoKeyPlain(Vec<CryptoKeyPlain>),
    /// A boxed array of *CryptoKeyResult*. To be used in *Any* placeholders.
    ArrayOfCryptoKeyResult(Vec<CryptoKeyResult>),
    /// A boxed array of *CryptoManagerHostKeyStatus*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfCryptoManagerHostKeyStatus(Vec<CryptoManagerHostKeyStatus>),
    /// A boxed array of *CryptoManagerKmipCertSignRequest*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfCryptoManagerKmipCertSignRequest(Vec<CryptoManagerKmipCertSignRequest>),
    /// A boxed array of *CryptoManagerKmipCertificateInfo*. To be used in *Any* placeholders.
    ArrayOfCryptoManagerKmipCertificateInfo(Vec<CryptoManagerKmipCertificateInfo>),
    /// A boxed array of *CryptoManagerKmipClusterStatus*. To be used in *Any* placeholders.
    ArrayOfCryptoManagerKmipClusterStatus(Vec<CryptoManagerKmipClusterStatus>),
    /// A boxed array of *CryptoManagerKmipCryptoKeyStatus*. To be used in *Any* placeholders.
    ArrayOfCryptoManagerKmipCryptoKeyStatus(Vec<CryptoManagerKmipCryptoKeyStatus>),
    /// A boxed array of *CryptoManagerKmipCustomAttributeSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfCryptoManagerKmipCustomAttributeSpec(Vec<CryptoManagerKmipCustomAttributeSpec>),
    /// A boxed array of *CryptoManagerKmipServerCertInfo*. To be used in *Any* placeholders.
    ArrayOfCryptoManagerKmipServerCertInfo(Vec<CryptoManagerKmipServerCertInfo>),
    /// A boxed array of *CryptoManagerKmipServerStatus*. To be used in *Any* placeholders.
    ArrayOfCryptoManagerKmipServerStatus(Vec<CryptoManagerKmipServerStatus>),
    /// A boxed array of *CryptoSpec*. To be used in *Any* placeholders.
    ArrayOfCryptoSpec(Vec<Box<dyn super::traits::CryptoSpecTrait>>),
    /// A boxed array of *CryptoSpecDecrypt*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecDecrypt(Vec<CryptoSpecDecrypt>),
    /// A boxed array of *CryptoSpecDeepRecrypt*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecDeepRecrypt(Vec<CryptoSpecDeepRecrypt>),
    /// A boxed array of *CryptoSpecEncrypt*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecEncrypt(Vec<CryptoSpecEncrypt>),
    /// A boxed array of *CryptoSpecNoOp*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecNoOp(Vec<Box<dyn super::traits::CryptoSpecNoOpTrait>>),
    /// A boxed array of *CryptoSpecRegister*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecRegister(Vec<CryptoSpecRegister>),
    /// A boxed array of *CryptoSpecShallowRecrypt*. To be used in *Any* placeholders.
    ArrayOfCryptoSpecShallowRecrypt(Vec<CryptoSpecShallowRecrypt>),
    /// A boxed array of *KeyProviderId*. To be used in *Any* placeholders.
    ArrayOfKeyProviderId(Vec<KeyProviderId>),
    /// A boxed array of *KmipClusterInfo*. To be used in *Any* placeholders.
    ArrayOfKmipClusterInfo(Vec<KmipClusterInfo>),
    /// A boxed array of *KmipServerInfo*. To be used in *Any* placeholders.
    ArrayOfKmipServerInfo(Vec<KmipServerInfo>),
    /// A boxed array of *KmipServerSpec*. To be used in *Any* placeholders.
    ArrayOfKmipServerSpec(Vec<KmipServerSpec>),
    /// A boxed array of *KmipServerStatus*. To be used in *Any* placeholders.
    ArrayOfKmipServerStatus(Vec<KmipServerStatus>),
    /// A boxed array of *AccountCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfAccountCreatedEvent(Vec<AccountCreatedEvent>),
    /// A boxed array of *AccountRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfAccountRemovedEvent(Vec<AccountRemovedEvent>),
    /// A boxed array of *AccountUpdatedEvent*. To be used in *Any* placeholders.
    ArrayOfAccountUpdatedEvent(Vec<AccountUpdatedEvent>),
    /// A boxed array of *AdminPasswordNotChangedEvent*. To be used in *Any* placeholders.
    ArrayOfAdminPasswordNotChangedEvent(Vec<AdminPasswordNotChangedEvent>),
    /// A boxed array of *AlarmAcknowledgedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmAcknowledgedEvent(Vec<AlarmAcknowledgedEvent>),
    /// A boxed array of *AlarmActionTriggeredEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmActionTriggeredEvent(Vec<AlarmActionTriggeredEvent>),
    /// A boxed array of *AlarmClearedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmClearedEvent(Vec<AlarmClearedEvent>),
    /// A boxed array of *AlarmCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmCreatedEvent(Vec<AlarmCreatedEvent>),
    /// A boxed array of *AlarmEmailCompletedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmEmailCompletedEvent(Vec<AlarmEmailCompletedEvent>),
    /// A boxed array of *AlarmEmailFailedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmEmailFailedEvent(Vec<AlarmEmailFailedEvent>),
    /// A boxed array of *AlarmEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmEvent(Vec<Box<dyn super::traits::AlarmEventTrait>>),
    /// A boxed array of *AlarmEventArgument*. To be used in *Any* placeholders.
    ArrayOfAlarmEventArgument(Vec<AlarmEventArgument>),
    /// A boxed array of *AlarmReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmReconfiguredEvent(Vec<AlarmReconfiguredEvent>),
    /// A boxed array of *AlarmRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmRemovedEvent(Vec<AlarmRemovedEvent>),
    /// A boxed array of *AlarmScriptCompleteEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmScriptCompleteEvent(Vec<AlarmScriptCompleteEvent>),
    /// A boxed array of *AlarmScriptFailedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmScriptFailedEvent(Vec<AlarmScriptFailedEvent>),
    /// A boxed array of *AlarmSnmpCompletedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmSnmpCompletedEvent(Vec<AlarmSnmpCompletedEvent>),
    /// A boxed array of *AlarmSnmpFailedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmSnmpFailedEvent(Vec<AlarmSnmpFailedEvent>),
    /// A boxed array of *AlarmStatusChangedEvent*. To be used in *Any* placeholders.
    ArrayOfAlarmStatusChangedEvent(Vec<AlarmStatusChangedEvent>),
    /// A boxed array of *AllVirtualMachinesLicensedEvent*. To be used in *Any* placeholders.
    ArrayOfAllVirtualMachinesLicensedEvent(Vec<AllVirtualMachinesLicensedEvent>),
    /// A boxed array of *AlreadyAuthenticatedSessionEvent*. To be used in *Any* placeholders.
    ArrayOfAlreadyAuthenticatedSessionEvent(Vec<AlreadyAuthenticatedSessionEvent>),
    /// A boxed array of *AuthorizationEvent*. To be used in *Any* placeholders.
    ArrayOfAuthorizationEvent(Vec<Box<dyn super::traits::AuthorizationEventTrait>>),
    /// A boxed array of *BadUsernameSessionEvent*. To be used in *Any* placeholders.
    ArrayOfBadUsernameSessionEvent(Vec<BadUsernameSessionEvent>),
    /// A boxed array of *CanceledHostOperationEvent*. To be used in *Any* placeholders.
    ArrayOfCanceledHostOperationEvent(Vec<CanceledHostOperationEvent>),
    /// A boxed array of *ChangesInfoEventArgument*. To be used in *Any* placeholders.
    ArrayOfChangesInfoEventArgument(Vec<ChangesInfoEventArgument>),
    /// A boxed array of *ClusterComplianceCheckedEvent*. To be used in *Any* placeholders.
    ArrayOfClusterComplianceCheckedEvent(Vec<ClusterComplianceCheckedEvent>),
    /// A boxed array of *ClusterCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfClusterCreatedEvent(Vec<ClusterCreatedEvent>),
    /// A boxed array of *ClusterDestroyedEvent*. To be used in *Any* placeholders.
    ArrayOfClusterDestroyedEvent(Vec<ClusterDestroyedEvent>),
    /// A boxed array of *ClusterEvent*. To be used in *Any* placeholders.
    ArrayOfClusterEvent(Vec<Box<dyn super::traits::ClusterEventTrait>>),
    /// A boxed array of *ClusterOvercommittedEvent*. To be used in *Any* placeholders.
    ArrayOfClusterOvercommittedEvent(Vec<Box<dyn super::traits::ClusterOvercommittedEventTrait>>),
    /// A boxed array of *ClusterReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfClusterReconfiguredEvent(Vec<ClusterReconfiguredEvent>),
    /// A boxed array of *ClusterStatusChangedEvent*. To be used in *Any* placeholders.
    ArrayOfClusterStatusChangedEvent(Vec<Box<dyn super::traits::ClusterStatusChangedEventTrait>>),
    /// A boxed array of *ComputeResourceEventArgument*. To be used in *Any* placeholders.
    ArrayOfComputeResourceEventArgument(Vec<ComputeResourceEventArgument>),
    /// A boxed array of *CustomFieldDefAddedEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldDefAddedEvent(Vec<CustomFieldDefAddedEvent>),
    /// A boxed array of *CustomFieldDefEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldDefEvent(Vec<Box<dyn super::traits::CustomFieldDefEventTrait>>),
    /// A boxed array of *CustomFieldDefRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldDefRemovedEvent(Vec<CustomFieldDefRemovedEvent>),
    /// A boxed array of *CustomFieldDefRenamedEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldDefRenamedEvent(Vec<CustomFieldDefRenamedEvent>),
    /// A boxed array of *CustomFieldEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldEvent(Vec<Box<dyn super::traits::CustomFieldEventTrait>>),
    /// A boxed array of *CustomFieldValueChangedEvent*. To be used in *Any* placeholders.
    ArrayOfCustomFieldValueChangedEvent(Vec<CustomFieldValueChangedEvent>),
    /// A boxed array of *CustomizationEvent*. To be used in *Any* placeholders.
    ArrayOfCustomizationEvent(Vec<Box<dyn super::traits::CustomizationEventTrait>>),
    /// A boxed array of *CustomizationFailed*. To be used in *Any* placeholders.
    ArrayOfCustomizationFailed(Vec<Box<dyn super::traits::CustomizationFailedTrait>>),
    /// A boxed array of *CustomizationLinuxIdentityFailed*. To be used in *Any* placeholders.
    ArrayOfCustomizationLinuxIdentityFailed(Vec<CustomizationLinuxIdentityFailed>),
    /// A boxed array of *CustomizationNetworkSetupFailed*. To be used in *Any* placeholders.
    ArrayOfCustomizationNetworkSetupFailed(Vec<CustomizationNetworkSetupFailed>),
    /// A boxed array of *CustomizationStartedEvent*. To be used in *Any* placeholders.
    ArrayOfCustomizationStartedEvent(Vec<CustomizationStartedEvent>),
    /// A boxed array of *CustomizationSucceeded*. To be used in *Any* placeholders.
    ArrayOfCustomizationSucceeded(Vec<CustomizationSucceeded>),
    /// A boxed array of *CustomizationSysprepFailed*. To be used in *Any* placeholders.
    ArrayOfCustomizationSysprepFailed(Vec<CustomizationSysprepFailed>),
    /// A boxed array of *CustomizationUnknownFailure*. To be used in *Any* placeholders.
    ArrayOfCustomizationUnknownFailure(Vec<CustomizationUnknownFailure>),
    /// A boxed array of *DVPortgroupCreatedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupCreatedEvent")]
    ArrayOfDvPortgroupCreatedEvent(Vec<DvPortgroupCreatedEvent>),
    /// A boxed array of *DVPortgroupDestroyedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupDestroyedEvent")]
    ArrayOfDvPortgroupDestroyedEvent(Vec<DvPortgroupDestroyedEvent>),
    /// A boxed array of *DVPortgroupEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupEvent")]
    ArrayOfDvPortgroupEvent(Vec<Box<dyn super::traits::DvPortgroupEventTrait>>),
    /// A boxed array of *DVPortgroupReconfiguredEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupReconfiguredEvent")]
    ArrayOfDvPortgroupReconfiguredEvent(Vec<DvPortgroupReconfiguredEvent>),
    /// A boxed array of *DVPortgroupRenamedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortgroupRenamedEvent")]
    ArrayOfDvPortgroupRenamedEvent(Vec<DvPortgroupRenamedEvent>),
    /// A boxed array of *DasAdmissionControlDisabledEvent*. To be used in *Any* placeholders.
    ArrayOfDasAdmissionControlDisabledEvent(Vec<DasAdmissionControlDisabledEvent>),
    /// A boxed array of *DasAdmissionControlEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfDasAdmissionControlEnabledEvent(Vec<DasAdmissionControlEnabledEvent>),
    /// A boxed array of *DasAgentFoundEvent*. To be used in *Any* placeholders.
    ArrayOfDasAgentFoundEvent(Vec<DasAgentFoundEvent>),
    /// A boxed array of *DasAgentUnavailableEvent*. To be used in *Any* placeholders.
    ArrayOfDasAgentUnavailableEvent(Vec<DasAgentUnavailableEvent>),
    /// A boxed array of *DasClusterIsolatedEvent*. To be used in *Any* placeholders.
    ArrayOfDasClusterIsolatedEvent(Vec<DasClusterIsolatedEvent>),
    /// A boxed array of *DasDisabledEvent*. To be used in *Any* placeholders.
    ArrayOfDasDisabledEvent(Vec<DasDisabledEvent>),
    /// A boxed array of *DasEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfDasEnabledEvent(Vec<DasEnabledEvent>),
    /// A boxed array of *DasHostFailedEvent*. To be used in *Any* placeholders.
    ArrayOfDasHostFailedEvent(Vec<DasHostFailedEvent>),
    /// A boxed array of *DasHostIsolatedEvent*. To be used in *Any* placeholders.
    ArrayOfDasHostIsolatedEvent(Vec<DasHostIsolatedEvent>),
    /// A boxed array of *DatacenterCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfDatacenterCreatedEvent(Vec<DatacenterCreatedEvent>),
    /// A boxed array of *DatacenterEvent*. To be used in *Any* placeholders.
    ArrayOfDatacenterEvent(Vec<Box<dyn super::traits::DatacenterEventTrait>>),
    /// A boxed array of *DatacenterEventArgument*. To be used in *Any* placeholders.
    ArrayOfDatacenterEventArgument(Vec<DatacenterEventArgument>),
    /// A boxed array of *DatacenterRenamedEvent*. To be used in *Any* placeholders.
    ArrayOfDatacenterRenamedEvent(Vec<DatacenterRenamedEvent>),
    /// A boxed array of *DatastoreCapacityIncreasedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreCapacityIncreasedEvent(Vec<DatastoreCapacityIncreasedEvent>),
    /// A boxed array of *DatastoreDestroyedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreDestroyedEvent(Vec<DatastoreDestroyedEvent>),
    /// A boxed array of *DatastoreDiscoveredEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreDiscoveredEvent(Vec<DatastoreDiscoveredEvent>),
    /// A boxed array of *DatastoreDuplicatedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreDuplicatedEvent(Vec<DatastoreDuplicatedEvent>),
    /// A boxed array of *DatastoreEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreEvent(Vec<Box<dyn super::traits::DatastoreEventTrait>>),
    /// A boxed array of *DatastoreEventArgument*. To be used in *Any* placeholders.
    ArrayOfDatastoreEventArgument(Vec<DatastoreEventArgument>),
    /// A boxed array of *DatastoreFileCopiedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreFileCopiedEvent(Vec<DatastoreFileCopiedEvent>),
    /// A boxed array of *DatastoreFileDeletedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreFileDeletedEvent(Vec<DatastoreFileDeletedEvent>),
    /// A boxed array of *DatastoreFileEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreFileEvent(Vec<Box<dyn super::traits::DatastoreFileEventTrait>>),
    /// A boxed array of *DatastoreFileMovedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreFileMovedEvent(Vec<DatastoreFileMovedEvent>),
    /// A boxed array of *DatastoreIORMReconfiguredEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDatastoreIORMReconfiguredEvent")]
    ArrayOfDatastoreIormReconfiguredEvent(Vec<DatastoreIormReconfiguredEvent>),
    /// A boxed array of *DatastorePrincipalConfigured*. To be used in *Any* placeholders.
    ArrayOfDatastorePrincipalConfigured(Vec<DatastorePrincipalConfigured>),
    /// A boxed array of *DatastoreRemovedOnHostEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreRemovedOnHostEvent(Vec<DatastoreRemovedOnHostEvent>),
    /// A boxed array of *DatastoreRenamedEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreRenamedEvent(Vec<DatastoreRenamedEvent>),
    /// A boxed array of *DatastoreRenamedOnHostEvent*. To be used in *Any* placeholders.
    ArrayOfDatastoreRenamedOnHostEvent(Vec<DatastoreRenamedOnHostEvent>),
    /// A boxed array of *DrsDisabledEvent*. To be used in *Any* placeholders.
    ArrayOfDrsDisabledEvent(Vec<DrsDisabledEvent>),
    /// A boxed array of *DrsEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfDrsEnabledEvent(Vec<DrsEnabledEvent>),
    /// A boxed array of *DrsEnteredStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfDrsEnteredStandbyModeEvent(Vec<DrsEnteredStandbyModeEvent>),
    /// A boxed array of *DrsEnteringStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfDrsEnteringStandbyModeEvent(Vec<DrsEnteringStandbyModeEvent>),
    /// A boxed array of *DrsExitStandbyModeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfDrsExitStandbyModeFailedEvent(Vec<DrsExitStandbyModeFailedEvent>),
    /// A boxed array of *DrsExitedStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfDrsExitedStandbyModeEvent(Vec<DrsExitedStandbyModeEvent>),
    /// A boxed array of *DrsExitingStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfDrsExitingStandbyModeEvent(Vec<DrsExitingStandbyModeEvent>),
    /// A boxed array of *DrsInvocationFailedEvent*. To be used in *Any* placeholders.
    ArrayOfDrsInvocationFailedEvent(Vec<DrsInvocationFailedEvent>),
    /// A boxed array of *DrsRecoveredFromFailureEvent*. To be used in *Any* placeholders.
    ArrayOfDrsRecoveredFromFailureEvent(Vec<DrsRecoveredFromFailureEvent>),
    /// A boxed array of *DrsResourceConfigureFailedEvent*. To be used in *Any* placeholders.
    ArrayOfDrsResourceConfigureFailedEvent(Vec<DrsResourceConfigureFailedEvent>),
    /// A boxed array of *DrsResourceConfigureSyncedEvent*. To be used in *Any* placeholders.
    ArrayOfDrsResourceConfigureSyncedEvent(Vec<DrsResourceConfigureSyncedEvent>),
    /// A boxed array of *DrsRuleComplianceEvent*. To be used in *Any* placeholders.
    ArrayOfDrsRuleComplianceEvent(Vec<DrsRuleComplianceEvent>),
    /// A boxed array of *DrsRuleViolationEvent*. To be used in *Any* placeholders.
    ArrayOfDrsRuleViolationEvent(Vec<DrsRuleViolationEvent>),
    /// A boxed array of *DrsSoftRuleViolationEvent*. To be used in *Any* placeholders.
    ArrayOfDrsSoftRuleViolationEvent(Vec<DrsSoftRuleViolationEvent>),
    /// A boxed array of *DrsVmMigratedEvent*. To be used in *Any* placeholders.
    ArrayOfDrsVmMigratedEvent(Vec<DrsVmMigratedEvent>),
    /// A boxed array of *DrsVmPoweredOnEvent*. To be used in *Any* placeholders.
    ArrayOfDrsVmPoweredOnEvent(Vec<DrsVmPoweredOnEvent>),
    /// A boxed array of *DuplicateIpDetectedEvent*. To be used in *Any* placeholders.
    ArrayOfDuplicateIpDetectedEvent(Vec<DuplicateIpDetectedEvent>),
    /// A boxed array of *DvpgImportEvent*. To be used in *Any* placeholders.
    ArrayOfDvpgImportEvent(Vec<DvpgImportEvent>),
    /// A boxed array of *DvpgRestoreEvent*. To be used in *Any* placeholders.
    ArrayOfDvpgRestoreEvent(Vec<DvpgRestoreEvent>),
    /// A boxed array of *DvsCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsCreatedEvent(Vec<DvsCreatedEvent>),
    /// A boxed array of *DvsDestroyedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsDestroyedEvent(Vec<DvsDestroyedEvent>),
    /// A boxed array of *DvsEvent*. To be used in *Any* placeholders.
    ArrayOfDvsEvent(Vec<Box<dyn super::traits::DvsEventTrait>>),
    /// A boxed array of *DvsEventArgument*. To be used in *Any* placeholders.
    ArrayOfDvsEventArgument(Vec<DvsEventArgument>),
    /// A boxed array of *DvsHealthStatusChangeEvent*. To be used in *Any* placeholders.
    ArrayOfDvsHealthStatusChangeEvent(Vec<Box<dyn super::traits::DvsHealthStatusChangeEventTrait>>),
    /// A boxed array of *DvsHostBackInSyncEvent*. To be used in *Any* placeholders.
    ArrayOfDvsHostBackInSyncEvent(Vec<DvsHostBackInSyncEvent>),
    /// A boxed array of *DvsHostJoinedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsHostJoinedEvent(Vec<DvsHostJoinedEvent>),
    /// A boxed array of *DvsHostLeftEvent*. To be used in *Any* placeholders.
    ArrayOfDvsHostLeftEvent(Vec<DvsHostLeftEvent>),
    /// A boxed array of *DvsHostStatusUpdated*. To be used in *Any* placeholders.
    ArrayOfDvsHostStatusUpdated(Vec<DvsHostStatusUpdated>),
    /// A boxed array of *DvsHostWentOutOfSyncEvent*. To be used in *Any* placeholders.
    ArrayOfDvsHostWentOutOfSyncEvent(Vec<DvsHostWentOutOfSyncEvent>),
    /// A boxed array of *DvsImportEvent*. To be used in *Any* placeholders.
    ArrayOfDvsImportEvent(Vec<DvsImportEvent>),
    /// A boxed array of *DvsMergedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsMergedEvent(Vec<DvsMergedEvent>),
    /// A boxed array of *DvsOutOfSyncHostArgument*. To be used in *Any* placeholders.
    ArrayOfDvsOutOfSyncHostArgument(Vec<DvsOutOfSyncHostArgument>),
    /// A boxed array of *DvsPortBlockedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortBlockedEvent(Vec<DvsPortBlockedEvent>),
    /// A boxed array of *DvsPortConnectedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortConnectedEvent(Vec<DvsPortConnectedEvent>),
    /// A boxed array of *DvsPortCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortCreatedEvent(Vec<DvsPortCreatedEvent>),
    /// A boxed array of *DvsPortDeletedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortDeletedEvent(Vec<DvsPortDeletedEvent>),
    /// A boxed array of *DvsPortDisconnectedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortDisconnectedEvent(Vec<DvsPortDisconnectedEvent>),
    /// A boxed array of *DvsPortEnteredPassthruEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortEnteredPassthruEvent(Vec<DvsPortEnteredPassthruEvent>),
    /// A boxed array of *DvsPortExitedPassthruEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortExitedPassthruEvent(Vec<DvsPortExitedPassthruEvent>),
    /// A boxed array of *DvsPortJoinPortgroupEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortJoinPortgroupEvent(Vec<DvsPortJoinPortgroupEvent>),
    /// A boxed array of *DvsPortLeavePortgroupEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortLeavePortgroupEvent(Vec<DvsPortLeavePortgroupEvent>),
    /// A boxed array of *DvsPortLinkDownEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortLinkDownEvent(Vec<DvsPortLinkDownEvent>),
    /// A boxed array of *DvsPortLinkUpEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortLinkUpEvent(Vec<DvsPortLinkUpEvent>),
    /// A boxed array of *DvsPortReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortReconfiguredEvent(Vec<DvsPortReconfiguredEvent>),
    /// A boxed array of *DvsPortRuntimeChangeEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortRuntimeChangeEvent(Vec<DvsPortRuntimeChangeEvent>),
    /// A boxed array of *DvsPortUnblockedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortUnblockedEvent(Vec<DvsPortUnblockedEvent>),
    /// A boxed array of *DvsPortVendorSpecificStateChangeEvent*. To be used in *Any* placeholders.
    ArrayOfDvsPortVendorSpecificStateChangeEvent(Vec<DvsPortVendorSpecificStateChangeEvent>),
    /// A boxed array of *DvsReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfDvsReconfiguredEvent(Vec<DvsReconfiguredEvent>),
    /// A boxed array of *DvsRenamedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsRenamedEvent(Vec<DvsRenamedEvent>),
    /// A boxed array of *DvsRestoreEvent*. To be used in *Any* placeholders.
    ArrayOfDvsRestoreEvent(Vec<DvsRestoreEvent>),
    /// A boxed array of *DvsUpgradeAvailableEvent*. To be used in *Any* placeholders.
    ArrayOfDvsUpgradeAvailableEvent(Vec<DvsUpgradeAvailableEvent>),
    /// A boxed array of *DvsUpgradeInProgressEvent*. To be used in *Any* placeholders.
    ArrayOfDvsUpgradeInProgressEvent(Vec<DvsUpgradeInProgressEvent>),
    /// A boxed array of *DvsUpgradeRejectedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsUpgradeRejectedEvent(Vec<DvsUpgradeRejectedEvent>),
    /// A boxed array of *DvsUpgradedEvent*. To be used in *Any* placeholders.
    ArrayOfDvsUpgradedEvent(Vec<DvsUpgradedEvent>),
    /// A boxed array of *EnteredMaintenanceModeEvent*. To be used in *Any* placeholders.
    ArrayOfEnteredMaintenanceModeEvent(Vec<EnteredMaintenanceModeEvent>),
    /// A boxed array of *EnteredStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfEnteredStandbyModeEvent(Vec<Box<dyn super::traits::EnteredStandbyModeEventTrait>>),
    /// A boxed array of *EnteringMaintenanceModeEvent*. To be used in *Any* placeholders.
    ArrayOfEnteringMaintenanceModeEvent(Vec<EnteringMaintenanceModeEvent>),
    /// A boxed array of *EnteringStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfEnteringStandbyModeEvent(Vec<Box<dyn super::traits::EnteringStandbyModeEventTrait>>),
    /// A boxed array of *EntityEventArgument*. To be used in *Any* placeholders.
    ArrayOfEntityEventArgument(Vec<Box<dyn super::traits::EntityEventArgumentTrait>>),
    /// A boxed array of *ErrorUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfErrorUpgradeEvent(Vec<ErrorUpgradeEvent>),
    /// A boxed array of *Event*. To be used in *Any* placeholders.
    ArrayOfEvent(Vec<Box<dyn super::traits::EventTrait>>),
    /// A boxed array of *EventArgument*. To be used in *Any* placeholders.
    ArrayOfEventArgument(Vec<Box<dyn super::traits::EventArgumentTrait>>),
    /// A boxed array of *EventDescription*. To be used in *Any* placeholders.
    ArrayOfEventDescription(Vec<EventDescription>),
    /// A boxed array of *EventArgDesc*. To be used in *Any* placeholders.
    ArrayOfEventArgDesc(Vec<EventArgDesc>),
    /// A boxed array of *EventDescriptionEventDetail*. To be used in *Any* placeholders.
    ArrayOfEventDescriptionEventDetail(Vec<EventDescriptionEventDetail>),
    /// A boxed array of *EventEx*. To be used in *Any* placeholders.
    ArrayOfEventEx(Vec<EventEx>),
    /// A boxed array of *EventFilterSpec*. To be used in *Any* placeholders.
    ArrayOfEventFilterSpec(Vec<EventFilterSpec>),
    /// A boxed array of *EventFilterSpecByEntity*. To be used in *Any* placeholders.
    ArrayOfEventFilterSpecByEntity(Vec<EventFilterSpecByEntity>),
    /// A boxed array of *EventFilterSpecByTime*. To be used in *Any* placeholders.
    ArrayOfEventFilterSpecByTime(Vec<EventFilterSpecByTime>),
    /// A boxed array of *EventFilterSpecByUsername*. To be used in *Any* placeholders.
    ArrayOfEventFilterSpecByUsername(Vec<EventFilterSpecByUsername>),
    /// A boxed array of *ExitMaintenanceModeEvent*. To be used in *Any* placeholders.
    ArrayOfExitMaintenanceModeEvent(Vec<ExitMaintenanceModeEvent>),
    /// A boxed array of *ExitStandbyModeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfExitStandbyModeFailedEvent(Vec<Box<dyn super::traits::ExitStandbyModeFailedEventTrait>>),
    /// A boxed array of *ExitedStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfExitedStandbyModeEvent(Vec<Box<dyn super::traits::ExitedStandbyModeEventTrait>>),
    /// A boxed array of *ExitingStandbyModeEvent*. To be used in *Any* placeholders.
    ArrayOfExitingStandbyModeEvent(Vec<Box<dyn super::traits::ExitingStandbyModeEventTrait>>),
    /// A boxed array of *ExtendedEvent*. To be used in *Any* placeholders.
    ArrayOfExtendedEvent(Vec<ExtendedEvent>),
    /// A boxed array of *ExtendedEventPair*. To be used in *Any* placeholders.
    ArrayOfExtendedEventPair(Vec<ExtendedEventPair>),
    /// A boxed array of *FailoverLevelRestored*. To be used in *Any* placeholders.
    ArrayOfFailoverLevelRestored(Vec<FailoverLevelRestored>),
    /// A boxed array of *FolderEventArgument*. To be used in *Any* placeholders.
    ArrayOfFolderEventArgument(Vec<FolderEventArgument>),
    /// A boxed array of *GeneralEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralEvent(Vec<Box<dyn super::traits::GeneralEventTrait>>),
    /// A boxed array of *GeneralHostErrorEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralHostErrorEvent(Vec<GeneralHostErrorEvent>),
    /// A boxed array of *GeneralHostInfoEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralHostInfoEvent(Vec<GeneralHostInfoEvent>),
    /// A boxed array of *GeneralHostWarningEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralHostWarningEvent(Vec<GeneralHostWarningEvent>),
    /// A boxed array of *GeneralUserEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralUserEvent(Vec<GeneralUserEvent>),
    /// A boxed array of *GeneralVmErrorEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralVmErrorEvent(Vec<GeneralVmErrorEvent>),
    /// A boxed array of *GeneralVmInfoEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralVmInfoEvent(Vec<GeneralVmInfoEvent>),
    /// A boxed array of *GeneralVmWarningEvent*. To be used in *Any* placeholders.
    ArrayOfGeneralVmWarningEvent(Vec<GeneralVmWarningEvent>),
    /// A boxed array of *GhostDvsProxySwitchDetectedEvent*. To be used in *Any* placeholders.
    ArrayOfGhostDvsProxySwitchDetectedEvent(Vec<GhostDvsProxySwitchDetectedEvent>),
    /// A boxed array of *GhostDvsProxySwitchRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfGhostDvsProxySwitchRemovedEvent(Vec<GhostDvsProxySwitchRemovedEvent>),
    /// A boxed array of *GlobalMessageChangedEvent*. To be used in *Any* placeholders.
    ArrayOfGlobalMessageChangedEvent(Vec<GlobalMessageChangedEvent>),
    /// A boxed array of *HealthStatusChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHealthStatusChangedEvent(Vec<HealthStatusChangedEvent>),
    /// A boxed array of *HostAddFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostAddFailedEvent(Vec<HostAddFailedEvent>),
    /// A boxed array of *HostAddedEvent*. To be used in *Any* placeholders.
    ArrayOfHostAddedEvent(Vec<HostAddedEvent>),
    /// A boxed array of *HostAdminDisableEvent*. To be used in *Any* placeholders.
    ArrayOfHostAdminDisableEvent(Vec<HostAdminDisableEvent>),
    /// A boxed array of *HostAdminEnableEvent*. To be used in *Any* placeholders.
    ArrayOfHostAdminEnableEvent(Vec<HostAdminEnableEvent>),
    /// A boxed array of *HostCnxFailedAccountFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedAccountFailedEvent(Vec<HostCnxFailedAccountFailedEvent>),
    /// A boxed array of *HostCnxFailedAlreadyManagedEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedAlreadyManagedEvent(Vec<HostCnxFailedAlreadyManagedEvent>),
    /// A boxed array of *HostCnxFailedBadCcagentEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedBadCcagentEvent(Vec<HostCnxFailedBadCcagentEvent>),
    /// A boxed array of *HostCnxFailedBadUsernameEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedBadUsernameEvent(Vec<HostCnxFailedBadUsernameEvent>),
    /// A boxed array of *HostCnxFailedBadVersionEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedBadVersionEvent(Vec<HostCnxFailedBadVersionEvent>),
    /// A boxed array of *HostCnxFailedCcagentUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedCcagentUpgradeEvent(Vec<HostCnxFailedCcagentUpgradeEvent>),
    /// A boxed array of *HostCnxFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedEvent(Vec<HostCnxFailedEvent>),
    /// A boxed array of *HostCnxFailedNetworkErrorEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedNetworkErrorEvent(Vec<HostCnxFailedNetworkErrorEvent>),
    /// A boxed array of *HostCnxFailedNoAccessEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedNoAccessEvent(Vec<HostCnxFailedNoAccessEvent>),
    /// A boxed array of *HostCnxFailedNoConnectionEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedNoConnectionEvent(Vec<HostCnxFailedNoConnectionEvent>),
    /// A boxed array of *HostCnxFailedNoLicenseEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedNoLicenseEvent(Vec<HostCnxFailedNoLicenseEvent>),
    /// A boxed array of *HostCnxFailedNotFoundEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedNotFoundEvent(Vec<HostCnxFailedNotFoundEvent>),
    /// A boxed array of *HostCnxFailedTimeoutEvent*. To be used in *Any* placeholders.
    ArrayOfHostCnxFailedTimeoutEvent(Vec<HostCnxFailedTimeoutEvent>),
    /// A boxed array of *HostComplianceCheckedEvent*. To be used in *Any* placeholders.
    ArrayOfHostComplianceCheckedEvent(Vec<HostComplianceCheckedEvent>),
    /// A boxed array of *HostCompliantEvent*. To be used in *Any* placeholders.
    ArrayOfHostCompliantEvent(Vec<HostCompliantEvent>),
    /// A boxed array of *HostConfigAppliedEvent*. To be used in *Any* placeholders.
    ArrayOfHostConfigAppliedEvent(Vec<HostConfigAppliedEvent>),
    /// A boxed array of *HostConnectedEvent*. To be used in *Any* placeholders.
    ArrayOfHostConnectedEvent(Vec<HostConnectedEvent>),
    /// A boxed array of *HostConnectionLostEvent*. To be used in *Any* placeholders.
    ArrayOfHostConnectionLostEvent(Vec<HostConnectionLostEvent>),
    /// A boxed array of *HostDasDisabledEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasDisabledEvent(Vec<HostDasDisabledEvent>),
    /// A boxed array of *HostDasDisablingEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasDisablingEvent(Vec<HostDasDisablingEvent>),
    /// A boxed array of *HostDasEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasEnabledEvent(Vec<HostDasEnabledEvent>),
    /// A boxed array of *HostDasEnablingEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasEnablingEvent(Vec<HostDasEnablingEvent>),
    /// A boxed array of *HostDasErrorEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasErrorEvent(Vec<HostDasErrorEvent>),
    /// A boxed array of *HostDasEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasEvent(Vec<Box<dyn super::traits::HostDasEventTrait>>),
    /// A boxed array of *HostDasOkEvent*. To be used in *Any* placeholders.
    ArrayOfHostDasOkEvent(Vec<HostDasOkEvent>),
    /// A boxed array of *HostDisconnectedEvent*. To be used in *Any* placeholders.
    ArrayOfHostDisconnectedEvent(Vec<HostDisconnectedEvent>),
    /// A boxed array of *HostEnableAdminFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostEnableAdminFailedEvent(Vec<HostEnableAdminFailedEvent>),
    /// A boxed array of *HostEvent*. To be used in *Any* placeholders.
    ArrayOfHostEvent(Vec<Box<dyn super::traits::HostEventTrait>>),
    /// A boxed array of *HostEventArgument*. To be used in *Any* placeholders.
    ArrayOfHostEventArgument(Vec<HostEventArgument>),
    /// A boxed array of *HostExtraNetworksEvent*. To be used in *Any* placeholders.
    ArrayOfHostExtraNetworksEvent(Vec<HostExtraNetworksEvent>),
    /// A boxed array of *HostGetShortNameFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostGetShortNameFailedEvent(Vec<HostGetShortNameFailedEvent>),
    /// A boxed array of *HostInAuditModeEvent*. To be used in *Any* placeholders.
    ArrayOfHostInAuditModeEvent(Vec<HostInAuditModeEvent>),
    /// A boxed array of *HostInventoryFullEvent*. To be used in *Any* placeholders.
    ArrayOfHostInventoryFullEvent(Vec<HostInventoryFullEvent>),
    /// A boxed array of *HostInventoryUnreadableEvent*. To be used in *Any* placeholders.
    ArrayOfHostInventoryUnreadableEvent(Vec<HostInventoryUnreadableEvent>),
    /// A boxed array of *HostIpChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHostIpChangedEvent(Vec<HostIpChangedEvent>),
    /// A boxed array of *HostIpInconsistentEvent*. To be used in *Any* placeholders.
    ArrayOfHostIpInconsistentEvent(Vec<HostIpInconsistentEvent>),
    /// A boxed array of *HostIpToShortNameFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostIpToShortNameFailedEvent(Vec<HostIpToShortNameFailedEvent>),
    /// A boxed array of *HostIsolationIpPingFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostIsolationIpPingFailedEvent(Vec<HostIsolationIpPingFailedEvent>),
    /// A boxed array of *HostLicenseExpiredEvent*. To be used in *Any* placeholders.
    ArrayOfHostLicenseExpiredEvent(Vec<HostLicenseExpiredEvent>),
    /// A boxed array of *HostLocalPortCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfHostLocalPortCreatedEvent(Vec<HostLocalPortCreatedEvent>),
    /// A boxed array of *HostMissingNetworksEvent*. To be used in *Any* placeholders.
    ArrayOfHostMissingNetworksEvent(Vec<HostMissingNetworksEvent>),
    /// A boxed array of *HostMonitoringStateChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHostMonitoringStateChangedEvent(Vec<HostMonitoringStateChangedEvent>),
    /// A boxed array of *HostNoAvailableNetworksEvent*. To be used in *Any* placeholders.
    ArrayOfHostNoAvailableNetworksEvent(Vec<HostNoAvailableNetworksEvent>),
    /// A boxed array of *HostNoHAEnabledPortGroupsEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHostNoHAEnabledPortGroupsEvent")]
    ArrayOfHostNoHaEnabledPortGroupsEvent(Vec<HostNoHaEnabledPortGroupsEvent>),
    /// A boxed array of *HostNoRedundantManagementNetworkEvent*. To be used in *Any* placeholders.
    ArrayOfHostNoRedundantManagementNetworkEvent(Vec<HostNoRedundantManagementNetworkEvent>),
    /// A boxed array of *HostNonCompliantEvent*. To be used in *Any* placeholders.
    ArrayOfHostNonCompliantEvent(Vec<HostNonCompliantEvent>),
    /// A boxed array of *HostNotInClusterEvent*. To be used in *Any* placeholders.
    ArrayOfHostNotInClusterEvent(Vec<HostNotInClusterEvent>),
    /// A boxed array of *HostOvercommittedEvent*. To be used in *Any* placeholders.
    ArrayOfHostOvercommittedEvent(Vec<HostOvercommittedEvent>),
    /// A boxed array of *HostPrimaryAgentNotShortNameEvent*. To be used in *Any* placeholders.
    ArrayOfHostPrimaryAgentNotShortNameEvent(Vec<HostPrimaryAgentNotShortNameEvent>),
    /// A boxed array of *HostProfileAppliedEvent*. To be used in *Any* placeholders.
    ArrayOfHostProfileAppliedEvent(Vec<HostProfileAppliedEvent>),
    /// A boxed array of *HostReconnectionFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostReconnectionFailedEvent(Vec<HostReconnectionFailedEvent>),
    /// A boxed array of *HostRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfHostRemovedEvent(Vec<HostRemovedEvent>),
    /// A boxed array of *HostShortNameInconsistentEvent*. To be used in *Any* placeholders.
    ArrayOfHostShortNameInconsistentEvent(Vec<HostShortNameInconsistentEvent>),
    /// A boxed array of *HostShortNameToIpFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostShortNameToIpFailedEvent(Vec<HostShortNameToIpFailedEvent>),
    /// A boxed array of *HostShutdownEvent*. To be used in *Any* placeholders.
    ArrayOfHostShutdownEvent(Vec<HostShutdownEvent>),
    /// A boxed array of *HostSpecificationChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHostSpecificationChangedEvent(Vec<HostSpecificationChangedEvent>),
    /// A boxed array of *HostSpecificationRequireEvent*. To be used in *Any* placeholders.
    ArrayOfHostSpecificationRequireEvent(Vec<HostSpecificationRequireEvent>),
    /// A boxed array of *HostSpecificationUpdateEvent*. To be used in *Any* placeholders.
    ArrayOfHostSpecificationUpdateEvent(Vec<HostSpecificationUpdateEvent>),
    /// A boxed array of *HostStatusChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHostStatusChangedEvent(Vec<HostStatusChangedEvent>),
    /// A boxed array of *HostSubSpecificationDeleteEvent*. To be used in *Any* placeholders.
    ArrayOfHostSubSpecificationDeleteEvent(Vec<HostSubSpecificationDeleteEvent>),
    /// A boxed array of *HostSubSpecificationUpdateEvent*. To be used in *Any* placeholders.
    ArrayOfHostSubSpecificationUpdateEvent(Vec<HostSubSpecificationUpdateEvent>),
    /// A boxed array of *HostSyncFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostSyncFailedEvent(Vec<HostSyncFailedEvent>),
    /// A boxed array of *HostUpgradeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfHostUpgradeFailedEvent(Vec<HostUpgradeFailedEvent>),
    /// A boxed array of *HostUserWorldSwapNotEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfHostUserWorldSwapNotEnabledEvent(Vec<HostUserWorldSwapNotEnabledEvent>),
    /// A boxed array of *HostVnicConnectedToCustomizedDVPortEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHostVnicConnectedToCustomizedDVPortEvent")]
    ArrayOfHostVnicConnectedToCustomizedDvPortEvent(Vec<HostVnicConnectedToCustomizedDvPortEvent>),
    /// A boxed array of *HostWwnChangedEvent*. To be used in *Any* placeholders.
    ArrayOfHostWwnChangedEvent(Vec<HostWwnChangedEvent>),
    /// A boxed array of *HostWwnConflictEvent*. To be used in *Any* placeholders.
    ArrayOfHostWwnConflictEvent(Vec<HostWwnConflictEvent>),
    /// A boxed array of *IncorrectHostInformationEvent*. To be used in *Any* placeholders.
    ArrayOfIncorrectHostInformationEvent(Vec<IncorrectHostInformationEvent>),
    /// A boxed array of *InfoUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfInfoUpgradeEvent(Vec<InfoUpgradeEvent>),
    /// A boxed array of *InsufficientFailoverResourcesEvent*. To be used in *Any* placeholders.
    ArrayOfInsufficientFailoverResourcesEvent(Vec<InsufficientFailoverResourcesEvent>),
    /// A boxed array of *InvalidEditionEvent*. To be used in *Any* placeholders.
    ArrayOfInvalidEditionEvent(Vec<InvalidEditionEvent>),
    /// A boxed array of *LicenseEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseEvent(Vec<Box<dyn super::traits::LicenseEventTrait>>),
    /// A boxed array of *LicenseExpiredEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseExpiredEvent(Vec<LicenseExpiredEvent>),
    /// A boxed array of *LicenseNonComplianceEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseNonComplianceEvent(Vec<LicenseNonComplianceEvent>),
    /// A boxed array of *LicenseRestrictedEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseRestrictedEvent(Vec<LicenseRestrictedEvent>),
    /// A boxed array of *LicenseServerAvailableEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseServerAvailableEvent(Vec<LicenseServerAvailableEvent>),
    /// A boxed array of *LicenseServerUnavailableEvent*. To be used in *Any* placeholders.
    ArrayOfLicenseServerUnavailableEvent(Vec<LicenseServerUnavailableEvent>),
    /// A boxed array of *LocalDatastoreCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfLocalDatastoreCreatedEvent(Vec<LocalDatastoreCreatedEvent>),
    /// A boxed array of *LocalTSMEnabledEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfLocalTSMEnabledEvent")]
    ArrayOfLocalTsmEnabledEvent(Vec<LocalTsmEnabledEvent>),
    /// A boxed array of *LockerMisconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfLockerMisconfiguredEvent(Vec<LockerMisconfiguredEvent>),
    /// A boxed array of *LockerReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfLockerReconfiguredEvent(Vec<LockerReconfiguredEvent>),
    /// A boxed array of *ManagedEntityEventArgument*. To be used in *Any* placeholders.
    ArrayOfManagedEntityEventArgument(Vec<ManagedEntityEventArgument>),
    /// A boxed array of *MigrationErrorEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationErrorEvent(Vec<MigrationErrorEvent>),
    /// A boxed array of *MigrationEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationEvent(Vec<Box<dyn super::traits::MigrationEventTrait>>),
    /// A boxed array of *MigrationHostErrorEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationHostErrorEvent(Vec<MigrationHostErrorEvent>),
    /// A boxed array of *MigrationHostWarningEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationHostWarningEvent(Vec<MigrationHostWarningEvent>),
    /// A boxed array of *MigrationResourceErrorEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationResourceErrorEvent(Vec<MigrationResourceErrorEvent>),
    /// A boxed array of *MigrationResourceWarningEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationResourceWarningEvent(Vec<MigrationResourceWarningEvent>),
    /// A boxed array of *MigrationWarningEvent*. To be used in *Any* placeholders.
    ArrayOfMigrationWarningEvent(Vec<MigrationWarningEvent>),
    /// A boxed array of *MtuMatchEvent*. To be used in *Any* placeholders.
    ArrayOfMtuMatchEvent(Vec<MtuMatchEvent>),
    /// A boxed array of *MtuMismatchEvent*. To be used in *Any* placeholders.
    ArrayOfMtuMismatchEvent(Vec<MtuMismatchEvent>),
    /// A boxed array of *NASDatastoreCreatedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNASDatastoreCreatedEvent")]
    ArrayOfNasDatastoreCreatedEvent(Vec<NasDatastoreCreatedEvent>),
    /// A boxed array of *NetworkEventArgument*. To be used in *Any* placeholders.
    ArrayOfNetworkEventArgument(Vec<NetworkEventArgument>),
    /// A boxed array of *NetworkRollbackEvent*. To be used in *Any* placeholders.
    ArrayOfNetworkRollbackEvent(Vec<NetworkRollbackEvent>),
    /// A boxed array of *NoAccessUserEvent*. To be used in *Any* placeholders.
    ArrayOfNoAccessUserEvent(Vec<NoAccessUserEvent>),
    /// A boxed array of *NoDatastoresConfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfNoDatastoresConfiguredEvent(Vec<NoDatastoresConfiguredEvent>),
    /// A boxed array of *NoLicenseEvent*. To be used in *Any* placeholders.
    ArrayOfNoLicenseEvent(Vec<NoLicenseEvent>),
    /// A boxed array of *NoMaintenanceModeDrsRecommendationForVM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNoMaintenanceModeDrsRecommendationForVM")]
    ArrayOfNoMaintenanceModeDrsRecommendationForVm(Vec<NoMaintenanceModeDrsRecommendationForVm>),
    /// A boxed array of *NonVIWorkloadDetectedOnDatastoreEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNonVIWorkloadDetectedOnDatastoreEvent")]
    ArrayOfNonViWorkloadDetectedOnDatastoreEvent(Vec<NonViWorkloadDetectedOnDatastoreEvent>),
    /// A boxed array of *NotEnoughResourcesToStartVmEvent*. To be used in *Any* placeholders.
    ArrayOfNotEnoughResourcesToStartVmEvent(Vec<NotEnoughResourcesToStartVmEvent>),
    /// A boxed array of *OutOfSyncDvsHost*. To be used in *Any* placeholders.
    ArrayOfOutOfSyncDvsHost(Vec<OutOfSyncDvsHost>),
    /// A boxed array of *PermissionAddedEvent*. To be used in *Any* placeholders.
    ArrayOfPermissionAddedEvent(Vec<PermissionAddedEvent>),
    /// A boxed array of *PermissionEvent*. To be used in *Any* placeholders.
    ArrayOfPermissionEvent(Vec<Box<dyn super::traits::PermissionEventTrait>>),
    /// A boxed array of *PermissionRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfPermissionRemovedEvent(Vec<PermissionRemovedEvent>),
    /// A boxed array of *PermissionUpdatedEvent*. To be used in *Any* placeholders.
    ArrayOfPermissionUpdatedEvent(Vec<PermissionUpdatedEvent>),
    /// A boxed array of *ProfileAssociatedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileAssociatedEvent(Vec<ProfileAssociatedEvent>),
    /// A boxed array of *ProfileChangedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileChangedEvent(Vec<ProfileChangedEvent>),
    /// A boxed array of *ProfileCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileCreatedEvent(Vec<ProfileCreatedEvent>),
    /// A boxed array of *ProfileDissociatedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileDissociatedEvent(Vec<ProfileDissociatedEvent>),
    /// A boxed array of *ProfileEvent*. To be used in *Any* placeholders.
    ArrayOfProfileEvent(Vec<Box<dyn super::traits::ProfileEventTrait>>),
    /// A boxed array of *ProfileEventArgument*. To be used in *Any* placeholders.
    ArrayOfProfileEventArgument(Vec<ProfileEventArgument>),
    /// A boxed array of *ProfileReferenceHostChangedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileReferenceHostChangedEvent(Vec<ProfileReferenceHostChangedEvent>),
    /// A boxed array of *ProfileRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfProfileRemovedEvent(Vec<ProfileRemovedEvent>),
    /// A boxed array of *RecoveryEvent*. To be used in *Any* placeholders.
    ArrayOfRecoveryEvent(Vec<RecoveryEvent>),
    /// A boxed array of *RemoteTSMEnabledEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRemoteTSMEnabledEvent")]
    ArrayOfRemoteTsmEnabledEvent(Vec<RemoteTsmEnabledEvent>),
    /// A boxed array of *ResourcePoolCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfResourcePoolCreatedEvent(Vec<ResourcePoolCreatedEvent>),
    /// A boxed array of *ResourcePoolDestroyedEvent*. To be used in *Any* placeholders.
    ArrayOfResourcePoolDestroyedEvent(Vec<ResourcePoolDestroyedEvent>),
    /// A boxed array of *ResourcePoolEvent*. To be used in *Any* placeholders.
    ArrayOfResourcePoolEvent(Vec<Box<dyn super::traits::ResourcePoolEventTrait>>),
    /// A boxed array of *ResourcePoolEventArgument*. To be used in *Any* placeholders.
    ArrayOfResourcePoolEventArgument(Vec<ResourcePoolEventArgument>),
    /// A boxed array of *ResourcePoolMovedEvent*. To be used in *Any* placeholders.
    ArrayOfResourcePoolMovedEvent(Vec<ResourcePoolMovedEvent>),
    /// A boxed array of *ResourcePoolReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfResourcePoolReconfiguredEvent(Vec<ResourcePoolReconfiguredEvent>),
    /// A boxed array of *ResourceViolatedEvent*. To be used in *Any* placeholders.
    ArrayOfResourceViolatedEvent(Vec<ResourceViolatedEvent>),
    /// A boxed array of *RoleAddedEvent*. To be used in *Any* placeholders.
    ArrayOfRoleAddedEvent(Vec<RoleAddedEvent>),
    /// A boxed array of *RoleEvent*. To be used in *Any* placeholders.
    ArrayOfRoleEvent(Vec<Box<dyn super::traits::RoleEventTrait>>),
    /// A boxed array of *RoleEventArgument*. To be used in *Any* placeholders.
    ArrayOfRoleEventArgument(Vec<RoleEventArgument>),
    /// A boxed array of *RoleRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfRoleRemovedEvent(Vec<RoleRemovedEvent>),
    /// A boxed array of *RoleUpdatedEvent*. To be used in *Any* placeholders.
    ArrayOfRoleUpdatedEvent(Vec<RoleUpdatedEvent>),
    /// A boxed array of *RollbackEvent*. To be used in *Any* placeholders.
    ArrayOfRollbackEvent(Vec<RollbackEvent>),
    /// A boxed array of *ScheduledTaskCompletedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskCompletedEvent(Vec<ScheduledTaskCompletedEvent>),
    /// A boxed array of *ScheduledTaskCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskCreatedEvent(Vec<ScheduledTaskCreatedEvent>),
    /// A boxed array of *ScheduledTaskEmailCompletedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskEmailCompletedEvent(Vec<ScheduledTaskEmailCompletedEvent>),
    /// A boxed array of *ScheduledTaskEmailFailedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskEmailFailedEvent(Vec<ScheduledTaskEmailFailedEvent>),
    /// A boxed array of *ScheduledTaskEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskEvent(Vec<Box<dyn super::traits::ScheduledTaskEventTrait>>),
    /// A boxed array of *ScheduledTaskEventArgument*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskEventArgument(Vec<ScheduledTaskEventArgument>),
    /// A boxed array of *ScheduledTaskFailedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskFailedEvent(Vec<ScheduledTaskFailedEvent>),
    /// A boxed array of *ScheduledTaskReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskReconfiguredEvent(Vec<ScheduledTaskReconfiguredEvent>),
    /// A boxed array of *ScheduledTaskRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskRemovedEvent(Vec<ScheduledTaskRemovedEvent>),
    /// A boxed array of *ScheduledTaskStartedEvent*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskStartedEvent(Vec<ScheduledTaskStartedEvent>),
    /// A boxed array of *ServerLicenseExpiredEvent*. To be used in *Any* placeholders.
    ArrayOfServerLicenseExpiredEvent(Vec<ServerLicenseExpiredEvent>),
    /// A boxed array of *ServerStartedSessionEvent*. To be used in *Any* placeholders.
    ArrayOfServerStartedSessionEvent(Vec<ServerStartedSessionEvent>),
    /// A boxed array of *SessionEvent*. To be used in *Any* placeholders.
    ArrayOfSessionEvent(Vec<Box<dyn super::traits::SessionEventTrait>>),
    /// A boxed array of *SessionTerminatedEvent*. To be used in *Any* placeholders.
    ArrayOfSessionTerminatedEvent(Vec<SessionTerminatedEvent>),
    /// A boxed array of *TaskEvent*. To be used in *Any* placeholders.
    ArrayOfTaskEvent(Vec<Box<dyn super::traits::TaskEventTrait>>),
    /// A boxed array of *TaskTimeoutEvent*. To be used in *Any* placeholders.
    ArrayOfTaskTimeoutEvent(Vec<TaskTimeoutEvent>),
    /// A boxed array of *TeamingMatchEvent*. To be used in *Any* placeholders.
    ArrayOfTeamingMatchEvent(Vec<TeamingMatchEvent>),
    /// A boxed array of *TeamingMisMatchEvent*. To be used in *Any* placeholders.
    ArrayOfTeamingMisMatchEvent(Vec<TeamingMisMatchEvent>),
    /// A boxed array of *TemplateBeingUpgradedEvent*. To be used in *Any* placeholders.
    ArrayOfTemplateBeingUpgradedEvent(Vec<TemplateBeingUpgradedEvent>),
    /// A boxed array of *TemplateUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfTemplateUpgradeEvent(Vec<Box<dyn super::traits::TemplateUpgradeEventTrait>>),
    /// A boxed array of *TemplateUpgradeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfTemplateUpgradeFailedEvent(Vec<TemplateUpgradeFailedEvent>),
    /// A boxed array of *TemplateUpgradedEvent*. To be used in *Any* placeholders.
    ArrayOfTemplateUpgradedEvent(Vec<TemplateUpgradedEvent>),
    /// A boxed array of *TimedOutHostOperationEvent*. To be used in *Any* placeholders.
    ArrayOfTimedOutHostOperationEvent(Vec<TimedOutHostOperationEvent>),
    /// A boxed array of *UnlicensedVirtualMachinesEvent*. To be used in *Any* placeholders.
    ArrayOfUnlicensedVirtualMachinesEvent(Vec<UnlicensedVirtualMachinesEvent>),
    /// A boxed array of *UnlicensedVirtualMachinesFoundEvent*. To be used in *Any* placeholders.
    ArrayOfUnlicensedVirtualMachinesFoundEvent(Vec<UnlicensedVirtualMachinesFoundEvent>),
    /// A boxed array of *UpdatedAgentBeingRestartedEvent*. To be used in *Any* placeholders.
    ArrayOfUpdatedAgentBeingRestartedEvent(Vec<UpdatedAgentBeingRestartedEvent>),
    /// A boxed array of *UpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfUpgradeEvent(Vec<Box<dyn super::traits::UpgradeEventTrait>>),
    /// A boxed array of *UplinkPortMtuNotSupportEvent*. To be used in *Any* placeholders.
    ArrayOfUplinkPortMtuNotSupportEvent(Vec<UplinkPortMtuNotSupportEvent>),
    /// A boxed array of *UplinkPortMtuSupportEvent*. To be used in *Any* placeholders.
    ArrayOfUplinkPortMtuSupportEvent(Vec<UplinkPortMtuSupportEvent>),
    /// A boxed array of *UplinkPortVlanTrunkedEvent*. To be used in *Any* placeholders.
    ArrayOfUplinkPortVlanTrunkedEvent(Vec<UplinkPortVlanTrunkedEvent>),
    /// A boxed array of *UplinkPortVlanUntrunkedEvent*. To be used in *Any* placeholders.
    ArrayOfUplinkPortVlanUntrunkedEvent(Vec<UplinkPortVlanUntrunkedEvent>),
    /// A boxed array of *UserAssignedToGroup*. To be used in *Any* placeholders.
    ArrayOfUserAssignedToGroup(Vec<UserAssignedToGroup>),
    /// A boxed array of *UserLoginSessionEvent*. To be used in *Any* placeholders.
    ArrayOfUserLoginSessionEvent(Vec<UserLoginSessionEvent>),
    /// A boxed array of *UserLogoutSessionEvent*. To be used in *Any* placeholders.
    ArrayOfUserLogoutSessionEvent(Vec<UserLogoutSessionEvent>),
    /// A boxed array of *UserPasswordChanged*. To be used in *Any* placeholders.
    ArrayOfUserPasswordChanged(Vec<UserPasswordChanged>),
    /// A boxed array of *UserUnassignedFromGroup*. To be used in *Any* placeholders.
    ArrayOfUserUnassignedFromGroup(Vec<UserUnassignedFromGroup>),
    /// A boxed array of *UserUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfUserUpgradeEvent(Vec<UserUpgradeEvent>),
    /// A boxed array of *VMFSDatastoreCreatedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMFSDatastoreCreatedEvent")]
    ArrayOfVmfsDatastoreCreatedEvent(Vec<VmfsDatastoreCreatedEvent>),
    /// A boxed array of *VMFSDatastoreExpandedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMFSDatastoreExpandedEvent")]
    ArrayOfVmfsDatastoreExpandedEvent(Vec<VmfsDatastoreExpandedEvent>),
    /// A boxed array of *VMFSDatastoreExtendedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMFSDatastoreExtendedEvent")]
    ArrayOfVmfsDatastoreExtendedEvent(Vec<VmfsDatastoreExtendedEvent>),
    /// A boxed array of *VMotionLicenseExpiredEvent*. To be used in *Any* placeholders.
    ArrayOfVMotionLicenseExpiredEvent(Vec<VMotionLicenseExpiredEvent>),
    /// A boxed array of *VcAgentUninstallFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVcAgentUninstallFailedEvent(Vec<VcAgentUninstallFailedEvent>),
    /// A boxed array of *VcAgentUninstalledEvent*. To be used in *Any* placeholders.
    ArrayOfVcAgentUninstalledEvent(Vec<VcAgentUninstalledEvent>),
    /// A boxed array of *VcAgentUpgradeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVcAgentUpgradeFailedEvent(Vec<VcAgentUpgradeFailedEvent>),
    /// A boxed array of *VcAgentUpgradedEvent*. To be used in *Any* placeholders.
    ArrayOfVcAgentUpgradedEvent(Vec<VcAgentUpgradedEvent>),
    /// A boxed array of *VimAccountPasswordChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVimAccountPasswordChangedEvent(Vec<VimAccountPasswordChangedEvent>),
    /// A boxed array of *VmAcquiredMksTicketEvent*. To be used in *Any* placeholders.
    ArrayOfVmAcquiredMksTicketEvent(Vec<VmAcquiredMksTicketEvent>),
    /// A boxed array of *VmAcquiredTicketEvent*. To be used in *Any* placeholders.
    ArrayOfVmAcquiredTicketEvent(Vec<VmAcquiredTicketEvent>),
    /// A boxed array of *VmAutoRenameEvent*. To be used in *Any* placeholders.
    ArrayOfVmAutoRenameEvent(Vec<VmAutoRenameEvent>),
    /// A boxed array of *VmBeingClonedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingClonedEvent(Vec<VmBeingClonedEvent>),
    /// A boxed array of *VmBeingClonedNoFolderEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingClonedNoFolderEvent(Vec<VmBeingClonedNoFolderEvent>),
    /// A boxed array of *VmBeingCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingCreatedEvent(Vec<VmBeingCreatedEvent>),
    /// A boxed array of *VmBeingDeployedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingDeployedEvent(Vec<VmBeingDeployedEvent>),
    /// A boxed array of *VmBeingHotMigratedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingHotMigratedEvent(Vec<VmBeingHotMigratedEvent>),
    /// A boxed array of *VmBeingMigratedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingMigratedEvent(Vec<VmBeingMigratedEvent>),
    /// A boxed array of *VmBeingRelocatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmBeingRelocatedEvent(Vec<VmBeingRelocatedEvent>),
    /// A boxed array of *VmCloneEvent*. To be used in *Any* placeholders.
    ArrayOfVmCloneEvent(Vec<Box<dyn super::traits::VmCloneEventTrait>>),
    /// A boxed array of *VmCloneFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmCloneFailedEvent(Vec<VmCloneFailedEvent>),
    /// A boxed array of *VmClonedEvent*. To be used in *Any* placeholders.
    ArrayOfVmClonedEvent(Vec<VmClonedEvent>),
    /// A boxed array of *VmConfigMissingEvent*. To be used in *Any* placeholders.
    ArrayOfVmConfigMissingEvent(Vec<VmConfigMissingEvent>),
    /// A boxed array of *VmConnectedEvent*. To be used in *Any* placeholders.
    ArrayOfVmConnectedEvent(Vec<VmConnectedEvent>),
    /// A boxed array of *VmCreatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmCreatedEvent(Vec<VmCreatedEvent>),
    /// A boxed array of *VmDasBeingResetEvent*. To be used in *Any* placeholders.
    ArrayOfVmDasBeingResetEvent(Vec<Box<dyn super::traits::VmDasBeingResetEventTrait>>),
    /// A boxed array of *VmDasBeingResetWithScreenshotEvent*. To be used in *Any* placeholders.
    ArrayOfVmDasBeingResetWithScreenshotEvent(Vec<VmDasBeingResetWithScreenshotEvent>),
    /// A boxed array of *VmDasResetFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmDasResetFailedEvent(Vec<VmDasResetFailedEvent>),
    /// A boxed array of *VmDasUpdateErrorEvent*. To be used in *Any* placeholders.
    ArrayOfVmDasUpdateErrorEvent(Vec<VmDasUpdateErrorEvent>),
    /// A boxed array of *VmDasUpdateOkEvent*. To be used in *Any* placeholders.
    ArrayOfVmDasUpdateOkEvent(Vec<VmDasUpdateOkEvent>),
    /// A boxed array of *VmDateRolledBackEvent*. To be used in *Any* placeholders.
    ArrayOfVmDateRolledBackEvent(Vec<VmDateRolledBackEvent>),
    /// A boxed array of *VmDeployFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmDeployFailedEvent(Vec<VmDeployFailedEvent>),
    /// A boxed array of *VmDeployedEvent*. To be used in *Any* placeholders.
    ArrayOfVmDeployedEvent(Vec<VmDeployedEvent>),
    /// A boxed array of *VmDisconnectedEvent*. To be used in *Any* placeholders.
    ArrayOfVmDisconnectedEvent(Vec<VmDisconnectedEvent>),
    /// A boxed array of *VmDiscoveredEvent*. To be used in *Any* placeholders.
    ArrayOfVmDiscoveredEvent(Vec<VmDiscoveredEvent>),
    /// A boxed array of *VmDiskFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmDiskFailedEvent(Vec<VmDiskFailedEvent>),
    /// A boxed array of *VmEmigratingEvent*. To be used in *Any* placeholders.
    ArrayOfVmEmigratingEvent(Vec<VmEmigratingEvent>),
    /// A boxed array of *VmEndRecordingEvent*. To be used in *Any* placeholders.
    ArrayOfVmEndRecordingEvent(Vec<VmEndRecordingEvent>),
    /// A boxed array of *VmEndReplayingEvent*. To be used in *Any* placeholders.
    ArrayOfVmEndReplayingEvent(Vec<VmEndReplayingEvent>),
    /// A boxed array of *VmEvent*. To be used in *Any* placeholders.
    ArrayOfVmEvent(Vec<Box<dyn super::traits::VmEventTrait>>),
    /// A boxed array of *VmEventArgument*. To be used in *Any* placeholders.
    ArrayOfVmEventArgument(Vec<VmEventArgument>),
    /// A boxed array of *VmFailedMigrateEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedMigrateEvent(Vec<VmFailedMigrateEvent>),
    /// A boxed array of *VmFailedRelayoutEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedRelayoutEvent(Vec<VmFailedRelayoutEvent>),
    /// A boxed array of *VmFailedRelayoutOnVmfs2DatastoreEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedRelayoutOnVmfs2DatastoreEvent(Vec<VmFailedRelayoutOnVmfs2DatastoreEvent>),
    /// A boxed array of *VmFailedStartingSecondaryEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedStartingSecondaryEvent(Vec<VmFailedStartingSecondaryEvent>),
    /// A boxed array of *VmFailedToPowerOffEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToPowerOffEvent(Vec<VmFailedToPowerOffEvent>),
    /// A boxed array of *VmFailedToPowerOnEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToPowerOnEvent(Vec<VmFailedToPowerOnEvent>),
    /// A boxed array of *VmFailedToRebootGuestEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToRebootGuestEvent(Vec<VmFailedToRebootGuestEvent>),
    /// A boxed array of *VmFailedToResetEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToResetEvent(Vec<VmFailedToResetEvent>),
    /// A boxed array of *VmFailedToShutdownGuestEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToShutdownGuestEvent(Vec<VmFailedToShutdownGuestEvent>),
    /// A boxed array of *VmFailedToStandbyGuestEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToStandbyGuestEvent(Vec<VmFailedToStandbyGuestEvent>),
    /// A boxed array of *VmFailedToSuspendEvent*. To be used in *Any* placeholders.
    ArrayOfVmFailedToSuspendEvent(Vec<VmFailedToSuspendEvent>),
    /// A boxed array of *VmFailedUpdatingSecondaryConfig*. To be used in *Any* placeholders.
    ArrayOfVmFailedUpdatingSecondaryConfig(Vec<VmFailedUpdatingSecondaryConfig>),
    /// A boxed array of *VmFailoverFailed*. To be used in *Any* placeholders.
    ArrayOfVmFailoverFailed(Vec<VmFailoverFailed>),
    /// A boxed array of *VmFaultToleranceStateChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceStateChangedEvent(Vec<VmFaultToleranceStateChangedEvent>),
    /// A boxed array of *VmFaultToleranceTurnedOffEvent*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceTurnedOffEvent(Vec<VmFaultToleranceTurnedOffEvent>),
    /// A boxed array of *VmFaultToleranceVmTerminatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceVmTerminatedEvent(Vec<VmFaultToleranceVmTerminatedEvent>),
    /// A boxed array of *VmGuestOSCrashedEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVmGuestOSCrashedEvent")]
    ArrayOfVmGuestOsCrashedEvent(Vec<VmGuestOsCrashedEvent>),
    /// A boxed array of *VmGuestRebootEvent*. To be used in *Any* placeholders.
    ArrayOfVmGuestRebootEvent(Vec<VmGuestRebootEvent>),
    /// A boxed array of *VmGuestShutdownEvent*. To be used in *Any* placeholders.
    ArrayOfVmGuestShutdownEvent(Vec<VmGuestShutdownEvent>),
    /// A boxed array of *VmGuestStandbyEvent*. To be used in *Any* placeholders.
    ArrayOfVmGuestStandbyEvent(Vec<VmGuestStandbyEvent>),
    /// A boxed array of *VmHealthMonitoringStateChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmHealthMonitoringStateChangedEvent(Vec<VmHealthMonitoringStateChangedEvent>),
    /// A boxed array of *VmInstanceUuidAssignedEvent*. To be used in *Any* placeholders.
    ArrayOfVmInstanceUuidAssignedEvent(Vec<VmInstanceUuidAssignedEvent>),
    /// A boxed array of *VmInstanceUuidChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmInstanceUuidChangedEvent(Vec<VmInstanceUuidChangedEvent>),
    /// A boxed array of *VmInstanceUuidConflictEvent*. To be used in *Any* placeholders.
    ArrayOfVmInstanceUuidConflictEvent(Vec<VmInstanceUuidConflictEvent>),
    /// A boxed array of *VmMacAssignedEvent*. To be used in *Any* placeholders.
    ArrayOfVmMacAssignedEvent(Vec<VmMacAssignedEvent>),
    /// A boxed array of *VmMacChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmMacChangedEvent(Vec<VmMacChangedEvent>),
    /// A boxed array of *VmMacConflictEvent*. To be used in *Any* placeholders.
    ArrayOfVmMacConflictEvent(Vec<VmMacConflictEvent>),
    /// A boxed array of *VmMaxFTRestartCountReached*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVmMaxFTRestartCountReached")]
    ArrayOfVmMaxFtRestartCountReached(Vec<VmMaxFtRestartCountReached>),
    /// A boxed array of *VmMaxRestartCountReached*. To be used in *Any* placeholders.
    ArrayOfVmMaxRestartCountReached(Vec<VmMaxRestartCountReached>),
    /// A boxed array of *VmMessageErrorEvent*. To be used in *Any* placeholders.
    ArrayOfVmMessageErrorEvent(Vec<VmMessageErrorEvent>),
    /// A boxed array of *VmMessageEvent*. To be used in *Any* placeholders.
    ArrayOfVmMessageEvent(Vec<VmMessageEvent>),
    /// A boxed array of *VmMessageWarningEvent*. To be used in *Any* placeholders.
    ArrayOfVmMessageWarningEvent(Vec<VmMessageWarningEvent>),
    /// A boxed array of *VmMigratedEvent*. To be used in *Any* placeholders.
    ArrayOfVmMigratedEvent(Vec<Box<dyn super::traits::VmMigratedEventTrait>>),
    /// A boxed array of *VmNoCompatibleHostForSecondaryEvent*. To be used in *Any* placeholders.
    ArrayOfVmNoCompatibleHostForSecondaryEvent(Vec<VmNoCompatibleHostForSecondaryEvent>),
    /// A boxed array of *VmNoNetworkAccessEvent*. To be used in *Any* placeholders.
    ArrayOfVmNoNetworkAccessEvent(Vec<VmNoNetworkAccessEvent>),
    /// A boxed array of *VmOrphanedEvent*. To be used in *Any* placeholders.
    ArrayOfVmOrphanedEvent(Vec<VmOrphanedEvent>),
    /// A boxed array of *VmPowerOffOnIsolationEvent*. To be used in *Any* placeholders.
    ArrayOfVmPowerOffOnIsolationEvent(Vec<VmPowerOffOnIsolationEvent>),
    /// A boxed array of *VmPoweredOffEvent*. To be used in *Any* placeholders.
    ArrayOfVmPoweredOffEvent(Vec<Box<dyn super::traits::VmPoweredOffEventTrait>>),
    /// A boxed array of *VmPoweredOnEvent*. To be used in *Any* placeholders.
    ArrayOfVmPoweredOnEvent(Vec<Box<dyn super::traits::VmPoweredOnEventTrait>>),
    /// A boxed array of *VmPoweringOnWithCustomizedDVPortEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVmPoweringOnWithCustomizedDVPortEvent")]
    ArrayOfVmPoweringOnWithCustomizedDvPortEvent(Vec<VmPoweringOnWithCustomizedDvPortEvent>),
    /// A boxed array of *VmPrimaryFailoverEvent*. To be used in *Any* placeholders.
    ArrayOfVmPrimaryFailoverEvent(Vec<VmPrimaryFailoverEvent>),
    /// A boxed array of *VmReconfiguredEvent*. To be used in *Any* placeholders.
    ArrayOfVmReconfiguredEvent(Vec<VmReconfiguredEvent>),
    /// A boxed array of *VmRegisteredEvent*. To be used in *Any* placeholders.
    ArrayOfVmRegisteredEvent(Vec<VmRegisteredEvent>),
    /// A boxed array of *VmRelayoutSuccessfulEvent*. To be used in *Any* placeholders.
    ArrayOfVmRelayoutSuccessfulEvent(Vec<VmRelayoutSuccessfulEvent>),
    /// A boxed array of *VmRelayoutUpToDateEvent*. To be used in *Any* placeholders.
    ArrayOfVmRelayoutUpToDateEvent(Vec<VmRelayoutUpToDateEvent>),
    /// A boxed array of *VmReloadFromPathEvent*. To be used in *Any* placeholders.
    ArrayOfVmReloadFromPathEvent(Vec<VmReloadFromPathEvent>),
    /// A boxed array of *VmReloadFromPathFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmReloadFromPathFailedEvent(Vec<VmReloadFromPathFailedEvent>),
    /// A boxed array of *VmRelocateFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRelocateFailedEvent(Vec<VmRelocateFailedEvent>),
    /// A boxed array of *VmRelocateSpecEvent*. To be used in *Any* placeholders.
    ArrayOfVmRelocateSpecEvent(Vec<Box<dyn super::traits::VmRelocateSpecEventTrait>>),
    /// A boxed array of *VmRelocatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRelocatedEvent(Vec<VmRelocatedEvent>),
    /// A boxed array of *VmRemoteConsoleConnectedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRemoteConsoleConnectedEvent(Vec<VmRemoteConsoleConnectedEvent>),
    /// A boxed array of *VmRemoteConsoleDisconnectedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRemoteConsoleDisconnectedEvent(Vec<VmRemoteConsoleDisconnectedEvent>),
    /// A boxed array of *VmRemovedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRemovedEvent(Vec<VmRemovedEvent>),
    /// A boxed array of *VmRenamedEvent*. To be used in *Any* placeholders.
    ArrayOfVmRenamedEvent(Vec<VmRenamedEvent>),
    /// A boxed array of *VmRequirementsExceedCurrentEVCModeEvent*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVmRequirementsExceedCurrentEVCModeEvent")]
    ArrayOfVmRequirementsExceedCurrentEvcModeEvent(Vec<VmRequirementsExceedCurrentEvcModeEvent>),
    /// A boxed array of *VmResettingEvent*. To be used in *Any* placeholders.
    ArrayOfVmResettingEvent(Vec<VmResettingEvent>),
    /// A boxed array of *VmResourcePoolMovedEvent*. To be used in *Any* placeholders.
    ArrayOfVmResourcePoolMovedEvent(Vec<VmResourcePoolMovedEvent>),
    /// A boxed array of *VmResourceReallocatedEvent*. To be used in *Any* placeholders.
    ArrayOfVmResourceReallocatedEvent(Vec<VmResourceReallocatedEvent>),
    /// A boxed array of *VmRestartedOnAlternateHostEvent*. To be used in *Any* placeholders.
    ArrayOfVmRestartedOnAlternateHostEvent(Vec<VmRestartedOnAlternateHostEvent>),
    /// A boxed array of *VmResumingEvent*. To be used in *Any* placeholders.
    ArrayOfVmResumingEvent(Vec<VmResumingEvent>),
    /// A boxed array of *VmSecondaryAddedEvent*. To be used in *Any* placeholders.
    ArrayOfVmSecondaryAddedEvent(Vec<VmSecondaryAddedEvent>),
    /// A boxed array of *VmSecondaryDisabledBySystemEvent*. To be used in *Any* placeholders.
    ArrayOfVmSecondaryDisabledBySystemEvent(Vec<VmSecondaryDisabledBySystemEvent>),
    /// A boxed array of *VmSecondaryDisabledEvent*. To be used in *Any* placeholders.
    ArrayOfVmSecondaryDisabledEvent(Vec<VmSecondaryDisabledEvent>),
    /// A boxed array of *VmSecondaryEnabledEvent*. To be used in *Any* placeholders.
    ArrayOfVmSecondaryEnabledEvent(Vec<VmSecondaryEnabledEvent>),
    /// A boxed array of *VmSecondaryStartedEvent*. To be used in *Any* placeholders.
    ArrayOfVmSecondaryStartedEvent(Vec<VmSecondaryStartedEvent>),
    /// A boxed array of *VmShutdownOnIsolationEvent*. To be used in *Any* placeholders.
    ArrayOfVmShutdownOnIsolationEvent(Vec<VmShutdownOnIsolationEvent>),
    /// A boxed array of *VmStartRecordingEvent*. To be used in *Any* placeholders.
    ArrayOfVmStartRecordingEvent(Vec<VmStartRecordingEvent>),
    /// A boxed array of *VmStartReplayingEvent*. To be used in *Any* placeholders.
    ArrayOfVmStartReplayingEvent(Vec<VmStartReplayingEvent>),
    /// A boxed array of *VmStartingEvent*. To be used in *Any* placeholders.
    ArrayOfVmStartingEvent(Vec<Box<dyn super::traits::VmStartingEventTrait>>),
    /// A boxed array of *VmStartingSecondaryEvent*. To be used in *Any* placeholders.
    ArrayOfVmStartingSecondaryEvent(Vec<VmStartingSecondaryEvent>),
    /// A boxed array of *VmStaticMacConflictEvent*. To be used in *Any* placeholders.
    ArrayOfVmStaticMacConflictEvent(Vec<VmStaticMacConflictEvent>),
    /// A boxed array of *VmStoppingEvent*. To be used in *Any* placeholders.
    ArrayOfVmStoppingEvent(Vec<VmStoppingEvent>),
    /// A boxed array of *VmSuspendedEvent*. To be used in *Any* placeholders.
    ArrayOfVmSuspendedEvent(Vec<VmSuspendedEvent>),
    /// A boxed array of *VmSuspendingEvent*. To be used in *Any* placeholders.
    ArrayOfVmSuspendingEvent(Vec<VmSuspendingEvent>),
    /// A boxed array of *VmTimedoutStartingSecondaryEvent*. To be used in *Any* placeholders.
    ArrayOfVmTimedoutStartingSecondaryEvent(Vec<VmTimedoutStartingSecondaryEvent>),
    /// A boxed array of *VmUnsupportedStartingEvent*. To be used in *Any* placeholders.
    ArrayOfVmUnsupportedStartingEvent(Vec<VmUnsupportedStartingEvent>),
    /// A boxed array of *VmUpgradeCompleteEvent*. To be used in *Any* placeholders.
    ArrayOfVmUpgradeCompleteEvent(Vec<VmUpgradeCompleteEvent>),
    /// A boxed array of *VmUpgradeFailedEvent*. To be used in *Any* placeholders.
    ArrayOfVmUpgradeFailedEvent(Vec<VmUpgradeFailedEvent>),
    /// A boxed array of *VmUpgradingEvent*. To be used in *Any* placeholders.
    ArrayOfVmUpgradingEvent(Vec<VmUpgradingEvent>),
    /// A boxed array of *VmUuidAssignedEvent*. To be used in *Any* placeholders.
    ArrayOfVmUuidAssignedEvent(Vec<VmUuidAssignedEvent>),
    /// A boxed array of *VmUuidChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmUuidChangedEvent(Vec<VmUuidChangedEvent>),
    /// A boxed array of *VmUuidConflictEvent*. To be used in *Any* placeholders.
    ArrayOfVmUuidConflictEvent(Vec<VmUuidConflictEvent>),
    /// A boxed array of *VmVnicPoolReservationViolationClearEvent*. To be used in *Any* placeholders.
    ArrayOfVmVnicPoolReservationViolationClearEvent(Vec<VmVnicPoolReservationViolationClearEvent>),
    /// A boxed array of *VmVnicPoolReservationViolationRaiseEvent*. To be used in *Any* placeholders.
    ArrayOfVmVnicPoolReservationViolationRaiseEvent(Vec<VmVnicPoolReservationViolationRaiseEvent>),
    /// A boxed array of *VmWwnAssignedEvent*. To be used in *Any* placeholders.
    ArrayOfVmWwnAssignedEvent(Vec<VmWwnAssignedEvent>),
    /// A boxed array of *VmWwnChangedEvent*. To be used in *Any* placeholders.
    ArrayOfVmWwnChangedEvent(Vec<VmWwnChangedEvent>),
    /// A boxed array of *VmWwnConflictEvent*. To be used in *Any* placeholders.
    ArrayOfVmWwnConflictEvent(Vec<VmWwnConflictEvent>),
    /// A boxed array of *VnicPortArgument*. To be used in *Any* placeholders.
    ArrayOfVnicPortArgument(Vec<VnicPortArgument>),
    /// A boxed array of *WarningUpgradeEvent*. To be used in *Any* placeholders.
    ArrayOfWarningUpgradeEvent(Vec<WarningUpgradeEvent>),
    /// A boxed array of *IScsiBootFailureEvent*. To be used in *Any* placeholders.
    ArrayOfIScsiBootFailureEvent(Vec<IScsiBootFailureEvent>),
    /// A boxed array of *ExtExtendedProductInfo*. To be used in *Any* placeholders.
    ArrayOfExtExtendedProductInfo(Vec<ExtExtendedProductInfo>),
    /// A boxed array of *ManagedByInfo*. To be used in *Any* placeholders.
    ArrayOfManagedByInfo(Vec<ManagedByInfo>),
    /// A boxed array of *ExtManagedEntityInfo*. To be used in *Any* placeholders.
    ArrayOfExtManagedEntityInfo(Vec<ExtManagedEntityInfo>),
    /// A boxed array of *ExtSolutionManagerInfo*. To be used in *Any* placeholders.
    ArrayOfExtSolutionManagerInfo(Vec<ExtSolutionManagerInfo>),
    /// A boxed array of *ExtSolutionManagerInfoTabInfo*. To be used in *Any* placeholders.
    ArrayOfExtSolutionManagerInfoTabInfo(Vec<ExtSolutionManagerInfoTabInfo>),
    /// A boxed array of *ActiveDirectoryFault*. To be used in *Any* placeholders.
    ArrayOfActiveDirectoryFault(Vec<Box<dyn super::traits::ActiveDirectoryFaultTrait>>),
    /// A boxed array of *ActiveVMsBlockingEVC*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfActiveVMsBlockingEVC")]
    ArrayOfActiveVMsBlockingEvc(Vec<ActiveVMsBlockingEvc>),
    /// A boxed array of *AdminDisabled*. To be used in *Any* placeholders.
    ArrayOfAdminDisabled(Vec<AdminDisabled>),
    /// A boxed array of *AdminNotDisabled*. To be used in *Any* placeholders.
    ArrayOfAdminNotDisabled(Vec<AdminNotDisabled>),
    /// A boxed array of *AffinityConfigured*. To be used in *Any* placeholders.
    ArrayOfAffinityConfigured(Vec<AffinityConfigured>),
    /// A boxed array of *AgentInstallFailed*. To be used in *Any* placeholders.
    ArrayOfAgentInstallFailed(Vec<AgentInstallFailed>),
    /// A boxed array of *AlreadyBeingManaged*. To be used in *Any* placeholders.
    ArrayOfAlreadyBeingManaged(Vec<AlreadyBeingManaged>),
    /// A boxed array of *AlreadyConnected*. To be used in *Any* placeholders.
    ArrayOfAlreadyConnected(Vec<AlreadyConnected>),
    /// A boxed array of *AlreadyExists*. To be used in *Any* placeholders.
    ArrayOfAlreadyExists(Vec<AlreadyExists>),
    /// A boxed array of *AlreadyUpgraded*. To be used in *Any* placeholders.
    ArrayOfAlreadyUpgraded(Vec<AlreadyUpgraded>),
    /// A boxed array of *AnswerFileUpdateFailed*. To be used in *Any* placeholders.
    ArrayOfAnswerFileUpdateFailed(Vec<AnswerFileUpdateFailed>),
    /// A boxed array of *AnswerFileUpdateFailure*. To be used in *Any* placeholders.
    ArrayOfAnswerFileUpdateFailure(Vec<AnswerFileUpdateFailure>),
    /// A boxed array of *ApplicationQuiesceFault*. To be used in *Any* placeholders.
    ArrayOfApplicationQuiesceFault(Vec<ApplicationQuiesceFault>),
    /// A boxed array of *AuthMinimumAdminPermission*. To be used in *Any* placeholders.
    ArrayOfAuthMinimumAdminPermission(Vec<AuthMinimumAdminPermission>),
    /// A boxed array of *BackupBlobReadFailure*. To be used in *Any* placeholders.
    ArrayOfBackupBlobReadFailure(Vec<BackupBlobReadFailure>),
    /// A boxed array of *BackupBlobWriteFailure*. To be used in *Any* placeholders.
    ArrayOfBackupBlobWriteFailure(Vec<BackupBlobWriteFailure>),
    /// A boxed array of *BlockedByFirewall*. To be used in *Any* placeholders.
    ArrayOfBlockedByFirewall(Vec<BlockedByFirewall>),
    /// A boxed array of *CAMServerRefusedConnection*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCAMServerRefusedConnection")]
    ArrayOfCamServerRefusedConnection(Vec<CamServerRefusedConnection>),
    /// A boxed array of *CannotAccessFile*. To be used in *Any* placeholders.
    ArrayOfCannotAccessFile(Vec<CannotAccessFile>),
    /// A boxed array of *CannotAccessLocalSource*. To be used in *Any* placeholders.
    ArrayOfCannotAccessLocalSource(Vec<CannotAccessLocalSource>),
    /// A boxed array of *CannotAccessNetwork*. To be used in *Any* placeholders.
    ArrayOfCannotAccessNetwork(Vec<Box<dyn super::traits::CannotAccessNetworkTrait>>),
    /// A boxed array of *CannotAccessVmComponent*. To be used in *Any* placeholders.
    ArrayOfCannotAccessVmComponent(Vec<Box<dyn super::traits::CannotAccessVmComponentTrait>>),
    /// A boxed array of *CannotAccessVmConfig*. To be used in *Any* placeholders.
    ArrayOfCannotAccessVmConfig(Vec<CannotAccessVmConfig>),
    /// A boxed array of *CannotAccessVmDevice*. To be used in *Any* placeholders.
    ArrayOfCannotAccessVmDevice(Vec<Box<dyn super::traits::CannotAccessVmDeviceTrait>>),
    /// A boxed array of *CannotAccessVmDisk*. To be used in *Any* placeholders.
    ArrayOfCannotAccessVmDisk(Vec<Box<dyn super::traits::CannotAccessVmDiskTrait>>),
    /// A boxed array of *CannotAddHostWithFTVmAsStandalone*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCannotAddHostWithFTVmAsStandalone")]
    ArrayOfCannotAddHostWithFtVmAsStandalone(Vec<CannotAddHostWithFtVmAsStandalone>),
    /// A boxed array of *CannotAddHostWithFTVmToDifferentCluster*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCannotAddHostWithFTVmToDifferentCluster")]
    ArrayOfCannotAddHostWithFtVmToDifferentCluster(Vec<CannotAddHostWithFtVmToDifferentCluster>),
    /// A boxed array of *CannotAddHostWithFTVmToNonHACluster*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCannotAddHostWithFTVmToNonHACluster")]
    ArrayOfCannotAddHostWithFtVmToNonHaCluster(Vec<CannotAddHostWithFtVmToNonHaCluster>),
    /// A boxed array of *CannotChangeDrsBehaviorForFtSecondary*. To be used in *Any* placeholders.
    ArrayOfCannotChangeDrsBehaviorForFtSecondary(Vec<CannotChangeDrsBehaviorForFtSecondary>),
    /// A boxed array of *CannotChangeHaSettingsForFtSecondary*. To be used in *Any* placeholders.
    ArrayOfCannotChangeHaSettingsForFtSecondary(Vec<CannotChangeHaSettingsForFtSecondary>),
    /// A boxed array of *CannotChangeVsanClusterUuid*. To be used in *Any* placeholders.
    ArrayOfCannotChangeVsanClusterUuid(Vec<CannotChangeVsanClusterUuid>),
    /// A boxed array of *CannotChangeVsanNodeUuid*. To be used in *Any* placeholders.
    ArrayOfCannotChangeVsanNodeUuid(Vec<CannotChangeVsanNodeUuid>),
    /// A boxed array of *CannotComputeFTCompatibleHosts*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCannotComputeFTCompatibleHosts")]
    ArrayOfCannotComputeFtCompatibleHosts(Vec<CannotComputeFtCompatibleHosts>),
    /// A boxed array of *CannotCreateFile*. To be used in *Any* placeholders.
    ArrayOfCannotCreateFile(Vec<CannotCreateFile>),
    /// A boxed array of *CannotDecryptPasswords*. To be used in *Any* placeholders.
    ArrayOfCannotDecryptPasswords(Vec<CannotDecryptPasswords>),
    /// A boxed array of *CannotDeleteFile*. To be used in *Any* placeholders.
    ArrayOfCannotDeleteFile(Vec<CannotDeleteFile>),
    /// A boxed array of *CannotDisableDrsOnClustersWithVApps*. To be used in *Any* placeholders.
    ArrayOfCannotDisableDrsOnClustersWithVApps(Vec<CannotDisableDrsOnClustersWithVApps>),
    /// A boxed array of *CannotDisableSnapshot*. To be used in *Any* placeholders.
    ArrayOfCannotDisableSnapshot(Vec<CannotDisableSnapshot>),
    /// A boxed array of *CannotDisconnectHostWithFaultToleranceVm*. To be used in *Any* placeholders.
    ArrayOfCannotDisconnectHostWithFaultToleranceVm(Vec<CannotDisconnectHostWithFaultToleranceVm>),
    /// A boxed array of *CannotEnableVmcpForCluster*. To be used in *Any* placeholders.
    ArrayOfCannotEnableVmcpForCluster(Vec<CannotEnableVmcpForCluster>),
    /// A boxed array of *CannotModifyConfigCpuRequirements*. To be used in *Any* placeholders.
    ArrayOfCannotModifyConfigCpuRequirements(Vec<CannotModifyConfigCpuRequirements>),
    /// A boxed array of *CannotMoveFaultToleranceVm*. To be used in *Any* placeholders.
    ArrayOfCannotMoveFaultToleranceVm(Vec<CannotMoveFaultToleranceVm>),
    /// A boxed array of *CannotMoveHostWithFaultToleranceVm*. To be used in *Any* placeholders.
    ArrayOfCannotMoveHostWithFaultToleranceVm(Vec<CannotMoveHostWithFaultToleranceVm>),
    /// A boxed array of *CannotMoveVmWithDeltaDisk*. To be used in *Any* placeholders.
    ArrayOfCannotMoveVmWithDeltaDisk(Vec<CannotMoveVmWithDeltaDisk>),
    /// A boxed array of *CannotMoveVmWithNativeDeltaDisk*. To be used in *Any* placeholders.
    ArrayOfCannotMoveVmWithNativeDeltaDisk(Vec<CannotMoveVmWithNativeDeltaDisk>),
    /// A boxed array of *CannotMoveVsanEnabledHost*. To be used in *Any* placeholders.
    ArrayOfCannotMoveVsanEnabledHost(Vec<Box<dyn super::traits::CannotMoveVsanEnabledHostTrait>>),
    /// A boxed array of *CannotPlaceWithoutPrerequisiteMoves*. To be used in *Any* placeholders.
    ArrayOfCannotPlaceWithoutPrerequisiteMoves(Vec<CannotPlaceWithoutPrerequisiteMoves>),
    /// A boxed array of *CannotPowerOffVmInCluster*. To be used in *Any* placeholders.
    ArrayOfCannotPowerOffVmInCluster(Vec<CannotPowerOffVmInCluster>),
    /// A boxed array of *CannotReconfigureVsanWhenHaEnabled*. To be used in *Any* placeholders.
    ArrayOfCannotReconfigureVsanWhenHaEnabled(Vec<CannotReconfigureVsanWhenHaEnabled>),
    /// A boxed array of *CannotUseNetwork*. To be used in *Any* placeholders.
    ArrayOfCannotUseNetwork(Vec<CannotUseNetwork>),
    /// A boxed array of *ClockSkew*. To be used in *Any* placeholders.
    ArrayOfClockSkew(Vec<ClockSkew>),
    /// A boxed array of *CloneFromSnapshotNotSupported*. To be used in *Any* placeholders.
    ArrayOfCloneFromSnapshotNotSupported(Vec<CloneFromSnapshotNotSupported>),
    /// A boxed array of *CollectorAddressUnset*. To be used in *Any* placeholders.
    ArrayOfCollectorAddressUnset(Vec<CollectorAddressUnset>),
    /// A boxed array of *ConcurrentAccess*. To be used in *Any* placeholders.
    ArrayOfConcurrentAccess(Vec<ConcurrentAccess>),
    /// A boxed array of *ConflictingConfiguration*. To be used in *Any* placeholders.
    ArrayOfConflictingConfiguration(Vec<ConflictingConfiguration>),
    /// A boxed array of *ConflictingConfigurationConfig*. To be used in *Any* placeholders.
    ArrayOfConflictingConfigurationConfig(Vec<ConflictingConfigurationConfig>),
    /// A boxed array of *ConflictingDatastoreFound*. To be used in *Any* placeholders.
    ArrayOfConflictingDatastoreFound(Vec<ConflictingDatastoreFound>),
    /// A boxed array of *ConnectedIso*. To be used in *Any* placeholders.
    ArrayOfConnectedIso(Vec<ConnectedIso>),
    /// A boxed array of *CpuCompatibilityUnknown*. To be used in *Any* placeholders.
    ArrayOfCpuCompatibilityUnknown(Vec<CpuCompatibilityUnknown>),
    /// A boxed array of *CpuHotPlugNotSupported*. To be used in *Any* placeholders.
    ArrayOfCpuHotPlugNotSupported(Vec<CpuHotPlugNotSupported>),
    /// A boxed array of *CpuIncompatible*. To be used in *Any* placeholders.
    ArrayOfCpuIncompatible(Vec<Box<dyn super::traits::CpuIncompatibleTrait>>),
    /// A boxed array of *CpuIncompatible1ECX*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCpuIncompatible1ECX")]
    ArrayOfCpuIncompatible1Ecx(Vec<CpuIncompatible1Ecx>),
    /// A boxed array of *CpuIncompatible81EDX*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCpuIncompatible81EDX")]
    ArrayOfCpuIncompatible81Edx(Vec<CpuIncompatible81Edx>),
    /// A boxed array of *CustomizationFault*. To be used in *Any* placeholders.
    ArrayOfCustomizationFault(Vec<Box<dyn super::traits::CustomizationFaultTrait>>),
    /// A boxed array of *CustomizationPending*. To be used in *Any* placeholders.
    ArrayOfCustomizationPending(Vec<CustomizationPending>),
    /// A boxed array of *DVPortNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDVPortNotSupported")]
    ArrayOfDvPortNotSupported(Vec<DvPortNotSupported>),
    /// A boxed array of *DasConfigFault*. To be used in *Any* placeholders.
    ArrayOfDasConfigFault(Vec<DasConfigFault>),
    /// A boxed array of *DatabaseError*. To be used in *Any* placeholders.
    ArrayOfDatabaseError(Vec<DatabaseError>),
    /// A boxed array of *DatacenterMismatch*. To be used in *Any* placeholders.
    ArrayOfDatacenterMismatch(Vec<DatacenterMismatch>),
    /// A boxed array of *DatacenterMismatchArgument*. To be used in *Any* placeholders.
    ArrayOfDatacenterMismatchArgument(Vec<DatacenterMismatchArgument>),
    /// A boxed array of *DatastoreNotWritableOnHost*. To be used in *Any* placeholders.
    ArrayOfDatastoreNotWritableOnHost(Vec<Box<dyn super::traits::DatastoreNotWritableOnHostTrait>>),
    /// A boxed array of *DeltaDiskFormatNotSupported*. To be used in *Any* placeholders.
    ArrayOfDeltaDiskFormatNotSupported(Vec<DeltaDiskFormatNotSupported>),
    /// A boxed array of *DestinationSwitchFull*. To be used in *Any* placeholders.
    ArrayOfDestinationSwitchFull(Vec<DestinationSwitchFull>),
    /// A boxed array of *DestinationVsanDisabled*. To be used in *Any* placeholders.
    ArrayOfDestinationVsanDisabled(Vec<DestinationVsanDisabled>),
    /// A boxed array of *DeviceBackingNotSupported*. To be used in *Any* placeholders.
    ArrayOfDeviceBackingNotSupported(Vec<Box<dyn super::traits::DeviceBackingNotSupportedTrait>>),
    /// A boxed array of *DeviceControllerNotSupported*. To be used in *Any* placeholders.
    ArrayOfDeviceControllerNotSupported(Vec<DeviceControllerNotSupported>),
    /// A boxed array of *DeviceHotPlugNotSupported*. To be used in *Any* placeholders.
    ArrayOfDeviceHotPlugNotSupported(Vec<DeviceHotPlugNotSupported>),
    /// A boxed array of *DeviceNotFound*. To be used in *Any* placeholders.
    ArrayOfDeviceNotFound(Vec<DeviceNotFound>),
    /// A boxed array of *DeviceNotSupported*. To be used in *Any* placeholders.
    ArrayOfDeviceNotSupported(Vec<Box<dyn super::traits::DeviceNotSupportedTrait>>),
    /// A boxed array of *DeviceUnsupportedForVmPlatform*. To be used in *Any* placeholders.
    ArrayOfDeviceUnsupportedForVmPlatform(Vec<DeviceUnsupportedForVmPlatform>),
    /// A boxed array of *DeviceUnsupportedForVmVersion*. To be used in *Any* placeholders.
    ArrayOfDeviceUnsupportedForVmVersion(Vec<DeviceUnsupportedForVmVersion>),
    /// A boxed array of *DigestNotSupported*. To be used in *Any* placeholders.
    ArrayOfDigestNotSupported(Vec<DigestNotSupported>),
    /// A boxed array of *DirectoryNotEmpty*. To be used in *Any* placeholders.
    ArrayOfDirectoryNotEmpty(Vec<DirectoryNotEmpty>),
    /// A boxed array of *DisableAdminNotSupported*. To be used in *Any* placeholders.
    ArrayOfDisableAdminNotSupported(Vec<DisableAdminNotSupported>),
    /// A boxed array of *DisallowedChangeByService*. To be used in *Any* placeholders.
    ArrayOfDisallowedChangeByService(Vec<DisallowedChangeByService>),
    /// A boxed array of *DisallowedDiskModeChange*. To be used in *Any* placeholders.
    ArrayOfDisallowedDiskModeChange(Vec<DisallowedDiskModeChange>),
    /// A boxed array of *DisallowedMigrationDeviceAttached*. To be used in *Any* placeholders.
    ArrayOfDisallowedMigrationDeviceAttached(Vec<DisallowedMigrationDeviceAttached>),
    /// A boxed array of *DisallowedOperationOnFailoverHost*. To be used in *Any* placeholders.
    ArrayOfDisallowedOperationOnFailoverHost(Vec<DisallowedOperationOnFailoverHost>),
    /// A boxed array of *DisconnectedHostsBlockingEVC*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDisconnectedHostsBlockingEVC")]
    ArrayOfDisconnectedHostsBlockingEvc(Vec<DisconnectedHostsBlockingEvc>),
    /// A boxed array of *DiskHasPartitions*. To be used in *Any* placeholders.
    ArrayOfDiskHasPartitions(Vec<DiskHasPartitions>),
    /// A boxed array of *DiskIsLastRemainingNonSSD*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDiskIsLastRemainingNonSSD")]
    ArrayOfDiskIsLastRemainingNonSsd(Vec<DiskIsLastRemainingNonSsd>),
    /// A boxed array of *DiskIsNonLocal*. To be used in *Any* placeholders.
    ArrayOfDiskIsNonLocal(Vec<DiskIsNonLocal>),
    /// A boxed array of *DiskIsUSB*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfDiskIsUSB")]
    ArrayOfDiskIsUsb(Vec<DiskIsUsb>),
    /// A boxed array of *DiskMoveTypeNotSupported*. To be used in *Any* placeholders.
    ArrayOfDiskMoveTypeNotSupported(Vec<DiskMoveTypeNotSupported>),
    /// A boxed array of *DiskNotSupported*. To be used in *Any* placeholders.
    ArrayOfDiskNotSupported(Vec<Box<dyn super::traits::DiskNotSupportedTrait>>),
    /// A boxed array of *DiskTooSmall*. To be used in *Any* placeholders.
    ArrayOfDiskTooSmall(Vec<DiskTooSmall>),
    /// A boxed array of *DomainNotFound*. To be used in *Any* placeholders.
    ArrayOfDomainNotFound(Vec<DomainNotFound>),
    /// A boxed array of *DrsDisabledOnVm*. To be used in *Any* placeholders.
    ArrayOfDrsDisabledOnVm(Vec<DrsDisabledOnVm>),
    /// A boxed array of *DrsVmotionIncompatibleFault*. To be used in *Any* placeholders.
    ArrayOfDrsVmotionIncompatibleFault(Vec<DrsVmotionIncompatibleFault>),
    /// A boxed array of *DuplicateDisks*. To be used in *Any* placeholders.
    ArrayOfDuplicateDisks(Vec<DuplicateDisks>),
    /// A boxed array of *DuplicateName*. To be used in *Any* placeholders.
    ArrayOfDuplicateName(Vec<DuplicateName>),
    /// A boxed array of *DuplicateVsanNetworkInterface*. To be used in *Any* placeholders.
    ArrayOfDuplicateVsanNetworkInterface(Vec<DuplicateVsanNetworkInterface>),
    /// A boxed array of *DvsApplyOperationFault*. To be used in *Any* placeholders.
    ArrayOfDvsApplyOperationFault(Vec<DvsApplyOperationFault>),
    /// A boxed array of *DvsApplyOperationFaultFaultOnObject*. To be used in *Any* placeholders.
    ArrayOfDvsApplyOperationFaultFaultOnObject(Vec<DvsApplyOperationFaultFaultOnObject>),
    /// A boxed array of *DvsFault*. To be used in *Any* placeholders.
    ArrayOfDvsFault(Vec<Box<dyn super::traits::DvsFaultTrait>>),
    /// A boxed array of *DvsNotAuthorized*. To be used in *Any* placeholders.
    ArrayOfDvsNotAuthorized(Vec<DvsNotAuthorized>),
    /// A boxed array of *DvsOperationBulkFault*. To be used in *Any* placeholders.
    ArrayOfDvsOperationBulkFault(Vec<DvsOperationBulkFault>),
    /// A boxed array of *DvsOperationBulkFaultFaultOnHost*. To be used in *Any* placeholders.
    ArrayOfDvsOperationBulkFaultFaultOnHost(Vec<DvsOperationBulkFaultFaultOnHost>),
    /// A boxed array of *DvsScopeViolated*. To be used in *Any* placeholders.
    ArrayOfDvsScopeViolated(Vec<DvsScopeViolated>),
    /// A boxed array of *EVCAdmissionFailed*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailed")]
    ArrayOfEvcAdmissionFailed(Vec<Box<dyn super::traits::EvcAdmissionFailedTrait>>),
    /// A boxed array of *EVCAdmissionFailedCPUFeaturesForMode*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedCPUFeaturesForMode")]
    ArrayOfEvcAdmissionFailedCpuFeaturesForMode(Vec<EvcAdmissionFailedCpuFeaturesForMode>),
    /// A boxed array of *EVCAdmissionFailedCPUModel*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedCPUModel")]
    ArrayOfEvcAdmissionFailedCpuModel(Vec<EvcAdmissionFailedCpuModel>),
    /// A boxed array of *EVCAdmissionFailedCPUModelForMode*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedCPUModelForMode")]
    ArrayOfEvcAdmissionFailedCpuModelForMode(Vec<EvcAdmissionFailedCpuModelForMode>),
    /// A boxed array of *EVCAdmissionFailedCPUVendor*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedCPUVendor")]
    ArrayOfEvcAdmissionFailedCpuVendor(Vec<EvcAdmissionFailedCpuVendor>),
    /// A boxed array of *EVCAdmissionFailedCPUVendorUnknown*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedCPUVendorUnknown")]
    ArrayOfEvcAdmissionFailedCpuVendorUnknown(Vec<EvcAdmissionFailedCpuVendorUnknown>),
    /// A boxed array of *EVCAdmissionFailedHostDisconnected*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedHostDisconnected")]
    ArrayOfEvcAdmissionFailedHostDisconnected(Vec<EvcAdmissionFailedHostDisconnected>),
    /// A boxed array of *EVCAdmissionFailedHostSoftware*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedHostSoftware")]
    ArrayOfEvcAdmissionFailedHostSoftware(Vec<EvcAdmissionFailedHostSoftware>),
    /// A boxed array of *EVCAdmissionFailedHostSoftwareForMode*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedHostSoftwareForMode")]
    ArrayOfEvcAdmissionFailedHostSoftwareForMode(Vec<EvcAdmissionFailedHostSoftwareForMode>),
    /// A boxed array of *EVCAdmissionFailedVmActive*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCAdmissionFailedVmActive")]
    ArrayOfEvcAdmissionFailedVmActive(Vec<EvcAdmissionFailedVmActive>),
    /// A boxed array of *EVCConfigFault*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCConfigFault")]
    ArrayOfEvcConfigFault(Vec<Box<dyn super::traits::EvcConfigFaultTrait>>),
    /// A boxed array of *EVCModeIllegalByVendor*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCModeIllegalByVendor")]
    ArrayOfEvcModeIllegalByVendor(Vec<EvcModeIllegalByVendor>),
    /// A boxed array of *EVCModeUnsupportedByHosts*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCModeUnsupportedByHosts")]
    ArrayOfEvcModeUnsupportedByHosts(Vec<EvcModeUnsupportedByHosts>),
    /// A boxed array of *EVCUnsupportedByHostHardware*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCUnsupportedByHostHardware")]
    ArrayOfEvcUnsupportedByHostHardware(Vec<EvcUnsupportedByHostHardware>),
    /// A boxed array of *EVCUnsupportedByHostSoftware*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfEVCUnsupportedByHostSoftware")]
    ArrayOfEvcUnsupportedByHostSoftware(Vec<EvcUnsupportedByHostSoftware>),
    /// A boxed array of *EightHostLimitViolated*. To be used in *Any* placeholders.
    ArrayOfEightHostLimitViolated(Vec<EightHostLimitViolated>),
    /// A boxed array of *EncryptionKeyRequired*. To be used in *Any* placeholders.
    ArrayOfEncryptionKeyRequired(Vec<EncryptionKeyRequired>),
    /// A boxed array of *ExpiredAddonLicense*. To be used in *Any* placeholders.
    ArrayOfExpiredAddonLicense(Vec<ExpiredAddonLicense>),
    /// A boxed array of *ExpiredEditionLicense*. To be used in *Any* placeholders.
    ArrayOfExpiredEditionLicense(Vec<ExpiredEditionLicense>),
    /// A boxed array of *ExpiredFeatureLicense*. To be used in *Any* placeholders.
    ArrayOfExpiredFeatureLicense(Vec<Box<dyn super::traits::ExpiredFeatureLicenseTrait>>),
    /// A boxed array of *ExtendedFault*. To be used in *Any* placeholders.
    ArrayOfExtendedFault(Vec<ExtendedFault>),
    /// A boxed array of *FailToEnableSPBM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfFailToEnableSPBM")]
    ArrayOfFailToEnableSpbm(Vec<FailToEnableSpbm>),
    /// A boxed array of *FailToLockFaultToleranceVMs*. To be used in *Any* placeholders.
    ArrayOfFailToLockFaultToleranceVMs(Vec<FailToLockFaultToleranceVMs>),
    /// A boxed array of *FaultToleranceAntiAffinityViolated*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceAntiAffinityViolated(Vec<FaultToleranceAntiAffinityViolated>),
    /// A boxed array of *FaultToleranceCannotEditMem*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceCannotEditMem(Vec<FaultToleranceCannotEditMem>),
    /// A boxed array of *FaultToleranceCpuIncompatible*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceCpuIncompatible(Vec<FaultToleranceCpuIncompatible>),
    /// A boxed array of *FaultToleranceNeedsThickDisk*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceNeedsThickDisk(Vec<FaultToleranceNeedsThickDisk>),
    /// A boxed array of *FaultToleranceNotLicensed*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceNotLicensed(Vec<FaultToleranceNotLicensed>),
    /// A boxed array of *FaultToleranceNotSameBuild*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceNotSameBuild(Vec<FaultToleranceNotSameBuild>),
    /// A boxed array of *FaultTolerancePrimaryPowerOnNotAttempted*. To be used in *Any* placeholders.
    ArrayOfFaultTolerancePrimaryPowerOnNotAttempted(Vec<FaultTolerancePrimaryPowerOnNotAttempted>),
    /// A boxed array of *FaultToleranceVmNotDasProtected*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceVmNotDasProtected(Vec<FaultToleranceVmNotDasProtected>),
    /// A boxed array of *FcoeFault*. To be used in *Any* placeholders.
    ArrayOfFcoeFault(Vec<Box<dyn super::traits::FcoeFaultTrait>>),
    /// A boxed array of *FcoeFaultPnicHasNoPortSet*. To be used in *Any* placeholders.
    ArrayOfFcoeFaultPnicHasNoPortSet(Vec<FcoeFaultPnicHasNoPortSet>),
    /// A boxed array of *FeatureRequirementsNotMet*. To be used in *Any* placeholders.
    ArrayOfFeatureRequirementsNotMet(Vec<FeatureRequirementsNotMet>),
    /// A boxed array of *FileAlreadyExists*. To be used in *Any* placeholders.
    ArrayOfFileAlreadyExists(Vec<FileAlreadyExists>),
    /// A boxed array of *FileBackedPortNotSupported*. To be used in *Any* placeholders.
    ArrayOfFileBackedPortNotSupported(Vec<FileBackedPortNotSupported>),
    /// A boxed array of *FileFault*. To be used in *Any* placeholders.
    ArrayOfFileFault(Vec<Box<dyn super::traits::FileFaultTrait>>),
    /// A boxed array of *FileLocked*. To be used in *Any* placeholders.
    ArrayOfFileLocked(Vec<FileLocked>),
    /// A boxed array of *FileNameTooLong*. To be used in *Any* placeholders.
    ArrayOfFileNameTooLong(Vec<FileNameTooLong>),
    /// A boxed array of *FileNotFound*. To be used in *Any* placeholders.
    ArrayOfFileNotFound(Vec<FileNotFound>),
    /// A boxed array of *FileNotWritable*. To be used in *Any* placeholders.
    ArrayOfFileNotWritable(Vec<FileNotWritable>),
    /// A boxed array of *FileTooLarge*. To be used in *Any* placeholders.
    ArrayOfFileTooLarge(Vec<FileTooLarge>),
    /// A boxed array of *FilesystemQuiesceFault*. To be used in *Any* placeholders.
    ArrayOfFilesystemQuiesceFault(Vec<FilesystemQuiesceFault>),
    /// A boxed array of *FilterInUse*. To be used in *Any* placeholders.
    ArrayOfFilterInUse(Vec<FilterInUse>),
    /// A boxed array of *FtIssuesOnHost*. To be used in *Any* placeholders.
    ArrayOfFtIssuesOnHost(Vec<FtIssuesOnHost>),
    /// A boxed array of *FullStorageVMotionNotSupported*. To be used in *Any* placeholders.
    ArrayOfFullStorageVMotionNotSupported(Vec<FullStorageVMotionNotSupported>),
    /// A boxed array of *GatewayConnectFault*. To be used in *Any* placeholders.
    ArrayOfGatewayConnectFault(Vec<Box<dyn super::traits::GatewayConnectFaultTrait>>),
    /// A boxed array of *GatewayHostNotReachable*. To be used in *Any* placeholders.
    ArrayOfGatewayHostNotReachable(Vec<GatewayHostNotReachable>),
    /// A boxed array of *GatewayNotFound*. To be used in *Any* placeholders.
    ArrayOfGatewayNotFound(Vec<GatewayNotFound>),
    /// A boxed array of *GatewayNotReachable*. To be used in *Any* placeholders.
    ArrayOfGatewayNotReachable(Vec<GatewayNotReachable>),
    /// A boxed array of *GatewayOperationRefused*. To be used in *Any* placeholders.
    ArrayOfGatewayOperationRefused(Vec<GatewayOperationRefused>),
    /// A boxed array of *GatewayToHostAuthFault*. To be used in *Any* placeholders.
    ArrayOfGatewayToHostAuthFault(Vec<GatewayToHostAuthFault>),
    /// A boxed array of *GatewayToHostConnectFault*. To be used in *Any* placeholders.
    ArrayOfGatewayToHostConnectFault(Vec<Box<dyn super::traits::GatewayToHostConnectFaultTrait>>),
    /// A boxed array of *GatewayToHostTrustVerifyFault*. To be used in *Any* placeholders.
    ArrayOfGatewayToHostTrustVerifyFault(Vec<GatewayToHostTrustVerifyFault>),
    /// A boxed array of *GenericDrsFault*. To be used in *Any* placeholders.
    ArrayOfGenericDrsFault(Vec<GenericDrsFault>),
    /// A boxed array of *GenericVmConfigFault*. To be used in *Any* placeholders.
    ArrayOfGenericVmConfigFault(Vec<GenericVmConfigFault>),
    /// A boxed array of *GuestAuthenticationChallenge*. To be used in *Any* placeholders.
    ArrayOfGuestAuthenticationChallenge(Vec<GuestAuthenticationChallenge>),
    /// A boxed array of *GuestComponentsOutOfDate*. To be used in *Any* placeholders.
    ArrayOfGuestComponentsOutOfDate(Vec<GuestComponentsOutOfDate>),
    /// A boxed array of *GuestMultipleMappings*. To be used in *Any* placeholders.
    ArrayOfGuestMultipleMappings(Vec<GuestMultipleMappings>),
    /// A boxed array of *GuestOperationsFault*. To be used in *Any* placeholders.
    ArrayOfGuestOperationsFault(Vec<Box<dyn super::traits::GuestOperationsFaultTrait>>),
    /// A boxed array of *GuestOperationsUnavailable*. To be used in *Any* placeholders.
    ArrayOfGuestOperationsUnavailable(Vec<GuestOperationsUnavailable>),
    /// A boxed array of *GuestPermissionDenied*. To be used in *Any* placeholders.
    ArrayOfGuestPermissionDenied(Vec<GuestPermissionDenied>),
    /// A boxed array of *GuestProcessNotFound*. To be used in *Any* placeholders.
    ArrayOfGuestProcessNotFound(Vec<GuestProcessNotFound>),
    /// A boxed array of *GuestRegistryFault*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryFault(Vec<Box<dyn super::traits::GuestRegistryFaultTrait>>),
    /// A boxed array of *GuestRegistryKeyAlreadyExists*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryKeyAlreadyExists(Vec<GuestRegistryKeyAlreadyExists>),
    /// A boxed array of *GuestRegistryKeyFault*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryKeyFault(Vec<Box<dyn super::traits::GuestRegistryKeyFaultTrait>>),
    /// A boxed array of *GuestRegistryKeyHasSubkeys*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryKeyHasSubkeys(Vec<GuestRegistryKeyHasSubkeys>),
    /// A boxed array of *GuestRegistryKeyInvalid*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryKeyInvalid(Vec<GuestRegistryKeyInvalid>),
    /// A boxed array of *GuestRegistryKeyParentVolatile*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryKeyParentVolatile(Vec<GuestRegistryKeyParentVolatile>),
    /// A boxed array of *GuestRegistryValueFault*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryValueFault(Vec<Box<dyn super::traits::GuestRegistryValueFaultTrait>>),
    /// A boxed array of *GuestRegistryValueNotFound*. To be used in *Any* placeholders.
    ArrayOfGuestRegistryValueNotFound(Vec<GuestRegistryValueNotFound>),
    /// A boxed array of *HAErrorsAtDest*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHAErrorsAtDest")]
    ArrayOfHaErrorsAtDest(Vec<HaErrorsAtDest>),
    /// A boxed array of *HeterogenousHostsBlockingEVC*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHeterogenousHostsBlockingEVC")]
    ArrayOfHeterogenousHostsBlockingEvc(Vec<HeterogenousHostsBlockingEvc>),
    /// A boxed array of *HostAccessRestrictedToManagementServer*. To be used in *Any* placeholders.
    ArrayOfHostAccessRestrictedToManagementServer(Vec<HostAccessRestrictedToManagementServer>),
    /// A boxed array of *HostConfigFailed*. To be used in *Any* placeholders.
    ArrayOfHostConfigFailed(Vec<HostConfigFailed>),
    /// A boxed array of *HostConfigFault*. To be used in *Any* placeholders.
    ArrayOfHostConfigFault(Vec<Box<dyn super::traits::HostConfigFaultTrait>>),
    /// A boxed array of *HostConnectFault*. To be used in *Any* placeholders.
    ArrayOfHostConnectFault(Vec<Box<dyn super::traits::HostConnectFaultTrait>>),
    /// A boxed array of *HostHasComponentFailure*. To be used in *Any* placeholders.
    ArrayOfHostHasComponentFailure(Vec<HostHasComponentFailure>),
    /// A boxed array of *HostInDomain*. To be used in *Any* placeholders.
    ArrayOfHostInDomain(Vec<HostInDomain>),
    /// A boxed array of *HostIncompatibleForFaultTolerance*. To be used in *Any* placeholders.
    ArrayOfHostIncompatibleForFaultTolerance(Vec<HostIncompatibleForFaultTolerance>),
    /// A boxed array of *HostIncompatibleForRecordReplay*. To be used in *Any* placeholders.
    ArrayOfHostIncompatibleForRecordReplay(Vec<HostIncompatibleForRecordReplay>),
    /// A boxed array of *HostInventoryFull*. To be used in *Any* placeholders.
    ArrayOfHostInventoryFull(Vec<HostInventoryFull>),
    /// A boxed array of *HostPowerOpFailed*. To be used in *Any* placeholders.
    ArrayOfHostPowerOpFailed(Vec<Box<dyn super::traits::HostPowerOpFailedTrait>>),
    /// A boxed array of *HostSpecificationOperationFailed*. To be used in *Any* placeholders.
    ArrayOfHostSpecificationOperationFailed(Vec<HostSpecificationOperationFailed>),
    /// A boxed array of *HotSnapshotMoveNotSupported*. To be used in *Any* placeholders.
    ArrayOfHotSnapshotMoveNotSupported(Vec<HotSnapshotMoveNotSupported>),
    /// A boxed array of *HttpFault*. To be used in *Any* placeholders.
    ArrayOfHttpFault(Vec<HttpFault>),
    /// A boxed array of *IDEDiskNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfIDEDiskNotSupported")]
    ArrayOfIdeDiskNotSupported(Vec<IdeDiskNotSupported>),
    /// A boxed array of *IORMNotSupportedHostOnDatastore*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfIORMNotSupportedHostOnDatastore")]
    ArrayOfIormNotSupportedHostOnDatastore(Vec<IormNotSupportedHostOnDatastore>),
    /// A boxed array of *ImportHostAddFailure*. To be used in *Any* placeholders.
    ArrayOfImportHostAddFailure(Vec<ImportHostAddFailure>),
    /// A boxed array of *ImportOperationBulkFault*. To be used in *Any* placeholders.
    ArrayOfImportOperationBulkFault(Vec<ImportOperationBulkFault>),
    /// A boxed array of *ImportOperationBulkFaultFaultOnImport*. To be used in *Any* placeholders.
    ArrayOfImportOperationBulkFaultFaultOnImport(Vec<ImportOperationBulkFaultFaultOnImport>),
    /// A boxed array of *InUseFeatureManipulationDisallowed*. To be used in *Any* placeholders.
    ArrayOfInUseFeatureManipulationDisallowed(Vec<InUseFeatureManipulationDisallowed>),
    /// A boxed array of *InaccessibleDatastore*. To be used in *Any* placeholders.
    ArrayOfInaccessibleDatastore(Vec<Box<dyn super::traits::InaccessibleDatastoreTrait>>),
    /// A boxed array of *InaccessibleFTMetadataDatastore*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfInaccessibleFTMetadataDatastore")]
    ArrayOfInaccessibleFtMetadataDatastore(Vec<InaccessibleFtMetadataDatastore>),
    /// A boxed array of *InaccessibleVFlashSource*. To be used in *Any* placeholders.
    ArrayOfInaccessibleVFlashSource(Vec<InaccessibleVFlashSource>),
    /// A boxed array of *IncompatibleDefaultDevice*. To be used in *Any* placeholders.
    ArrayOfIncompatibleDefaultDevice(Vec<IncompatibleDefaultDevice>),
    /// A boxed array of *IncompatibleHostForFtSecondary*. To be used in *Any* placeholders.
    ArrayOfIncompatibleHostForFtSecondary(Vec<IncompatibleHostForFtSecondary>),
    /// A boxed array of *IncompatibleHostForVmReplication*. To be used in *Any* placeholders.
    ArrayOfIncompatibleHostForVmReplication(Vec<IncompatibleHostForVmReplication>),
    /// A boxed array of *IncompatibleSetting*. To be used in *Any* placeholders.
    ArrayOfIncompatibleSetting(Vec<IncompatibleSetting>),
    /// A boxed array of *IncorrectFileType*. To be used in *Any* placeholders.
    ArrayOfIncorrectFileType(Vec<IncorrectFileType>),
    /// A boxed array of *IncorrectHostInformation*. To be used in *Any* placeholders.
    ArrayOfIncorrectHostInformation(Vec<IncorrectHostInformation>),
    /// A boxed array of *IndependentDiskVMotionNotSupported*. To be used in *Any* placeholders.
    ArrayOfIndependentDiskVMotionNotSupported(Vec<IndependentDiskVMotionNotSupported>),
    /// A boxed array of *InsufficientAgentVmsDeployed*. To be used in *Any* placeholders.
    ArrayOfInsufficientAgentVmsDeployed(Vec<InsufficientAgentVmsDeployed>),
    /// A boxed array of *InsufficientCpuResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientCpuResourcesFault(Vec<InsufficientCpuResourcesFault>),
    /// A boxed array of *InsufficientDisks*. To be used in *Any* placeholders.
    ArrayOfInsufficientDisks(Vec<InsufficientDisks>),
    /// A boxed array of *InsufficientFailoverResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientFailoverResourcesFault(Vec<InsufficientFailoverResourcesFault>),
    /// A boxed array of *InsufficientGraphicsResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientGraphicsResourcesFault(Vec<InsufficientGraphicsResourcesFault>),
    /// A boxed array of *InsufficientHostCapacityFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientHostCapacityFault(Vec<Box<dyn super::traits::InsufficientHostCapacityFaultTrait>>),
    /// A boxed array of *InsufficientHostCpuCapacityFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientHostCpuCapacityFault(Vec<InsufficientHostCpuCapacityFault>),
    /// A boxed array of *InsufficientHostMemoryCapacityFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientHostMemoryCapacityFault(Vec<InsufficientHostMemoryCapacityFault>),
    /// A boxed array of *InsufficientMemoryResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientMemoryResourcesFault(Vec<InsufficientMemoryResourcesFault>),
    /// A boxed array of *InsufficientNetworkCapacity*. To be used in *Any* placeholders.
    ArrayOfInsufficientNetworkCapacity(Vec<InsufficientNetworkCapacity>),
    /// A boxed array of *InsufficientNetworkResourcePoolCapacity*. To be used in *Any* placeholders.
    ArrayOfInsufficientNetworkResourcePoolCapacity(Vec<InsufficientNetworkResourcePoolCapacity>),
    /// A boxed array of *InsufficientPerCpuCapacity*. To be used in *Any* placeholders.
    ArrayOfInsufficientPerCpuCapacity(Vec<InsufficientPerCpuCapacity>),
    /// A boxed array of *InsufficientResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientResourcesFault(Vec<Box<dyn super::traits::InsufficientResourcesFaultTrait>>),
    /// A boxed array of *InsufficientStandbyCpuResource*. To be used in *Any* placeholders.
    ArrayOfInsufficientStandbyCpuResource(Vec<InsufficientStandbyCpuResource>),
    /// A boxed array of *InsufficientStandbyMemoryResource*. To be used in *Any* placeholders.
    ArrayOfInsufficientStandbyMemoryResource(Vec<InsufficientStandbyMemoryResource>),
    /// A boxed array of *InsufficientStandbyResource*. To be used in *Any* placeholders.
    ArrayOfInsufficientStandbyResource(Vec<Box<dyn super::traits::InsufficientStandbyResourceTrait>>),
    /// A boxed array of *InsufficientStorageIops*. To be used in *Any* placeholders.
    ArrayOfInsufficientStorageIops(Vec<InsufficientStorageIops>),
    /// A boxed array of *InsufficientStorageSpace*. To be used in *Any* placeholders.
    ArrayOfInsufficientStorageSpace(Vec<InsufficientStorageSpace>),
    /// A boxed array of *InsufficientVFlashResourcesFault*. To be used in *Any* placeholders.
    ArrayOfInsufficientVFlashResourcesFault(Vec<InsufficientVFlashResourcesFault>),
    /// A boxed array of *InvalidAffinitySettingFault*. To be used in *Any* placeholders.
    ArrayOfInvalidAffinitySettingFault(Vec<InvalidAffinitySettingFault>),
    /// A boxed array of *InvalidBmcRole*. To be used in *Any* placeholders.
    ArrayOfInvalidBmcRole(Vec<InvalidBmcRole>),
    /// A boxed array of *InvalidBundle*. To be used in *Any* placeholders.
    ArrayOfInvalidBundle(Vec<InvalidBundle>),
    /// A boxed array of *InvalidCAMCertificate*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfInvalidCAMCertificate")]
    ArrayOfInvalidCamCertificate(Vec<InvalidCamCertificate>),
    /// A boxed array of *InvalidCAMServer*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfInvalidCAMServer")]
    ArrayOfInvalidCamServer(Vec<Box<dyn super::traits::InvalidCamServerTrait>>),
    /// A boxed array of *InvalidClientCertificate*. To be used in *Any* placeholders.
    ArrayOfInvalidClientCertificate(Vec<InvalidClientCertificate>),
    /// A boxed array of *InvalidController*. To be used in *Any* placeholders.
    ArrayOfInvalidController(Vec<InvalidController>),
    /// A boxed array of *InvalidDasConfigArgument*. To be used in *Any* placeholders.
    ArrayOfInvalidDasConfigArgument(Vec<InvalidDasConfigArgument>),
    /// A boxed array of *InvalidDasRestartPriorityForFtVm*. To be used in *Any* placeholders.
    ArrayOfInvalidDasRestartPriorityForFtVm(Vec<InvalidDasRestartPriorityForFtVm>),
    /// A boxed array of *InvalidDatastore*. To be used in *Any* placeholders.
    ArrayOfInvalidDatastore(Vec<Box<dyn super::traits::InvalidDatastoreTrait>>),
    /// A boxed array of *InvalidDatastorePath*. To be used in *Any* placeholders.
    ArrayOfInvalidDatastorePath(Vec<InvalidDatastorePath>),
    /// A boxed array of *InvalidDatastoreState*. To be used in *Any* placeholders.
    ArrayOfInvalidDatastoreState(Vec<InvalidDatastoreState>),
    /// A boxed array of *InvalidDeviceBacking*. To be used in *Any* placeholders.
    ArrayOfInvalidDeviceBacking(Vec<InvalidDeviceBacking>),
    /// A boxed array of *InvalidDeviceOperation*. To be used in *Any* placeholders.
    ArrayOfInvalidDeviceOperation(Vec<InvalidDeviceOperation>),
    /// A boxed array of *InvalidDeviceSpec*. To be used in *Any* placeholders.
    ArrayOfInvalidDeviceSpec(Vec<Box<dyn super::traits::InvalidDeviceSpecTrait>>),
    /// A boxed array of *InvalidDiskFormat*. To be used in *Any* placeholders.
    ArrayOfInvalidDiskFormat(Vec<InvalidDiskFormat>),
    /// A boxed array of *InvalidDrsBehaviorForFtVm*. To be used in *Any* placeholders.
    ArrayOfInvalidDrsBehaviorForFtVm(Vec<InvalidDrsBehaviorForFtVm>),
    /// A boxed array of *InvalidEditionLicense*. To be used in *Any* placeholders.
    ArrayOfInvalidEditionLicense(Vec<InvalidEditionLicense>),
    /// A boxed array of *InvalidEvent*. To be used in *Any* placeholders.
    ArrayOfInvalidEvent(Vec<InvalidEvent>),
    /// A boxed array of *InvalidFolder*. To be used in *Any* placeholders.
    ArrayOfInvalidFolder(Vec<Box<dyn super::traits::InvalidFolderTrait>>),
    /// A boxed array of *InvalidFormat*. To be used in *Any* placeholders.
    ArrayOfInvalidFormat(Vec<Box<dyn super::traits::InvalidFormatTrait>>),
    /// A boxed array of *InvalidGuestLogin*. To be used in *Any* placeholders.
    ArrayOfInvalidGuestLogin(Vec<InvalidGuestLogin>),
    /// A boxed array of *InvalidHostConnectionState*. To be used in *Any* placeholders.
    ArrayOfInvalidHostConnectionState(Vec<InvalidHostConnectionState>),
    /// A boxed array of *InvalidHostName*. To be used in *Any* placeholders.
    ArrayOfInvalidHostName(Vec<InvalidHostName>),
    /// A boxed array of *InvalidHostState*. To be used in *Any* placeholders.
    ArrayOfInvalidHostState(Vec<Box<dyn super::traits::InvalidHostStateTrait>>),
    /// A boxed array of *InvalidIndexArgument*. To be used in *Any* placeholders.
    ArrayOfInvalidIndexArgument(Vec<InvalidIndexArgument>),
    /// A boxed array of *InvalidIpfixConfig*. To be used in *Any* placeholders.
    ArrayOfInvalidIpfixConfig(Vec<InvalidIpfixConfig>),
    /// A boxed array of *InvalidIpmiLoginInfo*. To be used in *Any* placeholders.
    ArrayOfInvalidIpmiLoginInfo(Vec<InvalidIpmiLoginInfo>),
    /// A boxed array of *InvalidIpmiMacAddress*. To be used in *Any* placeholders.
    ArrayOfInvalidIpmiMacAddress(Vec<InvalidIpmiMacAddress>),
    /// A boxed array of *InvalidLicense*. To be used in *Any* placeholders.
    ArrayOfInvalidLicense(Vec<InvalidLicense>),
    /// A boxed array of *InvalidLocale*. To be used in *Any* placeholders.
    ArrayOfInvalidLocale(Vec<InvalidLocale>),
    /// A boxed array of *InvalidLogin*. To be used in *Any* placeholders.
    ArrayOfInvalidLogin(Vec<Box<dyn super::traits::InvalidLoginTrait>>),
    /// A boxed array of *InvalidName*. To be used in *Any* placeholders.
    ArrayOfInvalidName(Vec<InvalidName>),
    /// A boxed array of *InvalidNasCredentials*. To be used in *Any* placeholders.
    ArrayOfInvalidNasCredentials(Vec<InvalidNasCredentials>),
    /// A boxed array of *InvalidNetworkInType*. To be used in *Any* placeholders.
    ArrayOfInvalidNetworkInType(Vec<InvalidNetworkInType>),
    /// A boxed array of *InvalidNetworkResource*. To be used in *Any* placeholders.
    ArrayOfInvalidNetworkResource(Vec<InvalidNetworkResource>),
    /// A boxed array of *InvalidOperationOnSecondaryVm*. To be used in *Any* placeholders.
    ArrayOfInvalidOperationOnSecondaryVm(Vec<InvalidOperationOnSecondaryVm>),
    /// A boxed array of *InvalidPowerState*. To be used in *Any* placeholders.
    ArrayOfInvalidPowerState(Vec<InvalidPowerState>),
    /// A boxed array of *InvalidPrivilege*. To be used in *Any* placeholders.
    ArrayOfInvalidPrivilege(Vec<InvalidPrivilege>),
    /// A boxed array of *InvalidProfileReferenceHost*. To be used in *Any* placeholders.
    ArrayOfInvalidProfileReferenceHost(Vec<InvalidProfileReferenceHost>),
    /// A boxed array of *InvalidPropertyType*. To be used in *Any* placeholders.
    ArrayOfInvalidPropertyType(Vec<InvalidPropertyType>),
    /// A boxed array of *InvalidPropertyValue*. To be used in *Any* placeholders.
    ArrayOfInvalidPropertyValue(Vec<Box<dyn super::traits::InvalidPropertyValueTrait>>),
    /// A boxed array of *InvalidResourcePoolStructureFault*. To be used in *Any* placeholders.
    ArrayOfInvalidResourcePoolStructureFault(Vec<InvalidResourcePoolStructureFault>),
    /// A boxed array of *InvalidScheduledTask*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    ArrayOfInvalidScheduledTask(Vec<InvalidScheduledTask>),
    /// A boxed array of *InvalidSnapshotFormat*. To be used in *Any* placeholders.
    ArrayOfInvalidSnapshotFormat(Vec<InvalidSnapshotFormat>),
    /// A boxed array of *InvalidState*. To be used in *Any* placeholders.
    ArrayOfInvalidState(Vec<Box<dyn super::traits::InvalidStateTrait>>),
    /// A boxed array of *InvalidVmConfig*. To be used in *Any* placeholders.
    ArrayOfInvalidVmConfig(Vec<Box<dyn super::traits::InvalidVmConfigTrait>>),
    /// A boxed array of *InvalidVmState*. To be used in *Any* placeholders.
    ArrayOfInvalidVmState(Vec<InvalidVmState>),
    /// A boxed array of *InventoryHasStandardAloneHosts*. To be used in *Any* placeholders.
    ArrayOfInventoryHasStandardAloneHosts(Vec<InventoryHasStandardAloneHosts>),
    /// A boxed array of *IpHostnameGeneratorError*. To be used in *Any* placeholders.
    ArrayOfIpHostnameGeneratorError(Vec<IpHostnameGeneratorError>),
    /// A boxed array of *IscsiFault*. To be used in *Any* placeholders.
    ArrayOfIscsiFault(Vec<Box<dyn super::traits::IscsiFaultTrait>>),
    /// A boxed array of *IscsiFaultInvalidVnic*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultInvalidVnic(Vec<IscsiFaultInvalidVnic>),
    /// A boxed array of *IscsiFaultPnicInUse*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultPnicInUse(Vec<IscsiFaultPnicInUse>),
    /// A boxed array of *IscsiFaultVnicAlreadyBound*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicAlreadyBound(Vec<IscsiFaultVnicAlreadyBound>),
    /// A boxed array of *IscsiFaultVnicHasActivePaths*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicHasActivePaths(Vec<IscsiFaultVnicHasActivePaths>),
    /// A boxed array of *IscsiFaultVnicHasMultipleUplinks*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicHasMultipleUplinks(Vec<IscsiFaultVnicHasMultipleUplinks>),
    /// A boxed array of *IscsiFaultVnicHasNoUplinks*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicHasNoUplinks(Vec<IscsiFaultVnicHasNoUplinks>),
    /// A boxed array of *IscsiFaultVnicHasWrongUplink*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicHasWrongUplink(Vec<IscsiFaultVnicHasWrongUplink>),
    /// A boxed array of *IscsiFaultVnicInUse*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicInUse(Vec<IscsiFaultVnicInUse>),
    /// A boxed array of *IscsiFaultVnicIsLastPath*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicIsLastPath(Vec<IscsiFaultVnicIsLastPath>),
    /// A boxed array of *IscsiFaultVnicNotBound*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicNotBound(Vec<IscsiFaultVnicNotBound>),
    /// A boxed array of *IscsiFaultVnicNotFound*. To be used in *Any* placeholders.
    ArrayOfIscsiFaultVnicNotFound(Vec<IscsiFaultVnicNotFound>),
    /// A boxed array of *KeyNotFound*. To be used in *Any* placeholders.
    ArrayOfKeyNotFound(Vec<KeyNotFound>),
    /// A boxed array of *LargeRDMConversionNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfLargeRDMConversionNotSupported")]
    ArrayOfLargeRdmConversionNotSupported(Vec<LargeRdmConversionNotSupported>),
    /// A boxed array of *LargeRDMNotSupportedOnDatastore*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfLargeRDMNotSupportedOnDatastore")]
    ArrayOfLargeRdmNotSupportedOnDatastore(Vec<LargeRdmNotSupportedOnDatastore>),
    /// A boxed array of *LegacyNetworkInterfaceInUse*. To be used in *Any* placeholders.
    ArrayOfLegacyNetworkInterfaceInUse(Vec<LegacyNetworkInterfaceInUse>),
    /// A boxed array of *LicenseAssignmentFailed*. To be used in *Any* placeholders.
    ArrayOfLicenseAssignmentFailed(Vec<LicenseAssignmentFailed>),
    /// A boxed array of *LicenseDowngradeDisallowed*. To be used in *Any* placeholders.
    ArrayOfLicenseDowngradeDisallowed(Vec<LicenseDowngradeDisallowed>),
    /// A boxed array of *LicenseEntityNotFound*. To be used in *Any* placeholders.
    ArrayOfLicenseEntityNotFound(Vec<LicenseEntityNotFound>),
    /// A boxed array of *LicenseExpired*. To be used in *Any* placeholders.
    ArrayOfLicenseExpired(Vec<LicenseExpired>),
    /// A boxed array of *LicenseKeyEntityMismatch*. To be used in *Any* placeholders.
    ArrayOfLicenseKeyEntityMismatch(Vec<LicenseKeyEntityMismatch>),
    /// A boxed array of *LicenseRestricted*. To be used in *Any* placeholders.
    ArrayOfLicenseRestricted(Vec<LicenseRestricted>),
    /// A boxed array of *LicenseServerUnavailable*. To be used in *Any* placeholders.
    ArrayOfLicenseServerUnavailable(Vec<LicenseServerUnavailable>),
    /// A boxed array of *LicenseSourceUnavailable*. To be used in *Any* placeholders.
    ArrayOfLicenseSourceUnavailable(Vec<LicenseSourceUnavailable>),
    /// A boxed array of *LimitExceeded*. To be used in *Any* placeholders.
    ArrayOfLimitExceeded(Vec<LimitExceeded>),
    /// A boxed array of *LinuxVolumeNotClean*. To be used in *Any* placeholders.
    ArrayOfLinuxVolumeNotClean(Vec<LinuxVolumeNotClean>),
    /// A boxed array of *LogBundlingFailed*. To be used in *Any* placeholders.
    ArrayOfLogBundlingFailed(Vec<LogBundlingFailed>),
    /// A boxed array of *MaintenanceModeFileMove*. To be used in *Any* placeholders.
    ArrayOfMaintenanceModeFileMove(Vec<MaintenanceModeFileMove>),
    /// A boxed array of *MemoryFileFormatNotSupportedByDatastore*. To be used in *Any* placeholders.
    ArrayOfMemoryFileFormatNotSupportedByDatastore(Vec<MemoryFileFormatNotSupportedByDatastore>),
    /// A boxed array of *MemoryHotPlugNotSupported*. To be used in *Any* placeholders.
    ArrayOfMemoryHotPlugNotSupported(Vec<MemoryHotPlugNotSupported>),
    /// A boxed array of *MemorySizeNotRecommended*. To be used in *Any* placeholders.
    ArrayOfMemorySizeNotRecommended(Vec<MemorySizeNotRecommended>),
    /// A boxed array of *MemorySizeNotSupported*. To be used in *Any* placeholders.
    ArrayOfMemorySizeNotSupported(Vec<MemorySizeNotSupported>),
    /// A boxed array of *MemorySizeNotSupportedByDatastore*. To be used in *Any* placeholders.
    ArrayOfMemorySizeNotSupportedByDatastore(Vec<MemorySizeNotSupportedByDatastore>),
    /// A boxed array of *MemorySnapshotOnIndependentDisk*. To be used in *Any* placeholders.
    ArrayOfMemorySnapshotOnIndependentDisk(Vec<MemorySnapshotOnIndependentDisk>),
    /// A boxed array of *MethodAlreadyDisabledFault*. To be used in *Any* placeholders.
    ArrayOfMethodAlreadyDisabledFault(Vec<MethodAlreadyDisabledFault>),
    /// A boxed array of *MethodDisabled*. To be used in *Any* placeholders.
    ArrayOfMethodDisabled(Vec<MethodDisabled>),
    /// A boxed array of *MigrationDisabled*. To be used in *Any* placeholders.
    ArrayOfMigrationDisabled(Vec<MigrationDisabled>),
    /// A boxed array of *MigrationFault*. To be used in *Any* placeholders.
    ArrayOfMigrationFault(Vec<Box<dyn super::traits::MigrationFaultTrait>>),
    /// A boxed array of *MigrationFeatureNotSupported*. To be used in *Any* placeholders.
    ArrayOfMigrationFeatureNotSupported(Vec<Box<dyn super::traits::MigrationFeatureNotSupportedTrait>>),
    /// A boxed array of *MigrationNotReady*. To be used in *Any* placeholders.
    ArrayOfMigrationNotReady(Vec<MigrationNotReady>),
    /// A boxed array of *MismatchedBundle*. To be used in *Any* placeholders.
    ArrayOfMismatchedBundle(Vec<MismatchedBundle>),
    /// A boxed array of *MismatchedNetworkPolicies*. To be used in *Any* placeholders.
    ArrayOfMismatchedNetworkPolicies(Vec<MismatchedNetworkPolicies>),
    /// A boxed array of *MismatchedVMotionNetworkNames*. To be used in *Any* placeholders.
    ArrayOfMismatchedVMotionNetworkNames(Vec<MismatchedVMotionNetworkNames>),
    /// A boxed array of *MissingBmcSupport*. To be used in *Any* placeholders.
    ArrayOfMissingBmcSupport(Vec<MissingBmcSupport>),
    /// A boxed array of *MissingController*. To be used in *Any* placeholders.
    ArrayOfMissingController(Vec<MissingController>),
    /// A boxed array of *MissingIpPool*. To be used in *Any* placeholders.
    ArrayOfMissingIpPool(Vec<MissingIpPool>),
    /// A boxed array of *MissingLinuxCustResources*. To be used in *Any* placeholders.
    ArrayOfMissingLinuxCustResources(Vec<MissingLinuxCustResources>),
    /// A boxed array of *MissingNetworkIpConfig*. To be used in *Any* placeholders.
    ArrayOfMissingNetworkIpConfig(Vec<MissingNetworkIpConfig>),
    /// A boxed array of *MissingPowerOffConfiguration*. To be used in *Any* placeholders.
    ArrayOfMissingPowerOffConfiguration(Vec<MissingPowerOffConfiguration>),
    /// A boxed array of *MissingPowerOnConfiguration*. To be used in *Any* placeholders.
    ArrayOfMissingPowerOnConfiguration(Vec<MissingPowerOnConfiguration>),
    /// A boxed array of *MissingWindowsCustResources*. To be used in *Any* placeholders.
    ArrayOfMissingWindowsCustResources(Vec<MissingWindowsCustResources>),
    /// A boxed array of *MksConnectionLimitReached*. To be used in *Any* placeholders.
    ArrayOfMksConnectionLimitReached(Vec<MksConnectionLimitReached>),
    /// A boxed array of *MountError*. To be used in *Any* placeholders.
    ArrayOfMountError(Vec<MountError>),
    /// A boxed array of *MultiWriterNotSupported*. To be used in *Any* placeholders.
    ArrayOfMultiWriterNotSupported(Vec<MultiWriterNotSupported>),
    /// A boxed array of *MultipleCertificatesVerifyFault*. To be used in *Any* placeholders.
    ArrayOfMultipleCertificatesVerifyFault(Vec<MultipleCertificatesVerifyFault>),
    /// A boxed array of *MultipleCertificatesVerifyFaultThumbprintData*. To be used in *Any* placeholders.
    ArrayOfMultipleCertificatesVerifyFaultThumbprintData(Vec<MultipleCertificatesVerifyFaultThumbprintData>),
    /// A boxed array of *MultipleSnapshotsNotSupported*. To be used in *Any* placeholders.
    ArrayOfMultipleSnapshotsNotSupported(Vec<MultipleSnapshotsNotSupported>),
    /// A boxed array of *NamespaceFull*. To be used in *Any* placeholders.
    ArrayOfNamespaceFull(Vec<NamespaceFull>),
    /// A boxed array of *NamespaceLimitReached*. To be used in *Any* placeholders.
    ArrayOfNamespaceLimitReached(Vec<NamespaceLimitReached>),
    /// A boxed array of *NamespaceWriteProtected*. To be used in *Any* placeholders.
    ArrayOfNamespaceWriteProtected(Vec<NamespaceWriteProtected>),
    /// A boxed array of *NasConfigFault*. To be used in *Any* placeholders.
    ArrayOfNasConfigFault(Vec<Box<dyn super::traits::NasConfigFaultTrait>>),
    /// A boxed array of *NasConnectionLimitReached*. To be used in *Any* placeholders.
    ArrayOfNasConnectionLimitReached(Vec<NasConnectionLimitReached>),
    /// A boxed array of *NasSessionCredentialConflict*. To be used in *Any* placeholders.
    ArrayOfNasSessionCredentialConflict(Vec<NasSessionCredentialConflict>),
    /// A boxed array of *NasVolumeNotMounted*. To be used in *Any* placeholders.
    ArrayOfNasVolumeNotMounted(Vec<NasVolumeNotMounted>),
    /// A boxed array of *NetworkCopyFault*. To be used in *Any* placeholders.
    ArrayOfNetworkCopyFault(Vec<NetworkCopyFault>),
    /// A boxed array of *NetworkDisruptedAndConfigRolledBack*. To be used in *Any* placeholders.
    ArrayOfNetworkDisruptedAndConfigRolledBack(Vec<NetworkDisruptedAndConfigRolledBack>),
    /// A boxed array of *NetworkInaccessible*. To be used in *Any* placeholders.
    ArrayOfNetworkInaccessible(Vec<NetworkInaccessible>),
    /// A boxed array of *NetworksMayNotBeTheSame*. To be used in *Any* placeholders.
    ArrayOfNetworksMayNotBeTheSame(Vec<NetworksMayNotBeTheSame>),
    /// A boxed array of *NicSettingMismatch*. To be used in *Any* placeholders.
    ArrayOfNicSettingMismatch(Vec<NicSettingMismatch>),
    /// A boxed array of *NoActiveHostInCluster*. To be used in *Any* placeholders.
    ArrayOfNoActiveHostInCluster(Vec<NoActiveHostInCluster>),
    /// A boxed array of *NoAvailableIp*. To be used in *Any* placeholders.
    ArrayOfNoAvailableIp(Vec<NoAvailableIp>),
    /// A boxed array of *NoClientCertificate*. To be used in *Any* placeholders.
    ArrayOfNoClientCertificate(Vec<NoClientCertificate>),
    /// A boxed array of *NoCompatibleDatastore*. To be used in *Any* placeholders.
    ArrayOfNoCompatibleDatastore(Vec<NoCompatibleDatastore>),
    /// A boxed array of *NoCompatibleHardAffinityHost*. To be used in *Any* placeholders.
    ArrayOfNoCompatibleHardAffinityHost(Vec<NoCompatibleHardAffinityHost>),
    /// A boxed array of *NoCompatibleHost*. To be used in *Any* placeholders.
    ArrayOfNoCompatibleHost(Vec<Box<dyn super::traits::NoCompatibleHostTrait>>),
    /// A boxed array of *NoCompatibleHostWithAccessToDevice*. To be used in *Any* placeholders.
    ArrayOfNoCompatibleHostWithAccessToDevice(Vec<NoCompatibleHostWithAccessToDevice>),
    /// A boxed array of *NoCompatibleSoftAffinityHost*. To be used in *Any* placeholders.
    ArrayOfNoCompatibleSoftAffinityHost(Vec<NoCompatibleSoftAffinityHost>),
    /// A boxed array of *NoConnectedDatastore*. To be used in *Any* placeholders.
    ArrayOfNoConnectedDatastore(Vec<NoConnectedDatastore>),
    /// A boxed array of *NoDiskFound*. To be used in *Any* placeholders.
    ArrayOfNoDiskFound(Vec<NoDiskFound>),
    /// A boxed array of *NoDiskSpace*. To be used in *Any* placeholders.
    ArrayOfNoDiskSpace(Vec<NoDiskSpace>),
    /// A boxed array of *NoDisksToCustomize*. To be used in *Any* placeholders.
    ArrayOfNoDisksToCustomize(Vec<NoDisksToCustomize>),
    /// A boxed array of *NoGateway*. To be used in *Any* placeholders.
    ArrayOfNoGateway(Vec<NoGateway>),
    /// A boxed array of *NoGuestHeartbeat*. To be used in *Any* placeholders.
    ArrayOfNoGuestHeartbeat(Vec<NoGuestHeartbeat>),
    /// A boxed array of *NoHost*. To be used in *Any* placeholders.
    ArrayOfNoHost(Vec<NoHost>),
    /// A boxed array of *NoHostSuitableForFtSecondary*. To be used in *Any* placeholders.
    ArrayOfNoHostSuitableForFtSecondary(Vec<NoHostSuitableForFtSecondary>),
    /// A boxed array of *NoLicenseServerConfigured*. To be used in *Any* placeholders.
    ArrayOfNoLicenseServerConfigured(Vec<NoLicenseServerConfigured>),
    /// A boxed array of *NoPeerHostFound*. To be used in *Any* placeholders.
    ArrayOfNoPeerHostFound(Vec<NoPeerHostFound>),
    /// A boxed array of *NoPermission*. To be used in *Any* placeholders.
    ArrayOfNoPermission(Vec<Box<dyn super::traits::NoPermissionTrait>>),
    /// A boxed array of *NoPermissionEntityPrivileges*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.2
    ArrayOfNoPermissionEntityPrivileges(Vec<NoPermissionEntityPrivileges>),
    /// A boxed array of *NoPermissionOnAD*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNoPermissionOnAD")]
    ArrayOfNoPermissionOnAd(Vec<NoPermissionOnAd>),
    /// A boxed array of *NoPermissionOnHost*. To be used in *Any* placeholders.
    ArrayOfNoPermissionOnHost(Vec<NoPermissionOnHost>),
    /// A boxed array of *NoPermissionOnNasVolume*. To be used in *Any* placeholders.
    ArrayOfNoPermissionOnNasVolume(Vec<NoPermissionOnNasVolume>),
    /// A boxed array of *NoSubjectName*. To be used in *Any* placeholders.
    ArrayOfNoSubjectName(Vec<NoSubjectName>),
    /// A boxed array of *NoVcManagedIpConfigured*. To be used in *Any* placeholders.
    ArrayOfNoVcManagedIpConfigured(Vec<NoVcManagedIpConfigured>),
    /// A boxed array of *NoVirtualNic*. To be used in *Any* placeholders.
    ArrayOfNoVirtualNic(Vec<NoVirtualNic>),
    /// A boxed array of *NoVmInVApp*. To be used in *Any* placeholders.
    ArrayOfNoVmInVApp(Vec<NoVmInVApp>),
    /// A boxed array of *NonADUserRequired*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNonADUserRequired")]
    ArrayOfNonAdUserRequired(Vec<NonAdUserRequired>),
    /// A boxed array of *NonHomeRDMVMotionNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNonHomeRDMVMotionNotSupported")]
    ArrayOfNonHomeRdmvMotionNotSupported(Vec<NonHomeRdmvMotionNotSupported>),
    /// A boxed array of *NonPersistentDisksNotSupported*. To be used in *Any* placeholders.
    ArrayOfNonPersistentDisksNotSupported(Vec<NonPersistentDisksNotSupported>),
    /// A boxed array of *NonVmwareOuiMacNotSupportedHost*. To be used in *Any* placeholders.
    ArrayOfNonVmwareOuiMacNotSupportedHost(Vec<NonVmwareOuiMacNotSupportedHost>),
    /// A boxed array of *NotADirectory*. To be used in *Any* placeholders.
    ArrayOfNotADirectory(Vec<NotADirectory>),
    /// A boxed array of *NotAFile*. To be used in *Any* placeholders.
    ArrayOfNotAFile(Vec<NotAFile>),
    /// A boxed array of *NotAuthenticated*. To be used in *Any* placeholders.
    ArrayOfNotAuthenticated(Vec<NotAuthenticated>),
    /// A boxed array of *NotEnoughCpus*. To be used in *Any* placeholders.
    ArrayOfNotEnoughCpus(Vec<Box<dyn super::traits::NotEnoughCpusTrait>>),
    /// A boxed array of *NotEnoughLogicalCpus*. To be used in *Any* placeholders.
    ArrayOfNotEnoughLogicalCpus(Vec<NotEnoughLogicalCpus>),
    /// A boxed array of *NotFound*. To be used in *Any* placeholders.
    ArrayOfNotFound(Vec<NotFound>),
    /// A boxed array of *NotSupportedDeviceForFT*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNotSupportedDeviceForFT")]
    ArrayOfNotSupportedDeviceForFt(Vec<NotSupportedDeviceForFt>),
    /// A boxed array of *NotSupportedHost*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHost(Vec<Box<dyn super::traits::NotSupportedHostTrait>>),
    /// A boxed array of *NotSupportedHostForChecksum*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostForChecksum(Vec<NotSupportedHostForChecksum>),
    /// A boxed array of *NotSupportedHostForVFlash*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostForVFlash(Vec<NotSupportedHostForVFlash>),
    /// A boxed array of *NotSupportedHostForVmcp*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostForVmcp(Vec<NotSupportedHostForVmcp>),
    /// A boxed array of *NotSupportedHostForVmemFile*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostForVmemFile(Vec<NotSupportedHostForVmemFile>),
    /// A boxed array of *NotSupportedHostForVsan*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostForVsan(Vec<NotSupportedHostForVsan>),
    /// A boxed array of *NotSupportedHostInCluster*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostInCluster(Vec<Box<dyn super::traits::NotSupportedHostInClusterTrait>>),
    /// A boxed array of *NotSupportedHostInDvs*. To be used in *Any* placeholders.
    ArrayOfNotSupportedHostInDvs(Vec<NotSupportedHostInDvs>),
    /// A boxed array of *NotSupportedHostInHACluster*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNotSupportedHostInHACluster")]
    ArrayOfNotSupportedHostInHaCluster(Vec<NotSupportedHostInHaCluster>),
    /// A boxed array of *NotUserConfigurableProperty*. To be used in *Any* placeholders.
    ArrayOfNotUserConfigurableProperty(Vec<NotUserConfigurableProperty>),
    /// A boxed array of *NumVirtualCoresPerSocketNotSupported*. To be used in *Any* placeholders.
    ArrayOfNumVirtualCoresPerSocketNotSupported(Vec<NumVirtualCoresPerSocketNotSupported>),
    /// A boxed array of *NumVirtualCpusExceedsLimit*. To be used in *Any* placeholders.
    ArrayOfNumVirtualCpusExceedsLimit(Vec<NumVirtualCpusExceedsLimit>),
    /// A boxed array of *NumVirtualCpusIncompatible*. To be used in *Any* placeholders.
    ArrayOfNumVirtualCpusIncompatible(Vec<NumVirtualCpusIncompatible>),
    /// A boxed array of *NumVirtualCpusNotSupported*. To be used in *Any* placeholders.
    ArrayOfNumVirtualCpusNotSupported(Vec<NumVirtualCpusNotSupported>),
    /// A boxed array of *OperationDisabledByGuest*. To be used in *Any* placeholders.
    ArrayOfOperationDisabledByGuest(Vec<OperationDisabledByGuest>),
    /// A boxed array of *OperationDisallowedOnHost*. To be used in *Any* placeholders.
    ArrayOfOperationDisallowedOnHost(Vec<OperationDisallowedOnHost>),
    /// A boxed array of *OperationNotSupportedByGuest*. To be used in *Any* placeholders.
    ArrayOfOperationNotSupportedByGuest(Vec<OperationNotSupportedByGuest>),
    /// A boxed array of *OutOfBounds*. To be used in *Any* placeholders.
    ArrayOfOutOfBounds(Vec<OutOfBounds>),
    /// A boxed array of *OvfAttribute*. To be used in *Any* placeholders.
    ArrayOfOvfAttribute(Vec<Box<dyn super::traits::OvfAttributeTrait>>),
    /// A boxed array of *OvfConnectedDevice*. To be used in *Any* placeholders.
    ArrayOfOvfConnectedDevice(Vec<Box<dyn super::traits::OvfConnectedDeviceTrait>>),
    /// A boxed array of *OvfConnectedDeviceFloppy*. To be used in *Any* placeholders.
    ArrayOfOvfConnectedDeviceFloppy(Vec<OvfConnectedDeviceFloppy>),
    /// A boxed array of *OvfConnectedDeviceIso*. To be used in *Any* placeholders.
    ArrayOfOvfConnectedDeviceIso(Vec<OvfConnectedDeviceIso>),
    /// A boxed array of *OvfConstraint*. To be used in *Any* placeholders.
    ArrayOfOvfConstraint(Vec<Box<dyn super::traits::OvfConstraintTrait>>),
    /// A boxed array of *OvfConsumerCallbackFault*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerCallbackFault(Vec<Box<dyn super::traits::OvfConsumerCallbackFaultTrait>>),
    /// A boxed array of *OvfConsumerCommunicationError*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerCommunicationError(Vec<OvfConsumerCommunicationError>),
    /// A boxed array of *OvfConsumerFault*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerFault(Vec<OvfConsumerFault>),
    /// A boxed array of *OvfConsumerInvalidSection*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerInvalidSection(Vec<OvfConsumerInvalidSection>),
    /// A boxed array of *OvfConsumerPowerOnFault*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerPowerOnFault(Vec<OvfConsumerPowerOnFault>),
    /// A boxed array of *OvfConsumerUndeclaredSection*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerUndeclaredSection(Vec<OvfConsumerUndeclaredSection>),
    /// A boxed array of *OvfConsumerUndefinedPrefix*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerUndefinedPrefix(Vec<OvfConsumerUndefinedPrefix>),
    /// A boxed array of *OvfConsumerValidationFault*. To be used in *Any* placeholders.
    ArrayOfOvfConsumerValidationFault(Vec<OvfConsumerValidationFault>),
    /// A boxed array of *OvfCpuCompatibility*. To be used in *Any* placeholders.
    ArrayOfOvfCpuCompatibility(Vec<OvfCpuCompatibility>),
    /// A boxed array of *OvfCpuCompatibilityCheckNotSupported*. To be used in *Any* placeholders.
    ArrayOfOvfCpuCompatibilityCheckNotSupported(Vec<OvfCpuCompatibilityCheckNotSupported>),
    /// A boxed array of *OvfDiskMappingNotFound*. To be used in *Any* placeholders.
    ArrayOfOvfDiskMappingNotFound(Vec<OvfDiskMappingNotFound>),
    /// A boxed array of *OvfDiskOrderConstraint*. To be used in *Any* placeholders.
    ArrayOfOvfDiskOrderConstraint(Vec<OvfDiskOrderConstraint>),
    /// A boxed array of *OvfDuplicateElement*. To be used in *Any* placeholders.
    ArrayOfOvfDuplicateElement(Vec<OvfDuplicateElement>),
    /// A boxed array of *OvfDuplicatedElementBoundary*. To be used in *Any* placeholders.
    ArrayOfOvfDuplicatedElementBoundary(Vec<OvfDuplicatedElementBoundary>),
    /// A boxed array of *OvfDuplicatedPropertyIdExport*. To be used in *Any* placeholders.
    ArrayOfOvfDuplicatedPropertyIdExport(Vec<OvfDuplicatedPropertyIdExport>),
    /// A boxed array of *OvfDuplicatedPropertyIdImport*. To be used in *Any* placeholders.
    ArrayOfOvfDuplicatedPropertyIdImport(Vec<OvfDuplicatedPropertyIdImport>),
    /// A boxed array of *OvfElement*. To be used in *Any* placeholders.
    ArrayOfOvfElement(Vec<Box<dyn super::traits::OvfElementTrait>>),
    /// A boxed array of *OvfElementInvalidValue*. To be used in *Any* placeholders.
    ArrayOfOvfElementInvalidValue(Vec<OvfElementInvalidValue>),
    /// A boxed array of *OvfExport*. To be used in *Any* placeholders.
    ArrayOfOvfExport(Vec<Box<dyn super::traits::OvfExportTrait>>),
    /// A boxed array of *OvfExportFailed*. To be used in *Any* placeholders.
    ArrayOfOvfExportFailed(Vec<OvfExportFailed>),
    /// A boxed array of *OvfFault*. To be used in *Any* placeholders.
    ArrayOfOvfFault(Vec<Box<dyn super::traits::OvfFaultTrait>>),
    /// A boxed array of *OvfHardwareCheck*. To be used in *Any* placeholders.
    ArrayOfOvfHardwareCheck(Vec<OvfHardwareCheck>),
    /// A boxed array of *OvfHardwareExport*. To be used in *Any* placeholders.
    ArrayOfOvfHardwareExport(Vec<Box<dyn super::traits::OvfHardwareExportTrait>>),
    /// A boxed array of *OvfHostResourceConstraint*. To be used in *Any* placeholders.
    ArrayOfOvfHostResourceConstraint(Vec<OvfHostResourceConstraint>),
    /// A boxed array of *OvfHostValueNotParsed*. To be used in *Any* placeholders.
    ArrayOfOvfHostValueNotParsed(Vec<OvfHostValueNotParsed>),
    /// A boxed array of *OvfImport*. To be used in *Any* placeholders.
    ArrayOfOvfImport(Vec<Box<dyn super::traits::OvfImportTrait>>),
    /// A boxed array of *OvfImportFailed*. To be used in *Any* placeholders.
    ArrayOfOvfImportFailed(Vec<OvfImportFailed>),
    /// A boxed array of *OvfInternalError*. To be used in *Any* placeholders.
    ArrayOfOvfInternalError(Vec<OvfInternalError>),
    /// A boxed array of *OvfInvalidPackage*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidPackage(Vec<Box<dyn super::traits::OvfInvalidPackageTrait>>),
    /// A boxed array of *OvfInvalidValue*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidValue(Vec<Box<dyn super::traits::OvfInvalidValueTrait>>),
    /// A boxed array of *OvfInvalidValueConfiguration*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidValueConfiguration(Vec<OvfInvalidValueConfiguration>),
    /// A boxed array of *OvfInvalidValueEmpty*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidValueEmpty(Vec<OvfInvalidValueEmpty>),
    /// A boxed array of *OvfInvalidValueFormatMalformed*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidValueFormatMalformed(Vec<OvfInvalidValueFormatMalformed>),
    /// A boxed array of *OvfInvalidValueReference*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidValueReference(Vec<OvfInvalidValueReference>),
    /// A boxed array of *OvfInvalidVmName*. To be used in *Any* placeholders.
    ArrayOfOvfInvalidVmName(Vec<OvfInvalidVmName>),
    /// A boxed array of *OvfMappedOsId*. To be used in *Any* placeholders.
    ArrayOfOvfMappedOsId(Vec<OvfMappedOsId>),
    /// A boxed array of *OvfMissingAttribute*. To be used in *Any* placeholders.
    ArrayOfOvfMissingAttribute(Vec<OvfMissingAttribute>),
    /// A boxed array of *OvfMissingElement*. To be used in *Any* placeholders.
    ArrayOfOvfMissingElement(Vec<Box<dyn super::traits::OvfMissingElementTrait>>),
    /// A boxed array of *OvfMissingElementNormalBoundary*. To be used in *Any* placeholders.
    ArrayOfOvfMissingElementNormalBoundary(Vec<OvfMissingElementNormalBoundary>),
    /// A boxed array of *OvfMissingHardware*. To be used in *Any* placeholders.
    ArrayOfOvfMissingHardware(Vec<OvfMissingHardware>),
    /// A boxed array of *OvfNetworkMappingNotSupported*. To be used in *Any* placeholders.
    ArrayOfOvfNetworkMappingNotSupported(Vec<OvfNetworkMappingNotSupported>),
    /// A boxed array of *OvfNoHostNic*. To be used in *Any* placeholders.
    ArrayOfOvfNoHostNic(Vec<OvfNoHostNic>),
    /// A boxed array of *OvfNoSpaceOnController*. To be used in *Any* placeholders.
    ArrayOfOvfNoSpaceOnController(Vec<OvfNoSpaceOnController>),
    /// A boxed array of *OvfNoSupportedHardwareFamily*. To be used in *Any* placeholders.
    ArrayOfOvfNoSupportedHardwareFamily(Vec<OvfNoSupportedHardwareFamily>),
    /// A boxed array of *OvfProperty*. To be used in *Any* placeholders.
    ArrayOfOvfProperty(Vec<Box<dyn super::traits::OvfPropertyTrait>>),
    /// A boxed array of *OvfPropertyExport*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyExport(Vec<OvfPropertyExport>),
    /// A boxed array of *OvfPropertyNetwork*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyNetwork(Vec<OvfPropertyNetwork>),
    /// A boxed array of *OvfPropertyNetworkExport*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyNetworkExport(Vec<OvfPropertyNetworkExport>),
    /// A boxed array of *OvfPropertyQualifier*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyQualifier(Vec<OvfPropertyQualifier>),
    /// A boxed array of *OvfPropertyQualifierDuplicate*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyQualifierDuplicate(Vec<OvfPropertyQualifierDuplicate>),
    /// A boxed array of *OvfPropertyQualifierIgnored*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyQualifierIgnored(Vec<OvfPropertyQualifierIgnored>),
    /// A boxed array of *OvfPropertyType*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyType(Vec<OvfPropertyType>),
    /// A boxed array of *OvfPropertyValue*. To be used in *Any* placeholders.
    ArrayOfOvfPropertyValue(Vec<OvfPropertyValue>),
    /// A boxed array of *OvfSystemFault*. To be used in *Any* placeholders.
    ArrayOfOvfSystemFault(Vec<Box<dyn super::traits::OvfSystemFaultTrait>>),
    /// A boxed array of *OvfToXmlUnsupportedElement*. To be used in *Any* placeholders.
    ArrayOfOvfToXmlUnsupportedElement(Vec<OvfToXmlUnsupportedElement>),
    /// A boxed array of *OvfUnableToExportDisk*. To be used in *Any* placeholders.
    ArrayOfOvfUnableToExportDisk(Vec<OvfUnableToExportDisk>),
    /// A boxed array of *OvfUnexpectedElement*. To be used in *Any* placeholders.
    ArrayOfOvfUnexpectedElement(Vec<OvfUnexpectedElement>),
    /// A boxed array of *OvfUnknownDevice*. To be used in *Any* placeholders.
    ArrayOfOvfUnknownDevice(Vec<OvfUnknownDevice>),
    /// A boxed array of *OvfUnknownDeviceBacking*. To be used in *Any* placeholders.
    ArrayOfOvfUnknownDeviceBacking(Vec<OvfUnknownDeviceBacking>),
    /// A boxed array of *OvfUnknownEntity*. To be used in *Any* placeholders.
    ArrayOfOvfUnknownEntity(Vec<OvfUnknownEntity>),
    /// A boxed array of *OvfUnsupportedAttribute*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedAttribute(Vec<Box<dyn super::traits::OvfUnsupportedAttributeTrait>>),
    /// A boxed array of *OvfUnsupportedAttributeValue*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedAttributeValue(Vec<OvfUnsupportedAttributeValue>),
    /// A boxed array of *OvfUnsupportedDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedDeviceBackingInfo(Vec<OvfUnsupportedDeviceBackingInfo>),
    /// A boxed array of *OvfUnsupportedDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedDeviceBackingOption(Vec<OvfUnsupportedDeviceBackingOption>),
    /// A boxed array of *OvfUnsupportedDeviceExport*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedDeviceExport(Vec<OvfUnsupportedDeviceExport>),
    /// A boxed array of *OvfUnsupportedDiskProvisioning*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedDiskProvisioning(Vec<OvfUnsupportedDiskProvisioning>),
    /// A boxed array of *OvfUnsupportedElement*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedElement(Vec<Box<dyn super::traits::OvfUnsupportedElementTrait>>),
    /// A boxed array of *OvfUnsupportedElementValue*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedElementValue(Vec<OvfUnsupportedElementValue>),
    /// A boxed array of *OvfUnsupportedPackage*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedPackage(Vec<Box<dyn super::traits::OvfUnsupportedPackageTrait>>),
    /// A boxed array of *OvfUnsupportedSection*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedSection(Vec<OvfUnsupportedSection>),
    /// A boxed array of *OvfUnsupportedSubType*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedSubType(Vec<OvfUnsupportedSubType>),
    /// A boxed array of *OvfUnsupportedType*. To be used in *Any* placeholders.
    ArrayOfOvfUnsupportedType(Vec<OvfUnsupportedType>),
    /// A boxed array of *OvfWrongElement*. To be used in *Any* placeholders.
    ArrayOfOvfWrongElement(Vec<OvfWrongElement>),
    /// A boxed array of *OvfWrongNamespace*. To be used in *Any* placeholders.
    ArrayOfOvfWrongNamespace(Vec<OvfWrongNamespace>),
    /// A boxed array of *OvfXmlFormat*. To be used in *Any* placeholders.
    ArrayOfOvfXmlFormat(Vec<OvfXmlFormat>),
    /// A boxed array of *PasswordExpired*. To be used in *Any* placeholders.
    ArrayOfPasswordExpired(Vec<PasswordExpired>),
    /// A boxed array of *PatchAlreadyInstalled*. To be used in *Any* placeholders.
    ArrayOfPatchAlreadyInstalled(Vec<PatchAlreadyInstalled>),
    /// A boxed array of *PatchBinariesNotFound*. To be used in *Any* placeholders.
    ArrayOfPatchBinariesNotFound(Vec<PatchBinariesNotFound>),
    /// A boxed array of *PatchInstallFailed*. To be used in *Any* placeholders.
    ArrayOfPatchInstallFailed(Vec<PatchInstallFailed>),
    /// A boxed array of *PatchIntegrityError*. To be used in *Any* placeholders.
    ArrayOfPatchIntegrityError(Vec<PatchIntegrityError>),
    /// A boxed array of *PatchMetadataCorrupted*. To be used in *Any* placeholders.
    ArrayOfPatchMetadataCorrupted(Vec<PatchMetadataCorrupted>),
    /// A boxed array of *PatchMetadataInvalid*. To be used in *Any* placeholders.
    ArrayOfPatchMetadataInvalid(Vec<Box<dyn super::traits::PatchMetadataInvalidTrait>>),
    /// A boxed array of *PatchMetadataNotFound*. To be used in *Any* placeholders.
    ArrayOfPatchMetadataNotFound(Vec<PatchMetadataNotFound>),
    /// A boxed array of *PatchMissingDependencies*. To be used in *Any* placeholders.
    ArrayOfPatchMissingDependencies(Vec<PatchMissingDependencies>),
    /// A boxed array of *PatchNotApplicable*. To be used in *Any* placeholders.
    ArrayOfPatchNotApplicable(Vec<Box<dyn super::traits::PatchNotApplicableTrait>>),
    /// A boxed array of *PatchSuperseded*. To be used in *Any* placeholders.
    ArrayOfPatchSuperseded(Vec<PatchSuperseded>),
    /// A boxed array of *PhysCompatRDMNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfPhysCompatRDMNotSupported")]
    ArrayOfPhysCompatRdmNotSupported(Vec<PhysCompatRdmNotSupported>),
    /// A boxed array of *PlatformConfigFault*. To be used in *Any* placeholders.
    ArrayOfPlatformConfigFault(Vec<Box<dyn super::traits::PlatformConfigFaultTrait>>),
    /// A boxed array of *PowerOnFtSecondaryFailed*. To be used in *Any* placeholders.
    ArrayOfPowerOnFtSecondaryFailed(Vec<PowerOnFtSecondaryFailed>),
    /// A boxed array of *PowerOnFtSecondaryTimedout*. To be used in *Any* placeholders.
    ArrayOfPowerOnFtSecondaryTimedout(Vec<PowerOnFtSecondaryTimedout>),
    /// A boxed array of *ProfileUpdateFailed*. To be used in *Any* placeholders.
    ArrayOfProfileUpdateFailed(Vec<ProfileUpdateFailed>),
    /// A boxed array of *ProfileUpdateFailedUpdateFailure*. To be used in *Any* placeholders.
    ArrayOfProfileUpdateFailedUpdateFailure(Vec<ProfileUpdateFailedUpdateFailure>),
    /// A boxed array of *QuarantineModeFault*. To be used in *Any* placeholders.
    ArrayOfQuarantineModeFault(Vec<QuarantineModeFault>),
    /// A boxed array of *QuestionPending*. To be used in *Any* placeholders.
    ArrayOfQuestionPending(Vec<QuestionPending>),
    /// A boxed array of *QuiesceDatastoreIOForHAFailed*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfQuiesceDatastoreIOForHAFailed")]
    ArrayOfQuiesceDatastoreIoForHaFailed(Vec<QuiesceDatastoreIoForHaFailed>),
    /// A boxed array of *RDMConversionNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRDMConversionNotSupported")]
    ArrayOfRdmConversionNotSupported(Vec<RdmConversionNotSupported>),
    /// A boxed array of *RDMNotPreserved*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRDMNotPreserved")]
    ArrayOfRdmNotPreserved(Vec<RdmNotPreserved>),
    /// A boxed array of *RDMNotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRDMNotSupported")]
    ArrayOfRdmNotSupported(Vec<Box<dyn super::traits::RdmNotSupportedTrait>>),
    /// A boxed array of *RDMNotSupportedOnDatastore*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRDMNotSupportedOnDatastore")]
    ArrayOfRdmNotSupportedOnDatastore(Vec<RdmNotSupportedOnDatastore>),
    /// A boxed array of *RDMPointsToInaccessibleDisk*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfRDMPointsToInaccessibleDisk")]
    ArrayOfRdmPointsToInaccessibleDisk(Vec<RdmPointsToInaccessibleDisk>),
    /// A boxed array of *RawDiskNotSupported*. To be used in *Any* placeholders.
    ArrayOfRawDiskNotSupported(Vec<RawDiskNotSupported>),
    /// A boxed array of *ReadHostResourcePoolTreeFailed*. To be used in *Any* placeholders.
    ArrayOfReadHostResourcePoolTreeFailed(Vec<ReadHostResourcePoolTreeFailed>),
    /// A boxed array of *ReadOnlyDisksWithLegacyDestination*. To be used in *Any* placeholders.
    ArrayOfReadOnlyDisksWithLegacyDestination(Vec<ReadOnlyDisksWithLegacyDestination>),
    /// A boxed array of *RebootRequired*. To be used in *Any* placeholders.
    ArrayOfRebootRequired(Vec<RebootRequired>),
    /// A boxed array of *RecordReplayDisabled*. To be used in *Any* placeholders.
    ArrayOfRecordReplayDisabled(Vec<RecordReplayDisabled>),
    /// A boxed array of *RemoteDeviceNotSupported*. To be used in *Any* placeholders.
    ArrayOfRemoteDeviceNotSupported(Vec<RemoteDeviceNotSupported>),
    /// A boxed array of *RemoveFailed*. To be used in *Any* placeholders.
    ArrayOfRemoveFailed(Vec<RemoveFailed>),
    /// A boxed array of *ReplicationConfigFault*. To be used in *Any* placeholders.
    ArrayOfReplicationConfigFault(Vec<Box<dyn super::traits::ReplicationConfigFaultTrait>>),
    /// A boxed array of *ReplicationDiskConfigFault*. To be used in *Any* placeholders.
    ArrayOfReplicationDiskConfigFault(Vec<ReplicationDiskConfigFault>),
    /// A boxed array of *ReplicationFault*. To be used in *Any* placeholders.
    ArrayOfReplicationFault(Vec<Box<dyn super::traits::ReplicationFaultTrait>>),
    /// A boxed array of *ReplicationIncompatibleWithFT*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfReplicationIncompatibleWithFT")]
    ArrayOfReplicationIncompatibleWithFt(Vec<ReplicationIncompatibleWithFt>),
    /// A boxed array of *ReplicationInvalidOptions*. To be used in *Any* placeholders.
    ArrayOfReplicationInvalidOptions(Vec<ReplicationInvalidOptions>),
    /// A boxed array of *ReplicationNotSupportedOnHost*. To be used in *Any* placeholders.
    ArrayOfReplicationNotSupportedOnHost(Vec<ReplicationNotSupportedOnHost>),
    /// A boxed array of *ReplicationVmConfigFault*. To be used in *Any* placeholders.
    ArrayOfReplicationVmConfigFault(Vec<ReplicationVmConfigFault>),
    /// A boxed array of *ReplicationVmFault*. To be used in *Any* placeholders.
    ArrayOfReplicationVmFault(Vec<Box<dyn super::traits::ReplicationVmFaultTrait>>),
    /// A boxed array of *ReplicationVmInProgressFault*. To be used in *Any* placeholders.
    ArrayOfReplicationVmInProgressFault(Vec<ReplicationVmInProgressFault>),
    /// A boxed array of *ResourceInUse*. To be used in *Any* placeholders.
    ArrayOfResourceInUse(Vec<Box<dyn super::traits::ResourceInUseTrait>>),
    /// A boxed array of *ResourceNotAvailable*. To be used in *Any* placeholders.
    ArrayOfResourceNotAvailable(Vec<ResourceNotAvailable>),
    /// A boxed array of *RestrictedByAdministrator*. To be used in *Any* placeholders.
    ArrayOfRestrictedByAdministrator(Vec<RestrictedByAdministrator>),
    /// A boxed array of *RestrictedVersion*. To be used in *Any* placeholders.
    ArrayOfRestrictedVersion(Vec<RestrictedVersion>),
    /// A boxed array of *RollbackFailure*. To be used in *Any* placeholders.
    ArrayOfRollbackFailure(Vec<RollbackFailure>),
    /// A boxed array of *RuleViolation*. To be used in *Any* placeholders.
    ArrayOfRuleViolation(Vec<RuleViolation>),
    /// A boxed array of *SSLDisabledFault*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSSLDisabledFault")]
    ArrayOfSslDisabledFault(Vec<SslDisabledFault>),
    /// A boxed array of *SSLVerifyFault*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSSLVerifyFault")]
    ArrayOfSslVerifyFault(Vec<SslVerifyFault>),
    /// A boxed array of *SSPIChallenge*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSSPIChallenge")]
    ArrayOfSspiChallenge(Vec<SspiChallenge>),
    /// A boxed array of *SecondaryVmAlreadyDisabled*. To be used in *Any* placeholders.
    ArrayOfSecondaryVmAlreadyDisabled(Vec<SecondaryVmAlreadyDisabled>),
    /// A boxed array of *SecondaryVmAlreadyEnabled*. To be used in *Any* placeholders.
    ArrayOfSecondaryVmAlreadyEnabled(Vec<SecondaryVmAlreadyEnabled>),
    /// A boxed array of *SecondaryVmAlreadyRegistered*. To be used in *Any* placeholders.
    ArrayOfSecondaryVmAlreadyRegistered(Vec<SecondaryVmAlreadyRegistered>),
    /// A boxed array of *SecondaryVmNotRegistered*. To be used in *Any* placeholders.
    ArrayOfSecondaryVmNotRegistered(Vec<SecondaryVmNotRegistered>),
    /// A boxed array of *SharedBusControllerNotSupported*. To be used in *Any* placeholders.
    ArrayOfSharedBusControllerNotSupported(Vec<SharedBusControllerNotSupported>),
    /// A boxed array of *ShrinkDiskFault*. To be used in *Any* placeholders.
    ArrayOfShrinkDiskFault(Vec<ShrinkDiskFault>),
    /// A boxed array of *SnapshotCloneNotSupported*. To be used in *Any* placeholders.
    ArrayOfSnapshotCloneNotSupported(Vec<SnapshotCloneNotSupported>),
    /// A boxed array of *SnapshotCopyNotSupported*. To be used in *Any* placeholders.
    ArrayOfSnapshotCopyNotSupported(Vec<Box<dyn super::traits::SnapshotCopyNotSupportedTrait>>),
    /// A boxed array of *SnapshotDisabled*. To be used in *Any* placeholders.
    ArrayOfSnapshotDisabled(Vec<SnapshotDisabled>),
    /// A boxed array of *SnapshotFault*. To be used in *Any* placeholders.
    ArrayOfSnapshotFault(Vec<Box<dyn super::traits::SnapshotFaultTrait>>),
    /// A boxed array of *SnapshotIncompatibleDeviceInVm*. To be used in *Any* placeholders.
    ArrayOfSnapshotIncompatibleDeviceInVm(Vec<SnapshotIncompatibleDeviceInVm>),
    /// A boxed array of *SnapshotLocked*. To be used in *Any* placeholders.
    ArrayOfSnapshotLocked(Vec<SnapshotLocked>),
    /// A boxed array of *SnapshotMoveFromNonHomeNotSupported*. To be used in *Any* placeholders.
    ArrayOfSnapshotMoveFromNonHomeNotSupported(Vec<SnapshotMoveFromNonHomeNotSupported>),
    /// A boxed array of *SnapshotMoveNotSupported*. To be used in *Any* placeholders.
    ArrayOfSnapshotMoveNotSupported(Vec<SnapshotMoveNotSupported>),
    /// A boxed array of *SnapshotMoveToNonHomeNotSupported*. To be used in *Any* placeholders.
    ArrayOfSnapshotMoveToNonHomeNotSupported(Vec<SnapshotMoveToNonHomeNotSupported>),
    /// A boxed array of *SnapshotNoChange*. To be used in *Any* placeholders.
    ArrayOfSnapshotNoChange(Vec<SnapshotNoChange>),
    /// A boxed array of *SnapshotRevertIssue*. To be used in *Any* placeholders.
    ArrayOfSnapshotRevertIssue(Vec<SnapshotRevertIssue>),
    /// A boxed array of *SoftRuleVioCorrectionDisallowed*. To be used in *Any* placeholders.
    ArrayOfSoftRuleVioCorrectionDisallowed(Vec<SoftRuleVioCorrectionDisallowed>),
    /// A boxed array of *SoftRuleVioCorrectionImpact*. To be used in *Any* placeholders.
    ArrayOfSoftRuleVioCorrectionImpact(Vec<SoftRuleVioCorrectionImpact>),
    /// A boxed array of *SolutionUserRequired*. To be used in *Any* placeholders.
    ArrayOfSolutionUserRequired(Vec<SolutionUserRequired>),
    /// A boxed array of *SsdDiskNotAvailable*. To be used in *Any* placeholders.
    ArrayOfSsdDiskNotAvailable(Vec<SsdDiskNotAvailable>),
    /// A boxed array of *StorageDrsCannotMoveDiskInMultiWriterMode*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveDiskInMultiWriterMode(Vec<StorageDrsCannotMoveDiskInMultiWriterMode>),
    /// A boxed array of *StorageDrsCannotMoveFTVm*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageDrsCannotMoveFTVm")]
    ArrayOfStorageDrsCannotMoveFtVm(Vec<StorageDrsCannotMoveFtVm>),
    /// A boxed array of *StorageDrsCannotMoveIndependentDisk*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveIndependentDisk(Vec<StorageDrsCannotMoveIndependentDisk>),
    /// A boxed array of *StorageDrsCannotMoveManuallyPlacedSwapFile*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveManuallyPlacedSwapFile(Vec<StorageDrsCannotMoveManuallyPlacedSwapFile>),
    /// A boxed array of *StorageDrsCannotMoveManuallyPlacedVm*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveManuallyPlacedVm(Vec<StorageDrsCannotMoveManuallyPlacedVm>),
    /// A boxed array of *StorageDrsCannotMoveSharedDisk*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveSharedDisk(Vec<StorageDrsCannotMoveSharedDisk>),
    /// A boxed array of *StorageDrsCannotMoveTemplate*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveTemplate(Vec<StorageDrsCannotMoveTemplate>),
    /// A boxed array of *StorageDrsCannotMoveVmInUserFolder*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveVmInUserFolder(Vec<StorageDrsCannotMoveVmInUserFolder>),
    /// A boxed array of *StorageDrsCannotMoveVmWithMountedCDROM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfStorageDrsCannotMoveVmWithMountedCDROM")]
    ArrayOfStorageDrsCannotMoveVmWithMountedCdrom(Vec<StorageDrsCannotMoveVmWithMountedCdrom>),
    /// A boxed array of *StorageDrsCannotMoveVmWithNoFilesInLayout*. To be used in *Any* placeholders.
    ArrayOfStorageDrsCannotMoveVmWithNoFilesInLayout(Vec<StorageDrsCannotMoveVmWithNoFilesInLayout>),
    /// A boxed array of *StorageDrsDatacentersCannotShareDatastore*. To be used in *Any* placeholders.
    ArrayOfStorageDrsDatacentersCannotShareDatastore(Vec<StorageDrsDatacentersCannotShareDatastore>),
    /// A boxed array of *StorageDrsDisabledOnVm*. To be used in *Any* placeholders.
    ArrayOfStorageDrsDisabledOnVm(Vec<StorageDrsDisabledOnVm>),
    /// A boxed array of *StorageDrsHbrDiskNotMovable*. To be used in *Any* placeholders.
    ArrayOfStorageDrsHbrDiskNotMovable(Vec<StorageDrsHbrDiskNotMovable>),
    /// A boxed array of *StorageDrsHmsMoveInProgress*. To be used in *Any* placeholders.
    ArrayOfStorageDrsHmsMoveInProgress(Vec<StorageDrsHmsMoveInProgress>),
    /// A boxed array of *StorageDrsHmsUnreachable*. To be used in *Any* placeholders.
    ArrayOfStorageDrsHmsUnreachable(Vec<StorageDrsHmsUnreachable>),
    /// A boxed array of *StorageDrsIolbDisabledInternally*. To be used in *Any* placeholders.
    ArrayOfStorageDrsIolbDisabledInternally(Vec<StorageDrsIolbDisabledInternally>),
    /// A boxed array of *StorageDrsRelocateDisabled*. To be used in *Any* placeholders.
    ArrayOfStorageDrsRelocateDisabled(Vec<StorageDrsRelocateDisabled>),
    /// A boxed array of *StorageDrsStaleHmsCollection*. To be used in *Any* placeholders.
    ArrayOfStorageDrsStaleHmsCollection(Vec<StorageDrsStaleHmsCollection>),
    /// A boxed array of *StorageDrsUnableToMoveFiles*. To be used in *Any* placeholders.
    ArrayOfStorageDrsUnableToMoveFiles(Vec<StorageDrsUnableToMoveFiles>),
    /// A boxed array of *StorageVMotionNotSupported*. To be used in *Any* placeholders.
    ArrayOfStorageVMotionNotSupported(Vec<StorageVMotionNotSupported>),
    /// A boxed array of *StorageVmotionIncompatible*. To be used in *Any* placeholders.
    ArrayOfStorageVmotionIncompatible(Vec<StorageVmotionIncompatible>),
    /// A boxed array of *SuspendedRelocateNotSupported*. To be used in *Any* placeholders.
    ArrayOfSuspendedRelocateNotSupported(Vec<SuspendedRelocateNotSupported>),
    /// A boxed array of *SwapDatastoreNotWritableOnHost*. To be used in *Any* placeholders.
    ArrayOfSwapDatastoreNotWritableOnHost(Vec<SwapDatastoreNotWritableOnHost>),
    /// A boxed array of *SwapDatastoreUnset*. To be used in *Any* placeholders.
    ArrayOfSwapDatastoreUnset(Vec<SwapDatastoreUnset>),
    /// A boxed array of *SwapPlacementOverrideNotSupported*. To be used in *Any* placeholders.
    ArrayOfSwapPlacementOverrideNotSupported(Vec<SwapPlacementOverrideNotSupported>),
    /// A boxed array of *SwitchIpUnset*. To be used in *Any* placeholders.
    ArrayOfSwitchIpUnset(Vec<SwitchIpUnset>),
    /// A boxed array of *SwitchNotInUpgradeMode*. To be used in *Any* placeholders.
    ArrayOfSwitchNotInUpgradeMode(Vec<SwitchNotInUpgradeMode>),
    /// A boxed array of *TaskInProgress*. To be used in *Any* placeholders.
    ArrayOfTaskInProgress(Vec<Box<dyn super::traits::TaskInProgressTrait>>),
    /// A boxed array of *ThirdPartyLicenseAssignmentFailed*. To be used in *Any* placeholders.
    ArrayOfThirdPartyLicenseAssignmentFailed(Vec<ThirdPartyLicenseAssignmentFailed>),
    /// A boxed array of *Timedout*. To be used in *Any* placeholders.
    ArrayOfTimedout(Vec<Box<dyn super::traits::TimedoutTrait>>),
    /// A boxed array of *TooManyConcurrentNativeClones*. To be used in *Any* placeholders.
    ArrayOfTooManyConcurrentNativeClones(Vec<TooManyConcurrentNativeClones>),
    /// A boxed array of *TooManyConsecutiveOverrides*. To be used in *Any* placeholders.
    ArrayOfTooManyConsecutiveOverrides(Vec<TooManyConsecutiveOverrides>),
    /// A boxed array of *TooManyDevices*. To be used in *Any* placeholders.
    ArrayOfTooManyDevices(Vec<TooManyDevices>),
    /// A boxed array of *TooManyDisksOnLegacyHost*. To be used in *Any* placeholders.
    ArrayOfTooManyDisksOnLegacyHost(Vec<TooManyDisksOnLegacyHost>),
    /// A boxed array of *TooManyGuestLogons*. To be used in *Any* placeholders.
    ArrayOfTooManyGuestLogons(Vec<TooManyGuestLogons>),
    /// A boxed array of *TooManyHosts*. To be used in *Any* placeholders.
    ArrayOfTooManyHosts(Vec<TooManyHosts>),
    /// A boxed array of *TooManyNativeCloneLevels*. To be used in *Any* placeholders.
    ArrayOfTooManyNativeCloneLevels(Vec<TooManyNativeCloneLevels>),
    /// A boxed array of *TooManyNativeClonesOnFile*. To be used in *Any* placeholders.
    ArrayOfTooManyNativeClonesOnFile(Vec<TooManyNativeClonesOnFile>),
    /// A boxed array of *TooManySnapshotLevels*. To be used in *Any* placeholders.
    ArrayOfTooManySnapshotLevels(Vec<TooManySnapshotLevels>),
    /// A boxed array of *ToolsAlreadyUpgraded*. To be used in *Any* placeholders.
    ArrayOfToolsAlreadyUpgraded(Vec<ToolsAlreadyUpgraded>),
    /// A boxed array of *ToolsAutoUpgradeNotSupported*. To be used in *Any* placeholders.
    ArrayOfToolsAutoUpgradeNotSupported(Vec<ToolsAutoUpgradeNotSupported>),
    /// A boxed array of *ToolsImageCopyFailed*. To be used in *Any* placeholders.
    ArrayOfToolsImageCopyFailed(Vec<ToolsImageCopyFailed>),
    /// A boxed array of *ToolsImageNotAvailable*. To be used in *Any* placeholders.
    ArrayOfToolsImageNotAvailable(Vec<ToolsImageNotAvailable>),
    /// A boxed array of *ToolsImageSignatureCheckFailed*. To be used in *Any* placeholders.
    ArrayOfToolsImageSignatureCheckFailed(Vec<ToolsImageSignatureCheckFailed>),
    /// A boxed array of *ToolsInstallationInProgress*. To be used in *Any* placeholders.
    ArrayOfToolsInstallationInProgress(Vec<ToolsInstallationInProgress>),
    /// A boxed array of *ToolsUnavailable*. To be used in *Any* placeholders.
    ArrayOfToolsUnavailable(Vec<ToolsUnavailable>),
    /// A boxed array of *ToolsUpgradeCancelled*. To be used in *Any* placeholders.
    ArrayOfToolsUpgradeCancelled(Vec<ToolsUpgradeCancelled>),
    /// A boxed array of *UnSupportedDatastoreForVFlash*. To be used in *Any* placeholders.
    ArrayOfUnSupportedDatastoreForVFlash(Vec<UnSupportedDatastoreForVFlash>),
    /// A boxed array of *UncommittedUndoableDisk*. To be used in *Any* placeholders.
    ArrayOfUncommittedUndoableDisk(Vec<UncommittedUndoableDisk>),
    /// A boxed array of *UnconfiguredPropertyValue*. To be used in *Any* placeholders.
    ArrayOfUnconfiguredPropertyValue(Vec<UnconfiguredPropertyValue>),
    /// A boxed array of *UncustomizableGuest*. To be used in *Any* placeholders.
    ArrayOfUncustomizableGuest(Vec<UncustomizableGuest>),
    /// A boxed array of *UnexpectedCustomizationFault*. To be used in *Any* placeholders.
    ArrayOfUnexpectedCustomizationFault(Vec<UnexpectedCustomizationFault>),
    /// A boxed array of *UnrecognizedHost*. To be used in *Any* placeholders.
    ArrayOfUnrecognizedHost(Vec<UnrecognizedHost>),
    /// A boxed array of *UnsharedSwapVMotionNotSupported*. To be used in *Any* placeholders.
    ArrayOfUnsharedSwapVMotionNotSupported(Vec<UnsharedSwapVMotionNotSupported>),
    /// A boxed array of *UnsupportedDatastore*. To be used in *Any* placeholders.
    ArrayOfUnsupportedDatastore(Vec<Box<dyn super::traits::UnsupportedDatastoreTrait>>),
    /// A boxed array of *UnsupportedGuest*. To be used in *Any* placeholders.
    ArrayOfUnsupportedGuest(Vec<UnsupportedGuest>),
    /// A boxed array of *UnsupportedVimApiVersion*. To be used in *Any* placeholders.
    ArrayOfUnsupportedVimApiVersion(Vec<UnsupportedVimApiVersion>),
    /// A boxed array of *UnsupportedVmxLocation*. To be used in *Any* placeholders.
    ArrayOfUnsupportedVmxLocation(Vec<UnsupportedVmxLocation>),
    /// A boxed array of *UnusedVirtualDiskBlocksNotScrubbed*. To be used in *Any* placeholders.
    ArrayOfUnusedVirtualDiskBlocksNotScrubbed(Vec<UnusedVirtualDiskBlocksNotScrubbed>),
    /// A boxed array of *UserNotFound*. To be used in *Any* placeholders.
    ArrayOfUserNotFound(Vec<UserNotFound>),
    /// A boxed array of *VAppConfigFault*. To be used in *Any* placeholders.
    ArrayOfVAppConfigFault(Vec<Box<dyn super::traits::VAppConfigFaultTrait>>),
    /// A boxed array of *VAppNotRunning*. To be used in *Any* placeholders.
    ArrayOfVAppNotRunning(Vec<VAppNotRunning>),
    /// A boxed array of *VAppOperationInProgress*. To be used in *Any* placeholders.
    ArrayOfVAppOperationInProgress(Vec<VAppOperationInProgress>),
    /// A boxed array of *VAppPropertyFault*. To be used in *Any* placeholders.
    ArrayOfVAppPropertyFault(Vec<Box<dyn super::traits::VAppPropertyFaultTrait>>),
    /// A boxed array of *VAppTaskInProgress*. To be used in *Any* placeholders.
    ArrayOfVAppTaskInProgress(Vec<VAppTaskInProgress>),
    /// A boxed array of *VFlashCacheHotConfigNotSupported*. To be used in *Any* placeholders.
    ArrayOfVFlashCacheHotConfigNotSupported(Vec<VFlashCacheHotConfigNotSupported>),
    /// A boxed array of *VFlashModuleNotSupported*. To be used in *Any* placeholders.
    ArrayOfVFlashModuleNotSupported(Vec<VFlashModuleNotSupported>),
    /// A boxed array of *VFlashModuleVersionIncompatible*. To be used in *Any* placeholders.
    ArrayOfVFlashModuleVersionIncompatible(Vec<VFlashModuleVersionIncompatible>),
    /// A boxed array of *VMINotSupported*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMINotSupported")]
    ArrayOfVmiNotSupported(Vec<VmiNotSupported>),
    /// A boxed array of *VMOnConflictDVPort*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMOnConflictDVPort")]
    ArrayOfVmOnConflictDvPort(Vec<VmOnConflictDvPort>),
    /// A boxed array of *VMOnVirtualIntranet*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVMOnVirtualIntranet")]
    ArrayOfVmOnVirtualIntranet(Vec<VmOnVirtualIntranet>),
    /// A boxed array of *VMotionAcrossNetworkNotSupported*. To be used in *Any* placeholders.
    ArrayOfVMotionAcrossNetworkNotSupported(Vec<VMotionAcrossNetworkNotSupported>),
    /// A boxed array of *VMotionInterfaceIssue*. To be used in *Any* placeholders.
    ArrayOfVMotionInterfaceIssue(Vec<Box<dyn super::traits::VMotionInterfaceIssueTrait>>),
    /// A boxed array of *VMotionLinkCapacityLow*. To be used in *Any* placeholders.
    ArrayOfVMotionLinkCapacityLow(Vec<VMotionLinkCapacityLow>),
    /// A boxed array of *VMotionLinkDown*. To be used in *Any* placeholders.
    ArrayOfVMotionLinkDown(Vec<VMotionLinkDown>),
    /// A boxed array of *VMotionNotConfigured*. To be used in *Any* placeholders.
    ArrayOfVMotionNotConfigured(Vec<VMotionNotConfigured>),
    /// A boxed array of *VMotionNotLicensed*. To be used in *Any* placeholders.
    ArrayOfVMotionNotLicensed(Vec<VMotionNotLicensed>),
    /// A boxed array of *VMotionNotSupported*. To be used in *Any* placeholders.
    ArrayOfVMotionNotSupported(Vec<VMotionNotSupported>),
    /// A boxed array of *VMotionProtocolIncompatible*. To be used in *Any* placeholders.
    ArrayOfVMotionProtocolIncompatible(Vec<VMotionProtocolIncompatible>),
    /// A boxed array of *VimFault*. To be used in *Any* placeholders.
    ArrayOfVimFault(Vec<Box<dyn super::traits::VimFaultTrait>>),
    /// A boxed array of *VirtualDiskBlocksNotFullyProvisioned*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskBlocksNotFullyProvisioned(Vec<VirtualDiskBlocksNotFullyProvisioned>),
    /// A boxed array of *VirtualDiskModeNotSupported*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskModeNotSupported(Vec<VirtualDiskModeNotSupported>),
    /// A boxed array of *VirtualEthernetCardNotSupported*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardNotSupported(Vec<VirtualEthernetCardNotSupported>),
    /// A boxed array of *VirtualHardwareCompatibilityIssue*. To be used in *Any* placeholders.
    ArrayOfVirtualHardwareCompatibilityIssue(Vec<Box<dyn super::traits::VirtualHardwareCompatibilityIssueTrait>>),
    /// A boxed array of *VirtualHardwareVersionNotSupported*. To be used in *Any* placeholders.
    ArrayOfVirtualHardwareVersionNotSupported(Vec<VirtualHardwareVersionNotSupported>),
    /// A boxed array of *VmAlreadyExistsInDatacenter*. To be used in *Any* placeholders.
    ArrayOfVmAlreadyExistsInDatacenter(Vec<VmAlreadyExistsInDatacenter>),
    /// A boxed array of *VmConfigFault*. To be used in *Any* placeholders.
    ArrayOfVmConfigFault(Vec<Box<dyn super::traits::VmConfigFaultTrait>>),
    /// A boxed array of *VmConfigIncompatibleForFaultTolerance*. To be used in *Any* placeholders.
    ArrayOfVmConfigIncompatibleForFaultTolerance(Vec<VmConfigIncompatibleForFaultTolerance>),
    /// A boxed array of *VmConfigIncompatibleForRecordReplay*. To be used in *Any* placeholders.
    ArrayOfVmConfigIncompatibleForRecordReplay(Vec<VmConfigIncompatibleForRecordReplay>),
    /// A boxed array of *VmFaultToleranceConfigIssue*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceConfigIssue(Vec<VmFaultToleranceConfigIssue>),
    /// A boxed array of *VmFaultToleranceConfigIssueWrapper*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceConfigIssueWrapper(Vec<VmFaultToleranceConfigIssueWrapper>),
    /// A boxed array of *VmFaultToleranceInvalidFileBacking*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceInvalidFileBacking(Vec<VmFaultToleranceInvalidFileBacking>),
    /// A boxed array of *VmFaultToleranceIssue*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceIssue(Vec<Box<dyn super::traits::VmFaultToleranceIssueTrait>>),
    /// A boxed array of *VmFaultToleranceOpIssuesList*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceOpIssuesList(Vec<VmFaultToleranceOpIssuesList>),
    /// A boxed array of *VmFaultToleranceTooManyFtVcpusOnHost*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceTooManyFtVcpusOnHost(Vec<VmFaultToleranceTooManyFtVcpusOnHost>),
    /// A boxed array of *VmFaultToleranceTooManyVMsOnHost*. To be used in *Any* placeholders.
    ArrayOfVmFaultToleranceTooManyVMsOnHost(Vec<VmFaultToleranceTooManyVMsOnHost>),
    /// A boxed array of *VmHostAffinityRuleViolation*. To be used in *Any* placeholders.
    ArrayOfVmHostAffinityRuleViolation(Vec<VmHostAffinityRuleViolation>),
    /// A boxed array of *VmLimitLicense*. To be used in *Any* placeholders.
    ArrayOfVmLimitLicense(Vec<VmLimitLicense>),
    /// A boxed array of *VmMetadataManagerFault*. To be used in *Any* placeholders.
    ArrayOfVmMetadataManagerFault(Vec<VmMetadataManagerFault>),
    /// A boxed array of *VmMonitorIncompatibleForFaultTolerance*. To be used in *Any* placeholders.
    ArrayOfVmMonitorIncompatibleForFaultTolerance(Vec<VmMonitorIncompatibleForFaultTolerance>),
    /// A boxed array of *VmPowerOnDisabled*. To be used in *Any* placeholders.
    ArrayOfVmPowerOnDisabled(Vec<VmPowerOnDisabled>),
    /// A boxed array of *VmSmpFaultToleranceTooManyVMsOnHost*. To be used in *Any* placeholders.
    ArrayOfVmSmpFaultToleranceTooManyVMsOnHost(Vec<VmSmpFaultToleranceTooManyVMsOnHost>),
    /// A boxed array of *VmToolsUpgradeFault*. To be used in *Any* placeholders.
    ArrayOfVmToolsUpgradeFault(Vec<Box<dyn super::traits::VmToolsUpgradeFaultTrait>>),
    /// A boxed array of *VmValidateMaxDevice*. To be used in *Any* placeholders.
    ArrayOfVmValidateMaxDevice(Vec<VmValidateMaxDevice>),
    /// A boxed array of *VmWwnConflict*. To be used in *Any* placeholders.
    ArrayOfVmWwnConflict(Vec<VmWwnConflict>),
    /// A boxed array of *VmfsAlreadyMounted*. To be used in *Any* placeholders.
    ArrayOfVmfsAlreadyMounted(Vec<VmfsAlreadyMounted>),
    /// A boxed array of *VmfsAmbiguousMount*. To be used in *Any* placeholders.
    ArrayOfVmfsAmbiguousMount(Vec<VmfsAmbiguousMount>),
    /// A boxed array of *VmfsMountFault*. To be used in *Any* placeholders.
    ArrayOfVmfsMountFault(Vec<Box<dyn super::traits::VmfsMountFaultTrait>>),
    /// A boxed array of *VmotionInterfaceNotEnabled*. To be used in *Any* placeholders.
    ArrayOfVmotionInterfaceNotEnabled(Vec<VmotionInterfaceNotEnabled>),
    /// A boxed array of *VolumeEditorError*. To be used in *Any* placeholders.
    ArrayOfVolumeEditorError(Vec<VolumeEditorError>),
    /// A boxed array of *VramLimitLicense*. To be used in *Any* placeholders.
    ArrayOfVramLimitLicense(Vec<VramLimitLicense>),
    /// A boxed array of *VsanClusterUuidMismatch*. To be used in *Any* placeholders.
    ArrayOfVsanClusterUuidMismatch(Vec<VsanClusterUuidMismatch>),
    /// A boxed array of *VsanDiskFault*. To be used in *Any* placeholders.
    ArrayOfVsanDiskFault(Vec<Box<dyn super::traits::VsanDiskFaultTrait>>),
    /// A boxed array of *VsanFault*. To be used in *Any* placeholders.
    ArrayOfVsanFault(Vec<Box<dyn super::traits::VsanFaultTrait>>),
    /// A boxed array of *VsanIncompatibleDiskMapping*. To be used in *Any* placeholders.
    ArrayOfVsanIncompatibleDiskMapping(Vec<VsanIncompatibleDiskMapping>),
    /// A boxed array of *VspanDestPortConflict*. To be used in *Any* placeholders.
    ArrayOfVspanDestPortConflict(Vec<VspanDestPortConflict>),
    /// A boxed array of *VspanPortConflict*. To be used in *Any* placeholders.
    ArrayOfVspanPortConflict(Vec<VspanPortConflict>),
    /// A boxed array of *VspanPortMoveFault*. To be used in *Any* placeholders.
    ArrayOfVspanPortMoveFault(Vec<VspanPortMoveFault>),
    /// A boxed array of *VspanPortPromiscChangeFault*. To be used in *Any* placeholders.
    ArrayOfVspanPortPromiscChangeFault(Vec<VspanPortPromiscChangeFault>),
    /// A boxed array of *VspanPortgroupPromiscChangeFault*. To be used in *Any* placeholders.
    ArrayOfVspanPortgroupPromiscChangeFault(Vec<VspanPortgroupPromiscChangeFault>),
    /// A boxed array of *VspanPortgroupTypeChangeFault*. To be used in *Any* placeholders.
    ArrayOfVspanPortgroupTypeChangeFault(Vec<VspanPortgroupTypeChangeFault>),
    /// A boxed array of *VspanPromiscuousPortNotSupported*. To be used in *Any* placeholders.
    ArrayOfVspanPromiscuousPortNotSupported(Vec<VspanPromiscuousPortNotSupported>),
    /// A boxed array of *VspanSameSessionPortConflict*. To be used in *Any* placeholders.
    ArrayOfVspanSameSessionPortConflict(Vec<VspanSameSessionPortConflict>),
    /// A boxed array of *WakeOnLanNotSupported*. To be used in *Any* placeholders.
    ArrayOfWakeOnLanNotSupported(Vec<WakeOnLanNotSupported>),
    /// A boxed array of *WakeOnLanNotSupportedByVmotionNIC*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfWakeOnLanNotSupportedByVmotionNIC")]
    ArrayOfWakeOnLanNotSupportedByVmotionNic(Vec<WakeOnLanNotSupportedByVmotionNic>),
    /// A boxed array of *WillLoseHAProtection*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfWillLoseHAProtection")]
    ArrayOfWillLoseHaProtection(Vec<WillLoseHaProtection>),
    /// A boxed array of *WillModifyConfigCpuRequirements*. To be used in *Any* placeholders.
    ArrayOfWillModifyConfigCpuRequirements(Vec<WillModifyConfigCpuRequirements>),
    /// A boxed array of *WillResetSnapshotDirectory*. To be used in *Any* placeholders.
    ArrayOfWillResetSnapshotDirectory(Vec<WillResetSnapshotDirectory>),
    /// A boxed array of *WipeDiskFault*. To be used in *Any* placeholders.
    ArrayOfWipeDiskFault(Vec<WipeDiskFault>),
    /// A boxed array of *HostActiveDirectoryInfo*. To be used in *Any* placeholders.
    ArrayOfHostActiveDirectoryInfo(Vec<HostActiveDirectoryInfo>),
    /// A boxed array of *HostActiveDirectory*. To be used in *Any* placeholders.
    ArrayOfHostActiveDirectory(Vec<HostActiveDirectory>),
    /// A boxed array of *HostActiveDirectorySpec*. To be used in *Any* placeholders.
    ArrayOfHostActiveDirectorySpec(Vec<HostActiveDirectorySpec>),
    /// A boxed array of *HostAssignableHardwareBinding*. To be used in *Any* placeholders.
    ArrayOfHostAssignableHardwareBinding(Vec<HostAssignableHardwareBinding>),
    /// A boxed array of *HostAssignableHardwareConfig*. To be used in *Any* placeholders.
    ArrayOfHostAssignableHardwareConfig(Vec<HostAssignableHardwareConfig>),
    /// A boxed array of *HostAssignableHardwareConfigAttributeOverride*. To be used in *Any* placeholders.
    ArrayOfHostAssignableHardwareConfigAttributeOverride(Vec<HostAssignableHardwareConfigAttributeOverride>),
    /// A boxed array of *HostAuthenticationManagerInfo*. To be used in *Any* placeholders.
    ArrayOfHostAuthenticationManagerInfo(Vec<HostAuthenticationManagerInfo>),
    /// A boxed array of *HostAuthenticationStoreInfo*. To be used in *Any* placeholders.
    ArrayOfHostAuthenticationStoreInfo(Vec<Box<dyn super::traits::HostAuthenticationStoreInfoTrait>>),
    /// A boxed array of *AutoStartPowerInfo*. To be used in *Any* placeholders.
    ArrayOfAutoStartPowerInfo(Vec<AutoStartPowerInfo>),
    /// A boxed array of *HostAutoStartManagerConfig*. To be used in *Any* placeholders.
    ArrayOfHostAutoStartManagerConfig(Vec<HostAutoStartManagerConfig>),
    /// A boxed array of *AutoStartDefaults*. To be used in *Any* placeholders.
    ArrayOfAutoStartDefaults(Vec<AutoStartDefaults>),
    /// A boxed array of *HostBIOSInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHostBIOSInfo")]
    ArrayOfHostBiosInfo(Vec<HostBiosInfo>),
    /// A boxed array of *HostBlockAdapterTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostBlockAdapterTargetTransport(Vec<HostBlockAdapterTargetTransport>),
    /// A boxed array of *HostBlockHba*. To be used in *Any* placeholders.
    ArrayOfHostBlockHba(Vec<HostBlockHba>),
    /// A boxed array of *HostBootDeviceInfo*. To be used in *Any* placeholders.
    ArrayOfHostBootDeviceInfo(Vec<HostBootDeviceInfo>),
    /// A boxed array of *HostBootDevice*. To be used in *Any* placeholders.
    ArrayOfHostBootDevice(Vec<HostBootDevice>),
    /// A boxed array of *HostCacheConfigurationInfo*. To be used in *Any* placeholders.
    ArrayOfHostCacheConfigurationInfo(Vec<HostCacheConfigurationInfo>),
    /// A boxed array of *HostCacheConfigurationSpec*. To be used in *Any* placeholders.
    ArrayOfHostCacheConfigurationSpec(Vec<HostCacheConfigurationSpec>),
    /// A boxed array of *HostCapability*. To be used in *Any* placeholders.
    ArrayOfHostCapability(Vec<HostCapability>),
    /// A boxed array of *HostCertificateManagerCertificateInfo*. To be used in *Any* placeholders.
    ArrayOfHostCertificateManagerCertificateInfo(Vec<HostCertificateManagerCertificateInfo>),
    /// A boxed array of *HostCertificateManagerCertificateSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfHostCertificateManagerCertificateSpec(Vec<HostCertificateManagerCertificateSpec>),
    /// A boxed array of *HostConfigChange*. To be used in *Any* placeholders.
    ArrayOfHostConfigChange(Vec<HostConfigChange>),
    /// A boxed array of *HostConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostConfigInfo(Vec<HostConfigInfo>),
    /// A boxed array of *HostConfigManager*. To be used in *Any* placeholders.
    ArrayOfHostConfigManager(Vec<HostConfigManager>),
    /// A boxed array of *HostConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostConfigSpec(Vec<HostConfigSpec>),
    /// A boxed array of *HostConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostConnectInfo(Vec<HostConnectInfo>),
    /// A boxed array of *HostDatastoreExistsConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreExistsConnectInfo(Vec<HostDatastoreExistsConnectInfo>),
    /// A boxed array of *HostDatastoreConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreConnectInfo(Vec<Box<dyn super::traits::HostDatastoreConnectInfoTrait>>),
    /// A boxed array of *HostDatastoreNameConflictConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreNameConflictConnectInfo(Vec<HostDatastoreNameConflictConnectInfo>),
    /// A boxed array of *HostLicenseConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostLicenseConnectInfo(Vec<HostLicenseConnectInfo>),
    /// A boxed array of *HostConnectInfoNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfHostConnectInfoNetworkInfo(Vec<Box<dyn super::traits::HostConnectInfoNetworkInfoTrait>>),
    /// A boxed array of *HostNewNetworkConnectInfo*. To be used in *Any* placeholders.
    ArrayOfHostNewNetworkConnectInfo(Vec<HostNewNetworkConnectInfo>),
    /// A boxed array of *HostConnectSpec*. To be used in *Any* placeholders.
    ArrayOfHostConnectSpec(Vec<HostConnectSpec>),
    /// A boxed array of *HostCpuIdInfo*. To be used in *Any* placeholders.
    ArrayOfHostCpuIdInfo(Vec<HostCpuIdInfo>),
    /// A boxed array of *HostCpuInfo*. To be used in *Any* placeholders.
    ArrayOfHostCpuInfo(Vec<HostCpuInfo>),
    /// A boxed array of *HostCpuPackage*. To be used in *Any* placeholders.
    ArrayOfHostCpuPackage(Vec<HostCpuPackage>),
    /// A boxed array of *HostCpuPowerManagementInfo*. To be used in *Any* placeholders.
    ArrayOfHostCpuPowerManagementInfo(Vec<HostCpuPowerManagementInfo>),
    /// A boxed array of *HostHyperThreadScheduleInfo*. To be used in *Any* placeholders.
    ArrayOfHostHyperThreadScheduleInfo(Vec<HostHyperThreadScheduleInfo>),
    /// A boxed array of *HostDataTransportConnectionInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostDataTransportConnectionInfo(Vec<Box<dyn super::traits::HostDataTransportConnectionInfoTrait>>),
    /// A boxed array of *FileInfo*. To be used in *Any* placeholders.
    ArrayOfFileInfo(Vec<Box<dyn super::traits::FileInfoTrait>>),
    /// A boxed array of *FileQueryFlags*. To be used in *Any* placeholders.
    ArrayOfFileQueryFlags(Vec<FileQueryFlags>),
    /// A boxed array of *FloppyImageFileInfo*. To be used in *Any* placeholders.
    ArrayOfFloppyImageFileInfo(Vec<FloppyImageFileInfo>),
    /// A boxed array of *FloppyImageFileQuery*. To be used in *Any* placeholders.
    ArrayOfFloppyImageFileQuery(Vec<FloppyImageFileQuery>),
    /// A boxed array of *FolderFileInfo*. To be used in *Any* placeholders.
    ArrayOfFolderFileInfo(Vec<FolderFileInfo>),
    /// A boxed array of *FolderFileQuery*. To be used in *Any* placeholders.
    ArrayOfFolderFileQuery(Vec<FolderFileQuery>),
    /// A boxed array of *IsoImageFileInfo*. To be used in *Any* placeholders.
    ArrayOfIsoImageFileInfo(Vec<IsoImageFileInfo>),
    /// A boxed array of *IsoImageFileQuery*. To be used in *Any* placeholders.
    ArrayOfIsoImageFileQuery(Vec<IsoImageFileQuery>),
    /// A boxed array of *FileQuery*. To be used in *Any* placeholders.
    ArrayOfFileQuery(Vec<Box<dyn super::traits::FileQueryTrait>>),
    /// A boxed array of *HostDatastoreBrowserSearchResults*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreBrowserSearchResults(Vec<HostDatastoreBrowserSearchResults>),
    /// A boxed array of *HostDatastoreBrowserSearchSpec*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreBrowserSearchSpec(Vec<HostDatastoreBrowserSearchSpec>),
    /// A boxed array of *TemplateConfigFileInfo*. To be used in *Any* placeholders.
    ArrayOfTemplateConfigFileInfo(Vec<TemplateConfigFileInfo>),
    /// A boxed array of *TemplateConfigFileQuery*. To be used in *Any* placeholders.
    ArrayOfTemplateConfigFileQuery(Vec<TemplateConfigFileQuery>),
    /// A boxed array of *VmConfigFileInfo*. To be used in *Any* placeholders.
    ArrayOfVmConfigFileInfo(Vec<Box<dyn super::traits::VmConfigFileInfoTrait>>),
    /// A boxed array of *VmConfigFileEncryptionInfo*. To be used in *Any* placeholders.
    ArrayOfVmConfigFileEncryptionInfo(Vec<VmConfigFileEncryptionInfo>),
    /// A boxed array of *VmConfigFileQuery*. To be used in *Any* placeholders.
    ArrayOfVmConfigFileQuery(Vec<Box<dyn super::traits::VmConfigFileQueryTrait>>),
    /// A boxed array of *VmConfigFileQueryFlags*. To be used in *Any* placeholders.
    ArrayOfVmConfigFileQueryFlags(Vec<VmConfigFileQueryFlags>),
    /// A boxed array of *VmConfigFileQueryFilter*. To be used in *Any* placeholders.
    ArrayOfVmConfigFileQueryFilter(Vec<VmConfigFileQueryFilter>),
    /// A boxed array of *VmDiskFileInfo*. To be used in *Any* placeholders.
    ArrayOfVmDiskFileInfo(Vec<VmDiskFileInfo>),
    /// A boxed array of *VmDiskFileEncryptionInfo*. To be used in *Any* placeholders.
    ArrayOfVmDiskFileEncryptionInfo(Vec<VmDiskFileEncryptionInfo>),
    /// A boxed array of *VmDiskFileQuery*. To be used in *Any* placeholders.
    ArrayOfVmDiskFileQuery(Vec<VmDiskFileQuery>),
    /// A boxed array of *VmDiskFileQueryFlags*. To be used in *Any* placeholders.
    ArrayOfVmDiskFileQueryFlags(Vec<VmDiskFileQueryFlags>),
    /// A boxed array of *VmDiskFileQueryFilter*. To be used in *Any* placeholders.
    ArrayOfVmDiskFileQueryFilter(Vec<VmDiskFileQueryFilter>),
    /// A boxed array of *VmLogFileInfo*. To be used in *Any* placeholders.
    ArrayOfVmLogFileInfo(Vec<VmLogFileInfo>),
    /// A boxed array of *VmLogFileQuery*. To be used in *Any* placeholders.
    ArrayOfVmLogFileQuery(Vec<VmLogFileQuery>),
    /// A boxed array of *VmNvramFileInfo*. To be used in *Any* placeholders.
    ArrayOfVmNvramFileInfo(Vec<VmNvramFileInfo>),
    /// A boxed array of *VmNvramFileQuery*. To be used in *Any* placeholders.
    ArrayOfVmNvramFileQuery(Vec<VmNvramFileQuery>),
    /// A boxed array of *VmSnapshotFileInfo*. To be used in *Any* placeholders.
    ArrayOfVmSnapshotFileInfo(Vec<VmSnapshotFileInfo>),
    /// A boxed array of *VmSnapshotFileQuery*. To be used in *Any* placeholders.
    ArrayOfVmSnapshotFileQuery(Vec<VmSnapshotFileQuery>),
    /// A boxed array of *HostDatastoreSystemCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreSystemCapabilities(Vec<HostDatastoreSystemCapabilities>),
    /// A boxed array of *HostDatastoreSystemDatastoreResult*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreSystemDatastoreResult(Vec<HostDatastoreSystemDatastoreResult>),
    /// A boxed array of *HostDatastoreSystemVvolDatastoreSpec*. To be used in *Any* placeholders.
    ArrayOfHostDatastoreSystemVvolDatastoreSpec(Vec<HostDatastoreSystemVvolDatastoreSpec>),
    /// A boxed array of *HostDateTimeConfig*. To be used in *Any* placeholders.
    ArrayOfHostDateTimeConfig(Vec<HostDateTimeConfig>),
    /// A boxed array of *HostDateTimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostDateTimeInfo(Vec<HostDateTimeInfo>),
    /// A boxed array of *HostDateTimeSystemServiceTestResult*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostDateTimeSystemServiceTestResult(Vec<HostDateTimeSystemServiceTestResult>),
    /// A boxed array of *HostDateTimeSystemTimeZone*. To be used in *Any* placeholders.
    ArrayOfHostDateTimeSystemTimeZone(Vec<HostDateTimeSystemTimeZone>),
    /// A boxed array of *HostDeploymentInfo*. To be used in *Any* placeholders.
    ArrayOfHostDeploymentInfo(Vec<HostDeploymentInfo>),
    /// A boxed array of *HostDevice*. To be used in *Any* placeholders.
    ArrayOfHostDevice(Vec<Box<dyn super::traits::HostDeviceTrait>>),
    /// A boxed array of *HostDhcpService*. To be used in *Any* placeholders.
    ArrayOfHostDhcpService(Vec<HostDhcpService>),
    /// A boxed array of *HostDhcpServiceConfig*. To be used in *Any* placeholders.
    ArrayOfHostDhcpServiceConfig(Vec<HostDhcpServiceConfig>),
    /// A boxed array of *HostDhcpServiceSpec*. To be used in *Any* placeholders.
    ArrayOfHostDhcpServiceSpec(Vec<HostDhcpServiceSpec>),
    /// A boxed array of *HostDiagnosticPartition*. To be used in *Any* placeholders.
    ArrayOfHostDiagnosticPartition(Vec<HostDiagnosticPartition>),
    /// A boxed array of *HostDiagnosticPartitionCreateDescription*. To be used in *Any* placeholders.
    ArrayOfHostDiagnosticPartitionCreateDescription(Vec<HostDiagnosticPartitionCreateDescription>),
    /// A boxed array of *HostDiagnosticPartitionCreateOption*. To be used in *Any* placeholders.
    ArrayOfHostDiagnosticPartitionCreateOption(Vec<HostDiagnosticPartitionCreateOption>),
    /// A boxed array of *HostDiagnosticPartitionCreateSpec*. To be used in *Any* placeholders.
    ArrayOfHostDiagnosticPartitionCreateSpec(Vec<HostDiagnosticPartitionCreateSpec>),
    /// A boxed array of *HostDigestInfo*. To be used in *Any* placeholders.
    ArrayOfHostDigestInfo(Vec<Box<dyn super::traits::HostDigestInfoTrait>>),
    /// A boxed array of *HostDirectoryStoreInfo*. To be used in *Any* placeholders.
    ArrayOfHostDirectoryStoreInfo(Vec<Box<dyn super::traits::HostDirectoryStoreInfoTrait>>),
    /// A boxed array of *HostDiskConfigurationResult*. To be used in *Any* placeholders.
    ArrayOfHostDiskConfigurationResult(Vec<HostDiskConfigurationResult>),
    /// A boxed array of *HostDiskDimensions*. To be used in *Any* placeholders.
    ArrayOfHostDiskDimensions(Vec<HostDiskDimensions>),
    /// A boxed array of *HostDiskDimensionsChs*. To be used in *Any* placeholders.
    ArrayOfHostDiskDimensionsChs(Vec<HostDiskDimensionsChs>),
    /// A boxed array of *HostDiskDimensionsLba*. To be used in *Any* placeholders.
    ArrayOfHostDiskDimensionsLba(Vec<HostDiskDimensionsLba>),
    /// A boxed array of *HostDiskPartitionInfo*. To be used in *Any* placeholders.
    ArrayOfHostDiskPartitionInfo(Vec<HostDiskPartitionInfo>),
    /// A boxed array of *HostDiskPartitionBlockRange*. To be used in *Any* placeholders.
    ArrayOfHostDiskPartitionBlockRange(Vec<HostDiskPartitionBlockRange>),
    /// A boxed array of *HostDiskPartitionLayout*. To be used in *Any* placeholders.
    ArrayOfHostDiskPartitionLayout(Vec<HostDiskPartitionLayout>),
    /// A boxed array of *HostDiskPartitionAttributes*. To be used in *Any* placeholders.
    ArrayOfHostDiskPartitionAttributes(Vec<HostDiskPartitionAttributes>),
    /// A boxed array of *HostDiskPartitionSpec*. To be used in *Any* placeholders.
    ArrayOfHostDiskPartitionSpec(Vec<HostDiskPartitionSpec>),
    /// A boxed array of *HostDnsConfig*. To be used in *Any* placeholders.
    ArrayOfHostDnsConfig(Vec<Box<dyn super::traits::HostDnsConfigTrait>>),
    /// A boxed array of *HostDnsConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostDnsConfigSpec(Vec<HostDnsConfigSpec>),
    /// A boxed array of *HostDvxClass*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostDvxClass(Vec<HostDvxClass>),
    /// A boxed array of *HostEnterMaintenanceResult*. To be used in *Any* placeholders.
    ArrayOfHostEnterMaintenanceResult(Vec<HostEnterMaintenanceResult>),
    /// A boxed array of *HostEsxAgentHostManagerConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostEsxAgentHostManagerConfigInfo(Vec<HostEsxAgentHostManagerConfigInfo>),
    /// A boxed array of *HostFaultToleranceManagerComponentHealthInfo*. To be used in *Any* placeholders.
    ArrayOfHostFaultToleranceManagerComponentHealthInfo(Vec<HostFaultToleranceManagerComponentHealthInfo>),
    /// A boxed array of *FcoeConfig*. To be used in *Any* placeholders.
    ArrayOfFcoeConfig(Vec<FcoeConfig>),
    /// A boxed array of *FcoeConfigFcoeCapabilities*. To be used in *Any* placeholders.
    ArrayOfFcoeConfigFcoeCapabilities(Vec<FcoeConfigFcoeCapabilities>),
    /// A boxed array of *FcoeConfigFcoeSpecification*. To be used in *Any* placeholders.
    ArrayOfFcoeConfigFcoeSpecification(Vec<FcoeConfigFcoeSpecification>),
    /// A boxed array of *FcoeConfigVlanRange*. To be used in *Any* placeholders.
    ArrayOfFcoeConfigVlanRange(Vec<FcoeConfigVlanRange>),
    /// A boxed array of *HostFeatureCapability*. To be used in *Any* placeholders.
    ArrayOfHostFeatureCapability(Vec<HostFeatureCapability>),
    /// A boxed array of *HostFeatureMask*. To be used in *Any* placeholders.
    ArrayOfHostFeatureMask(Vec<HostFeatureMask>),
    /// A boxed array of *HostFeatureVersionInfo*. To be used in *Any* placeholders.
    ArrayOfHostFeatureVersionInfo(Vec<HostFeatureVersionInfo>),
    /// A boxed array of *HostFibreChannelHba*. To be used in *Any* placeholders.
    ArrayOfHostFibreChannelHba(Vec<Box<dyn super::traits::HostFibreChannelHbaTrait>>),
    /// A boxed array of *HostFibreChannelOverEthernetHba*. To be used in *Any* placeholders.
    ArrayOfHostFibreChannelOverEthernetHba(Vec<HostFibreChannelOverEthernetHba>),
    /// A boxed array of *HostFibreChannelOverEthernetHbaLinkInfo*. To be used in *Any* placeholders.
    ArrayOfHostFibreChannelOverEthernetHbaLinkInfo(Vec<HostFibreChannelOverEthernetHbaLinkInfo>),
    /// A boxed array of *HostFibreChannelOverEthernetTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostFibreChannelOverEthernetTargetTransport(Vec<HostFibreChannelOverEthernetTargetTransport>),
    /// A boxed array of *HostFibreChannelTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostFibreChannelTargetTransport(Vec<Box<dyn super::traits::HostFibreChannelTargetTransportTrait>>),
    /// A boxed array of *HostFileAccess*. To be used in *Any* placeholders.
    ArrayOfHostFileAccess(Vec<HostFileAccess>),
    /// A boxed array of *ModeInfo*. To be used in *Any* placeholders.
    ArrayOfModeInfo(Vec<ModeInfo>),
    /// A boxed array of *HostFileSystemMountInfo*. To be used in *Any* placeholders.
    ArrayOfHostFileSystemMountInfo(Vec<HostFileSystemMountInfo>),
    /// A boxed array of *HostFileSystemVolume*. To be used in *Any* placeholders.
    ArrayOfHostFileSystemVolume(Vec<Box<dyn super::traits::HostFileSystemVolumeTrait>>),
    /// A boxed array of *HostFileSystemVolumeInfo*. To be used in *Any* placeholders.
    ArrayOfHostFileSystemVolumeInfo(Vec<HostFileSystemVolumeInfo>),
    /// A boxed array of *HostFirewallConfig*. To be used in *Any* placeholders.
    ArrayOfHostFirewallConfig(Vec<HostFirewallConfig>),
    /// A boxed array of *HostFirewallConfigRuleSetConfig*. To be used in *Any* placeholders.
    ArrayOfHostFirewallConfigRuleSetConfig(Vec<HostFirewallConfigRuleSetConfig>),
    /// A boxed array of *HostFirewallInfo*. To be used in *Any* placeholders.
    ArrayOfHostFirewallInfo(Vec<HostFirewallInfo>),
    /// A boxed array of *HostFirewallDefaultPolicy*. To be used in *Any* placeholders.
    ArrayOfHostFirewallDefaultPolicy(Vec<HostFirewallDefaultPolicy>),
    /// A boxed array of *HostFlagInfo*. To be used in *Any* placeholders.
    ArrayOfHostFlagInfo(Vec<HostFlagInfo>),
    /// A boxed array of *HostForceMountedInfo*. To be used in *Any* placeholders.
    ArrayOfHostForceMountedInfo(Vec<HostForceMountedInfo>),
    /// A boxed array of *HostFru*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostFru(Vec<HostFru>),
    /// A boxed array of *HostGatewaySpec*. To be used in *Any* placeholders.
    ArrayOfHostGatewaySpec(Vec<HostGatewaySpec>),
    /// A boxed array of *HostGraphicsConfig*. To be used in *Any* placeholders.
    ArrayOfHostGraphicsConfig(Vec<HostGraphicsConfig>),
    /// A boxed array of *HostGraphicsConfigDeviceType*. To be used in *Any* placeholders.
    ArrayOfHostGraphicsConfigDeviceType(Vec<HostGraphicsConfigDeviceType>),
    /// A boxed array of *HostGraphicsInfo*. To be used in *Any* placeholders.
    ArrayOfHostGraphicsInfo(Vec<HostGraphicsInfo>),
    /// A boxed array of *HostHardwareInfo*. To be used in *Any* placeholders.
    ArrayOfHostHardwareInfo(Vec<HostHardwareInfo>),
    /// A boxed array of *HostHardwareStatusInfo*. To be used in *Any* placeholders.
    ArrayOfHostHardwareStatusInfo(Vec<HostHardwareStatusInfo>),
    /// A boxed array of *DpuStatusInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfDpuStatusInfo(Vec<DpuStatusInfo>),
    /// A boxed array of *DpuStatusInfoOperationalInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfDpuStatusInfoOperationalInfo(Vec<DpuStatusInfoOperationalInfo>),
    /// A boxed array of *HostHardwareElementInfo*. To be used in *Any* placeholders.
    ArrayOfHostHardwareElementInfo(Vec<Box<dyn super::traits::HostHardwareElementInfoTrait>>),
    /// A boxed array of *HostStorageElementInfo*. To be used in *Any* placeholders.
    ArrayOfHostStorageElementInfo(Vec<HostStorageElementInfo>),
    /// A boxed array of *HostStorageOperationalInfo*. To be used in *Any* placeholders.
    ArrayOfHostStorageOperationalInfo(Vec<HostStorageOperationalInfo>),
    /// A boxed array of *HostHbaCreateSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostHbaCreateSpec(Vec<Box<dyn super::traits::HostHbaCreateSpecTrait>>),
    /// A boxed array of *HealthSystemRuntime*. To be used in *Any* placeholders.
    ArrayOfHealthSystemRuntime(Vec<HealthSystemRuntime>),
    /// A boxed array of *HostAccessControlEntry*. To be used in *Any* placeholders.
    ArrayOfHostAccessControlEntry(Vec<HostAccessControlEntry>),
    /// A boxed array of *HostHostBusAdapter*. To be used in *Any* placeholders.
    ArrayOfHostHostBusAdapter(Vec<Box<dyn super::traits::HostHostBusAdapterTrait>>),
    /// A boxed array of *HostProxySwitch*. To be used in *Any* placeholders.
    ArrayOfHostProxySwitch(Vec<HostProxySwitch>),
    /// A boxed array of *HostProxySwitchConfig*. To be used in *Any* placeholders.
    ArrayOfHostProxySwitchConfig(Vec<HostProxySwitchConfig>),
    /// A boxed array of *HostProxySwitchEnsInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostProxySwitchEnsInfo(Vec<HostProxySwitchEnsInfo>),
    /// A boxed array of *HostProxySwitchHostLagConfig*. To be used in *Any* placeholders.
    ArrayOfHostProxySwitchHostLagConfig(Vec<HostProxySwitchHostLagConfig>),
    /// A boxed array of *HostProxySwitchSpec*. To be used in *Any* placeholders.
    ArrayOfHostProxySwitchSpec(Vec<HostProxySwitchSpec>),
    /// A boxed array of *HostImageProfileSummary*. To be used in *Any* placeholders.
    ArrayOfHostImageProfileSummary(Vec<HostImageProfileSummary>),
    /// A boxed array of *HostInternetScsiHba*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHba(Vec<HostInternetScsiHba>),
    /// A boxed array of *HostInternetScsiHbaAuthenticationCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaAuthenticationCapabilities(Vec<HostInternetScsiHbaAuthenticationCapabilities>),
    /// A boxed array of *HostInternetScsiHbaAuthenticationProperties*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaAuthenticationProperties(Vec<HostInternetScsiHbaAuthenticationProperties>),
    /// A boxed array of *HostInternetScsiHbaDigestCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaDigestCapabilities(Vec<HostInternetScsiHbaDigestCapabilities>),
    /// A boxed array of *HostInternetScsiHbaDigestProperties*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaDigestProperties(Vec<HostInternetScsiHbaDigestProperties>),
    /// A boxed array of *HostInternetScsiHbaDiscoveryCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaDiscoveryCapabilities(Vec<HostInternetScsiHbaDiscoveryCapabilities>),
    /// A boxed array of *HostInternetScsiHbaDiscoveryProperties*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaDiscoveryProperties(Vec<HostInternetScsiHbaDiscoveryProperties>),
    /// A boxed array of *HostInternetScsiHbaIPCapabilities*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHostInternetScsiHbaIPCapabilities")]
    ArrayOfHostInternetScsiHbaIpCapabilities(Vec<HostInternetScsiHbaIpCapabilities>),
    /// A boxed array of *HostInternetScsiHbaIPProperties*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfHostInternetScsiHbaIPProperties")]
    ArrayOfHostInternetScsiHbaIpProperties(Vec<HostInternetScsiHbaIpProperties>),
    /// A boxed array of *HostInternetScsiHbaIPv6Properties*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaIPv6Properties(Vec<HostInternetScsiHbaIPv6Properties>),
    /// A boxed array of *HostInternetScsiHbaIscsiIpv6Address*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaIscsiIpv6Address(Vec<HostInternetScsiHbaIscsiIpv6Address>),
    /// A boxed array of *HostInternetScsiHbaParamValue*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaParamValue(Vec<HostInternetScsiHbaParamValue>),
    /// A boxed array of *HostInternetScsiHbaSendTarget*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaSendTarget(Vec<HostInternetScsiHbaSendTarget>),
    /// A boxed array of *HostInternetScsiHbaStaticTarget*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaStaticTarget(Vec<HostInternetScsiHbaStaticTarget>),
    /// A boxed array of *HostInternetScsiHbaTargetSet*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaTargetSet(Vec<HostInternetScsiHbaTargetSet>),
    /// A boxed array of *HostInternetScsiTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiTargetTransport(Vec<HostInternetScsiTargetTransport>),
    /// A boxed array of *HostIpConfig*. To be used in *Any* placeholders.
    ArrayOfHostIpConfig(Vec<HostIpConfig>),
    /// A boxed array of *HostIpConfigIpV6Address*. To be used in *Any* placeholders.
    ArrayOfHostIpConfigIpV6Address(Vec<HostIpConfigIpV6Address>),
    /// A boxed array of *HostIpConfigIpV6AddressConfiguration*. To be used in *Any* placeholders.
    ArrayOfHostIpConfigIpV6AddressConfiguration(Vec<HostIpConfigIpV6AddressConfiguration>),
    /// A boxed array of *HostIpRouteConfig*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteConfig(Vec<Box<dyn super::traits::HostIpRouteConfigTrait>>),
    /// A boxed array of *HostIpRouteConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteConfigSpec(Vec<HostIpRouteConfigSpec>),
    /// A boxed array of *HostIpRouteEntry*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteEntry(Vec<HostIpRouteEntry>),
    /// A boxed array of *HostIpRouteOp*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteOp(Vec<HostIpRouteOp>),
    /// A boxed array of *HostIpRouteTableConfig*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteTableConfig(Vec<HostIpRouteTableConfig>),
    /// A boxed array of *HostIpRouteTableInfo*. To be used in *Any* placeholders.
    ArrayOfHostIpRouteTableInfo(Vec<HostIpRouteTableInfo>),
    /// A boxed array of *HostIpmiInfo*. To be used in *Any* placeholders.
    ArrayOfHostIpmiInfo(Vec<HostIpmiInfo>),
    /// A boxed array of *IscsiDependencyEntity*. To be used in *Any* placeholders.
    ArrayOfIscsiDependencyEntity(Vec<IscsiDependencyEntity>),
    /// A boxed array of *IscsiMigrationDependency*. To be used in *Any* placeholders.
    ArrayOfIscsiMigrationDependency(Vec<IscsiMigrationDependency>),
    /// A boxed array of *IscsiPortInfo*. To be used in *Any* placeholders.
    ArrayOfIscsiPortInfo(Vec<IscsiPortInfo>),
    /// A boxed array of *IscsiStatus*. To be used in *Any* placeholders.
    ArrayOfIscsiStatus(Vec<IscsiStatus>),
    /// A boxed array of *KernelModuleInfo*. To be used in *Any* placeholders.
    ArrayOfKernelModuleInfo(Vec<KernelModuleInfo>),
    /// A boxed array of *KernelModuleSectionInfo*. To be used in *Any* placeholders.
    ArrayOfKernelModuleSectionInfo(Vec<KernelModuleSectionInfo>),
    /// A boxed array of *HostLicenseSpec*. To be used in *Any* placeholders.
    ArrayOfHostLicenseSpec(Vec<HostLicenseSpec>),
    /// A boxed array of *LinkDiscoveryProtocolConfig*. To be used in *Any* placeholders.
    ArrayOfLinkDiscoveryProtocolConfig(Vec<LinkDiscoveryProtocolConfig>),
    /// A boxed array of *HostAccountSpec*. To be used in *Any* placeholders.
    ArrayOfHostAccountSpec(Vec<Box<dyn super::traits::HostAccountSpecTrait>>),
    /// A boxed array of *HostPosixAccountSpec*. To be used in *Any* placeholders.
    ArrayOfHostPosixAccountSpec(Vec<HostPosixAccountSpec>),
    /// A boxed array of *HostLocalAuthenticationInfo*. To be used in *Any* placeholders.
    ArrayOfHostLocalAuthenticationInfo(Vec<HostLocalAuthenticationInfo>),
    /// A boxed array of *LocalDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfLocalDatastoreInfo(Vec<LocalDatastoreInfo>),
    /// A boxed array of *HostLocalFileSystemVolume*. To be used in *Any* placeholders.
    ArrayOfHostLocalFileSystemVolume(Vec<HostLocalFileSystemVolume>),
    /// A boxed array of *HostLocalFileSystemVolumeSpec*. To be used in *Any* placeholders.
    ArrayOfHostLocalFileSystemVolumeSpec(Vec<HostLocalFileSystemVolumeSpec>),
    /// A boxed array of *HostLowLevelProvisioningManagerDiskLayoutSpec*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerDiskLayoutSpec(Vec<HostLowLevelProvisioningManagerDiskLayoutSpec>),
    /// A boxed array of *HostLowLevelProvisioningManagerFileDeleteResult*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerFileDeleteResult(Vec<HostLowLevelProvisioningManagerFileDeleteResult>),
    /// A boxed array of *HostLowLevelProvisioningManagerFileDeleteSpec*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerFileDeleteSpec(Vec<HostLowLevelProvisioningManagerFileDeleteSpec>),
    /// A boxed array of *HostLowLevelProvisioningManagerFileReserveResult*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerFileReserveResult(Vec<HostLowLevelProvisioningManagerFileReserveResult>),
    /// A boxed array of *HostLowLevelProvisioningManagerFileReserveSpec*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerFileReserveSpec(Vec<HostLowLevelProvisioningManagerFileReserveSpec>),
    /// A boxed array of *HostLowLevelProvisioningManagerSnapshotLayoutSpec*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerSnapshotLayoutSpec(Vec<HostLowLevelProvisioningManagerSnapshotLayoutSpec>),
    /// A boxed array of *HostLowLevelProvisioningManagerVmMigrationStatus*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerVmMigrationStatus(Vec<HostLowLevelProvisioningManagerVmMigrationStatus>),
    /// A boxed array of *HostLowLevelProvisioningManagerVmRecoveryInfo*. To be used in *Any* placeholders.
    ArrayOfHostLowLevelProvisioningManagerVmRecoveryInfo(Vec<HostLowLevelProvisioningManagerVmRecoveryInfo>),
    /// A boxed array of *HostMaintenanceSpec*. To be used in *Any* placeholders.
    ArrayOfHostMaintenanceSpec(Vec<HostMaintenanceSpec>),
    /// A boxed array of *ServiceConsoleReservationInfo*. To be used in *Any* placeholders.
    ArrayOfServiceConsoleReservationInfo(Vec<ServiceConsoleReservationInfo>),
    /// A boxed array of *VirtualMachineMemoryReservationInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMemoryReservationInfo(Vec<VirtualMachineMemoryReservationInfo>),
    /// A boxed array of *VirtualMachineMemoryReservationSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMemoryReservationSpec(Vec<VirtualMachineMemoryReservationSpec>),
    /// A boxed array of *HostMemorySpec*. To be used in *Any* placeholders.
    ArrayOfHostMemorySpec(Vec<HostMemorySpec>),
    /// A boxed array of *HostMemoryTierInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostMemoryTierInfo(Vec<HostMemoryTierInfo>),
    /// A boxed array of *HostMountInfo*. To be used in *Any* placeholders.
    ArrayOfHostMountInfo(Vec<HostMountInfo>),
    /// A boxed array of *HostMultipathInfo*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfo(Vec<HostMultipathInfo>),
    /// A boxed array of *HostMultipathInfoFixedLogicalUnitPolicy*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoFixedLogicalUnitPolicy(Vec<HostMultipathInfoFixedLogicalUnitPolicy>),
    /// A boxed array of *HostMultipathInfoHppLogicalUnitPolicy*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoHppLogicalUnitPolicy(Vec<HostMultipathInfoHppLogicalUnitPolicy>),
    /// A boxed array of *HostMultipathInfoLogicalUnit*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoLogicalUnit(Vec<HostMultipathInfoLogicalUnit>),
    /// A boxed array of *HostMultipathInfoLogicalUnitPolicy*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoLogicalUnitPolicy(Vec<Box<dyn super::traits::HostMultipathInfoLogicalUnitPolicyTrait>>),
    /// A boxed array of *HostMultipathInfoLogicalUnitStorageArrayTypePolicy*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoLogicalUnitStorageArrayTypePolicy(Vec<HostMultipathInfoLogicalUnitStorageArrayTypePolicy>),
    /// A boxed array of *HostMultipathInfoPath*. To be used in *Any* placeholders.
    ArrayOfHostMultipathInfoPath(Vec<HostMultipathInfoPath>),
    /// A boxed array of *HostMultipathStateInfo*. To be used in *Any* placeholders.
    ArrayOfHostMultipathStateInfo(Vec<HostMultipathStateInfo>),
    /// A boxed array of *HostMultipathStateInfoPath*. To be used in *Any* placeholders.
    ArrayOfHostMultipathStateInfoPath(Vec<HostMultipathStateInfoPath>),
    /// A boxed array of *NasDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfNasDatastoreInfo(Vec<NasDatastoreInfo>),
    /// A boxed array of *HostNasVolume*. To be used in *Any* placeholders.
    ArrayOfHostNasVolume(Vec<HostNasVolume>),
    /// A boxed array of *HostNasVolumeConfig*. To be used in *Any* placeholders.
    ArrayOfHostNasVolumeConfig(Vec<HostNasVolumeConfig>),
    /// A boxed array of *HostNasVolumeSpec*. To be used in *Any* placeholders.
    ArrayOfHostNasVolumeSpec(Vec<HostNasVolumeSpec>),
    /// A boxed array of *HostNasVolumeUserInfo*. To be used in *Any* placeholders.
    ArrayOfHostNasVolumeUserInfo(Vec<HostNasVolumeUserInfo>),
    /// A boxed array of *HostNatService*. To be used in *Any* placeholders.
    ArrayOfHostNatService(Vec<HostNatService>),
    /// A boxed array of *HostNatServiceConfig*. To be used in *Any* placeholders.
    ArrayOfHostNatServiceConfig(Vec<HostNatServiceConfig>),
    /// A boxed array of *HostNatServiceNameServiceSpec*. To be used in *Any* placeholders.
    ArrayOfHostNatServiceNameServiceSpec(Vec<HostNatServiceNameServiceSpec>),
    /// A boxed array of *HostNatServicePortForwardSpec*. To be used in *Any* placeholders.
    ArrayOfHostNatServicePortForwardSpec(Vec<HostNatServicePortForwardSpec>),
    /// A boxed array of *HostNatServiceSpec*. To be used in *Any* placeholders.
    ArrayOfHostNatServiceSpec(Vec<HostNatServiceSpec>),
    /// A boxed array of *HostNetCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostNetCapabilities(Vec<HostNetCapabilities>),
    /// A boxed array of *HostNetOffloadCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostNetOffloadCapabilities(Vec<HostNetOffloadCapabilities>),
    /// A boxed array of *HostNetStackInstance*. To be used in *Any* placeholders.
    ArrayOfHostNetStackInstance(Vec<HostNetStackInstance>),
    /// A boxed array of *HostNetworkConfig*. To be used in *Any* placeholders.
    ArrayOfHostNetworkConfig(Vec<HostNetworkConfig>),
    /// A boxed array of *HostNetworkConfigNetStackSpec*. To be used in *Any* placeholders.
    ArrayOfHostNetworkConfigNetStackSpec(Vec<HostNetworkConfigNetStackSpec>),
    /// A boxed array of *HostNetworkConfigResult*. To be used in *Any* placeholders.
    ArrayOfHostNetworkConfigResult(Vec<HostNetworkConfigResult>),
    /// A boxed array of *HostNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfHostNetworkInfo(Vec<HostNetworkInfo>),
    /// A boxed array of *HostNetworkPolicy*. To be used in *Any* placeholders.
    ArrayOfHostNetworkPolicy(Vec<HostNetworkPolicy>),
    /// A boxed array of *HostNicFailureCriteria*. To be used in *Any* placeholders.
    ArrayOfHostNicFailureCriteria(Vec<HostNicFailureCriteria>),
    /// A boxed array of *HostNicOrderPolicy*. To be used in *Any* placeholders.
    ArrayOfHostNicOrderPolicy(Vec<HostNicOrderPolicy>),
    /// A boxed array of *HostNicTeamingPolicy*. To be used in *Any* placeholders.
    ArrayOfHostNicTeamingPolicy(Vec<HostNicTeamingPolicy>),
    /// A boxed array of *HostNetworkSecurityPolicy*. To be used in *Any* placeholders.
    ArrayOfHostNetworkSecurityPolicy(Vec<HostNetworkSecurityPolicy>),
    /// A boxed array of *HostNetworkTrafficShapingPolicy*. To be used in *Any* placeholders.
    ArrayOfHostNetworkTrafficShapingPolicy(Vec<HostNetworkTrafficShapingPolicy>),
    /// A boxed array of *HostNfcConnectionInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostNfcConnectionInfo(Vec<HostNfcConnectionInfo>),
    /// A boxed array of *HostNtpConfig*. To be used in *Any* placeholders.
    ArrayOfHostNtpConfig(Vec<HostNtpConfig>),
    /// A boxed array of *HostNumaInfo*. To be used in *Any* placeholders.
    ArrayOfHostNumaInfo(Vec<HostNumaInfo>),
    /// A boxed array of *HostNumaNode*. To be used in *Any* placeholders.
    ArrayOfHostNumaNode(Vec<HostNumaNode>),
    /// A boxed array of *HostNumericSensorInfo*. To be used in *Any* placeholders.
    ArrayOfHostNumericSensorInfo(Vec<HostNumericSensorInfo>),
    /// A boxed array of *NvdimmDimmInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmDimmInfo(Vec<NvdimmDimmInfo>),
    /// A boxed array of *NvdimmGuid*. To be used in *Any* placeholders.
    ArrayOfNvdimmGuid(Vec<NvdimmGuid>),
    /// A boxed array of *NvdimmHealthInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmHealthInfo(Vec<NvdimmHealthInfo>),
    /// A boxed array of *NvdimmInterleaveSetInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmInterleaveSetInfo(Vec<NvdimmInterleaveSetInfo>),
    /// A boxed array of *NvdimmNamespaceCreateSpec*. To be used in *Any* placeholders.
    ArrayOfNvdimmNamespaceCreateSpec(Vec<NvdimmNamespaceCreateSpec>),
    /// A boxed array of *NvdimmNamespaceDeleteSpec*. To be used in *Any* placeholders.
    ArrayOfNvdimmNamespaceDeleteSpec(Vec<NvdimmNamespaceDeleteSpec>),
    /// A boxed array of *NvdimmNamespaceDetails*. To be used in *Any* placeholders.
    ArrayOfNvdimmNamespaceDetails(Vec<NvdimmNamespaceDetails>),
    /// A boxed array of *NvdimmNamespaceInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmNamespaceInfo(Vec<NvdimmNamespaceInfo>),
    /// A boxed array of *NvdimmSystemInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmSystemInfo(Vec<NvdimmSystemInfo>),
    /// A boxed array of *NvdimmPMemNamespaceCreateSpec*. To be used in *Any* placeholders.
    ArrayOfNvdimmPMemNamespaceCreateSpec(Vec<NvdimmPMemNamespaceCreateSpec>),
    /// A boxed array of *NvdimmRegionInfo*. To be used in *Any* placeholders.
    ArrayOfNvdimmRegionInfo(Vec<NvdimmRegionInfo>),
    /// A boxed array of *NvdimmSummary*. To be used in *Any* placeholders.
    ArrayOfNvdimmSummary(Vec<NvdimmSummary>),
    /// A boxed array of *HostNvmeConnectSpec*. To be used in *Any* placeholders.
    ArrayOfHostNvmeConnectSpec(Vec<HostNvmeConnectSpec>),
    /// A boxed array of *HostNvmeController*. To be used in *Any* placeholders.
    ArrayOfHostNvmeController(Vec<HostNvmeController>),
    /// A boxed array of *HostNvmeDisconnectSpec*. To be used in *Any* placeholders.
    ArrayOfHostNvmeDisconnectSpec(Vec<HostNvmeDisconnectSpec>),
    /// A boxed array of *HostNvmeDiscoverSpec*. To be used in *Any* placeholders.
    ArrayOfHostNvmeDiscoverSpec(Vec<HostNvmeDiscoverSpec>),
    /// A boxed array of *HostNvmeDiscoveryLog*. To be used in *Any* placeholders.
    ArrayOfHostNvmeDiscoveryLog(Vec<HostNvmeDiscoveryLog>),
    /// A boxed array of *HostNvmeDiscoveryLogEntry*. To be used in *Any* placeholders.
    ArrayOfHostNvmeDiscoveryLogEntry(Vec<HostNvmeDiscoveryLogEntry>),
    /// A boxed array of *HostNvmeNamespace*. To be used in *Any* placeholders.
    ArrayOfHostNvmeNamespace(Vec<HostNvmeNamespace>),
    /// A boxed array of *HostNvmeOpaqueTransportParameters*. To be used in *Any* placeholders.
    ArrayOfHostNvmeOpaqueTransportParameters(Vec<HostNvmeOpaqueTransportParameters>),
    /// A boxed array of *HostNvmeOverFibreChannelParameters*. To be used in *Any* placeholders.
    ArrayOfHostNvmeOverFibreChannelParameters(Vec<HostNvmeOverFibreChannelParameters>),
    /// A boxed array of *HostNvmeOverRdmaParameters*. To be used in *Any* placeholders.
    ArrayOfHostNvmeOverRdmaParameters(Vec<HostNvmeOverRdmaParameters>),
    /// A boxed array of *HostNvmeOverTcpParameters*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostNvmeOverTcpParameters(Vec<HostNvmeOverTcpParameters>),
    /// A boxed array of *HostNvmeSpec*. To be used in *Any* placeholders.
    ArrayOfHostNvmeSpec(Vec<Box<dyn super::traits::HostNvmeSpecTrait>>),
    /// A boxed array of *HostNvmeTopology*. To be used in *Any* placeholders.
    ArrayOfHostNvmeTopology(Vec<HostNvmeTopology>),
    /// A boxed array of *HostNvmeTopologyInterface*. To be used in *Any* placeholders.
    ArrayOfHostNvmeTopologyInterface(Vec<HostNvmeTopologyInterface>),
    /// A boxed array of *HostNvmeTransportParameters*. To be used in *Any* placeholders.
    ArrayOfHostNvmeTransportParameters(Vec<Box<dyn super::traits::HostNvmeTransportParametersTrait>>),
    /// A boxed array of *HostOpaqueNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfHostOpaqueNetworkInfo(Vec<HostOpaqueNetworkInfo>),
    /// A boxed array of *HostOpaqueSwitch*. To be used in *Any* placeholders.
    ArrayOfHostOpaqueSwitch(Vec<HostOpaqueSwitch>),
    /// A boxed array of *HostOpaqueSwitchPhysicalNicZone*. To be used in *Any* placeholders.
    ArrayOfHostOpaqueSwitchPhysicalNicZone(Vec<HostOpaqueSwitchPhysicalNicZone>),
    /// A boxed array of *PMemDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfPMemDatastoreInfo(Vec<PMemDatastoreInfo>),
    /// A boxed array of *HostPMemVolume*. To be used in *Any* placeholders.
    ArrayOfHostPMemVolume(Vec<HostPMemVolume>),
    /// A boxed array of *HostParallelScsiHba*. To be used in *Any* placeholders.
    ArrayOfHostParallelScsiHba(Vec<HostParallelScsiHba>),
    /// A boxed array of *HostParallelScsiTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostParallelScsiTargetTransport(Vec<HostParallelScsiTargetTransport>),
    /// A boxed array of *HostPatchManagerLocator*. To be used in *Any* placeholders.
    ArrayOfHostPatchManagerLocator(Vec<HostPatchManagerLocator>),
    /// A boxed array of *HostPatchManagerPatchManagerOperationSpec*. To be used in *Any* placeholders.
    ArrayOfHostPatchManagerPatchManagerOperationSpec(Vec<HostPatchManagerPatchManagerOperationSpec>),
    /// A boxed array of *HostPatchManagerResult*. To be used in *Any* placeholders.
    ArrayOfHostPatchManagerResult(Vec<HostPatchManagerResult>),
    /// A boxed array of *HostPatchManagerStatus*. To be used in *Any* placeholders.
    ArrayOfHostPatchManagerStatus(Vec<HostPatchManagerStatus>),
    /// A boxed array of *HostPatchManagerStatusPrerequisitePatch*. To be used in *Any* placeholders.
    ArrayOfHostPatchManagerStatusPrerequisitePatch(Vec<HostPatchManagerStatusPrerequisitePatch>),
    /// A boxed array of *HostPathSelectionPolicyOption*. To be used in *Any* placeholders.
    ArrayOfHostPathSelectionPolicyOption(Vec<HostPathSelectionPolicyOption>),
    /// A boxed array of *HostPciDevice*. To be used in *Any* placeholders.
    ArrayOfHostPciDevice(Vec<HostPciDevice>),
    /// A boxed array of *HostPciPassthruConfig*. To be used in *Any* placeholders.
    ArrayOfHostPciPassthruConfig(Vec<Box<dyn super::traits::HostPciPassthruConfigTrait>>),
    /// A boxed array of *HostPciPassthruInfo*. To be used in *Any* placeholders.
    ArrayOfHostPciPassthruInfo(Vec<Box<dyn super::traits::HostPciPassthruInfoTrait>>),
    /// A boxed array of *HostPcieHba*. To be used in *Any* placeholders.
    ArrayOfHostPcieHba(Vec<HostPcieHba>),
    /// A boxed array of *HostPcieTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostPcieTargetTransport(Vec<HostPcieTargetTransport>),
    /// A boxed array of *HostPersistentMemoryInfo*. To be used in *Any* placeholders.
    ArrayOfHostPersistentMemoryInfo(Vec<HostPersistentMemoryInfo>),
    /// A boxed array of *PhysicalNic*. To be used in *Any* placeholders.
    ArrayOfPhysicalNic(Vec<PhysicalNic>),
    /// A boxed array of *PhysicalNicCdpDeviceCapability*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicCdpDeviceCapability(Vec<PhysicalNicCdpDeviceCapability>),
    /// A boxed array of *PhysicalNicCdpInfo*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicCdpInfo(Vec<PhysicalNicCdpInfo>),
    /// A boxed array of *PhysicalNicConfig*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicConfig(Vec<PhysicalNicConfig>),
    /// A boxed array of *PhysicalNicLinkInfo*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicLinkInfo(Vec<PhysicalNicLinkInfo>),
    /// A boxed array of *LinkLayerDiscoveryProtocolInfo*. To be used in *Any* placeholders.
    ArrayOfLinkLayerDiscoveryProtocolInfo(Vec<LinkLayerDiscoveryProtocolInfo>),
    /// A boxed array of *PhysicalNicHintInfo*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicHintInfo(Vec<PhysicalNicHintInfo>),
    /// A boxed array of *PhysicalNicHint*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicHint(Vec<Box<dyn super::traits::PhysicalNicHintTrait>>),
    /// A boxed array of *PhysicalNicIpHint*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicIpHint(Vec<PhysicalNicIpHint>),
    /// A boxed array of *PhysicalNicNameHint*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicNameHint(Vec<PhysicalNicNameHint>),
    /// A boxed array of *PhysicalNicSpec*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicSpec(Vec<PhysicalNicSpec>),
    /// A boxed array of *HostPlugStoreTopology*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopology(Vec<HostPlugStoreTopology>),
    /// A boxed array of *HostPlugStoreTopologyAdapter*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopologyAdapter(Vec<HostPlugStoreTopologyAdapter>),
    /// A boxed array of *HostPlugStoreTopologyDevice*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopologyDevice(Vec<HostPlugStoreTopologyDevice>),
    /// A boxed array of *HostPlugStoreTopologyPath*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopologyPath(Vec<HostPlugStoreTopologyPath>),
    /// A boxed array of *HostPlugStoreTopologyPlugin*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopologyPlugin(Vec<HostPlugStoreTopologyPlugin>),
    /// A boxed array of *HostPlugStoreTopologyTarget*. To be used in *Any* placeholders.
    ArrayOfHostPlugStoreTopologyTarget(Vec<HostPlugStoreTopologyTarget>),
    /// A boxed array of *HostPortGroup*. To be used in *Any* placeholders.
    ArrayOfHostPortGroup(Vec<HostPortGroup>),
    /// A boxed array of *HostPortGroupConfig*. To be used in *Any* placeholders.
    ArrayOfHostPortGroupConfig(Vec<HostPortGroupConfig>),
    /// A boxed array of *HostPortGroupPort*. To be used in *Any* placeholders.
    ArrayOfHostPortGroupPort(Vec<HostPortGroupPort>),
    /// A boxed array of *HostPortGroupSpec*. To be used in *Any* placeholders.
    ArrayOfHostPortGroupSpec(Vec<HostPortGroupSpec>),
    /// A boxed array of *PowerSystemCapability*. To be used in *Any* placeholders.
    ArrayOfPowerSystemCapability(Vec<PowerSystemCapability>),
    /// A boxed array of *PowerSystemInfo*. To be used in *Any* placeholders.
    ArrayOfPowerSystemInfo(Vec<PowerSystemInfo>),
    /// A boxed array of *HostPowerPolicy*. To be used in *Any* placeholders.
    ArrayOfHostPowerPolicy(Vec<HostPowerPolicy>),
    /// A boxed array of *HostProtocolEndpoint*. To be used in *Any* placeholders.
    ArrayOfHostProtocolEndpoint(Vec<HostProtocolEndpoint>),
    /// A boxed array of *HostPtpConfig*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostPtpConfig(Vec<HostPtpConfig>),
    /// A boxed array of *HostPtpConfigPtpPort*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostPtpConfigPtpPort(Vec<HostPtpConfigPtpPort>),
    /// A boxed array of *HostQualifiedName*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostQualifiedName(Vec<HostQualifiedName>),
    /// A boxed array of *HostRdmaDevice*. To be used in *Any* placeholders.
    ArrayOfHostRdmaDevice(Vec<HostRdmaDevice>),
    /// A boxed array of *HostRdmaDeviceBacking*. To be used in *Any* placeholders.
    ArrayOfHostRdmaDeviceBacking(Vec<Box<dyn super::traits::HostRdmaDeviceBackingTrait>>),
    /// A boxed array of *HostRdmaDeviceCapability*. To be used in *Any* placeholders.
    ArrayOfHostRdmaDeviceCapability(Vec<HostRdmaDeviceCapability>),
    /// A boxed array of *HostRdmaDeviceConnectionInfo*. To be used in *Any* placeholders.
    ArrayOfHostRdmaDeviceConnectionInfo(Vec<HostRdmaDeviceConnectionInfo>),
    /// A boxed array of *HostRdmaDevicePnicBacking*. To be used in *Any* placeholders.
    ArrayOfHostRdmaDevicePnicBacking(Vec<HostRdmaDevicePnicBacking>),
    /// A boxed array of *HostRdmaHba*. To be used in *Any* placeholders.
    ArrayOfHostRdmaHba(Vec<HostRdmaHba>),
    /// A boxed array of *HostRdmaTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostRdmaTargetTransport(Vec<HostRdmaTargetTransport>),
    /// A boxed array of *HostReliableMemoryInfo*. To be used in *Any* placeholders.
    ArrayOfHostReliableMemoryInfo(Vec<HostReliableMemoryInfo>),
    /// A boxed array of *HostResignatureRescanResult*. To be used in *Any* placeholders.
    ArrayOfHostResignatureRescanResult(Vec<HostResignatureRescanResult>),
    /// A boxed array of *HostFirewallRuleset*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRuleset(Vec<HostFirewallRuleset>),
    /// A boxed array of *HostFirewallRulesetIpList*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRulesetIpList(Vec<HostFirewallRulesetIpList>),
    /// A boxed array of *HostFirewallRulesetIpNetwork*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRulesetIpNetwork(Vec<HostFirewallRulesetIpNetwork>),
    /// A boxed array of *HostFirewallRule*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRule(Vec<HostFirewallRule>),
    /// A boxed array of *HostFirewallRulesetRulesetSpec*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRulesetRulesetSpec(Vec<HostFirewallRulesetRulesetSpec>),
    /// A boxed array of *HostRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostRuntimeInfo(Vec<HostRuntimeInfo>),
    /// A boxed array of *HostRuntimeInfoNetStackInstanceRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostRuntimeInfoNetStackInstanceRuntimeInfo(Vec<HostRuntimeInfoNetStackInstanceRuntimeInfo>),
    /// A boxed array of *HostNetworkResourceRuntime*. To be used in *Any* placeholders.
    ArrayOfHostNetworkResourceRuntime(Vec<HostNetworkResourceRuntime>),
    /// A boxed array of *HostRuntimeInfoNetworkRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostRuntimeInfoNetworkRuntimeInfo(Vec<HostRuntimeInfoNetworkRuntimeInfo>),
    /// A boxed array of *HostPlacedVirtualNicIdentifier*. To be used in *Any* placeholders.
    ArrayOfHostPlacedVirtualNicIdentifier(Vec<HostPlacedVirtualNicIdentifier>),
    /// A boxed array of *HostPnicNetworkResourceInfo*. To be used in *Any* placeholders.
    ArrayOfHostPnicNetworkResourceInfo(Vec<HostPnicNetworkResourceInfo>),
    /// A boxed array of *HostRuntimeInfoStateEncryptionInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostRuntimeInfoStateEncryptionInfo(Vec<HostRuntimeInfoStateEncryptionInfo>),
    /// A boxed array of *HostScsiDisk*. To be used in *Any* placeholders.
    ArrayOfHostScsiDisk(Vec<HostScsiDisk>),
    /// A boxed array of *HostScsiDiskPartition*. To be used in *Any* placeholders.
    ArrayOfHostScsiDiskPartition(Vec<HostScsiDiskPartition>),
    /// A boxed array of *ScsiLun*. To be used in *Any* placeholders.
    ArrayOfScsiLun(Vec<Box<dyn super::traits::ScsiLunTrait>>),
    /// A boxed array of *ScsiLunCapabilities*. To be used in *Any* placeholders.
    ArrayOfScsiLunCapabilities(Vec<ScsiLunCapabilities>),
    /// A boxed array of *ScsiLunDescriptor*. To be used in *Any* placeholders.
    ArrayOfScsiLunDescriptor(Vec<ScsiLunDescriptor>),
    /// A boxed array of *ScsiLunDurableName*. To be used in *Any* placeholders.
    ArrayOfScsiLunDurableName(Vec<ScsiLunDurableName>),
    /// A boxed array of *HostScsiTopology*. To be used in *Any* placeholders.
    ArrayOfHostScsiTopology(Vec<HostScsiTopology>),
    /// A boxed array of *HostScsiTopologyInterface*. To be used in *Any* placeholders.
    ArrayOfHostScsiTopologyInterface(Vec<HostScsiTopologyInterface>),
    /// A boxed array of *HostScsiTopologyLun*. To be used in *Any* placeholders.
    ArrayOfHostScsiTopologyLun(Vec<HostScsiTopologyLun>),
    /// A boxed array of *HostScsiTopologyTarget*. To be used in *Any* placeholders.
    ArrayOfHostScsiTopologyTarget(Vec<HostScsiTopologyTarget>),
    /// A boxed array of *HostSecuritySpec*. To be used in *Any* placeholders.
    ArrayOfHostSecuritySpec(Vec<HostSecuritySpec>),
    /// A boxed array of *HostSerialAttachedHba*. To be used in *Any* placeholders.
    ArrayOfHostSerialAttachedHba(Vec<HostSerialAttachedHba>),
    /// A boxed array of *HostSerialAttachedTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostSerialAttachedTargetTransport(Vec<HostSerialAttachedTargetTransport>),
    /// A boxed array of *HostService*. To be used in *Any* placeholders.
    ArrayOfHostService(Vec<HostService>),
    /// A boxed array of *HostServiceSourcePackage*. To be used in *Any* placeholders.
    ArrayOfHostServiceSourcePackage(Vec<HostServiceSourcePackage>),
    /// A boxed array of *HostServiceConfig*. To be used in *Any* placeholders.
    ArrayOfHostServiceConfig(Vec<HostServiceConfig>),
    /// A boxed array of *HostServiceInfo*. To be used in *Any* placeholders.
    ArrayOfHostServiceInfo(Vec<HostServiceInfo>),
    /// A boxed array of *HostSevInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ArrayOfHostSevInfo(Vec<HostSevInfo>),
    /// A boxed array of *HostSgxInfo*. To be used in *Any* placeholders.
    ArrayOfHostSgxInfo(Vec<HostSgxInfo>),
    /// A boxed array of *HostSgxRegistrationInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostSgxRegistrationInfo(Vec<HostSgxRegistrationInfo>),
    /// A boxed array of *HostSharedGpuCapabilities*. To be used in *Any* placeholders.
    ArrayOfHostSharedGpuCapabilities(Vec<HostSharedGpuCapabilities>),
    /// A boxed array of *HostSnmpSystemAgentLimits*. To be used in *Any* placeholders.
    ArrayOfHostSnmpSystemAgentLimits(Vec<HostSnmpSystemAgentLimits>),
    /// A boxed array of *HostSnmpConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostSnmpConfigSpec(Vec<HostSnmpConfigSpec>),
    /// A boxed array of *HostSnmpDestination*. To be used in *Any* placeholders.
    ArrayOfHostSnmpDestination(Vec<HostSnmpDestination>),
    /// A boxed array of *SoftwarePackage*. To be used in *Any* placeholders.
    ArrayOfSoftwarePackage(Vec<SoftwarePackage>),
    /// A boxed array of *SoftwarePackageCapability*. To be used in *Any* placeholders.
    ArrayOfSoftwarePackageCapability(Vec<SoftwarePackageCapability>),
    /// A boxed array of *Relation*. To be used in *Any* placeholders.
    ArrayOfRelation(Vec<Relation>),
    /// A boxed array of *HostSriovConfig*. To be used in *Any* placeholders.
    ArrayOfHostSriovConfig(Vec<HostSriovConfig>),
    /// A boxed array of *HostSriovDevicePoolInfo*. To be used in *Any* placeholders.
    ArrayOfHostSriovDevicePoolInfo(Vec<Box<dyn super::traits::HostSriovDevicePoolInfoTrait>>),
    /// A boxed array of *HostSriovInfo*. To be used in *Any* placeholders.
    ArrayOfHostSriovInfo(Vec<HostSriovInfo>),
    /// A boxed array of *HostSriovNetworkDevicePoolInfo*. To be used in *Any* placeholders.
    ArrayOfHostSriovNetworkDevicePoolInfo(Vec<HostSriovNetworkDevicePoolInfo>),
    /// A boxed array of *HostSslThumbprintInfo*. To be used in *Any* placeholders.
    ArrayOfHostSslThumbprintInfo(Vec<HostSslThumbprintInfo>),
    /// A boxed array of *HostStorageArrayTypePolicyOption*. To be used in *Any* placeholders.
    ArrayOfHostStorageArrayTypePolicyOption(Vec<HostStorageArrayTypePolicyOption>),
    /// A boxed array of *HostStorageDeviceInfo*. To be used in *Any* placeholders.
    ArrayOfHostStorageDeviceInfo(Vec<HostStorageDeviceInfo>),
    /// A boxed array of *HostStorageSystemDiskLocatorLedResult*. To be used in *Any* placeholders.
    ArrayOfHostStorageSystemDiskLocatorLedResult(Vec<HostStorageSystemDiskLocatorLedResult>),
    /// A boxed array of *HostStorageSystemScsiLunResult*. To be used in *Any* placeholders.
    ArrayOfHostStorageSystemScsiLunResult(Vec<HostStorageSystemScsiLunResult>),
    /// A boxed array of *HostStorageSystemVmfsVolumeResult*. To be used in *Any* placeholders.
    ArrayOfHostStorageSystemVmfsVolumeResult(Vec<HostStorageSystemVmfsVolumeResult>),
    /// A boxed array of *HostListSummary*. To be used in *Any* placeholders.
    ArrayOfHostListSummary(Vec<HostListSummary>),
    /// A boxed array of *HostConfigSummary*. To be used in *Any* placeholders.
    ArrayOfHostConfigSummary(Vec<HostConfigSummary>),
    /// A boxed array of *HostListSummaryGatewaySummary*. To be used in *Any* placeholders.
    ArrayOfHostListSummaryGatewaySummary(Vec<HostListSummaryGatewaySummary>),
    /// A boxed array of *HostHardwareSummary*. To be used in *Any* placeholders.
    ArrayOfHostHardwareSummary(Vec<HostHardwareSummary>),
    /// A boxed array of *HostListSummaryQuickStats*. To be used in *Any* placeholders.
    ArrayOfHostListSummaryQuickStats(Vec<HostListSummaryQuickStats>),
    /// A boxed array of *SystemEventInfo*. To be used in *Any* placeholders.
    ArrayOfSystemEventInfo(Vec<SystemEventInfo>),
    /// A boxed array of *HostSystemHealthInfo*. To be used in *Any* placeholders.
    ArrayOfHostSystemHealthInfo(Vec<HostSystemHealthInfo>),
    /// A boxed array of *HostSystemIdentificationInfo*. To be used in *Any* placeholders.
    ArrayOfHostSystemIdentificationInfo(Vec<HostSystemIdentificationInfo>),
    /// A boxed array of *HostSystemInfo*. To be used in *Any* placeholders.
    ArrayOfHostSystemInfo(Vec<HostSystemInfo>),
    /// A boxed array of *HostSystemResourceInfo*. To be used in *Any* placeholders.
    ArrayOfHostSystemResourceInfo(Vec<HostSystemResourceInfo>),
    /// A boxed array of *HostSystemSwapConfiguration*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfiguration(Vec<HostSystemSwapConfiguration>),
    /// A boxed array of *HostSystemSwapConfigurationDatastoreOption*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfigurationDatastoreOption(Vec<HostSystemSwapConfigurationDatastoreOption>),
    /// A boxed array of *HostSystemSwapConfigurationDisabledOption*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfigurationDisabledOption(Vec<HostSystemSwapConfigurationDisabledOption>),
    /// A boxed array of *HostSystemSwapConfigurationHostCacheOption*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfigurationHostCacheOption(Vec<HostSystemSwapConfigurationHostCacheOption>),
    /// A boxed array of *HostSystemSwapConfigurationHostLocalSwapOption*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfigurationHostLocalSwapOption(Vec<HostSystemSwapConfigurationHostLocalSwapOption>),
    /// A boxed array of *HostSystemSwapConfigurationSystemSwapOption*. To be used in *Any* placeholders.
    ArrayOfHostSystemSwapConfigurationSystemSwapOption(Vec<Box<dyn super::traits::HostSystemSwapConfigurationSystemSwapOptionTrait>>),
    /// A boxed array of *HostTargetTransport*. To be used in *Any* placeholders.
    ArrayOfHostTargetTransport(Vec<Box<dyn super::traits::HostTargetTransportTrait>>),
    /// A boxed array of *HostTcpHba*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostTcpHba(Vec<HostTcpHba>),
    /// A boxed array of *HostTcpHbaCreateSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostTcpHbaCreateSpec(Vec<HostTcpHbaCreateSpec>),
    /// A boxed array of *HostTcpTargetTransport*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfHostTcpTargetTransport(Vec<HostTcpTargetTransport>),
    /// A boxed array of *HostTpmAttestationInfo*. To be used in *Any* placeholders.
    ArrayOfHostTpmAttestationInfo(Vec<HostTpmAttestationInfo>),
    /// A boxed array of *HostTpmAttestationReport*. To be used in *Any* placeholders.
    ArrayOfHostTpmAttestationReport(Vec<HostTpmAttestationReport>),
    /// A boxed array of *HostTpmBootCompleteEventDetails*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.1.0
    ArrayOfHostTpmBootCompleteEventDetails(Vec<HostTpmBootCompleteEventDetails>),
    /// A boxed array of *HostTpmBootSecurityOptionEventDetails*. To be used in *Any* placeholders.
    ArrayOfHostTpmBootSecurityOptionEventDetails(Vec<Box<dyn super::traits::HostTpmBootSecurityOptionEventDetailsTrait>>),
    /// A boxed array of *HostTpmCommandEventDetails*. To be used in *Any* placeholders.
    ArrayOfHostTpmCommandEventDetails(Vec<HostTpmCommandEventDetails>),
    /// A boxed array of *HostTpmDigestInfo*. To be used in *Any* placeholders.
    ArrayOfHostTpmDigestInfo(Vec<HostTpmDigestInfo>),
    /// A boxed array of *HostTpmEventDetails*. To be used in *Any* placeholders.
    ArrayOfHostTpmEventDetails(Vec<Box<dyn super::traits::HostTpmEventDetailsTrait>>),
    /// A boxed array of *HostTpmEventLogEntry*. To be used in *Any* placeholders.
    ArrayOfHostTpmEventLogEntry(Vec<HostTpmEventLogEntry>),
    /// A boxed array of *HostTpmNvTagEventDetails*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ArrayOfHostTpmNvTagEventDetails(Vec<HostTpmNvTagEventDetails>),
    /// A boxed array of *HostTpmOptionEventDetails*. To be used in *Any* placeholders.
    ArrayOfHostTpmOptionEventDetails(Vec<HostTpmOptionEventDetails>),
    /// A boxed array of *HostTpmSignerEventDetails*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostTpmSignerEventDetails(Vec<HostTpmSignerEventDetails>),
    /// A boxed array of *HostTpmSoftwareComponentEventDetails*. To be used in *Any* placeholders.
    ArrayOfHostTpmSoftwareComponentEventDetails(Vec<HostTpmSoftwareComponentEventDetails>),
    /// A boxed array of *HostTpmVersionEventDetails*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfHostTpmVersionEventDetails(Vec<HostTpmVersionEventDetails>),
    /// A boxed array of *HostTrustAuthorityAttestationInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ArrayOfHostTrustAuthorityAttestationInfo(Vec<HostTrustAuthorityAttestationInfo>),
    /// A boxed array of *HostUnresolvedVmfsExtent*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsExtent(Vec<HostUnresolvedVmfsExtent>),
    /// A boxed array of *HostUnresolvedVmfsResignatureSpec*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsResignatureSpec(Vec<HostUnresolvedVmfsResignatureSpec>),
    /// A boxed array of *HostUnresolvedVmfsResolutionResult*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsResolutionResult(Vec<HostUnresolvedVmfsResolutionResult>),
    /// A boxed array of *HostUnresolvedVmfsResolutionSpec*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsResolutionSpec(Vec<HostUnresolvedVmfsResolutionSpec>),
    /// A boxed array of *HostUnresolvedVmfsVolume*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsVolume(Vec<HostUnresolvedVmfsVolume>),
    /// A boxed array of *HostUnresolvedVmfsVolumeResolveStatus*. To be used in *Any* placeholders.
    ArrayOfHostUnresolvedVmfsVolumeResolveStatus(Vec<HostUnresolvedVmfsVolumeResolveStatus>),
    /// A boxed array of *HostVFlashManagerVFlashCacheConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashCacheConfigInfo(Vec<HostVFlashManagerVFlashCacheConfigInfo>),
    /// A boxed array of *HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption(Vec<HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption>),
    /// A boxed array of *HostVFlashManagerVFlashCacheConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashCacheConfigSpec(Vec<HostVFlashManagerVFlashCacheConfigSpec>),
    /// A boxed array of *HostVFlashManagerVFlashConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashConfigInfo(Vec<HostVFlashManagerVFlashConfigInfo>),
    /// A boxed array of *HostVFlashManagerVFlashResourceConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashResourceConfigInfo(Vec<HostVFlashManagerVFlashResourceConfigInfo>),
    /// A boxed array of *HostVFlashManagerVFlashResourceConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashResourceConfigSpec(Vec<HostVFlashManagerVFlashResourceConfigSpec>),
    /// A boxed array of *HostVFlashManagerVFlashResourceRunTimeInfo*. To be used in *Any* placeholders.
    ArrayOfHostVFlashManagerVFlashResourceRunTimeInfo(Vec<HostVFlashManagerVFlashResourceRunTimeInfo>),
    /// A boxed array of *HostVFlashResourceConfigurationResult*. To be used in *Any* placeholders.
    ArrayOfHostVFlashResourceConfigurationResult(Vec<HostVFlashResourceConfigurationResult>),
    /// A boxed array of *HostVMotionConfig*. To be used in *Any* placeholders.
    ArrayOfHostVMotionConfig(Vec<HostVMotionConfig>),
    /// A boxed array of *HostVMotionInfo*. To be used in *Any* placeholders.
    ArrayOfHostVMotionInfo(Vec<HostVMotionInfo>),
    /// A boxed array of *HostVMotionManagerDstInstantCloneResult*. To be used in *Any* placeholders.
    ArrayOfHostVMotionManagerDstInstantCloneResult(Vec<HostVMotionManagerDstInstantCloneResult>),
    /// A boxed array of *HostVMotionManagerSrcInstantCloneResult*. To be used in *Any* placeholders.
    ArrayOfHostVMotionManagerSrcInstantCloneResult(Vec<HostVMotionManagerSrcInstantCloneResult>),
    /// A boxed array of *HostVMotionNetConfig*. To be used in *Any* placeholders.
    ArrayOfHostVMotionNetConfig(Vec<HostVMotionNetConfig>),
    /// A boxed array of *HostVfatVolume*. To be used in *Any* placeholders.
    ArrayOfHostVfatVolume(Vec<HostVfatVolume>),
    /// A boxed array of *HostVffsVolume*. To be used in *Any* placeholders.
    ArrayOfHostVffsVolume(Vec<HostVffsVolume>),
    /// A boxed array of *HostVffsSpec*. To be used in *Any* placeholders.
    ArrayOfHostVffsSpec(Vec<HostVffsSpec>),
    /// A boxed array of *HostVirtualNic*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNic(Vec<HostVirtualNic>),
    /// A boxed array of *HostVirtualNicConfig*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicConfig(Vec<HostVirtualNicConfig>),
    /// A boxed array of *HostVirtualNicIpRouteSpec*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicIpRouteSpec(Vec<HostVirtualNicIpRouteSpec>),
    /// A boxed array of *HostVirtualNicOpaqueNetworkSpec*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicOpaqueNetworkSpec(Vec<HostVirtualNicOpaqueNetworkSpec>),
    /// A boxed array of *HostVirtualNicSpec*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicSpec(Vec<HostVirtualNicSpec>),
    /// A boxed array of *HostVirtualNicConnection*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicConnection(Vec<HostVirtualNicConnection>),
    /// A boxed array of *VirtualNicManagerNetConfig*. To be used in *Any* placeholders.
    ArrayOfVirtualNicManagerNetConfig(Vec<VirtualNicManagerNetConfig>),
    /// A boxed array of *HostVirtualNicManagerNicTypeSelection*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicManagerNicTypeSelection(Vec<HostVirtualNicManagerNicTypeSelection>),
    /// A boxed array of *HostVirtualNicManagerInfo*. To be used in *Any* placeholders.
    ArrayOfHostVirtualNicManagerInfo(Vec<HostVirtualNicManagerInfo>),
    /// A boxed array of *HostVirtualSwitch*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitch(Vec<HostVirtualSwitch>),
    /// A boxed array of *HostVirtualSwitchAutoBridge*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchAutoBridge(Vec<HostVirtualSwitchAutoBridge>),
    /// A boxed array of *HostVirtualSwitchBeaconConfig*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchBeaconConfig(Vec<HostVirtualSwitchBeaconConfig>),
    /// A boxed array of *HostVirtualSwitchBondBridge*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchBondBridge(Vec<HostVirtualSwitchBondBridge>),
    /// A boxed array of *HostVirtualSwitchBridge*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchBridge(Vec<Box<dyn super::traits::HostVirtualSwitchBridgeTrait>>),
    /// A boxed array of *HostVirtualSwitchConfig*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchConfig(Vec<HostVirtualSwitchConfig>),
    /// A boxed array of *HostVirtualSwitchSimpleBridge*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchSimpleBridge(Vec<HostVirtualSwitchSimpleBridge>),
    /// A boxed array of *HostVirtualSwitchSpec*. To be used in *Any* placeholders.
    ArrayOfHostVirtualSwitchSpec(Vec<HostVirtualSwitchSpec>),
    /// A boxed array of *HostVmciAccessManagerAccessSpec*. To be used in *Any* placeholders.
    ArrayOfHostVmciAccessManagerAccessSpec(Vec<HostVmciAccessManagerAccessSpec>),
    /// A boxed array of *VmfsDatastoreCreateSpec*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreCreateSpec(Vec<VmfsDatastoreCreateSpec>),
    /// A boxed array of *VmfsDatastoreExpandSpec*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreExpandSpec(Vec<VmfsDatastoreExpandSpec>),
    /// A boxed array of *VmfsDatastoreExtendSpec*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreExtendSpec(Vec<VmfsDatastoreExtendSpec>),
    /// A boxed array of *VmfsDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreInfo(Vec<VmfsDatastoreInfo>),
    /// A boxed array of *VmfsDatastoreOption*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreOption(Vec<VmfsDatastoreOption>),
    /// A boxed array of *VmfsDatastoreAllExtentOption*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreAllExtentOption(Vec<VmfsDatastoreAllExtentOption>),
    /// A boxed array of *VmfsDatastoreBaseOption*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreBaseOption(Vec<Box<dyn super::traits::VmfsDatastoreBaseOptionTrait>>),
    /// A boxed array of *VmfsDatastoreMultipleExtentOption*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreMultipleExtentOption(Vec<VmfsDatastoreMultipleExtentOption>),
    /// A boxed array of *VmfsDatastoreSingleExtentOption*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreSingleExtentOption(Vec<Box<dyn super::traits::VmfsDatastoreSingleExtentOptionTrait>>),
    /// A boxed array of *VmfsDatastoreSpec*. To be used in *Any* placeholders.
    ArrayOfVmfsDatastoreSpec(Vec<Box<dyn super::traits::VmfsDatastoreSpecTrait>>),
    /// A boxed array of *HostVmfsRescanResult*. To be used in *Any* placeholders.
    ArrayOfHostVmfsRescanResult(Vec<HostVmfsRescanResult>),
    /// A boxed array of *HostVmfsVolume*. To be used in *Any* placeholders.
    ArrayOfHostVmfsVolume(Vec<HostVmfsVolume>),
    /// A boxed array of *VmfsConfigOption*. To be used in *Any* placeholders.
    ArrayOfVmfsConfigOption(Vec<VmfsConfigOption>),
    /// A boxed array of *HostVmfsSpec*. To be used in *Any* placeholders.
    ArrayOfHostVmfsSpec(Vec<HostVmfsSpec>),
    /// A boxed array of *VmfsUnmapBandwidthSpec*. To be used in *Any* placeholders.
    ArrayOfVmfsUnmapBandwidthSpec(Vec<VmfsUnmapBandwidthSpec>),
    /// A boxed array of *VsanDatastoreInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.1.0
    ArrayOfVsanDatastoreInfo(Vec<VsanDatastoreInfo>),
    /// A boxed array of *HostVsanInternalSystemCmmdsQuery*. To be used in *Any* placeholders.
    ArrayOfHostVsanInternalSystemCmmdsQuery(Vec<HostVsanInternalSystemCmmdsQuery>),
    /// A boxed array of *HostVsanInternalSystemDeleteVsanObjectsResult*. To be used in *Any* placeholders.
    ArrayOfHostVsanInternalSystemDeleteVsanObjectsResult(Vec<HostVsanInternalSystemDeleteVsanObjectsResult>),
    /// A boxed array of *VsanNewPolicyBatch*. To be used in *Any* placeholders.
    ArrayOfVsanNewPolicyBatch(Vec<VsanNewPolicyBatch>),
    /// A boxed array of *VsanPolicyChangeBatch*. To be used in *Any* placeholders.
    ArrayOfVsanPolicyChangeBatch(Vec<VsanPolicyChangeBatch>),
    /// A boxed array of *VsanPolicyCost*. To be used in *Any* placeholders.
    ArrayOfVsanPolicyCost(Vec<VsanPolicyCost>),
    /// A boxed array of *VsanPolicySatisfiability*. To be used in *Any* placeholders.
    ArrayOfVsanPolicySatisfiability(Vec<VsanPolicySatisfiability>),
    /// A boxed array of *HostVsanInternalSystemVsanObjectOperationResult*. To be used in *Any* placeholders.
    ArrayOfHostVsanInternalSystemVsanObjectOperationResult(Vec<HostVsanInternalSystemVsanObjectOperationResult>),
    /// A boxed array of *HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult*. To be used in *Any* placeholders.
    ArrayOfHostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult(Vec<HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult>),
    /// A boxed array of *VvolDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfVvolDatastoreInfo(Vec<VvolDatastoreInfo>),
    /// A boxed array of *HostVvolNQN*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    #[serde(rename = "ArrayOfHostVvolNQN")]
    ArrayOfHostVvolNqn(Vec<HostVvolNqn>),
    /// A boxed array of *HostVvolVolume*. To be used in *Any* placeholders.
    ArrayOfHostVvolVolume(Vec<HostVvolVolume>),
    /// A boxed array of *VVolHostPE*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVVolHostPE")]
    ArrayOfVVolHostPe(Vec<VVolHostPe>),
    /// A boxed array of *HostVvolVolumeHostVvolNQN*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    #[serde(rename = "ArrayOfHostVvolVolumeHostVvolNQN")]
    ArrayOfHostVvolVolumeHostVvolNqn(Vec<HostVvolVolumeHostVvolNqn>),
    /// A boxed array of *HostVvolVolumeSpecification*. To be used in *Any* placeholders.
    ArrayOfHostVvolVolumeSpecification(Vec<HostVvolVolumeSpecification>),
    /// A boxed array of *NetDhcpConfigInfo*. To be used in *Any* placeholders.
    ArrayOfNetDhcpConfigInfo(Vec<NetDhcpConfigInfo>),
    /// A boxed array of *NetDhcpConfigInfoDhcpOptions*. To be used in *Any* placeholders.
    ArrayOfNetDhcpConfigInfoDhcpOptions(Vec<NetDhcpConfigInfoDhcpOptions>),
    /// A boxed array of *NetDhcpConfigSpec*. To be used in *Any* placeholders.
    ArrayOfNetDhcpConfigSpec(Vec<NetDhcpConfigSpec>),
    /// A boxed array of *NetDhcpConfigSpecDhcpOptionsSpec*. To be used in *Any* placeholders.
    ArrayOfNetDhcpConfigSpecDhcpOptionsSpec(Vec<NetDhcpConfigSpecDhcpOptionsSpec>),
    /// A boxed array of *NetDnsConfigInfo*. To be used in *Any* placeholders.
    ArrayOfNetDnsConfigInfo(Vec<NetDnsConfigInfo>),
    /// A boxed array of *NetDnsConfigSpec*. To be used in *Any* placeholders.
    ArrayOfNetDnsConfigSpec(Vec<NetDnsConfigSpec>),
    /// A boxed array of *NetIpConfigInfo*. To be used in *Any* placeholders.
    ArrayOfNetIpConfigInfo(Vec<NetIpConfigInfo>),
    /// A boxed array of *NetIpConfigInfoIpAddress*. To be used in *Any* placeholders.
    ArrayOfNetIpConfigInfoIpAddress(Vec<NetIpConfigInfoIpAddress>),
    /// A boxed array of *NetIpConfigSpec*. To be used in *Any* placeholders.
    ArrayOfNetIpConfigSpec(Vec<NetIpConfigSpec>),
    /// A boxed array of *NetIpConfigSpecIpAddressSpec*. To be used in *Any* placeholders.
    ArrayOfNetIpConfigSpecIpAddressSpec(Vec<NetIpConfigSpecIpAddressSpec>),
    /// A boxed array of *NetIpRouteConfigInfo*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigInfo(Vec<NetIpRouteConfigInfo>),
    /// A boxed array of *NetIpRouteConfigInfoGateway*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigInfoGateway(Vec<NetIpRouteConfigInfoGateway>),
    /// A boxed array of *NetIpRouteConfigInfoIpRoute*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigInfoIpRoute(Vec<NetIpRouteConfigInfoIpRoute>),
    /// A boxed array of *NetIpRouteConfigSpec*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigSpec(Vec<NetIpRouteConfigSpec>),
    /// A boxed array of *NetIpRouteConfigSpecGatewaySpec*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigSpecGatewaySpec(Vec<NetIpRouteConfigSpecGatewaySpec>),
    /// A boxed array of *NetIpRouteConfigSpecIpRouteSpec*. To be used in *Any* placeholders.
    ArrayOfNetIpRouteConfigSpecIpRouteSpec(Vec<NetIpRouteConfigSpecIpRouteSpec>),
    /// A boxed array of *NetIpStackInfo*. To be used in *Any* placeholders.
    ArrayOfNetIpStackInfo(Vec<NetIpStackInfo>),
    /// A boxed array of *NetIpStackInfoDefaultRouter*. To be used in *Any* placeholders.
    ArrayOfNetIpStackInfoDefaultRouter(Vec<NetIpStackInfoDefaultRouter>),
    /// A boxed array of *NetIpStackInfoNetToMedia*. To be used in *Any* placeholders.
    ArrayOfNetIpStackInfoNetToMedia(Vec<NetIpStackInfoNetToMedia>),
    /// A boxed array of *NetBIOSConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfNetBIOSConfigInfo")]
    ArrayOfNetBiosConfigInfo(Vec<Box<dyn super::traits::NetBiosConfigInfoTrait>>),
    /// A boxed array of *WinNetBIOSConfigInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfWinNetBIOSConfigInfo")]
    ArrayOfWinNetBiosConfigInfo(Vec<WinNetBiosConfigInfo>),
    /// A boxed array of *ArrayUpdateSpec*. To be used in *Any* placeholders.
    ArrayOfArrayUpdateSpec(Vec<Box<dyn super::traits::ArrayUpdateSpecTrait>>),
    /// A boxed array of *BoolOption*. To be used in *Any* placeholders.
    ArrayOfBoolOption(Vec<BoolOption>),
    /// A boxed array of *ChoiceOption*. To be used in *Any* placeholders.
    ArrayOfChoiceOption(Vec<ChoiceOption>),
    /// A boxed array of *FloatOption*. To be used in *Any* placeholders.
    ArrayOfFloatOption(Vec<FloatOption>),
    /// A boxed array of *IntOption*. To be used in *Any* placeholders.
    ArrayOfIntOption(Vec<IntOption>),
    /// A boxed array of *LongOption*. To be used in *Any* placeholders.
    ArrayOfLongOption(Vec<LongOption>),
    /// A boxed array of *OptionDef*. To be used in *Any* placeholders.
    ArrayOfOptionDef(Vec<OptionDef>),
    /// A boxed array of *OptionType*. To be used in *Any* placeholders.
    ArrayOfOptionType(Vec<Box<dyn super::traits::OptionTypeTrait>>),
    /// A boxed array of *OptionValue*. To be used in *Any* placeholders.
    ArrayOfOptionValue(Vec<Box<dyn super::traits::OptionValueTrait>>),
    /// A boxed array of *StringOption*. To be used in *Any* placeholders.
    ArrayOfStringOption(Vec<StringOption>),
    /// A boxed array of *ApplyProfile*. To be used in *Any* placeholders.
    ArrayOfApplyProfile(Vec<Box<dyn super::traits::ApplyProfileTrait>>),
    /// A boxed array of *ProfileApplyProfileElement*. To be used in *Any* placeholders.
    ArrayOfProfileApplyProfileElement(Vec<ProfileApplyProfileElement>),
    /// A boxed array of *ProfileApplyProfileProperty*. To be used in *Any* placeholders.
    ArrayOfProfileApplyProfileProperty(Vec<ProfileApplyProfileProperty>),
    /// A boxed array of *ComplianceLocator*. To be used in *Any* placeholders.
    ArrayOfComplianceLocator(Vec<ComplianceLocator>),
    /// A boxed array of *ComplianceProfile*. To be used in *Any* placeholders.
    ArrayOfComplianceProfile(Vec<ComplianceProfile>),
    /// A boxed array of *ComplianceResult*. To be used in *Any* placeholders.
    ArrayOfComplianceResult(Vec<ComplianceResult>),
    /// A boxed array of *ComplianceFailure*. To be used in *Any* placeholders.
    ArrayOfComplianceFailure(Vec<ComplianceFailure>),
    /// A boxed array of *ComplianceFailureComplianceFailureValues*. To be used in *Any* placeholders.
    ArrayOfComplianceFailureComplianceFailureValues(Vec<ComplianceFailureComplianceFailureValues>),
    /// A boxed array of *ProfileCompositeExpression*. To be used in *Any* placeholders.
    ArrayOfProfileCompositeExpression(Vec<ProfileCompositeExpression>),
    /// A boxed array of *CompositePolicyOption*. To be used in *Any* placeholders.
    ArrayOfCompositePolicyOption(Vec<CompositePolicyOption>),
    /// A boxed array of *ProfileCompositePolicyOptionMetadata*. To be used in *Any* placeholders.
    ArrayOfProfileCompositePolicyOptionMetadata(Vec<ProfileCompositePolicyOptionMetadata>),
    /// A boxed array of *ProfileDeferredPolicyOptionParameter*. To be used in *Any* placeholders.
    ArrayOfProfileDeferredPolicyOptionParameter(Vec<ProfileDeferredPolicyOptionParameter>),
    /// A boxed array of *ProfileExpression*. To be used in *Any* placeholders.
    ArrayOfProfileExpression(Vec<Box<dyn super::traits::ProfileExpressionTrait>>),
    /// A boxed array of *ProfileExpressionMetadata*. To be used in *Any* placeholders.
    ArrayOfProfileExpressionMetadata(Vec<ProfileExpressionMetadata>),
    /// A boxed array of *ProfileParameterMetadata*. To be used in *Any* placeholders.
    ArrayOfProfileParameterMetadata(Vec<ProfileParameterMetadata>),
    /// A boxed array of *ProfileParameterMetadataParameterRelationMetadata*. To be used in *Any* placeholders.
    ArrayOfProfileParameterMetadataParameterRelationMetadata(Vec<ProfileParameterMetadataParameterRelationMetadata>),
    /// A boxed array of *ProfilePolicy*. To be used in *Any* placeholders.
    ArrayOfProfilePolicy(Vec<ProfilePolicy>),
    /// A boxed array of *ProfilePolicyMetadata*. To be used in *Any* placeholders.
    ArrayOfProfilePolicyMetadata(Vec<ProfilePolicyMetadata>),
    /// A boxed array of *PolicyOption*. To be used in *Any* placeholders.
    ArrayOfPolicyOption(Vec<Box<dyn super::traits::PolicyOptionTrait>>),
    /// A boxed array of *ProfilePolicyOptionMetadata*. To be used in *Any* placeholders.
    ArrayOfProfilePolicyOptionMetadata(Vec<Box<dyn super::traits::ProfilePolicyOptionMetadataTrait>>),
    /// A boxed array of *ProfileConfigInfo*. To be used in *Any* placeholders.
    ArrayOfProfileConfigInfo(Vec<Box<dyn super::traits::ProfileConfigInfoTrait>>),
    /// A boxed array of *ProfileCreateSpec*. To be used in *Any* placeholders.
    ArrayOfProfileCreateSpec(Vec<Box<dyn super::traits::ProfileCreateSpecTrait>>),
    /// A boxed array of *ProfileDescription*. To be used in *Any* placeholders.
    ArrayOfProfileDescription(Vec<ProfileDescription>),
    /// A boxed array of *ProfileDescriptionSection*. To be used in *Any* placeholders.
    ArrayOfProfileDescriptionSection(Vec<ProfileDescriptionSection>),
    /// A boxed array of *ProfileSerializedCreateSpec*. To be used in *Any* placeholders.
    ArrayOfProfileSerializedCreateSpec(Vec<Box<dyn super::traits::ProfileSerializedCreateSpecTrait>>),
    /// A boxed array of *ProfileMetadata*. To be used in *Any* placeholders.
    ArrayOfProfileMetadata(Vec<ProfileMetadata>),
    /// A boxed array of *ProfileMetadataProfileOperationMessage*. To be used in *Any* placeholders.
    ArrayOfProfileMetadataProfileOperationMessage(Vec<ProfileMetadataProfileOperationMessage>),
    /// A boxed array of *ProfileMetadataProfileSortSpec*. To be used in *Any* placeholders.
    ArrayOfProfileMetadataProfileSortSpec(Vec<ProfileMetadataProfileSortSpec>),
    /// A boxed array of *ProfilePropertyPath*. To be used in *Any* placeholders.
    ArrayOfProfilePropertyPath(Vec<ProfilePropertyPath>),
    /// A boxed array of *ProfileProfileStructure*. To be used in *Any* placeholders.
    ArrayOfProfileProfileStructure(Vec<ProfileProfileStructure>),
    /// A boxed array of *ProfileProfileStructureProperty*. To be used in *Any* placeholders.
    ArrayOfProfileProfileStructureProperty(Vec<ProfileProfileStructureProperty>),
    /// A boxed array of *ProfileSimpleExpression*. To be used in *Any* placeholders.
    ArrayOfProfileSimpleExpression(Vec<ProfileSimpleExpression>),
    /// A boxed array of *UserInputRequiredParameterMetadata*. To be used in *Any* placeholders.
    ArrayOfUserInputRequiredParameterMetadata(Vec<UserInputRequiredParameterMetadata>),
    /// A boxed array of *ClusterProfileCompleteConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterProfileCompleteConfigSpec(Vec<ClusterProfileCompleteConfigSpec>),
    /// A boxed array of *ClusterProfileConfigInfo*. To be used in *Any* placeholders.
    ArrayOfClusterProfileConfigInfo(Vec<ClusterProfileConfigInfo>),
    /// A boxed array of *ClusterProfileConfigServiceCreateSpec*. To be used in *Any* placeholders.
    ArrayOfClusterProfileConfigServiceCreateSpec(Vec<ClusterProfileConfigServiceCreateSpec>),
    /// A boxed array of *ClusterProfileConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterProfileConfigSpec(Vec<Box<dyn super::traits::ClusterProfileConfigSpecTrait>>),
    /// A boxed array of *ClusterProfileCreateSpec*. To be used in *Any* placeholders.
    ArrayOfClusterProfileCreateSpec(Vec<Box<dyn super::traits::ClusterProfileCreateSpecTrait>>),
    /// A boxed array of *ActiveDirectoryProfile*. To be used in *Any* placeholders.
    ArrayOfActiveDirectoryProfile(Vec<ActiveDirectoryProfile>),
    /// A boxed array of *AnswerFile*. To be used in *Any* placeholders.
    ArrayOfAnswerFile(Vec<AnswerFile>),
    /// A boxed array of *AnswerFileStatusResult*. To be used in *Any* placeholders.
    ArrayOfAnswerFileStatusResult(Vec<AnswerFileStatusResult>),
    /// A boxed array of *AnswerFileStatusError*. To be used in *Any* placeholders.
    ArrayOfAnswerFileStatusError(Vec<AnswerFileStatusError>),
    /// A boxed array of *AuthenticationProfile*. To be used in *Any* placeholders.
    ArrayOfAuthenticationProfile(Vec<AuthenticationProfile>),
    /// A boxed array of *DateTimeProfile*. To be used in *Any* placeholders.
    ArrayOfDateTimeProfile(Vec<DateTimeProfile>),
    /// A boxed array of *DvsHostVNicProfile*. To be used in *Any* placeholders.
    ArrayOfDvsHostVNicProfile(Vec<DvsHostVNicProfile>),
    /// A boxed array of *DvsProfile*. To be used in *Any* placeholders.
    ArrayOfDvsProfile(Vec<DvsProfile>),
    /// A boxed array of *DvsServiceConsoleVNicProfile*. To be used in *Any* placeholders.
    ArrayOfDvsServiceConsoleVNicProfile(Vec<DvsServiceConsoleVNicProfile>),
    /// A boxed array of *DvsVNicProfile*. To be used in *Any* placeholders.
    ArrayOfDvsVNicProfile(Vec<Box<dyn super::traits::DvsVNicProfileTrait>>),
    /// A boxed array of *ProfileExecuteResult*. To be used in *Any* placeholders.
    ArrayOfProfileExecuteResult(Vec<Box<dyn super::traits::ProfileExecuteResultTrait>>),
    /// A boxed array of *ProfileExecuteError*. To be used in *Any* placeholders.
    ArrayOfProfileExecuteError(Vec<ProfileExecuteError>),
    /// A boxed array of *FirewallProfile*. To be used in *Any* placeholders.
    ArrayOfFirewallProfile(Vec<FirewallProfile>),
    /// A boxed array of *FirewallProfileRulesetProfile*. To be used in *Any* placeholders.
    ArrayOfFirewallProfileRulesetProfile(Vec<FirewallProfileRulesetProfile>),
    /// A boxed array of *HostApplyProfile*. To be used in *Any* placeholders.
    ArrayOfHostApplyProfile(Vec<HostApplyProfile>),
    /// A boxed array of *HostMemoryProfile*. To be used in *Any* placeholders.
    ArrayOfHostMemoryProfile(Vec<HostMemoryProfile>),
    /// A boxed array of *HostPortGroupProfile*. To be used in *Any* placeholders.
    ArrayOfHostPortGroupProfile(Vec<HostPortGroupProfile>),
    /// A boxed array of *HostProfileCompleteConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostProfileCompleteConfigSpec(Vec<HostProfileCompleteConfigSpec>),
    /// A boxed array of *HostProfileConfigInfo*. To be used in *Any* placeholders.
    ArrayOfHostProfileConfigInfo(Vec<HostProfileConfigInfo>),
    /// A boxed array of *HostProfileConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostProfileConfigSpec(Vec<Box<dyn super::traits::HostProfileConfigSpecTrait>>),
    /// A boxed array of *HostProfileHostBasedConfigSpec*. To be used in *Any* placeholders.
    ArrayOfHostProfileHostBasedConfigSpec(Vec<HostProfileHostBasedConfigSpec>),
    /// A boxed array of *HostProfileSerializedHostProfileSpec*. To be used in *Any* placeholders.
    ArrayOfHostProfileSerializedHostProfileSpec(Vec<HostProfileSerializedHostProfileSpec>),
    /// A boxed array of *HostProfileValidationFailureInfo*. To be used in *Any* placeholders.
    ArrayOfHostProfileValidationFailureInfo(Vec<HostProfileValidationFailureInfo>),
    /// A boxed array of *HostSpecification*. To be used in *Any* placeholders.
    ArrayOfHostSpecification(Vec<HostSpecification>),
    /// A boxed array of *HostSubSpecification*. To be used in *Any* placeholders.
    ArrayOfHostSubSpecification(Vec<HostSubSpecification>),
    /// A boxed array of *IpAddressProfile*. To be used in *Any* placeholders.
    ArrayOfIpAddressProfile(Vec<IpAddressProfile>),
    /// A boxed array of *IpRouteProfile*. To be used in *Any* placeholders.
    ArrayOfIpRouteProfile(Vec<IpRouteProfile>),
    /// A boxed array of *NasStorageProfile*. To be used in *Any* placeholders.
    ArrayOfNasStorageProfile(Vec<NasStorageProfile>),
    /// A boxed array of *NetStackInstanceProfile*. To be used in *Any* placeholders.
    ArrayOfNetStackInstanceProfile(Vec<NetStackInstanceProfile>),
    /// A boxed array of *NetworkPolicyProfile*. To be used in *Any* placeholders.
    ArrayOfNetworkPolicyProfile(Vec<NetworkPolicyProfile>),
    /// A boxed array of *NetworkProfile*. To be used in *Any* placeholders.
    ArrayOfNetworkProfile(Vec<NetworkProfile>),
    /// A boxed array of *NetworkProfileDnsConfigProfile*. To be used in *Any* placeholders.
    ArrayOfNetworkProfileDnsConfigProfile(Vec<NetworkProfileDnsConfigProfile>),
    /// A boxed array of *NsxHostVNicProfile*. To be used in *Any* placeholders.
    ArrayOfNsxHostVNicProfile(Vec<NsxHostVNicProfile>),
    /// A boxed array of *OpaqueSwitchProfile*. To be used in *Any* placeholders.
    ArrayOfOpaqueSwitchProfile(Vec<OpaqueSwitchProfile>),
    /// A boxed array of *OptionProfile*. To be used in *Any* placeholders.
    ArrayOfOptionProfile(Vec<OptionProfile>),
    /// A boxed array of *PermissionProfile*. To be used in *Any* placeholders.
    ArrayOfPermissionProfile(Vec<PermissionProfile>),
    /// A boxed array of *PhysicalNicProfile*. To be used in *Any* placeholders.
    ArrayOfPhysicalNicProfile(Vec<PhysicalNicProfile>),
    /// A boxed array of *PnicUplinkProfile*. To be used in *Any* placeholders.
    ArrayOfPnicUplinkProfile(Vec<PnicUplinkProfile>),
    /// A boxed array of *PortGroupProfile*. To be used in *Any* placeholders.
    ArrayOfPortGroupProfile(Vec<Box<dyn super::traits::PortGroupProfileTrait>>),
    /// A boxed array of *VirtualSwitchSelectionProfile*. To be used in *Any* placeholders.
    ArrayOfVirtualSwitchSelectionProfile(Vec<VirtualSwitchSelectionProfile>),
    /// A boxed array of *VlanProfile*. To be used in *Any* placeholders.
    ArrayOfVlanProfile(Vec<VlanProfile>),
    /// A boxed array of *AnswerFileCreateSpec*. To be used in *Any* placeholders.
    ArrayOfAnswerFileCreateSpec(Vec<Box<dyn super::traits::AnswerFileCreateSpecTrait>>),
    /// A boxed array of *AnswerFileOptionsCreateSpec*. To be used in *Any* placeholders.
    ArrayOfAnswerFileOptionsCreateSpec(Vec<AnswerFileOptionsCreateSpec>),
    /// A boxed array of *AnswerFileSerializedCreateSpec*. To be used in *Any* placeholders.
    ArrayOfAnswerFileSerializedCreateSpec(Vec<AnswerFileSerializedCreateSpec>),
    /// A boxed array of *ApplyHostProfileConfigurationResult*. To be used in *Any* placeholders.
    ArrayOfApplyHostProfileConfigurationResult(Vec<ApplyHostProfileConfigurationResult>),
    /// A boxed array of *ApplyHostProfileConfigurationSpec*. To be used in *Any* placeholders.
    ArrayOfApplyHostProfileConfigurationSpec(Vec<ApplyHostProfileConfigurationSpec>),
    /// A boxed array of *HostProfileManagerCompositionResult*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerCompositionResult(Vec<HostProfileManagerCompositionResult>),
    /// A boxed array of *HostProfileManagerCompositionResultResultElement*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerCompositionResultResultElement(Vec<HostProfileManagerCompositionResultResultElement>),
    /// A boxed array of *HostProfileManagerCompositionValidationResult*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerCompositionValidationResult(Vec<HostProfileManagerCompositionValidationResult>),
    /// A boxed array of *HostProfileManagerCompositionValidationResultResultElement*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerCompositionValidationResultResultElement(Vec<HostProfileManagerCompositionValidationResultResultElement>),
    /// A boxed array of *HostProfileManagerConfigTaskList*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerConfigTaskList(Vec<HostProfileManagerConfigTaskList>),
    /// A boxed array of *HostProfilesEntityCustomizations*. To be used in *Any* placeholders.
    ArrayOfHostProfilesEntityCustomizations(Vec<Box<dyn super::traits::HostProfilesEntityCustomizationsTrait>>),
    /// A boxed array of *HostProfileManagerHostToConfigSpecMap*. To be used in *Any* placeholders.
    ArrayOfHostProfileManagerHostToConfigSpecMap(Vec<HostProfileManagerHostToConfigSpecMap>),
    /// A boxed array of *StructuredCustomizations*. To be used in *Any* placeholders.
    ArrayOfStructuredCustomizations(Vec<StructuredCustomizations>),
    /// A boxed array of *SecurityProfile*. To be used in *Any* placeholders.
    ArrayOfSecurityProfile(Vec<SecurityProfile>),
    /// A boxed array of *ServiceConsolePortGroupProfile*. To be used in *Any* placeholders.
    ArrayOfServiceConsolePortGroupProfile(Vec<ServiceConsolePortGroupProfile>),
    /// A boxed array of *ServiceProfile*. To be used in *Any* placeholders.
    ArrayOfServiceProfile(Vec<ServiceProfile>),
    /// A boxed array of *StaticRouteProfile*. To be used in *Any* placeholders.
    ArrayOfStaticRouteProfile(Vec<StaticRouteProfile>),
    /// A boxed array of *StorageProfile*. To be used in *Any* placeholders.
    ArrayOfStorageProfile(Vec<StorageProfile>),
    /// A boxed array of *UserGroupProfile*. To be used in *Any* placeholders.
    ArrayOfUserGroupProfile(Vec<UserGroupProfile>),
    /// A boxed array of *UserProfile*. To be used in *Any* placeholders.
    ArrayOfUserProfile(Vec<UserProfile>),
    /// A boxed array of *VirtualSwitchProfile*. To be used in *Any* placeholders.
    ArrayOfVirtualSwitchProfile(Vec<VirtualSwitchProfile>),
    /// A boxed array of *LinkProfile*. To be used in *Any* placeholders.
    ArrayOfLinkProfile(Vec<LinkProfile>),
    /// A boxed array of *NumPortsProfile*. To be used in *Any* placeholders.
    ArrayOfNumPortsProfile(Vec<NumPortsProfile>),
    /// A boxed array of *VmPortGroupProfile*. To be used in *Any* placeholders.
    ArrayOfVmPortGroupProfile(Vec<VmPortGroupProfile>),
    /// A boxed array of *AfterStartupTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfAfterStartupTaskScheduler(Vec<AfterStartupTaskScheduler>),
    /// A boxed array of *DailyTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfDailyTaskScheduler(Vec<Box<dyn super::traits::DailyTaskSchedulerTrait>>),
    /// A boxed array of *HourlyTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfHourlyTaskScheduler(Vec<Box<dyn super::traits::HourlyTaskSchedulerTrait>>),
    /// A boxed array of *MonthlyByDayTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfMonthlyByDayTaskScheduler(Vec<MonthlyByDayTaskScheduler>),
    /// A boxed array of *MonthlyByWeekdayTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfMonthlyByWeekdayTaskScheduler(Vec<MonthlyByWeekdayTaskScheduler>),
    /// A boxed array of *MonthlyTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfMonthlyTaskScheduler(Vec<Box<dyn super::traits::MonthlyTaskSchedulerTrait>>),
    /// A boxed array of *OnceTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfOnceTaskScheduler(Vec<OnceTaskScheduler>),
    /// A boxed array of *RecurrentTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfRecurrentTaskScheduler(Vec<Box<dyn super::traits::RecurrentTaskSchedulerTrait>>),
    /// A boxed array of *ScheduledTaskDescription*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskDescription(Vec<ScheduledTaskDescription>),
    /// A boxed array of *ScheduledTaskDetail*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskDetail(Vec<ScheduledTaskDetail>),
    /// A boxed array of *ScheduledTaskInfo*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskInfo(Vec<ScheduledTaskInfo>),
    /// A boxed array of *ScheduledTaskSpec*. To be used in *Any* placeholders.
    ArrayOfScheduledTaskSpec(Vec<Box<dyn super::traits::ScheduledTaskSpecTrait>>),
    /// A boxed array of *TaskScheduler*. To be used in *Any* placeholders.
    ArrayOfTaskScheduler(Vec<Box<dyn super::traits::TaskSchedulerTrait>>),
    /// A boxed array of *WeeklyTaskScheduler*. To be used in *Any* placeholders.
    ArrayOfWeeklyTaskScheduler(Vec<WeeklyTaskScheduler>),
    /// A boxed array of *ApplyStorageRecommendationResult*. To be used in *Any* placeholders.
    ArrayOfApplyStorageRecommendationResult(Vec<ApplyStorageRecommendationResult>),
    /// A boxed array of *StorageDrsAutomationConfig*. To be used in *Any* placeholders.
    ArrayOfStorageDrsAutomationConfig(Vec<StorageDrsAutomationConfig>),
    /// A boxed array of *StorageDrsConfigInfo*. To be used in *Any* placeholders.
    ArrayOfStorageDrsConfigInfo(Vec<StorageDrsConfigInfo>),
    /// A boxed array of *StorageDrsConfigSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsConfigSpec(Vec<StorageDrsConfigSpec>),
    /// A boxed array of *HbrDiskMigrationAction*. To be used in *Any* placeholders.
    ArrayOfHbrDiskMigrationAction(Vec<HbrDiskMigrationAction>),
    /// A boxed array of *StorageDrsIoLoadBalanceConfig*. To be used in *Any* placeholders.
    ArrayOfStorageDrsIoLoadBalanceConfig(Vec<StorageDrsIoLoadBalanceConfig>),
    /// A boxed array of *StorageDrsOptionSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsOptionSpec(Vec<StorageDrsOptionSpec>),
    /// A boxed array of *PlacementAffinityRule*. To be used in *Any* placeholders.
    ArrayOfPlacementAffinityRule(Vec<PlacementAffinityRule>),
    /// A boxed array of *PlacementRankResult*. To be used in *Any* placeholders.
    ArrayOfPlacementRankResult(Vec<PlacementRankResult>),
    /// A boxed array of *PlacementRankSpec*. To be used in *Any* placeholders.
    ArrayOfPlacementRankSpec(Vec<PlacementRankSpec>),
    /// A boxed array of *StorageDrsPlacementRankVmSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsPlacementRankVmSpec(Vec<StorageDrsPlacementRankVmSpec>),
    /// A boxed array of *StorageDrsPodConfigInfo*. To be used in *Any* placeholders.
    ArrayOfStorageDrsPodConfigInfo(Vec<StorageDrsPodConfigInfo>),
    /// A boxed array of *StorageDrsPodConfigSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsPodConfigSpec(Vec<StorageDrsPodConfigSpec>),
    /// A boxed array of *StorageDrsPodSelectionSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsPodSelectionSpec(Vec<StorageDrsPodSelectionSpec>),
    /// A boxed array of *PodDiskLocator*. To be used in *Any* placeholders.
    ArrayOfPodDiskLocator(Vec<PodDiskLocator>),
    /// A boxed array of *VmPodConfigForPlacement*. To be used in *Any* placeholders.
    ArrayOfVmPodConfigForPlacement(Vec<VmPodConfigForPlacement>),
    /// A boxed array of *StorageDrsSpaceLoadBalanceConfig*. To be used in *Any* placeholders.
    ArrayOfStorageDrsSpaceLoadBalanceConfig(Vec<StorageDrsSpaceLoadBalanceConfig>),
    /// A boxed array of *StorageMigrationAction*. To be used in *Any* placeholders.
    ArrayOfStorageMigrationAction(Vec<StorageMigrationAction>),
    /// A boxed array of *StoragePlacementAction*. To be used in *Any* placeholders.
    ArrayOfStoragePlacementAction(Vec<StoragePlacementAction>),
    /// A boxed array of *StoragePlacementResult*. To be used in *Any* placeholders.
    ArrayOfStoragePlacementResult(Vec<StoragePlacementResult>),
    /// A boxed array of *StoragePlacementSpec*. To be used in *Any* placeholders.
    ArrayOfStoragePlacementSpec(Vec<StoragePlacementSpec>),
    /// A boxed array of *VirtualDiskAntiAffinityRuleSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskAntiAffinityRuleSpec(Vec<VirtualDiskAntiAffinityRuleSpec>),
    /// A boxed array of *VirtualDiskRuleSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskRuleSpec(Vec<VirtualDiskRuleSpec>),
    /// A boxed array of *StorageDrsVmConfigInfo*. To be used in *Any* placeholders.
    ArrayOfStorageDrsVmConfigInfo(Vec<StorageDrsVmConfigInfo>),
    /// A boxed array of *StorageDrsVmConfigSpec*. To be used in *Any* placeholders.
    ArrayOfStorageDrsVmConfigSpec(Vec<StorageDrsVmConfigSpec>),
    /// A boxed array of *VAppCloneSpec*. To be used in *Any* placeholders.
    ArrayOfVAppCloneSpec(Vec<VAppCloneSpec>),
    /// A boxed array of *VAppCloneSpecNetworkMappingPair*. To be used in *Any* placeholders.
    ArrayOfVAppCloneSpecNetworkMappingPair(Vec<VAppCloneSpecNetworkMappingPair>),
    /// A boxed array of *VAppCloneSpecResourceMap*. To be used in *Any* placeholders.
    ArrayOfVAppCloneSpecResourceMap(Vec<VAppCloneSpecResourceMap>),
    /// A boxed array of *VAppEntityConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVAppEntityConfigInfo(Vec<VAppEntityConfigInfo>),
    /// A boxed array of *VAppIPAssignmentInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVAppIPAssignmentInfo")]
    ArrayOfVAppIpAssignmentInfo(Vec<VAppIpAssignmentInfo>),
    /// A boxed array of *IpPool*. To be used in *Any* placeholders.
    ArrayOfIpPool(Vec<IpPool>),
    /// A boxed array of *IpPoolAssociation*. To be used in *Any* placeholders.
    ArrayOfIpPoolAssociation(Vec<IpPoolAssociation>),
    /// A boxed array of *IpPoolIpPoolConfigInfo*. To be used in *Any* placeholders.
    ArrayOfIpPoolIpPoolConfigInfo(Vec<IpPoolIpPoolConfigInfo>),
    /// A boxed array of *VAppOvfSectionInfo*. To be used in *Any* placeholders.
    ArrayOfVAppOvfSectionInfo(Vec<VAppOvfSectionInfo>),
    /// A boxed array of *VAppOvfSectionSpec*. To be used in *Any* placeholders.
    ArrayOfVAppOvfSectionSpec(Vec<VAppOvfSectionSpec>),
    /// A boxed array of *VAppProductInfo*. To be used in *Any* placeholders.
    ArrayOfVAppProductInfo(Vec<VAppProductInfo>),
    /// A boxed array of *VAppProductSpec*. To be used in *Any* placeholders.
    ArrayOfVAppProductSpec(Vec<VAppProductSpec>),
    /// A boxed array of *VAppPropertyInfo*. To be used in *Any* placeholders.
    ArrayOfVAppPropertyInfo(Vec<VAppPropertyInfo>),
    /// A boxed array of *VAppPropertySpec*. To be used in *Any* placeholders.
    ArrayOfVAppPropertySpec(Vec<VAppPropertySpec>),
    /// A boxed array of *VAppConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVAppConfigInfo(Vec<VAppConfigInfo>),
    /// A boxed array of *VAppConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVAppConfigSpec(Vec<VAppConfigSpec>),
    /// A boxed array of *VirtualAppImportSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualAppImportSpec(Vec<VirtualAppImportSpec>),
    /// A boxed array of *VmConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVmConfigInfo(Vec<Box<dyn super::traits::VmConfigInfoTrait>>),
    /// A boxed array of *VmConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVmConfigSpec(Vec<Box<dyn super::traits::VmConfigSpecTrait>>),
    /// A boxed array of *ClusterNetworkConfigSpec*. To be used in *Any* placeholders.
    ArrayOfClusterNetworkConfigSpec(Vec<ClusterNetworkConfigSpec>),
    /// A boxed array of *FailoverNodeInfo*. To be used in *Any* placeholders.
    ArrayOfFailoverNodeInfo(Vec<FailoverNodeInfo>),
    /// A boxed array of *NodeDeploymentSpec*. To be used in *Any* placeholders.
    ArrayOfNodeDeploymentSpec(Vec<Box<dyn super::traits::NodeDeploymentSpecTrait>>),
    /// A boxed array of *NodeNetworkSpec*. To be used in *Any* placeholders.
    ArrayOfNodeNetworkSpec(Vec<Box<dyn super::traits::NodeNetworkSpecTrait>>),
    /// A boxed array of *PassiveNodeDeploymentSpec*. To be used in *Any* placeholders.
    ArrayOfPassiveNodeDeploymentSpec(Vec<PassiveNodeDeploymentSpec>),
    /// A boxed array of *PassiveNodeNetworkSpec*. To be used in *Any* placeholders.
    ArrayOfPassiveNodeNetworkSpec(Vec<PassiveNodeNetworkSpec>),
    /// A boxed array of *SourceNodeSpec*. To be used in *Any* placeholders.
    ArrayOfSourceNodeSpec(Vec<SourceNodeSpec>),
    /// A boxed array of *VchaClusterConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVchaClusterConfigInfo(Vec<VchaClusterConfigInfo>),
    /// A boxed array of *VchaClusterConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVchaClusterConfigSpec(Vec<VchaClusterConfigSpec>),
    /// A boxed array of *VchaClusterDeploymentSpec*. To be used in *Any* placeholders.
    ArrayOfVchaClusterDeploymentSpec(Vec<VchaClusterDeploymentSpec>),
    /// A boxed array of *VchaClusterNetworkSpec*. To be used in *Any* placeholders.
    ArrayOfVchaClusterNetworkSpec(Vec<VchaClusterNetworkSpec>),
    /// A boxed array of *WitnessNodeInfo*. To be used in *Any* placeholders.
    ArrayOfWitnessNodeInfo(Vec<WitnessNodeInfo>),
    /// A boxed array of *VchaClusterHealth*. To be used in *Any* placeholders.
    ArrayOfVchaClusterHealth(Vec<VchaClusterHealth>),
    /// A boxed array of *VchaClusterRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfVchaClusterRuntimeInfo(Vec<VchaClusterRuntimeInfo>),
    /// A boxed array of *VchaNodeRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfVchaNodeRuntimeInfo(Vec<VchaNodeRuntimeInfo>),
    /// A boxed array of *VirtualMachineAffinityInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineAffinityInfo(Vec<VirtualMachineAffinityInfo>),
    /// A boxed array of *VirtualMachineBaseIndependentFilterSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.1
    ArrayOfVirtualMachineBaseIndependentFilterSpec(Vec<Box<dyn super::traits::VirtualMachineBaseIndependentFilterSpecTrait>>),
    /// A boxed array of *VirtualMachineBootOptions*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptions(Vec<VirtualMachineBootOptions>),
    /// A boxed array of *VirtualMachineBootOptionsBootableCdromDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptionsBootableCdromDevice(Vec<VirtualMachineBootOptionsBootableCdromDevice>),
    /// A boxed array of *VirtualMachineBootOptionsBootableDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptionsBootableDevice(Vec<Box<dyn super::traits::VirtualMachineBootOptionsBootableDeviceTrait>>),
    /// A boxed array of *VirtualMachineBootOptionsBootableDiskDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptionsBootableDiskDevice(Vec<VirtualMachineBootOptionsBootableDiskDevice>),
    /// A boxed array of *VirtualMachineBootOptionsBootableEthernetDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptionsBootableEthernetDevice(Vec<VirtualMachineBootOptionsBootableEthernetDevice>),
    /// A boxed array of *VirtualMachineBootOptionsBootableFloppyDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineBootOptionsBootableFloppyDevice(Vec<VirtualMachineBootOptionsBootableFloppyDevice>),
    /// A boxed array of *VirtualMachineCapability*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineCapability(Vec<VirtualMachineCapability>),
    /// A boxed array of *VirtualMachineCdromInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineCdromInfo(Vec<VirtualMachineCdromInfo>),
    /// A boxed array of *VirtualMachineCertThumbprint*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.1
    ArrayOfVirtualMachineCertThumbprint(Vec<VirtualMachineCertThumbprint>),
    /// A boxed array of *VirtualMachineCloneSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineCloneSpec(Vec<VirtualMachineCloneSpec>),
    /// A boxed array of *VirtualMachineConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigInfo(Vec<VirtualMachineConfigInfo>),
    /// A boxed array of *VirtualMachineConfigInfoDatastoreUrlPair*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigInfoDatastoreUrlPair(Vec<VirtualMachineConfigInfoDatastoreUrlPair>),
    /// A boxed array of *VirtualMachineConfigInfoOverheadInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigInfoOverheadInfo(Vec<VirtualMachineConfigInfoOverheadInfo>),
    /// A boxed array of *VirtualMachineConfigOption*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigOption(Vec<VirtualMachineConfigOption>),
    /// A boxed array of *VirtualMachineConfigOptionDescriptor*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigOptionDescriptor(Vec<VirtualMachineConfigOptionDescriptor>),
    /// A boxed array of *VirtualMachineConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigSpec(Vec<VirtualMachineConfigSpec>),
    /// A boxed array of *VirtualMachineCpuIdInfoSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineCpuIdInfoSpec(Vec<VirtualMachineCpuIdInfoSpec>),
    /// A boxed array of *ConfigTarget*. To be used in *Any* placeholders.
    ArrayOfConfigTarget(Vec<ConfigTarget>),
    /// A boxed array of *VirtualMachineConsolePreferences*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConsolePreferences(Vec<VirtualMachineConsolePreferences>),
    /// A boxed array of *VirtualMachineContentLibraryItemInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineContentLibraryItemInfo(Vec<VirtualMachineContentLibraryItemInfo>),
    /// A boxed array of *VirtualMachineDatastoreInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDatastoreInfo(Vec<VirtualMachineDatastoreInfo>),
    /// A boxed array of *DatastoreOption*. To be used in *Any* placeholders.
    ArrayOfDatastoreOption(Vec<DatastoreOption>),
    /// A boxed array of *VirtualMachineDatastoreVolumeOption*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDatastoreVolumeOption(Vec<VirtualMachineDatastoreVolumeOption>),
    /// A boxed array of *VirtualMachineDefaultPowerOpInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDefaultPowerOpInfo(Vec<VirtualMachineDefaultPowerOpInfo>),
    /// A boxed array of *VirtualMachineDefaultProfileSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDefaultProfileSpec(Vec<VirtualMachineDefaultProfileSpec>),
    /// A boxed array of *VirtualMachineDefinedProfileSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDefinedProfileSpec(Vec<VirtualMachineDefinedProfileSpec>),
    /// A boxed array of *VirtualMachineDeviceRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDeviceRuntimeInfo(Vec<VirtualMachineDeviceRuntimeInfo>),
    /// A boxed array of *VirtualMachineDeviceRuntimeInfoDeviceRuntimeState*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDeviceRuntimeInfoDeviceRuntimeState(Vec<Box<dyn super::traits::VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait>>),
    /// A boxed array of *VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState(Vec<VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState>),
    /// A boxed array of *VirtualMachineDiskDeviceInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDiskDeviceInfo(Vec<Box<dyn super::traits::VirtualMachineDiskDeviceInfoTrait>>),
    /// A boxed array of *VirtualMachineDvxClassInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineDvxClassInfo(Vec<VirtualMachineDvxClassInfo>),
    /// A boxed array of *VirtualMachineDynamicPassthroughInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineDynamicPassthroughInfo(Vec<VirtualMachineDynamicPassthroughInfo>),
    /// A boxed array of *VirtualMachineEmptyIndependentFilterSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.1
    ArrayOfVirtualMachineEmptyIndependentFilterSpec(Vec<VirtualMachineEmptyIndependentFilterSpec>),
    /// A boxed array of *VirtualMachineEmptyProfileSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineEmptyProfileSpec(Vec<VirtualMachineEmptyProfileSpec>),
    /// A boxed array of *FaultToleranceConfigInfo*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceConfigInfo(Vec<Box<dyn super::traits::FaultToleranceConfigInfoTrait>>),
    /// A boxed array of *FaultToleranceConfigSpec*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceConfigSpec(Vec<FaultToleranceConfigSpec>),
    /// A boxed array of *FaultToleranceMetaSpec*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceMetaSpec(Vec<FaultToleranceMetaSpec>),
    /// A boxed array of *FaultTolerancePrimaryConfigInfo*. To be used in *Any* placeholders.
    ArrayOfFaultTolerancePrimaryConfigInfo(Vec<FaultTolerancePrimaryConfigInfo>),
    /// A boxed array of *FaultToleranceSecondaryConfigInfo*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceSecondaryConfigInfo(Vec<FaultToleranceSecondaryConfigInfo>),
    /// A boxed array of *FaultToleranceSecondaryOpResult*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceSecondaryOpResult(Vec<FaultToleranceSecondaryOpResult>),
    /// A boxed array of *FaultToleranceVMConfigSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfFaultToleranceVMConfigSpec")]
    ArrayOfFaultToleranceVmConfigSpec(Vec<FaultToleranceVmConfigSpec>),
    /// A boxed array of *FaultToleranceDiskSpec*. To be used in *Any* placeholders.
    ArrayOfFaultToleranceDiskSpec(Vec<FaultToleranceDiskSpec>),
    /// A boxed array of *VirtualMachineFeatureRequirement*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFeatureRequirement(Vec<VirtualMachineFeatureRequirement>),
    /// A boxed array of *VirtualMachineFileInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileInfo(Vec<VirtualMachineFileInfo>),
    /// A boxed array of *VirtualMachineFileLayout*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayout(Vec<VirtualMachineFileLayout>),
    /// A boxed array of *VirtualMachineFileLayoutDiskLayout*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutDiskLayout(Vec<VirtualMachineFileLayoutDiskLayout>),
    /// A boxed array of *VirtualMachineFileLayoutSnapshotLayout*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutSnapshotLayout(Vec<VirtualMachineFileLayoutSnapshotLayout>),
    /// A boxed array of *VirtualMachineFileLayoutEx*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutEx(Vec<VirtualMachineFileLayoutEx>),
    /// A boxed array of *VirtualMachineFileLayoutExDiskLayout*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutExDiskLayout(Vec<VirtualMachineFileLayoutExDiskLayout>),
    /// A boxed array of *VirtualMachineFileLayoutExDiskUnit*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutExDiskUnit(Vec<VirtualMachineFileLayoutExDiskUnit>),
    /// A boxed array of *VirtualMachineFileLayoutExFileInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutExFileInfo(Vec<VirtualMachineFileLayoutExFileInfo>),
    /// A boxed array of *VirtualMachineFileLayoutExSnapshotLayout*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFileLayoutExSnapshotLayout(Vec<VirtualMachineFileLayoutExSnapshotLayout>),
    /// A boxed array of *VirtualMachineFlagInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFlagInfo(Vec<VirtualMachineFlagInfo>),
    /// A boxed array of *VirtualMachineFloppyInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFloppyInfo(Vec<VirtualMachineFloppyInfo>),
    /// A boxed array of *VirtualMachineForkConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineForkConfigInfo(Vec<VirtualMachineForkConfigInfo>),
    /// A boxed array of *GuestInfo*. To be used in *Any* placeholders.
    ArrayOfGuestInfo(Vec<GuestInfo>),
    /// A boxed array of *GuestInfoCustomizationInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.0
    ArrayOfGuestInfoCustomizationInfo(Vec<GuestInfoCustomizationInfo>),
    /// A boxed array of *GuestDiskInfo*. To be used in *Any* placeholders.
    ArrayOfGuestDiskInfo(Vec<GuestDiskInfo>),
    /// A boxed array of *GuestInfoNamespaceGenerationInfo*. To be used in *Any* placeholders.
    ArrayOfGuestInfoNamespaceGenerationInfo(Vec<GuestInfoNamespaceGenerationInfo>),
    /// A boxed array of *GuestNicInfo*. To be used in *Any* placeholders.
    ArrayOfGuestNicInfo(Vec<GuestNicInfo>),
    /// A boxed array of *GuestScreenInfo*. To be used in *Any* placeholders.
    ArrayOfGuestScreenInfo(Vec<GuestScreenInfo>),
    /// A boxed array of *GuestStackInfo*. To be used in *Any* placeholders.
    ArrayOfGuestStackInfo(Vec<GuestStackInfo>),
    /// A boxed array of *GuestInfoVirtualDiskMapping*. To be used in *Any* placeholders.
    ArrayOfGuestInfoVirtualDiskMapping(Vec<GuestInfoVirtualDiskMapping>),
    /// A boxed array of *VirtualMachineGuestIntegrityInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineGuestIntegrityInfo(Vec<VirtualMachineGuestIntegrityInfo>),
    /// A boxed array of *VirtualMachineGuestMonitoringModeInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineGuestMonitoringModeInfo(Vec<VirtualMachineGuestMonitoringModeInfo>),
    /// A boxed array of *GuestOsDescriptor*. To be used in *Any* placeholders.
    ArrayOfGuestOsDescriptor(Vec<GuestOsDescriptor>),
    /// A boxed array of *VirtualMachineGuestQuiesceSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineGuestQuiesceSpec(Vec<Box<dyn super::traits::VirtualMachineGuestQuiesceSpecTrait>>),
    /// A boxed array of *VirtualMachineIdeDiskDeviceInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineIdeDiskDeviceInfo(Vec<VirtualMachineIdeDiskDeviceInfo>),
    /// A boxed array of *VirtualMachineIdeDiskDevicePartitionInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineIdeDiskDevicePartitionInfo(Vec<VirtualMachineIdeDiskDevicePartitionInfo>),
    /// A boxed array of *VirtualMachineIndependentFilterSpec*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.2.1
    ArrayOfVirtualMachineIndependentFilterSpec(Vec<VirtualMachineIndependentFilterSpec>),
    /// A boxed array of *VirtualMachineInstantCloneSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineInstantCloneSpec(Vec<VirtualMachineInstantCloneSpec>),
    /// A boxed array of *VirtualMachineLegacyNetworkSwitchInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineLegacyNetworkSwitchInfo(Vec<VirtualMachineLegacyNetworkSwitchInfo>),
    /// A boxed array of *VirtualMachineMessage*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMessage(Vec<VirtualMachineMessage>),
    /// A boxed array of *VirtualMachineMetadataManagerVmMetadata*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMetadataManagerVmMetadata(Vec<VirtualMachineMetadataManagerVmMetadata>),
    /// A boxed array of *VirtualMachineMetadataManagerVmMetadataInput*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMetadataManagerVmMetadataInput(Vec<VirtualMachineMetadataManagerVmMetadataInput>),
    /// A boxed array of *VirtualMachineMetadataManagerVmMetadataOwner*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMetadataManagerVmMetadataOwner(Vec<VirtualMachineMetadataManagerVmMetadataOwner>),
    /// A boxed array of *VirtualMachineMetadataManagerVmMetadataResult*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMetadataManagerVmMetadataResult(Vec<VirtualMachineMetadataManagerVmMetadataResult>),
    /// A boxed array of *VirtualMachineNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineNetworkInfo(Vec<VirtualMachineNetworkInfo>),
    /// A boxed array of *VirtualMachineNetworkShaperInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineNetworkShaperInfo(Vec<VirtualMachineNetworkShaperInfo>),
    /// A boxed array of *OpaqueNetworkTargetInfo*. To be used in *Any* placeholders.
    ArrayOfOpaqueNetworkTargetInfo(Vec<OpaqueNetworkTargetInfo>),
    /// A boxed array of *VirtualMachineParallelInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineParallelInfo(Vec<VirtualMachineParallelInfo>),
    /// A boxed array of *VirtualMachinePciPassthroughInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachinePciPassthroughInfo(Vec<Box<dyn super::traits::VirtualMachinePciPassthroughInfoTrait>>),
    /// A boxed array of *VirtualMachinePciSharedGpuPassthroughInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachinePciSharedGpuPassthroughInfo(Vec<VirtualMachinePciSharedGpuPassthroughInfo>),
    /// A boxed array of *VirtualMachinePrecisionClockInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachinePrecisionClockInfo(Vec<VirtualMachinePrecisionClockInfo>),
    /// A boxed array of *VirtualMachineProfileDetails*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineProfileDetails(Vec<VirtualMachineProfileDetails>),
    /// A boxed array of *VirtualMachineProfileDetailsDiskProfileDetails*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineProfileDetailsDiskProfileDetails(Vec<VirtualMachineProfileDetailsDiskProfileDetails>),
    /// A boxed array of *VirtualMachineProfileRawData*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineProfileRawData(Vec<VirtualMachineProfileRawData>),
    /// A boxed array of *VirtualMachineProfileSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineProfileSpec(Vec<Box<dyn super::traits::VirtualMachineProfileSpecTrait>>),
    /// A boxed array of *VirtualMachinePropertyRelation*. To be used in *Any* placeholders.
    ArrayOfVirtualMachinePropertyRelation(Vec<VirtualMachinePropertyRelation>),
    /// A boxed array of *VirtualMachineQuestionInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineQuestionInfo(Vec<VirtualMachineQuestionInfo>),
    /// A boxed array of *VirtualMachineRelocateSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRelocateSpec(Vec<VirtualMachineRelocateSpec>),
    /// A boxed array of *VirtualMachineRelocateSpecDiskLocator*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRelocateSpecDiskLocator(Vec<VirtualMachineRelocateSpecDiskLocator>),
    /// A boxed array of *VirtualMachineRelocateSpecDiskLocatorBackingSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRelocateSpecDiskLocatorBackingSpec(Vec<VirtualMachineRelocateSpecDiskLocatorBackingSpec>),
    /// A boxed array of *ReplicationConfigSpec*. To be used in *Any* placeholders.
    ArrayOfReplicationConfigSpec(Vec<ReplicationConfigSpec>),
    /// A boxed array of *ReplicationInfoDiskSettings*. To be used in *Any* placeholders.
    ArrayOfReplicationInfoDiskSettings(Vec<ReplicationInfoDiskSettings>),
    /// A boxed array of *VirtualMachineRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRuntimeInfo(Vec<VirtualMachineRuntimeInfo>),
    /// A boxed array of *VirtualMachineRuntimeInfoDasProtectionState*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRuntimeInfoDasProtectionState(Vec<VirtualMachineRuntimeInfoDasProtectionState>),
    /// A boxed array of *ScheduledHardwareUpgradeInfo*. To be used in *Any* placeholders.
    ArrayOfScheduledHardwareUpgradeInfo(Vec<ScheduledHardwareUpgradeInfo>),
    /// A boxed array of *VirtualMachineScsiDiskDeviceInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineScsiDiskDeviceInfo(Vec<VirtualMachineScsiDiskDeviceInfo>),
    /// A boxed array of *VirtualMachineScsiPassthroughInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineScsiPassthroughInfo(Vec<VirtualMachineScsiPassthroughInfo>),
    /// A boxed array of *VirtualMachineSerialInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSerialInfo(Vec<VirtualMachineSerialInfo>),
    /// A boxed array of *VirtualMachineSgxInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSgxInfo(Vec<VirtualMachineSgxInfo>),
    /// A boxed array of *VirtualMachineSgxTargetInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSgxTargetInfo(Vec<VirtualMachineSgxTargetInfo>),
    /// A boxed array of *VirtualMachineSnapshotInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSnapshotInfo(Vec<VirtualMachineSnapshotInfo>),
    /// A boxed array of *VirtualMachineSnapshotTree*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSnapshotTree(Vec<VirtualMachineSnapshotTree>),
    /// A boxed array of *VirtualMachineSoundInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSoundInfo(Vec<VirtualMachineSoundInfo>),
    /// A boxed array of *VirtualMachineSriovDevicePoolInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSriovDevicePoolInfo(Vec<Box<dyn super::traits::VirtualMachineSriovDevicePoolInfoTrait>>),
    /// A boxed array of *VirtualMachineSriovInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSriovInfo(Vec<VirtualMachineSriovInfo>),
    /// A boxed array of *VirtualMachineSriovNetworkDevicePoolInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSriovNetworkDevicePoolInfo(Vec<VirtualMachineSriovNetworkDevicePoolInfo>),
    /// A boxed array of *VirtualMachineStorageInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineStorageInfo(Vec<VirtualMachineStorageInfo>),
    /// A boxed array of *VirtualMachineUsageOnDatastore*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineUsageOnDatastore(Vec<VirtualMachineUsageOnDatastore>),
    /// A boxed array of *VirtualMachineSummary*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineSummary(Vec<VirtualMachineSummary>),
    /// A boxed array of *VirtualMachineConfigSummary*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigSummary(Vec<VirtualMachineConfigSummary>),
    /// A boxed array of *VirtualMachineGuestSummary*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineGuestSummary(Vec<VirtualMachineGuestSummary>),
    /// A boxed array of *VirtualMachineQuickStats*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineQuickStats(Vec<VirtualMachineQuickStats>),
    /// A boxed array of *VirtualMachineQuickStatsMemoryTierStats*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfVirtualMachineQuickStatsMemoryTierStats(Vec<VirtualMachineQuickStatsMemoryTierStats>),
    /// A boxed array of *VirtualMachineStorageSummary*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineStorageSummary(Vec<VirtualMachineStorageSummary>),
    /// A boxed array of *VirtualMachineTargetInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineTargetInfo(Vec<Box<dyn super::traits::VirtualMachineTargetInfoTrait>>),
    /// A boxed array of *ToolsConfigInfo*. To be used in *Any* placeholders.
    ArrayOfToolsConfigInfo(Vec<ToolsConfigInfo>),
    /// A boxed array of *ToolsConfigInfoToolsLastInstallInfo*. To be used in *Any* placeholders.
    ArrayOfToolsConfigInfoToolsLastInstallInfo(Vec<ToolsConfigInfoToolsLastInstallInfo>),
    /// A boxed array of *VirtualMachineUsbInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineUsbInfo(Vec<VirtualMachineUsbInfo>),
    /// A boxed array of *UsbScanCodeSpec*. To be used in *Any* placeholders.
    ArrayOfUsbScanCodeSpec(Vec<UsbScanCodeSpec>),
    /// A boxed array of *UsbScanCodeSpecKeyEvent*. To be used in *Any* placeholders.
    ArrayOfUsbScanCodeSpecKeyEvent(Vec<UsbScanCodeSpecKeyEvent>),
    /// A boxed array of *UsbScanCodeSpecModifierType*. To be used in *Any* placeholders.
    ArrayOfUsbScanCodeSpecModifierType(Vec<UsbScanCodeSpecModifierType>),
    /// A boxed array of *VirtualMachineVFlashModuleInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineVFlashModuleInfo(Vec<VirtualMachineVFlashModuleInfo>),
    /// A boxed array of *VirtualMachineVMotionStunTimeInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    ArrayOfVirtualMachineVMotionStunTimeInfo(Vec<VirtualMachineVMotionStunTimeInfo>),
    /// A boxed array of *VirtualMachineVcpuConfig*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineVcpuConfig(Vec<VirtualMachineVcpuConfig>),
    /// A boxed array of *VirtualMachineVendorDeviceGroupInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVendorDeviceGroupInfo(Vec<VirtualMachineVendorDeviceGroupInfo>),
    /// A boxed array of *VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVendorDeviceGroupInfoComponentDeviceInfo(Vec<VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo>),
    /// A boxed array of *VirtualMachineVgpuDeviceInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfVirtualMachineVgpuDeviceInfo(Vec<VirtualMachineVgpuDeviceInfo>),
    /// A boxed array of *VirtualMachineVgpuProfileInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfVirtualMachineVgpuProfileInfo(Vec<VirtualMachineVgpuProfileInfo>),
    /// A boxed array of *VirtualMachineVirtualDeviceGroups*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVirtualDeviceGroups(Vec<VirtualMachineVirtualDeviceGroups>),
    /// A boxed array of *VirtualMachineVirtualDeviceGroupsDeviceGroup*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineVirtualDeviceGroupsDeviceGroup(Vec<Box<dyn super::traits::VirtualMachineVirtualDeviceGroupsDeviceGroupTrait>>),
    /// A boxed array of *VirtualMachineVirtualDeviceGroupsVendorDeviceGroup*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineVirtualDeviceGroupsVendorDeviceGroup(Vec<VirtualMachineVirtualDeviceGroupsVendorDeviceGroup>),
    /// A boxed array of *VirtualMachineVirtualDeviceSwap*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVirtualDeviceSwap(Vec<VirtualMachineVirtualDeviceSwap>),
    /// A boxed array of *VirtualMachineVirtualDeviceSwapDeviceSwapInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVirtualDeviceSwapDeviceSwapInfo(Vec<VirtualMachineVirtualDeviceSwapDeviceSwapInfo>),
    /// A boxed array of *VirtualHardware*. To be used in *Any* placeholders.
    ArrayOfVirtualHardware(Vec<VirtualHardware>),
    /// A boxed array of *VirtualHardwareOption*. To be used in *Any* placeholders.
    ArrayOfVirtualHardwareOption(Vec<VirtualHardwareOption>),
    /// A boxed array of *VirtualMachineVirtualNuma*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVirtualNuma(Vec<VirtualMachineVirtualNuma>),
    /// A boxed array of *VirtualMachineVirtualNumaInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineVirtualNumaInfo(Vec<VirtualMachineVirtualNumaInfo>),
    /// A boxed array of *VirtualMachineVirtualPMem*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfVirtualMachineVirtualPMem(Vec<VirtualMachineVirtualPMem>),
    /// A boxed array of *VirtualMachineImportSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineImportSpec(Vec<VirtualMachineImportSpec>),
    /// A boxed array of *VirtualMachineWindowsQuiesceSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineWindowsQuiesceSpec(Vec<VirtualMachineWindowsQuiesceSpec>),
    /// A boxed array of *CheckResult*. To be used in *Any* placeholders.
    ArrayOfCheckResult(Vec<CheckResult>),
    /// A boxed array of *CustomizationAdapterMapping*. To be used in *Any* placeholders.
    ArrayOfCustomizationAdapterMapping(Vec<CustomizationAdapterMapping>),
    /// A boxed array of *CustomizationAutoIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationAutoIpV6Generator(Vec<CustomizationAutoIpV6Generator>),
    /// A boxed array of *CustomizationCloudinitPrep*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 7.0.3.0
    ArrayOfCustomizationCloudinitPrep(Vec<CustomizationCloudinitPrep>),
    /// A boxed array of *CustomizationCustomIpGenerator*. To be used in *Any* placeholders.
    ArrayOfCustomizationCustomIpGenerator(Vec<CustomizationCustomIpGenerator>),
    /// A boxed array of *CustomizationCustomIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationCustomIpV6Generator(Vec<CustomizationCustomIpV6Generator>),
    /// A boxed array of *CustomizationCustomName*. To be used in *Any* placeholders.
    ArrayOfCustomizationCustomName(Vec<CustomizationCustomName>),
    /// A boxed array of *CustomizationDhcpIpGenerator*. To be used in *Any* placeholders.
    ArrayOfCustomizationDhcpIpGenerator(Vec<CustomizationDhcpIpGenerator>),
    /// A boxed array of *CustomizationDhcpIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationDhcpIpV6Generator(Vec<CustomizationDhcpIpV6Generator>),
    /// A boxed array of *CustomizationFixedIp*. To be used in *Any* placeholders.
    ArrayOfCustomizationFixedIp(Vec<CustomizationFixedIp>),
    /// A boxed array of *CustomizationFixedIpV6*. To be used in *Any* placeholders.
    ArrayOfCustomizationFixedIpV6(Vec<CustomizationFixedIpV6>),
    /// A boxed array of *CustomizationFixedName*. To be used in *Any* placeholders.
    ArrayOfCustomizationFixedName(Vec<CustomizationFixedName>),
    /// A boxed array of *CustomizationGlobalIPSettings*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCustomizationGlobalIPSettings")]
    ArrayOfCustomizationGlobalIpSettings(Vec<CustomizationGlobalIpSettings>),
    /// A boxed array of *CustomizationGuiRunOnce*. To be used in *Any* placeholders.
    ArrayOfCustomizationGuiRunOnce(Vec<CustomizationGuiRunOnce>),
    /// A boxed array of *CustomizationGuiUnattended*. To be used in *Any* placeholders.
    ArrayOfCustomizationGuiUnattended(Vec<CustomizationGuiUnattended>),
    /// A boxed array of *CustomizationIPSettings*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCustomizationIPSettings")]
    ArrayOfCustomizationIpSettings(Vec<CustomizationIpSettings>),
    /// A boxed array of *CustomizationIPSettingsIpV6AddressSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCustomizationIPSettingsIpV6AddressSpec")]
    ArrayOfCustomizationIpSettingsIpV6AddressSpec(Vec<CustomizationIpSettingsIpV6AddressSpec>),
    /// A boxed array of *CustomizationIdentification*. To be used in *Any* placeholders.
    ArrayOfCustomizationIdentification(Vec<CustomizationIdentification>),
    /// A boxed array of *CustomizationIdentitySettings*. To be used in *Any* placeholders.
    ArrayOfCustomizationIdentitySettings(Vec<Box<dyn super::traits::CustomizationIdentitySettingsTrait>>),
    /// A boxed array of *CustomizationIpGenerator*. To be used in *Any* placeholders.
    ArrayOfCustomizationIpGenerator(Vec<Box<dyn super::traits::CustomizationIpGeneratorTrait>>),
    /// A boxed array of *CustomizationIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationIpV6Generator(Vec<Box<dyn super::traits::CustomizationIpV6GeneratorTrait>>),
    /// A boxed array of *CustomizationLicenseFilePrintData*. To be used in *Any* placeholders.
    ArrayOfCustomizationLicenseFilePrintData(Vec<CustomizationLicenseFilePrintData>),
    /// A boxed array of *CustomizationLinuxOptions*. To be used in *Any* placeholders.
    ArrayOfCustomizationLinuxOptions(Vec<CustomizationLinuxOptions>),
    /// A boxed array of *CustomizationLinuxPrep*. To be used in *Any* placeholders.
    ArrayOfCustomizationLinuxPrep(Vec<CustomizationLinuxPrep>),
    /// A boxed array of *CustomizationName*. To be used in *Any* placeholders.
    ArrayOfCustomizationName(Vec<Box<dyn super::traits::CustomizationNameTrait>>),
    /// A boxed array of *CustomizationOptions*. To be used in *Any* placeholders.
    ArrayOfCustomizationOptions(Vec<Box<dyn super::traits::CustomizationOptionsTrait>>),
    /// A boxed array of *CustomizationPassword*. To be used in *Any* placeholders.
    ArrayOfCustomizationPassword(Vec<CustomizationPassword>),
    /// A boxed array of *CustomizationPrefixName*. To be used in *Any* placeholders.
    ArrayOfCustomizationPrefixName(Vec<CustomizationPrefixName>),
    /// A boxed array of *CustomizationSpec*. To be used in *Any* placeholders.
    ArrayOfCustomizationSpec(Vec<CustomizationSpec>),
    /// A boxed array of *CustomizationStatelessIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationStatelessIpV6Generator(Vec<CustomizationStatelessIpV6Generator>),
    /// A boxed array of *CustomizationSysprep*. To be used in *Any* placeholders.
    ArrayOfCustomizationSysprep(Vec<CustomizationSysprep>),
    /// A boxed array of *CustomizationSysprepText*. To be used in *Any* placeholders.
    ArrayOfCustomizationSysprepText(Vec<CustomizationSysprepText>),
    /// A boxed array of *CustomizationUnknownIpGenerator*. To be used in *Any* placeholders.
    ArrayOfCustomizationUnknownIpGenerator(Vec<CustomizationUnknownIpGenerator>),
    /// A boxed array of *CustomizationUnknownIpV6Generator*. To be used in *Any* placeholders.
    ArrayOfCustomizationUnknownIpV6Generator(Vec<CustomizationUnknownIpV6Generator>),
    /// A boxed array of *CustomizationUnknownName*. To be used in *Any* placeholders.
    ArrayOfCustomizationUnknownName(Vec<CustomizationUnknownName>),
    /// A boxed array of *CustomizationUserData*. To be used in *Any* placeholders.
    ArrayOfCustomizationUserData(Vec<CustomizationUserData>),
    /// A boxed array of *CustomizationVirtualMachineName*. To be used in *Any* placeholders.
    ArrayOfCustomizationVirtualMachineName(Vec<CustomizationVirtualMachineName>),
    /// A boxed array of *CustomizationWinOptions*. To be used in *Any* placeholders.
    ArrayOfCustomizationWinOptions(Vec<CustomizationWinOptions>),
    /// A boxed array of *HostDiskMappingInfo*. To be used in *Any* placeholders.
    ArrayOfHostDiskMappingInfo(Vec<HostDiskMappingInfo>),
    /// A boxed array of *HostDiskMappingPartitionInfo*. To be used in *Any* placeholders.
    ArrayOfHostDiskMappingPartitionInfo(Vec<HostDiskMappingPartitionInfo>),
    /// A boxed array of *HostDiskMappingOption*. To be used in *Any* placeholders.
    ArrayOfHostDiskMappingOption(Vec<HostDiskMappingOption>),
    /// A boxed array of *HostDiskMappingPartitionOption*. To be used in *Any* placeholders.
    ArrayOfHostDiskMappingPartitionOption(Vec<HostDiskMappingPartitionOption>),
    /// A boxed array of *ParaVirtualSCSIController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfParaVirtualSCSIController")]
    ArrayOfParaVirtualScsiController(Vec<ParaVirtualScsiController>),
    /// A boxed array of *ParaVirtualSCSIControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfParaVirtualSCSIControllerOption")]
    ArrayOfParaVirtualScsiControllerOption(Vec<ParaVirtualScsiControllerOption>),
    /// A boxed array of *VirtualAHCIController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualAHCIController")]
    ArrayOfVirtualAhciController(Vec<VirtualAhciController>),
    /// A boxed array of *VirtualAHCIControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualAHCIControllerOption")]
    ArrayOfVirtualAhciControllerOption(Vec<VirtualAhciControllerOption>),
    /// A boxed array of *VirtualBusLogicController*. To be used in *Any* placeholders.
    ArrayOfVirtualBusLogicController(Vec<VirtualBusLogicController>),
    /// A boxed array of *VirtualBusLogicControllerOption*. To be used in *Any* placeholders.
    ArrayOfVirtualBusLogicControllerOption(Vec<VirtualBusLogicControllerOption>),
    /// A boxed array of *VirtualCdrom*. To be used in *Any* placeholders.
    ArrayOfVirtualCdrom(Vec<VirtualCdrom>),
    /// A boxed array of *VirtualCdromAtapiBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromAtapiBackingInfo(Vec<VirtualCdromAtapiBackingInfo>),
    /// A boxed array of *VirtualCdromIsoBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromIsoBackingInfo(Vec<VirtualCdromIsoBackingInfo>),
    /// A boxed array of *VirtualCdromPassthroughBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromPassthroughBackingInfo(Vec<VirtualCdromPassthroughBackingInfo>),
    /// A boxed array of *VirtualCdromRemoteAtapiBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromRemoteAtapiBackingInfo(Vec<VirtualCdromRemoteAtapiBackingInfo>),
    /// A boxed array of *VirtualCdromRemotePassthroughBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromRemotePassthroughBackingInfo(Vec<VirtualCdromRemotePassthroughBackingInfo>),
    /// A boxed array of *VirtualCdromOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromOption(Vec<VirtualCdromOption>),
    /// A boxed array of *VirtualCdromAtapiBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromAtapiBackingOption(Vec<VirtualCdromAtapiBackingOption>),
    /// A boxed array of *VirtualCdromIsoBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromIsoBackingOption(Vec<VirtualCdromIsoBackingOption>),
    /// A boxed array of *VirtualCdromPassthroughBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromPassthroughBackingOption(Vec<VirtualCdromPassthroughBackingOption>),
    /// A boxed array of *VirtualCdromRemoteAtapiBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromRemoteAtapiBackingOption(Vec<VirtualCdromRemoteAtapiBackingOption>),
    /// A boxed array of *VirtualCdromRemotePassthroughBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualCdromRemotePassthroughBackingOption(Vec<VirtualCdromRemotePassthroughBackingOption>),
    /// A boxed array of *VirtualController*. To be used in *Any* placeholders.
    ArrayOfVirtualController(Vec<Box<dyn super::traits::VirtualControllerTrait>>),
    /// A boxed array of *VirtualControllerOption*. To be used in *Any* placeholders.
    ArrayOfVirtualControllerOption(Vec<Box<dyn super::traits::VirtualControllerOptionTrait>>),
    /// A boxed array of *VirtualDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualDevice(Vec<Box<dyn super::traits::VirtualDeviceTrait>>),
    /// A boxed array of *VirtualDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceBackingInfo(Vec<Box<dyn super::traits::VirtualDeviceBackingInfoTrait>>),
    /// A boxed array of *VirtualDeviceBusSlotInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceBusSlotInfo(Vec<Box<dyn super::traits::VirtualDeviceBusSlotInfoTrait>>),
    /// A boxed array of *VirtualDeviceConnectInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConnectInfo(Vec<VirtualDeviceConnectInfo>),
    /// A boxed array of *VirtualDeviceDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceDeviceBackingInfo(Vec<Box<dyn super::traits::VirtualDeviceDeviceBackingInfoTrait>>),
    /// A boxed array of *VirtualDeviceDeviceGroupInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualDeviceDeviceGroupInfo(Vec<VirtualDeviceDeviceGroupInfo>),
    /// A boxed array of *VirtualDeviceFileBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceFileBackingInfo(Vec<Box<dyn super::traits::VirtualDeviceFileBackingInfoTrait>>),
    /// A boxed array of *VirtualDevicePciBusSlotInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDevicePciBusSlotInfo(Vec<Box<dyn super::traits::VirtualDevicePciBusSlotInfoTrait>>),
    /// A boxed array of *VirtualDevicePipeBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDevicePipeBackingInfo(Vec<Box<dyn super::traits::VirtualDevicePipeBackingInfoTrait>>),
    /// A boxed array of *VirtualDeviceRemoteDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceRemoteDeviceBackingInfo(Vec<Box<dyn super::traits::VirtualDeviceRemoteDeviceBackingInfoTrait>>),
    /// A boxed array of *VirtualDeviceURIBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualDeviceURIBackingInfo")]
    ArrayOfVirtualDeviceUriBackingInfo(Vec<Box<dyn super::traits::VirtualDeviceUriBackingInfoTrait>>),
    /// A boxed array of *VirtualDeviceOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceOption(Vec<Box<dyn super::traits::VirtualDeviceOptionTrait>>),
    /// A boxed array of *VirtualDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceBackingOption(Vec<Box<dyn super::traits::VirtualDeviceBackingOptionTrait>>),
    /// A boxed array of *VirtualDeviceBusSlotOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceBusSlotOption(Vec<VirtualDeviceBusSlotOption>),
    /// A boxed array of *VirtualDeviceConnectOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConnectOption(Vec<VirtualDeviceConnectOption>),
    /// A boxed array of *VirtualDeviceDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceDeviceBackingOption(Vec<Box<dyn super::traits::VirtualDeviceDeviceBackingOptionTrait>>),
    /// A boxed array of *VirtualDeviceFileBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceFileBackingOption(Vec<Box<dyn super::traits::VirtualDeviceFileBackingOptionTrait>>),
    /// A boxed array of *VirtualDevicePipeBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDevicePipeBackingOption(Vec<Box<dyn super::traits::VirtualDevicePipeBackingOptionTrait>>),
    /// A boxed array of *VirtualDeviceRemoteDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceRemoteDeviceBackingOption(Vec<Box<dyn super::traits::VirtualDeviceRemoteDeviceBackingOptionTrait>>),
    /// A boxed array of *VirtualDeviceURIBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualDeviceURIBackingOption")]
    ArrayOfVirtualDeviceUriBackingOption(Vec<Box<dyn super::traits::VirtualDeviceUriBackingOptionTrait>>),
    /// A boxed array of *VirtualDeviceConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConfigSpec(Vec<Box<dyn super::traits::VirtualDeviceConfigSpecTrait>>),
    /// A boxed array of *VirtualDeviceConfigSpecBackingSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConfigSpecBackingSpec(Vec<VirtualDeviceConfigSpecBackingSpec>),
    /// A boxed array of *VirtualDisk*. To be used in *Any* placeholders.
    ArrayOfVirtualDisk(Vec<VirtualDisk>),
    /// A boxed array of *VirtualDiskFlatVer1BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskFlatVer1BackingInfo(Vec<VirtualDiskFlatVer1BackingInfo>),
    /// A boxed array of *VirtualDiskFlatVer2BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskFlatVer2BackingInfo(Vec<VirtualDiskFlatVer2BackingInfo>),
    /// A boxed array of *VirtualDiskLocalPMemBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskLocalPMemBackingInfo(Vec<VirtualDiskLocalPMemBackingInfo>),
    /// A boxed array of *VirtualDiskPartitionedRawDiskVer2BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskPartitionedRawDiskVer2BackingInfo(Vec<VirtualDiskPartitionedRawDiskVer2BackingInfo>),
    /// A boxed array of *VirtualDiskRawDiskMappingVer1BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskRawDiskMappingVer1BackingInfo(Vec<VirtualDiskRawDiskMappingVer1BackingInfo>),
    /// A boxed array of *VirtualDiskRawDiskVer2BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskRawDiskVer2BackingInfo(Vec<Box<dyn super::traits::VirtualDiskRawDiskVer2BackingInfoTrait>>),
    /// A boxed array of *VirtualDiskSeSparseBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSeSparseBackingInfo(Vec<VirtualDiskSeSparseBackingInfo>),
    /// A boxed array of *VirtualDiskSparseVer1BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSparseVer1BackingInfo(Vec<VirtualDiskSparseVer1BackingInfo>),
    /// A boxed array of *VirtualDiskSparseVer2BackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSparseVer2BackingInfo(Vec<VirtualDiskSparseVer2BackingInfo>),
    /// A boxed array of *VirtualDiskVFlashCacheConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskVFlashCacheConfigInfo(Vec<VirtualDiskVFlashCacheConfigInfo>),
    /// A boxed array of *VirtualDiskId*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskId(Vec<VirtualDiskId>),
    /// A boxed array of *VirtualDiskOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskOption(Vec<VirtualDiskOption>),
    /// A boxed array of *VirtualDiskDeltaDiskFormatsSupported*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskDeltaDiskFormatsSupported(Vec<VirtualDiskDeltaDiskFormatsSupported>),
    /// A boxed array of *VirtualDiskFlatVer1BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskFlatVer1BackingOption(Vec<VirtualDiskFlatVer1BackingOption>),
    /// A boxed array of *VirtualDiskFlatVer2BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskFlatVer2BackingOption(Vec<VirtualDiskFlatVer2BackingOption>),
    /// A boxed array of *VirtualDiskLocalPMemBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskLocalPMemBackingOption(Vec<VirtualDiskLocalPMemBackingOption>),
    /// A boxed array of *VirtualDiskPartitionedRawDiskVer2BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskPartitionedRawDiskVer2BackingOption(Vec<VirtualDiskPartitionedRawDiskVer2BackingOption>),
    /// A boxed array of *VirtualDiskRawDiskMappingVer1BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskRawDiskMappingVer1BackingOption(Vec<VirtualDiskRawDiskMappingVer1BackingOption>),
    /// A boxed array of *VirtualDiskRawDiskVer2BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskRawDiskVer2BackingOption(Vec<Box<dyn super::traits::VirtualDiskRawDiskVer2BackingOptionTrait>>),
    /// A boxed array of *VirtualDiskSeSparseBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSeSparseBackingOption(Vec<VirtualDiskSeSparseBackingOption>),
    /// A boxed array of *VirtualDiskSparseVer1BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSparseVer1BackingOption(Vec<VirtualDiskSparseVer1BackingOption>),
    /// A boxed array of *VirtualDiskSparseVer2BackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskSparseVer2BackingOption(Vec<VirtualDiskSparseVer2BackingOption>),
    /// A boxed array of *VirtualDiskOptionVFlashCacheConfigOption*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskOptionVFlashCacheConfigOption(Vec<VirtualDiskOptionVFlashCacheConfigOption>),
    /// A boxed array of *VirtualDiskConfigSpec*. To be used in *Any* placeholders.
    ArrayOfVirtualDiskConfigSpec(Vec<VirtualDiskConfigSpec>),
    /// A boxed array of *VirtualE1000*. To be used in *Any* placeholders.
    ArrayOfVirtualE1000(Vec<VirtualE1000>),
    /// A boxed array of *VirtualE1000Option*. To be used in *Any* placeholders.
    ArrayOfVirtualE1000Option(Vec<VirtualE1000Option>),
    /// A boxed array of *VirtualE1000e*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualE1000e")]
    ArrayOfVirtualE1000E(Vec<VirtualE1000E>),
    /// A boxed array of *VirtualE1000eOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualE1000eOption")]
    ArrayOfVirtualE1000EOption(Vec<VirtualE1000EOption>),
    /// A boxed array of *VirtualEnsoniq1371*. To be used in *Any* placeholders.
    ArrayOfVirtualEnsoniq1371(Vec<VirtualEnsoniq1371>),
    /// A boxed array of *VirtualEnsoniq1371Option*. To be used in *Any* placeholders.
    ArrayOfVirtualEnsoniq1371Option(Vec<VirtualEnsoniq1371Option>),
    /// A boxed array of *VirtualEthernetCard*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCard(Vec<Box<dyn super::traits::VirtualEthernetCardTrait>>),
    /// A boxed array of *VirtualEthernetCardDistributedVirtualPortBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardDistributedVirtualPortBackingInfo(Vec<VirtualEthernetCardDistributedVirtualPortBackingInfo>),
    /// A boxed array of *VirtualEthernetCardLegacyNetworkBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardLegacyNetworkBackingInfo(Vec<VirtualEthernetCardLegacyNetworkBackingInfo>),
    /// A boxed array of *VirtualEthernetCardNetworkBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardNetworkBackingInfo(Vec<VirtualEthernetCardNetworkBackingInfo>),
    /// A boxed array of *VirtualEthernetCardOpaqueNetworkBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardOpaqueNetworkBackingInfo(Vec<VirtualEthernetCardOpaqueNetworkBackingInfo>),
    /// A boxed array of *VirtualEthernetCardResourceAllocation*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardResourceAllocation(Vec<VirtualEthernetCardResourceAllocation>),
    /// A boxed array of *VirtualEthernetCardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardOption(Vec<Box<dyn super::traits::VirtualEthernetCardOptionTrait>>),
    /// A boxed array of *VirtualEthernetCardDVPortBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualEthernetCardDVPortBackingOption")]
    ArrayOfVirtualEthernetCardDvPortBackingOption(Vec<VirtualEthernetCardDvPortBackingOption>),
    /// A boxed array of *VirtualEthernetCardLegacyNetworkBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardLegacyNetworkBackingOption(Vec<VirtualEthernetCardLegacyNetworkBackingOption>),
    /// A boxed array of *VirtualEthernetCardNetworkBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardNetworkBackingOption(Vec<VirtualEthernetCardNetworkBackingOption>),
    /// A boxed array of *VirtualEthernetCardOpaqueNetworkBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualEthernetCardOpaqueNetworkBackingOption(Vec<VirtualEthernetCardOpaqueNetworkBackingOption>),
    /// A boxed array of *VirtualFloppy*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppy(Vec<VirtualFloppy>),
    /// A boxed array of *VirtualFloppyDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyDeviceBackingInfo(Vec<VirtualFloppyDeviceBackingInfo>),
    /// A boxed array of *VirtualFloppyImageBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyImageBackingInfo(Vec<VirtualFloppyImageBackingInfo>),
    /// A boxed array of *VirtualFloppyRemoteDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyRemoteDeviceBackingInfo(Vec<VirtualFloppyRemoteDeviceBackingInfo>),
    /// A boxed array of *VirtualFloppyOption*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyOption(Vec<VirtualFloppyOption>),
    /// A boxed array of *VirtualFloppyDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyDeviceBackingOption(Vec<VirtualFloppyDeviceBackingOption>),
    /// A boxed array of *VirtualFloppyImageBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyImageBackingOption(Vec<VirtualFloppyImageBackingOption>),
    /// A boxed array of *VirtualFloppyRemoteDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualFloppyRemoteDeviceBackingOption(Vec<VirtualFloppyRemoteDeviceBackingOption>),
    /// A boxed array of *VirtualHdAudioCard*. To be used in *Any* placeholders.
    ArrayOfVirtualHdAudioCard(Vec<VirtualHdAudioCard>),
    /// A boxed array of *VirtualHdAudioCardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualHdAudioCardOption(Vec<VirtualHdAudioCardOption>),
    /// A boxed array of *VirtualIDEController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualIDEController")]
    ArrayOfVirtualIdeController(Vec<VirtualIdeController>),
    /// A boxed array of *VirtualIDEControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualIDEControllerOption")]
    ArrayOfVirtualIdeControllerOption(Vec<VirtualIdeControllerOption>),
    /// A boxed array of *VirtualKeyboard*. To be used in *Any* placeholders.
    ArrayOfVirtualKeyboard(Vec<VirtualKeyboard>),
    /// A boxed array of *VirtualKeyboardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualKeyboardOption(Vec<VirtualKeyboardOption>),
    /// A boxed array of *VirtualLsiLogicController*. To be used in *Any* placeholders.
    ArrayOfVirtualLsiLogicController(Vec<VirtualLsiLogicController>),
    /// A boxed array of *VirtualLsiLogicControllerOption*. To be used in *Any* placeholders.
    ArrayOfVirtualLsiLogicControllerOption(Vec<VirtualLsiLogicControllerOption>),
    /// A boxed array of *VirtualLsiLogicSASController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualLsiLogicSASController")]
    ArrayOfVirtualLsiLogicSasController(Vec<VirtualLsiLogicSasController>),
    /// A boxed array of *VirtualLsiLogicSASControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualLsiLogicSASControllerOption")]
    ArrayOfVirtualLsiLogicSasControllerOption(Vec<VirtualLsiLogicSasControllerOption>),
    /// A boxed array of *VirtualNVDIMM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVDIMM")]
    ArrayOfVirtualNvdimm(Vec<VirtualNvdimm>),
    /// A boxed array of *VirtualNVDIMMBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVDIMMBackingInfo")]
    ArrayOfVirtualNvdimmBackingInfo(Vec<VirtualNvdimmBackingInfo>),
    /// A boxed array of *VirtualNVDIMMController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVDIMMController")]
    ArrayOfVirtualNvdimmController(Vec<VirtualNvdimmController>),
    /// A boxed array of *VirtualNVDIMMControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVDIMMControllerOption")]
    ArrayOfVirtualNvdimmControllerOption(Vec<VirtualNvdimmControllerOption>),
    /// A boxed array of *VirtualNVDIMMOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVDIMMOption")]
    ArrayOfVirtualNvdimmOption(Vec<VirtualNvdimmOption>),
    /// A boxed array of *VirtualNVMEController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVMEController")]
    ArrayOfVirtualNvmeController(Vec<VirtualNvmeController>),
    /// A boxed array of *VirtualNVMEControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualNVMEControllerOption")]
    ArrayOfVirtualNvmeControllerOption(Vec<VirtualNvmeControllerOption>),
    /// A boxed array of *VirtualPCIController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIController")]
    ArrayOfVirtualPciController(Vec<VirtualPciController>),
    /// A boxed array of *VirtualPCIControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIControllerOption")]
    ArrayOfVirtualPciControllerOption(Vec<VirtualPciControllerOption>),
    /// A boxed array of *VirtualPCIPassthrough*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthrough")]
    ArrayOfVirtualPciPassthrough(Vec<VirtualPciPassthrough>),
    /// A boxed array of *VirtualPCIPassthroughAllowedDevice*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughAllowedDevice")]
    ArrayOfVirtualPciPassthroughAllowedDevice(Vec<VirtualPciPassthroughAllowedDevice>),
    /// A boxed array of *VirtualPCIPassthroughDeviceBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDeviceBackingInfo")]
    ArrayOfVirtualPciPassthroughDeviceBackingInfo(Vec<VirtualPciPassthroughDeviceBackingInfo>),
    /// A boxed array of *VirtualPCIPassthroughDvxBackingInfo*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDvxBackingInfo")]
    ArrayOfVirtualPciPassthroughDvxBackingInfo(Vec<VirtualPciPassthroughDvxBackingInfo>),
    /// A boxed array of *VirtualPCIPassthroughDynamicBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDynamicBackingInfo")]
    ArrayOfVirtualPciPassthroughDynamicBackingInfo(Vec<VirtualPciPassthroughDynamicBackingInfo>),
    /// A boxed array of *VirtualPCIPassthroughPluginBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughPluginBackingInfo")]
    ArrayOfVirtualPciPassthroughPluginBackingInfo(Vec<Box<dyn super::traits::VirtualPciPassthroughPluginBackingInfoTrait>>),
    /// A boxed array of *VirtualPCIPassthroughVmiopBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughVmiopBackingInfo")]
    ArrayOfVirtualPciPassthroughVmiopBackingInfo(Vec<VirtualPciPassthroughVmiopBackingInfo>),
    /// A boxed array of *VirtualPCIPassthroughOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughOption")]
    ArrayOfVirtualPciPassthroughOption(Vec<VirtualPciPassthroughOption>),
    /// A boxed array of *VirtualPCIPassthroughDeviceBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDeviceBackingOption")]
    ArrayOfVirtualPciPassthroughDeviceBackingOption(Vec<VirtualPciPassthroughDeviceBackingOption>),
    /// A boxed array of *VirtualPCIPassthroughDvxBackingOption*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDvxBackingOption")]
    ArrayOfVirtualPciPassthroughDvxBackingOption(Vec<VirtualPciPassthroughDvxBackingOption>),
    /// A boxed array of *VirtualPCIPassthroughDynamicBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughDynamicBackingOption")]
    ArrayOfVirtualPciPassthroughDynamicBackingOption(Vec<VirtualPciPassthroughDynamicBackingOption>),
    /// A boxed array of *VirtualPCIPassthroughPluginBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughPluginBackingOption")]
    ArrayOfVirtualPciPassthroughPluginBackingOption(Vec<Box<dyn super::traits::VirtualPciPassthroughPluginBackingOptionTrait>>),
    /// A boxed array of *VirtualPCIPassthroughVmiopBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCIPassthroughVmiopBackingOption")]
    ArrayOfVirtualPciPassthroughVmiopBackingOption(Vec<VirtualPciPassthroughVmiopBackingOption>),
    /// A boxed array of *VirtualPCNet32*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCNet32")]
    ArrayOfVirtualPcNet32(Vec<VirtualPcNet32>),
    /// A boxed array of *VirtualPCNet32Option*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPCNet32Option")]
    ArrayOfVirtualPcNet32Option(Vec<VirtualPcNet32Option>),
    /// A boxed array of *VirtualPS2Controller*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPS2Controller")]
    ArrayOfVirtualPs2Controller(Vec<VirtualPs2Controller>),
    /// A boxed array of *VirtualPS2ControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualPS2ControllerOption")]
    ArrayOfVirtualPs2ControllerOption(Vec<VirtualPs2ControllerOption>),
    /// A boxed array of *VirtualParallelPort*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPort(Vec<VirtualParallelPort>),
    /// A boxed array of *VirtualParallelPortDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPortDeviceBackingInfo(Vec<VirtualParallelPortDeviceBackingInfo>),
    /// A boxed array of *VirtualParallelPortFileBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPortFileBackingInfo(Vec<VirtualParallelPortFileBackingInfo>),
    /// A boxed array of *VirtualParallelPortOption*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPortOption(Vec<VirtualParallelPortOption>),
    /// A boxed array of *VirtualParallelPortDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPortDeviceBackingOption(Vec<VirtualParallelPortDeviceBackingOption>),
    /// A boxed array of *VirtualParallelPortFileBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualParallelPortFileBackingOption(Vec<VirtualParallelPortFileBackingOption>),
    /// A boxed array of *VirtualPointingDevice*. To be used in *Any* placeholders.
    ArrayOfVirtualPointingDevice(Vec<VirtualPointingDevice>),
    /// A boxed array of *VirtualPointingDeviceDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualPointingDeviceDeviceBackingInfo(Vec<VirtualPointingDeviceDeviceBackingInfo>),
    /// A boxed array of *VirtualPointingDeviceOption*. To be used in *Any* placeholders.
    ArrayOfVirtualPointingDeviceOption(Vec<VirtualPointingDeviceOption>),
    /// A boxed array of *VirtualPointingDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualPointingDeviceBackingOption(Vec<VirtualPointingDeviceBackingOption>),
    /// A boxed array of *VirtualPrecisionClock*. To be used in *Any* placeholders.
    ArrayOfVirtualPrecisionClock(Vec<VirtualPrecisionClock>),
    /// A boxed array of *VirtualPrecisionClockSystemClockBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualPrecisionClockSystemClockBackingInfo(Vec<VirtualPrecisionClockSystemClockBackingInfo>),
    /// A boxed array of *VirtualPrecisionClockOption*. To be used in *Any* placeholders.
    ArrayOfVirtualPrecisionClockOption(Vec<VirtualPrecisionClockOption>),
    /// A boxed array of *VirtualPrecisionClockSystemClockBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualPrecisionClockSystemClockBackingOption(Vec<VirtualPrecisionClockSystemClockBackingOption>),
    /// A boxed array of *VirtualSATAController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSATAController")]
    ArrayOfVirtualSataController(Vec<Box<dyn super::traits::VirtualSataControllerTrait>>),
    /// A boxed array of *VirtualSATAControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSATAControllerOption")]
    ArrayOfVirtualSataControllerOption(Vec<Box<dyn super::traits::VirtualSataControllerOptionTrait>>),
    /// A boxed array of *VirtualSCSIController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIController")]
    ArrayOfVirtualScsiController(Vec<Box<dyn super::traits::VirtualScsiControllerTrait>>),
    /// A boxed array of *VirtualSCSIControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIControllerOption")]
    ArrayOfVirtualScsiControllerOption(Vec<Box<dyn super::traits::VirtualScsiControllerOptionTrait>>),
    /// A boxed array of *VirtualSCSIPassthrough*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIPassthrough")]
    ArrayOfVirtualScsiPassthrough(Vec<VirtualScsiPassthrough>),
    /// A boxed array of *VirtualSCSIPassthroughDeviceBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIPassthroughDeviceBackingInfo")]
    ArrayOfVirtualScsiPassthroughDeviceBackingInfo(Vec<VirtualScsiPassthroughDeviceBackingInfo>),
    /// A boxed array of *VirtualSCSIPassthroughOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIPassthroughOption")]
    ArrayOfVirtualScsiPassthroughOption(Vec<VirtualScsiPassthroughOption>),
    /// A boxed array of *VirtualSCSIPassthroughDeviceBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSIPassthroughDeviceBackingOption")]
    ArrayOfVirtualScsiPassthroughDeviceBackingOption(Vec<VirtualScsiPassthroughDeviceBackingOption>),
    /// A boxed array of *VirtualSIOController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSIOController")]
    ArrayOfVirtualSioController(Vec<VirtualSioController>),
    /// A boxed array of *VirtualSIOControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSIOControllerOption")]
    ArrayOfVirtualSioControllerOption(Vec<VirtualSioControllerOption>),
    /// A boxed array of *VirtualSerialPort*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPort(Vec<VirtualSerialPort>),
    /// A boxed array of *VirtualSerialPortDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortDeviceBackingInfo(Vec<VirtualSerialPortDeviceBackingInfo>),
    /// A boxed array of *VirtualSerialPortFileBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortFileBackingInfo(Vec<VirtualSerialPortFileBackingInfo>),
    /// A boxed array of *VirtualSerialPortPipeBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortPipeBackingInfo(Vec<VirtualSerialPortPipeBackingInfo>),
    /// A boxed array of *VirtualSerialPortThinPrintBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortThinPrintBackingInfo(Vec<VirtualSerialPortThinPrintBackingInfo>),
    /// A boxed array of *VirtualSerialPortURIBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSerialPortURIBackingInfo")]
    ArrayOfVirtualSerialPortUriBackingInfo(Vec<VirtualSerialPortUriBackingInfo>),
    /// A boxed array of *VirtualSerialPortOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortOption(Vec<VirtualSerialPortOption>),
    /// A boxed array of *VirtualSerialPortDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortDeviceBackingOption(Vec<VirtualSerialPortDeviceBackingOption>),
    /// A boxed array of *VirtualSerialPortFileBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortFileBackingOption(Vec<VirtualSerialPortFileBackingOption>),
    /// A boxed array of *VirtualSerialPortPipeBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortPipeBackingOption(Vec<VirtualSerialPortPipeBackingOption>),
    /// A boxed array of *VirtualSerialPortThinPrintBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSerialPortThinPrintBackingOption(Vec<VirtualSerialPortThinPrintBackingOption>),
    /// A boxed array of *VirtualSerialPortURIBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSerialPortURIBackingOption")]
    ArrayOfVirtualSerialPortUriBackingOption(Vec<VirtualSerialPortUriBackingOption>),
    /// A boxed array of *VirtualSoundBlaster16*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundBlaster16(Vec<VirtualSoundBlaster16>),
    /// A boxed array of *VirtualSoundBlaster16Option*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundBlaster16Option(Vec<VirtualSoundBlaster16Option>),
    /// A boxed array of *VirtualSoundCard*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundCard(Vec<Box<dyn super::traits::VirtualSoundCardTrait>>),
    /// A boxed array of *VirtualSoundCardDeviceBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundCardDeviceBackingInfo(Vec<VirtualSoundCardDeviceBackingInfo>),
    /// A boxed array of *VirtualSoundCardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundCardOption(Vec<Box<dyn super::traits::VirtualSoundCardOptionTrait>>),
    /// A boxed array of *VirtualSoundCardDeviceBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSoundCardDeviceBackingOption(Vec<VirtualSoundCardDeviceBackingOption>),
    /// A boxed array of *VirtualSriovEthernetCard*. To be used in *Any* placeholders.
    ArrayOfVirtualSriovEthernetCard(Vec<VirtualSriovEthernetCard>),
    /// A boxed array of *VirtualSriovEthernetCardSriovBackingInfo*. To be used in *Any* placeholders.
    ArrayOfVirtualSriovEthernetCardSriovBackingInfo(Vec<VirtualSriovEthernetCardSriovBackingInfo>),
    /// A boxed array of *VirtualSriovEthernetCardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSriovEthernetCardOption(Vec<VirtualSriovEthernetCardOption>),
    /// A boxed array of *VirtualSriovEthernetCardSriovBackingOption*. To be used in *Any* placeholders.
    ArrayOfVirtualSriovEthernetCardSriovBackingOption(Vec<VirtualSriovEthernetCardSriovBackingOption>),
    /// A boxed array of *VirtualTPM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualTPM")]
    ArrayOfVirtualTpm(Vec<VirtualTpm>),
    /// A boxed array of *VirtualTPMOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualTPMOption")]
    ArrayOfVirtualTpmOption(Vec<VirtualTpmOption>),
    /// A boxed array of *VirtualUSB*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSB")]
    ArrayOfVirtualUsb(Vec<VirtualUsb>),
    /// A boxed array of *VirtualUSBRemoteClientBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBRemoteClientBackingInfo")]
    ArrayOfVirtualUsbRemoteClientBackingInfo(Vec<VirtualUsbRemoteClientBackingInfo>),
    /// A boxed array of *VirtualUSBRemoteHostBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBRemoteHostBackingInfo")]
    ArrayOfVirtualUsbRemoteHostBackingInfo(Vec<VirtualUsbRemoteHostBackingInfo>),
    /// A boxed array of *VirtualUSBUSBBackingInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBUSBBackingInfo")]
    ArrayOfVirtualUsbusbBackingInfo(Vec<VirtualUsbusbBackingInfo>),
    /// A boxed array of *VirtualUSBController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBController")]
    ArrayOfVirtualUsbController(Vec<VirtualUsbController>),
    /// A boxed array of *VirtualUSBControllerPciBusSlotInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBControllerPciBusSlotInfo")]
    ArrayOfVirtualUsbControllerPciBusSlotInfo(Vec<VirtualUsbControllerPciBusSlotInfo>),
    /// A boxed array of *VirtualUSBControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBControllerOption")]
    ArrayOfVirtualUsbControllerOption(Vec<VirtualUsbControllerOption>),
    /// A boxed array of *VirtualUSBOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBOption")]
    ArrayOfVirtualUsbOption(Vec<VirtualUsbOption>),
    /// A boxed array of *VirtualUSBRemoteClientBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBRemoteClientBackingOption")]
    ArrayOfVirtualUsbRemoteClientBackingOption(Vec<VirtualUsbRemoteClientBackingOption>),
    /// A boxed array of *VirtualUSBRemoteHostBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBRemoteHostBackingOption")]
    ArrayOfVirtualUsbRemoteHostBackingOption(Vec<VirtualUsbRemoteHostBackingOption>),
    /// A boxed array of *VirtualUSBUSBBackingOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBUSBBackingOption")]
    ArrayOfVirtualUsbusbBackingOption(Vec<VirtualUsbusbBackingOption>),
    /// A boxed array of *VirtualUSBXHCIController*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBXHCIController")]
    ArrayOfVirtualUsbxhciController(Vec<VirtualUsbxhciController>),
    /// A boxed array of *VirtualUSBXHCIControllerOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualUSBXHCIControllerOption")]
    ArrayOfVirtualUsbxhciControllerOption(Vec<VirtualUsbxhciControllerOption>),
    /// A boxed array of *VirtualMachineVMCIDevice*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMCIDevice")]
    ArrayOfVirtualMachineVmciDevice(Vec<VirtualMachineVmciDevice>),
    /// A boxed array of *VirtualMachineVMCIDeviceFilterInfo*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMCIDeviceFilterInfo")]
    ArrayOfVirtualMachineVmciDeviceFilterInfo(Vec<VirtualMachineVmciDeviceFilterInfo>),
    /// A boxed array of *VirtualMachineVMCIDeviceFilterSpec*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMCIDeviceFilterSpec")]
    ArrayOfVirtualMachineVmciDeviceFilterSpec(Vec<VirtualMachineVmciDeviceFilterSpec>),
    /// A boxed array of *VirtualMachineVMCIDeviceOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMCIDeviceOption")]
    ArrayOfVirtualMachineVmciDeviceOption(Vec<VirtualMachineVmciDeviceOption>),
    /// A boxed array of *VirtualMachineVMCIDeviceOptionFilterSpecOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMCIDeviceOptionFilterSpecOption")]
    ArrayOfVirtualMachineVmciDeviceOptionFilterSpecOption(Vec<VirtualMachineVmciDeviceOptionFilterSpecOption>),
    /// A boxed array of *VirtualMachineVMIROM*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualMachineVMIROM")]
    ArrayOfVirtualMachineVmirom(Vec<VirtualMachineVmirom>),
    /// A boxed array of *VirtualVMIROMOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualVMIROMOption")]
    ArrayOfVirtualVmiromOption(Vec<VirtualVmiromOption>),
    /// A boxed array of *VirtualMachineVideoCard*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineVideoCard(Vec<VirtualMachineVideoCard>),
    /// A boxed array of *VirtualVideoCardOption*. To be used in *Any* placeholders.
    ArrayOfVirtualVideoCardOption(Vec<VirtualVideoCardOption>),
    /// A boxed array of *VirtualVmxnet*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet(Vec<Box<dyn super::traits::VirtualVmxnetTrait>>),
    /// A boxed array of *VirtualVmxnet2*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet2(Vec<VirtualVmxnet2>),
    /// A boxed array of *VirtualVmxnet2Option*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet2Option(Vec<VirtualVmxnet2Option>),
    /// A boxed array of *VirtualVmxnet3*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet3(Vec<Box<dyn super::traits::VirtualVmxnet3Trait>>),
    /// A boxed array of *VirtualVmxnet3Option*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet3Option(Vec<Box<dyn super::traits::VirtualVmxnet3OptionTrait>>),
    /// A boxed array of *VirtualVmxnet3Vrdma*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet3Vrdma(Vec<VirtualVmxnet3Vrdma>),
    /// A boxed array of *VirtualVmxnet3VrdmaOption*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnet3VrdmaOption(Vec<VirtualVmxnet3VrdmaOption>),
    /// A boxed array of *VirtualVmxnetOption*. To be used in *Any* placeholders.
    ArrayOfVirtualVmxnetOption(Vec<Box<dyn super::traits::VirtualVmxnetOptionTrait>>),
    /// A boxed array of *VirtualWDT*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualWDT")]
    ArrayOfVirtualWdt(Vec<VirtualWdt>),
    /// A boxed array of *VirtualWDTOption*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualWDTOption")]
    ArrayOfVirtualWdtOption(Vec<VirtualWdtOption>),
    /// A boxed array of *GuestAliases*. To be used in *Any* placeholders.
    ArrayOfGuestAliases(Vec<GuestAliases>),
    /// A boxed array of *GuestAuthAliasInfo*. To be used in *Any* placeholders.
    ArrayOfGuestAuthAliasInfo(Vec<GuestAuthAliasInfo>),
    /// A boxed array of *GuestAuthAnySubject*. To be used in *Any* placeholders.
    ArrayOfGuestAuthAnySubject(Vec<GuestAuthAnySubject>),
    /// A boxed array of *GuestAuthNamedSubject*. To be used in *Any* placeholders.
    ArrayOfGuestAuthNamedSubject(Vec<GuestAuthNamedSubject>),
    /// A boxed array of *GuestAuthSubject*. To be used in *Any* placeholders.
    ArrayOfGuestAuthSubject(Vec<Box<dyn super::traits::GuestAuthSubjectTrait>>),
    /// A boxed array of *GuestMappedAliases*. To be used in *Any* placeholders.
    ArrayOfGuestMappedAliases(Vec<GuestMappedAliases>),
    /// A boxed array of *GuestFileAttributes*. To be used in *Any* placeholders.
    ArrayOfGuestFileAttributes(Vec<Box<dyn super::traits::GuestFileAttributesTrait>>),
    /// A boxed array of *GuestFileInfo*. To be used in *Any* placeholders.
    ArrayOfGuestFileInfo(Vec<GuestFileInfo>),
    /// A boxed array of *FileTransferInformation*. To be used in *Any* placeholders.
    ArrayOfFileTransferInformation(Vec<FileTransferInformation>),
    /// A boxed array of *GuestListFileInfo*. To be used in *Any* placeholders.
    ArrayOfGuestListFileInfo(Vec<GuestListFileInfo>),
    /// A boxed array of *GuestPosixFileAttributes*. To be used in *Any* placeholders.
    ArrayOfGuestPosixFileAttributes(Vec<GuestPosixFileAttributes>),
    /// A boxed array of *GuestWindowsFileAttributes*. To be used in *Any* placeholders.
    ArrayOfGuestWindowsFileAttributes(Vec<GuestWindowsFileAttributes>),
    /// A boxed array of *GuestAuthentication*. To be used in *Any* placeholders.
    ArrayOfGuestAuthentication(Vec<Box<dyn super::traits::GuestAuthenticationTrait>>),
    /// A boxed array of *NamePasswordAuthentication*. To be used in *Any* placeholders.
    ArrayOfNamePasswordAuthentication(Vec<NamePasswordAuthentication>),
    /// A boxed array of *GuestProcessInfo*. To be used in *Any* placeholders.
    ArrayOfGuestProcessInfo(Vec<GuestProcessInfo>),
    /// A boxed array of *GuestProgramSpec*. To be used in *Any* placeholders.
    ArrayOfGuestProgramSpec(Vec<Box<dyn super::traits::GuestProgramSpecTrait>>),
    /// A boxed array of *GuestWindowsProgramSpec*. To be used in *Any* placeholders.
    ArrayOfGuestWindowsProgramSpec(Vec<GuestWindowsProgramSpec>),
    /// A boxed array of *SAMLTokenAuthentication*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSAMLTokenAuthentication")]
    ArrayOfSamlTokenAuthentication(Vec<SamlTokenAuthentication>),
    /// A boxed array of *SSPIAuthentication*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfSSPIAuthentication")]
    ArrayOfSspiAuthentication(Vec<SspiAuthentication>),
    /// A boxed array of *TicketedSessionAuthentication*. To be used in *Any* placeholders.
    ArrayOfTicketedSessionAuthentication(Vec<TicketedSessionAuthentication>),
    /// A boxed array of *GuestRegKeySpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegKeySpec(Vec<GuestRegKeySpec>),
    /// A boxed array of *GuestRegKeyNameSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegKeyNameSpec(Vec<GuestRegKeyNameSpec>),
    /// A boxed array of *GuestRegKeyRecordSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegKeyRecordSpec(Vec<GuestRegKeyRecordSpec>),
    /// A boxed array of *GuestRegValueSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueSpec(Vec<GuestRegValueSpec>),
    /// A boxed array of *GuestRegValueBinarySpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueBinarySpec(Vec<GuestRegValueBinarySpec>),
    /// A boxed array of *GuestRegValueDataSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueDataSpec(Vec<Box<dyn super::traits::GuestRegValueDataSpecTrait>>),
    /// A boxed array of *GuestRegValueDwordSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueDwordSpec(Vec<GuestRegValueDwordSpec>),
    /// A boxed array of *GuestRegValueExpandStringSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueExpandStringSpec(Vec<GuestRegValueExpandStringSpec>),
    /// A boxed array of *GuestRegValueMultiStringSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueMultiStringSpec(Vec<GuestRegValueMultiStringSpec>),
    /// A boxed array of *GuestRegValueNameSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueNameSpec(Vec<GuestRegValueNameSpec>),
    /// A boxed array of *GuestRegValueQwordSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueQwordSpec(Vec<GuestRegValueQwordSpec>),
    /// A boxed array of *GuestRegValueStringSpec*. To be used in *Any* placeholders.
    ArrayOfGuestRegValueStringSpec(Vec<GuestRegValueStringSpec>),
    /// A boxed array of *DeviceGroupId*. To be used in *Any* placeholders.
    ArrayOfDeviceGroupId(Vec<DeviceGroupId>),
    /// A boxed array of *FaultDomainId*. To be used in *Any* placeholders.
    ArrayOfFaultDomainId(Vec<FaultDomainId>),
    /// A boxed array of *ReplicationGroupId*. To be used in *Any* placeholders.
    ArrayOfReplicationGroupId(Vec<ReplicationGroupId>),
    /// A boxed array of *ReplicationSpec*. To be used in *Any* placeholders.
    ArrayOfReplicationSpec(Vec<ReplicationSpec>),
    /// A boxed array of *VsanClusterConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVsanClusterConfigInfo(Vec<VsanClusterConfigInfo>),
    /// A boxed array of *VsanClusterConfigInfoHostDefaultInfo*. To be used in *Any* placeholders.
    ArrayOfVsanClusterConfigInfoHostDefaultInfo(Vec<VsanClusterConfigInfoHostDefaultInfo>),
    /// A boxed array of *VsanHostClusterStatus*. To be used in *Any* placeholders.
    ArrayOfVsanHostClusterStatus(Vec<VsanHostClusterStatus>),
    /// A boxed array of *VsanHostClusterStatusState*. To be used in *Any* placeholders.
    ArrayOfVsanHostClusterStatusState(Vec<VsanHostClusterStatusState>),
    /// A boxed array of *VsanHostClusterStatusStateCompletionEstimate*. To be used in *Any* placeholders.
    ArrayOfVsanHostClusterStatusStateCompletionEstimate(Vec<VsanHostClusterStatusStateCompletionEstimate>),
    /// A boxed array of *VsanHostConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostConfigInfo(Vec<VsanHostConfigInfo>),
    /// A boxed array of *VsanHostConfigInfoClusterInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostConfigInfoClusterInfo(Vec<VsanHostConfigInfoClusterInfo>),
    /// A boxed array of *VsanHostFaultDomainInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostFaultDomainInfo(Vec<VsanHostFaultDomainInfo>),
    /// A boxed array of *VsanHostConfigInfoNetworkInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostConfigInfoNetworkInfo(Vec<VsanHostConfigInfoNetworkInfo>),
    /// A boxed array of *VsanHostConfigInfoNetworkInfoPortConfig*. To be used in *Any* placeholders.
    ArrayOfVsanHostConfigInfoNetworkInfoPortConfig(Vec<VsanHostConfigInfoNetworkInfoPortConfig>),
    /// A boxed array of *VsanHostConfigInfoStorageInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostConfigInfoStorageInfo(Vec<VsanHostConfigInfoStorageInfo>),
    /// A boxed array of *VsanHostDecommissionMode*. To be used in *Any* placeholders.
    ArrayOfVsanHostDecommissionMode(Vec<VsanHostDecommissionMode>),
    /// A boxed array of *VsanHostDiskMapInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostDiskMapInfo(Vec<VsanHostDiskMapInfo>),
    /// A boxed array of *VsanHostDiskMapResult*. To be used in *Any* placeholders.
    ArrayOfVsanHostDiskMapResult(Vec<VsanHostDiskMapResult>),
    /// A boxed array of *VsanHostDiskMapping*. To be used in *Any* placeholders.
    ArrayOfVsanHostDiskMapping(Vec<VsanHostDiskMapping>),
    /// A boxed array of *VsanHostDiskResult*. To be used in *Any* placeholders.
    ArrayOfVsanHostDiskResult(Vec<VsanHostDiskResult>),
    /// A boxed array of *VsanHostIpConfig*. To be used in *Any* placeholders.
    ArrayOfVsanHostIpConfig(Vec<VsanHostIpConfig>),
    /// A boxed array of *VsanHostMembershipInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostMembershipInfo(Vec<VsanHostMembershipInfo>),
    /// A boxed array of *VsanHostVsanDiskInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostVsanDiskInfo(Vec<VsanHostVsanDiskInfo>),
    /// A boxed array of *VsanHostRuntimeInfo*. To be used in *Any* placeholders.
    ArrayOfVsanHostRuntimeInfo(Vec<VsanHostRuntimeInfo>),
    /// A boxed array of *VsanHostRuntimeInfoDiskIssue*. To be used in *Any* placeholders.
    ArrayOfVsanHostRuntimeInfoDiskIssue(Vec<VsanHostRuntimeInfoDiskIssue>),
    /// A boxed array of *BaseConfigInfo*. To be used in *Any* placeholders.
    ArrayOfBaseConfigInfo(Vec<Box<dyn super::traits::BaseConfigInfoTrait>>),
    /// A boxed array of *BaseConfigInfoBackingInfo*. To be used in *Any* placeholders.
    ArrayOfBaseConfigInfoBackingInfo(Vec<Box<dyn super::traits::BaseConfigInfoBackingInfoTrait>>),
    /// A boxed array of *BaseConfigInfoDiskFileBackingInfo*. To be used in *Any* placeholders.
    ArrayOfBaseConfigInfoDiskFileBackingInfo(Vec<BaseConfigInfoDiskFileBackingInfo>),
    /// A boxed array of *BaseConfigInfoFileBackingInfo*. To be used in *Any* placeholders.
    ArrayOfBaseConfigInfoFileBackingInfo(Vec<Box<dyn super::traits::BaseConfigInfoFileBackingInfoTrait>>),
    /// A boxed array of *BaseConfigInfoRawDiskMappingBackingInfo*. To be used in *Any* placeholders.
    ArrayOfBaseConfigInfoRawDiskMappingBackingInfo(Vec<BaseConfigInfoRawDiskMappingBackingInfo>),
    /// A boxed array of *VslmCloneSpec*. To be used in *Any* placeholders.
    ArrayOfVslmCloneSpec(Vec<VslmCloneSpec>),
    /// A boxed array of *VslmCreateSpec*. To be used in *Any* placeholders.
    ArrayOfVslmCreateSpec(Vec<VslmCreateSpec>),
    /// A boxed array of *VslmCreateSpecBackingSpec*. To be used in *Any* placeholders.
    ArrayOfVslmCreateSpecBackingSpec(Vec<Box<dyn super::traits::VslmCreateSpecBackingSpecTrait>>),
    /// A boxed array of *VslmCreateSpecDiskFileBackingSpec*. To be used in *Any* placeholders.
    ArrayOfVslmCreateSpecDiskFileBackingSpec(Vec<VslmCreateSpecDiskFileBackingSpec>),
    /// A boxed array of *VslmCreateSpecRawDiskMappingBackingSpec*. To be used in *Any* placeholders.
    ArrayOfVslmCreateSpecRawDiskMappingBackingSpec(Vec<VslmCreateSpecRawDiskMappingBackingSpec>),
    /// A boxed array of *DiskCryptoSpec*. To be used in *Any* placeholders.
    ArrayOfDiskCryptoSpec(Vec<DiskCryptoSpec>),
    /// A boxed array of *ID*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfID")]
    ArrayOfId(Vec<Id>),
    /// A boxed array of *vslmInfrastructureObjectPolicy*. To be used in *Any* placeholders.
    ArrayOfvslmInfrastructureObjectPolicy(Vec<VslmInfrastructureObjectPolicy>),
    /// A boxed array of *vslmInfrastructureObjectPolicySpec*. To be used in *Any* placeholders.
    ArrayOfvslmInfrastructureObjectPolicySpec(Vec<VslmInfrastructureObjectPolicySpec>),
    /// A boxed array of *VslmMigrateSpec*. To be used in *Any* placeholders.
    ArrayOfVslmMigrateSpec(Vec<Box<dyn super::traits::VslmMigrateSpecTrait>>),
    /// A boxed array of *VslmRelocateSpec*. To be used in *Any* placeholders.
    ArrayOfVslmRelocateSpec(Vec<VslmRelocateSpec>),
    /// A boxed array of *VStorageObjectStateInfo*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectStateInfo(Vec<VStorageObjectStateInfo>),
    /// A boxed array of *VslmTagEntry*. To be used in *Any* placeholders.
    ArrayOfVslmTagEntry(Vec<VslmTagEntry>),
    /// A boxed array of *vslmVClockInfo*. To be used in *Any* placeholders.
    ArrayOfvslmVClockInfo(Vec<VslmVClockInfo>),
    /// A boxed array of *VStorageObject*. To be used in *Any* placeholders.
    ArrayOfVStorageObject(Vec<VStorageObject>),
    /// A boxed array of *VStorageObjectConfigInfo*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectConfigInfo(Vec<VStorageObjectConfigInfo>),
    /// A boxed array of *VStorageObjectSnapshot*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.2.0
    ArrayOfVStorageObjectSnapshot(Vec<VStorageObjectSnapshot>),
    /// A boxed array of *VStorageObjectSnapshotDetails*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectSnapshotDetails(Vec<VStorageObjectSnapshotDetails>),
    /// A boxed array of *VStorageObjectSnapshotInfo*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectSnapshotInfo(Vec<VStorageObjectSnapshotInfo>),
    /// A boxed array of *VStorageObjectSnapshotInfoVStorageObjectSnapshot*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectSnapshotInfoVStorageObjectSnapshot(Vec<VStorageObjectSnapshotInfoVStorageObjectSnapshot>),
    /// A boxed array of *RetrieveVStorageObjSpec*. To be used in *Any* placeholders.
    ArrayOfRetrieveVStorageObjSpec(Vec<RetrieveVStorageObjSpec>),
    /// A boxed array of *VStorageObjectAssociations*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectAssociations(Vec<VStorageObjectAssociations>),
    /// A boxed array of *VStorageObjectAssociationsVmDiskAssociations*. To be used in *Any* placeholders.
    ArrayOfVStorageObjectAssociationsVmDiskAssociations(Vec<VStorageObjectAssociationsVmDiskAssociations>),
    /// A boxed array of *DataObject*. To be used in *Any* placeholders.
    ArrayOfDataObject(Vec<Box<dyn super::traits::DataObjectTrait>>),
    /// A boxed array of *DynamicArray*. To be used in *Any* placeholders.
    ArrayOfDynamicArray(Vec<DynamicArray>),
    /// A boxed array of *DynamicProperty*. To be used in *Any* placeholders.
    ArrayOfDynamicProperty(Vec<DynamicProperty>),
    /// A boxed array of *KeyAnyValue*. To be used in *Any* placeholders.
    ArrayOfKeyAnyValue(Vec<KeyAnyValue>),
    /// A boxed array of *LocalizableMessage*. To be used in *Any* placeholders.
    ArrayOfLocalizableMessage(Vec<LocalizableMessage>),
    /// A boxed array of *LocalizedMethodFault*. To be used in *Any* placeholders.
    ArrayOfLocalizedMethodFault(Vec<LocalizedMethodFault>),
    /// A boxed array of *MethodFault*. To be used in *Any* placeholders.
    ArrayOfMethodFault(Vec<Box<dyn super::traits::MethodFaultTrait>>),
    /// A boxed array of *RuntimeFault*. To be used in *Any* placeholders.
    ArrayOfRuntimeFault(Vec<Box<dyn super::traits::RuntimeFaultTrait>>),
    /// A boxed array of *HostCommunication*. To be used in *Any* placeholders.
    ArrayOfHostCommunication(Vec<Box<dyn super::traits::HostCommunicationTrait>>),
    /// A boxed array of *HostNotConnected*. To be used in *Any* placeholders.
    ArrayOfHostNotConnected(Vec<HostNotConnected>),
    /// A boxed array of *HostNotReachable*. To be used in *Any* placeholders.
    ArrayOfHostNotReachable(Vec<HostNotReachable>),
    /// A boxed array of *InvalidArgument*. To be used in *Any* placeholders.
    ArrayOfInvalidArgument(Vec<Box<dyn super::traits::InvalidArgumentTrait>>),
    /// A boxed array of *InvalidRequest*. To be used in *Any* placeholders.
    ArrayOfInvalidRequest(Vec<Box<dyn super::traits::InvalidRequestTrait>>),
    /// A boxed array of *InvalidType*. To be used in *Any* placeholders.
    ArrayOfInvalidType(Vec<InvalidType>),
    /// A boxed array of *ManagedObjectNotFound*. To be used in *Any* placeholders.
    ArrayOfManagedObjectNotFound(Vec<ManagedObjectNotFound>),
    /// A boxed array of *MethodNotFound*. To be used in *Any* placeholders.
    ArrayOfMethodNotFound(Vec<MethodNotFound>),
    /// A boxed array of *NotEnoughLicenses*. To be used in *Any* placeholders.
    ArrayOfNotEnoughLicenses(Vec<Box<dyn super::traits::NotEnoughLicensesTrait>>),
    /// A boxed array of *NotImplemented*. To be used in *Any* placeholders.
    ArrayOfNotImplemented(Vec<NotImplemented>),
    /// A boxed array of *NotSupported*. To be used in *Any* placeholders.
    ArrayOfNotSupported(Vec<Box<dyn super::traits::NotSupportedTrait>>),
    /// A boxed array of *RequestCanceled*. To be used in *Any* placeholders.
    ArrayOfRequestCanceled(Vec<RequestCanceled>),
    /// A boxed array of *SecurityError*. To be used in *Any* placeholders.
    ArrayOfSecurityError(Vec<Box<dyn super::traits::SecurityErrorTrait>>),
    /// A boxed array of *SystemError*. To be used in *Any* placeholders.
    ArrayOfSystemError(Vec<SystemError>),
    /// A boxed array of *UnexpectedFault*. To be used in *Any* placeholders.
    ArrayOfUnexpectedFault(Vec<UnexpectedFault>),
    /// A boxed array of *InvalidCollectorVersion*. To be used in *Any* placeholders.
    ArrayOfInvalidCollectorVersion(Vec<InvalidCollectorVersion>),
    /// A boxed array of *InvalidProperty*. To be used in *Any* placeholders.
    ArrayOfInvalidProperty(Vec<InvalidProperty>),
    /// A boxed array of *PropertyChange*. To be used in *Any* placeholders.
    ArrayOfPropertyChange(Vec<PropertyChange>),
    /// A boxed array of *PropertyFilterSpec*. To be used in *Any* placeholders.
    ArrayOfPropertyFilterSpec(Vec<PropertyFilterSpec>),
    /// A boxed array of *PropertyFilterUpdate*. To be used in *Any* placeholders.
    ArrayOfPropertyFilterUpdate(Vec<PropertyFilterUpdate>),
    /// A boxed array of *MissingObject*. To be used in *Any* placeholders.
    ArrayOfMissingObject(Vec<MissingObject>),
    /// A boxed array of *MissingProperty*. To be used in *Any* placeholders.
    ArrayOfMissingProperty(Vec<MissingProperty>),
    /// A boxed array of *ObjectContent*. To be used in *Any* placeholders.
    ArrayOfObjectContent(Vec<ObjectContent>),
    /// A boxed array of *ObjectSpec*. To be used in *Any* placeholders.
    ArrayOfObjectSpec(Vec<ObjectSpec>),
    /// A boxed array of *ObjectUpdate*. To be used in *Any* placeholders.
    ArrayOfObjectUpdate(Vec<ObjectUpdate>),
    /// A boxed array of *PropertySpec*. To be used in *Any* placeholders.
    ArrayOfPropertySpec(Vec<PropertySpec>),
    /// A boxed array of *RetrieveOptions*. To be used in *Any* placeholders.
    ArrayOfRetrieveOptions(Vec<RetrieveOptions>),
    /// A boxed array of *RetrieveResult*. To be used in *Any* placeholders.
    ArrayOfRetrieveResult(Vec<RetrieveResult>),
    /// A boxed array of *SelectionSpec*. To be used in *Any* placeholders.
    ArrayOfSelectionSpec(Vec<Box<dyn super::traits::SelectionSpecTrait>>),
    /// A boxed array of *TraversalSpec*. To be used in *Any* placeholders.
    ArrayOfTraversalSpec(Vec<TraversalSpec>),
    /// A boxed array of *UpdateSet*. To be used in *Any* placeholders.
    ArrayOfUpdateSet(Vec<UpdateSet>),
    /// A boxed array of *WaitOptions*. To be used in *Any* placeholders.
    ArrayOfWaitOptions(Vec<WaitOptions>),
    /// A boxed *ComputeResourceHostSPBMLicenseInfoHostSPBMLicenseState_enum*. To be used in *Any* placeholders.
    #[serde(rename = "ComputeResourceHostSPBMLicenseInfoHostSPBMLicenseState")]
    ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseState(super::enums::ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum),
    /// A boxed array of *ComputeResourceHostSPBMLicenseInfoHostSPBMLicenseState_enum*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfComputeResourceHostSPBMLicenseInfoHostSPBMLicenseState")]
    ArrayOfComputeResourceHostSpbmLicenseInfoHostSpbmLicenseState(Vec<super::enums::ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum>),
    /// A boxed *DatastoreAccessible_enum*. To be used in *Any* placeholders.
    DatastoreAccessible(super::enums::DatastoreAccessibleEnum),
    /// A boxed array of *DatastoreAccessible_enum*. To be used in *Any* placeholders.
    ArrayOfDatastoreAccessible(Vec<super::enums::DatastoreAccessibleEnum>),
    /// A boxed *DrsInjectorWorkloadCorrelationState_enum*. To be used in *Any* placeholders.
    DrsInjectorWorkloadCorrelationState(super::enums::DrsInjectorWorkloadCorrelationStateEnum),
    /// A boxed array of *DrsInjectorWorkloadCorrelationState_enum*. To be used in *Any* placeholders.
    ArrayOfDrsInjectorWorkloadCorrelationState(Vec<super::enums::DrsInjectorWorkloadCorrelationStateEnum>),
    /// A boxed *HostSystemConnectionState_enum*. To be used in *Any* placeholders.
    HostSystemConnectionState(super::enums::HostSystemConnectionStateEnum),
    /// A boxed array of *HostSystemConnectionState_enum*. To be used in *Any* placeholders.
    ArrayOfHostSystemConnectionState(Vec<super::enums::HostSystemConnectionStateEnum>),
    /// A boxed *HostSystemPowerState_enum*. To be used in *Any* placeholders.
    HostSystemPowerState(super::enums::HostSystemPowerStateEnum),
    /// A boxed array of *HostSystemPowerState_enum*. To be used in *Any* placeholders.
    ArrayOfHostSystemPowerState(Vec<super::enums::HostSystemPowerStateEnum>),
    /// A boxed *HttpNfcLeaseState_enum*. To be used in *Any* placeholders.
    HttpNfcLeaseState(super::enums::HttpNfcLeaseStateEnum),
    /// A boxed array of *HttpNfcLeaseState_enum*. To be used in *Any* placeholders.
    ArrayOfHttpNfcLeaseState(Vec<super::enums::HttpNfcLeaseStateEnum>),
    /// A boxed *LatencySensitivitySensitivityLevel_enum*. To be used in *Any* placeholders.
    LatencySensitivitySensitivityLevel(super::enums::LatencySensitivitySensitivityLevelEnum),
    /// A boxed array of *LatencySensitivitySensitivityLevel_enum*. To be used in *Any* placeholders.
    ArrayOfLatencySensitivitySensitivityLevel(Vec<super::enums::LatencySensitivitySensitivityLevelEnum>),
    /// A boxed *LicenseFeatureInfoState_enum*. To be used in *Any* placeholders.
    LicenseFeatureInfoState(super::enums::LicenseFeatureInfoStateEnum),
    /// A boxed array of *LicenseFeatureInfoState_enum*. To be used in *Any* placeholders.
    ArrayOfLicenseFeatureInfoState(Vec<super::enums::LicenseFeatureInfoStateEnum>),
    /// A boxed *HostLicensableResourceKey_enum*. To be used in *Any* placeholders.
    HostLicensableResourceKey(super::enums::HostLicensableResourceKeyEnum),
    /// A boxed array of *HostLicensableResourceKey_enum*. To be used in *Any* placeholders.
    ArrayOfHostLicensableResourceKey(Vec<super::enums::HostLicensableResourceKeyEnum>),
    /// A boxed *LicenseManagerState_enum*. To be used in *Any* placeholders.
    LicenseManagerState(super::enums::LicenseManagerStateEnum),
    /// A boxed array of *LicenseManagerState_enum*. To be used in *Any* placeholders.
    ArrayOfLicenseManagerState(Vec<super::enums::LicenseManagerStateEnum>),
    /// A boxed *LicenseReservationInfoState_enum*. To be used in *Any* placeholders.
    LicenseReservationInfoState(super::enums::LicenseReservationInfoStateEnum),
    /// A boxed array of *LicenseReservationInfoState_enum*. To be used in *Any* placeholders.
    ArrayOfLicenseReservationInfoState(Vec<super::enums::LicenseReservationInfoStateEnum>),
    /// A boxed *ManagedEntityStatus_enum*. To be used in *Any* placeholders.
    ManagedEntityStatus(super::enums::ManagedEntityStatusEnum),
    /// A boxed array of *ManagedEntityStatus_enum*. To be used in *Any* placeholders.
    ArrayOfManagedEntityStatus(Vec<super::enums::ManagedEntityStatusEnum>),
    /// A boxed *PerfSummaryType_enum*. To be used in *Any* placeholders.
    PerfSummaryType(super::enums::PerfSummaryTypeEnum),
    /// A boxed array of *PerfSummaryType_enum*. To be used in *Any* placeholders.
    ArrayOfPerfSummaryType(Vec<super::enums::PerfSummaryTypeEnum>),
    /// A boxed *PerfStatsType_enum*. To be used in *Any* placeholders.
    PerfStatsType(super::enums::PerfStatsTypeEnum),
    /// A boxed array of *PerfStatsType_enum*. To be used in *Any* placeholders.
    ArrayOfPerfStatsType(Vec<super::enums::PerfStatsTypeEnum>),
    /// A boxed *SharesLevel_enum*. To be used in *Any* placeholders.
    SharesLevel(super::enums::SharesLevelEnum),
    /// A boxed array of *SharesLevel_enum*. To be used in *Any* placeholders.
    ArrayOfSharesLevel(Vec<super::enums::SharesLevelEnum>),
    /// A boxed *SimpleCommandEncoding_enum*. To be used in *Any* placeholders.
    SimpleCommandEncoding(super::enums::SimpleCommandEncodingEnum),
    /// A boxed array of *SimpleCommandEncoding_enum*. To be used in *Any* placeholders.
    ArrayOfSimpleCommandEncoding(Vec<super::enums::SimpleCommandEncodingEnum>),
    /// A boxed *TaskFilterSpecRecursionOption_enum*. To be used in *Any* placeholders.
    TaskFilterSpecRecursionOption(super::enums::TaskFilterSpecRecursionOptionEnum),
    /// A boxed array of *TaskFilterSpecRecursionOption_enum*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpecRecursionOption(Vec<super::enums::TaskFilterSpecRecursionOptionEnum>),
    /// A boxed *TaskFilterSpecTimeOption_enum*. To be used in *Any* placeholders.
    TaskFilterSpecTimeOption(super::enums::TaskFilterSpecTimeOptionEnum),
    /// A boxed array of *TaskFilterSpecTimeOption_enum*. To be used in *Any* placeholders.
    ArrayOfTaskFilterSpecTimeOption(Vec<super::enums::TaskFilterSpecTimeOptionEnum>),
    /// A boxed *TaskInfoState_enum*. To be used in *Any* placeholders.
    TaskInfoState(super::enums::TaskInfoStateEnum),
    /// A boxed array of *TaskInfoState_enum*. To be used in *Any* placeholders.
    ArrayOfTaskInfoState(Vec<super::enums::TaskInfoStateEnum>),
    /// A boxed *VirtualAppVAppState_enum*. To be used in *Any* placeholders.
    VirtualAppVAppState(super::enums::VirtualAppVAppStateEnum),
    /// A boxed array of *VirtualAppVAppState_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualAppVAppState(Vec<super::enums::VirtualAppVAppStateEnum>),
    /// A boxed *VirtualMachineConnectionState_enum*. To be used in *Any* placeholders.
    VirtualMachineConnectionState(super::enums::VirtualMachineConnectionStateEnum),
    /// A boxed array of *VirtualMachineConnectionState_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConnectionState(Vec<super::enums::VirtualMachineConnectionStateEnum>),
    /// A boxed *VirtualMachineFaultToleranceState_enum*. To be used in *Any* placeholders.
    VirtualMachineFaultToleranceState(super::enums::VirtualMachineFaultToleranceStateEnum),
    /// A boxed array of *VirtualMachineFaultToleranceState_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineFaultToleranceState(Vec<super::enums::VirtualMachineFaultToleranceStateEnum>),
    /// A boxed *VirtualMachineMovePriority_enum*. To be used in *Any* placeholders.
    VirtualMachineMovePriority(super::enums::VirtualMachineMovePriorityEnum),
    /// A boxed array of *VirtualMachineMovePriority_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMovePriority(Vec<super::enums::VirtualMachineMovePriorityEnum>),
    /// A boxed *VirtualMachinePowerState_enum*. To be used in *Any* placeholders.
    VirtualMachinePowerState(super::enums::VirtualMachinePowerStateEnum),
    /// A boxed array of *VirtualMachinePowerState_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachinePowerState(Vec<super::enums::VirtualMachinePowerStateEnum>),
    /// A boxed *VirtualMachineRecordReplayState_enum*. To be used in *Any* placeholders.
    VirtualMachineRecordReplayState(super::enums::VirtualMachineRecordReplayStateEnum),
    /// A boxed array of *VirtualMachineRecordReplayState_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRecordReplayState(Vec<super::enums::VirtualMachineRecordReplayStateEnum>),
    /// A boxed *VsanUpgradeSystemUpgradeHistoryDiskGroupOpType_enum*. To be used in *Any* placeholders.
    VsanUpgradeSystemUpgradeHistoryDiskGroupOpType(super::enums::VsanUpgradeSystemUpgradeHistoryDiskGroupOpTypeEnum),
    /// A boxed array of *VsanUpgradeSystemUpgradeHistoryDiskGroupOpType_enum*. To be used in *Any* placeholders.
    ArrayOfVsanUpgradeSystemUpgradeHistoryDiskGroupOpType(Vec<super::enums::VsanUpgradeSystemUpgradeHistoryDiskGroupOpTypeEnum>),
    /// A boxed *MetricAlarmOperator_enum*. To be used in *Any* placeholders.
    MetricAlarmOperator(super::enums::MetricAlarmOperatorEnum),
    /// A boxed array of *MetricAlarmOperator_enum*. To be used in *Any* placeholders.
    ArrayOfMetricAlarmOperator(Vec<super::enums::MetricAlarmOperatorEnum>),
    /// A boxed *StateAlarmOperator_enum*. To be used in *Any* placeholders.
    StateAlarmOperator(super::enums::StateAlarmOperatorEnum),
    /// A boxed array of *StateAlarmOperator_enum*. To be used in *Any* placeholders.
    ArrayOfStateAlarmOperator(Vec<super::enums::StateAlarmOperatorEnum>),
    /// A boxed *DasVmPriority_enum*. To be used in *Any* placeholders.
    DasVmPriority(super::enums::DasVmPriorityEnum),
    /// A boxed array of *DasVmPriority_enum*. To be used in *Any* placeholders.
    ArrayOfDasVmPriority(Vec<super::enums::DasVmPriorityEnum>),
    /// A boxed *DpmBehavior_enum*. To be used in *Any* placeholders.
    DpmBehavior(super::enums::DpmBehaviorEnum),
    /// A boxed array of *DpmBehavior_enum*. To be used in *Any* placeholders.
    ArrayOfDpmBehavior(Vec<super::enums::DpmBehaviorEnum>),
    /// A boxed *DrsBehavior_enum*. To be used in *Any* placeholders.
    DrsBehavior(super::enums::DrsBehaviorEnum),
    /// A boxed array of *DrsBehavior_enum*. To be used in *Any* placeholders.
    ArrayOfDrsBehavior(Vec<super::enums::DrsBehaviorEnum>),
    /// A boxed *HostPowerOperationType_enum*. To be used in *Any* placeholders.
    HostPowerOperationType(super::enums::HostPowerOperationTypeEnum),
    /// A boxed array of *HostPowerOperationType_enum*. To be used in *Any* placeholders.
    ArrayOfHostPowerOperationType(Vec<super::enums::HostPowerOperationTypeEnum>),
    /// A boxed *EventFilterSpecRecursionOption_enum*. To be used in *Any* placeholders.
    EventFilterSpecRecursionOption(super::enums::EventFilterSpecRecursionOptionEnum),
    /// A boxed array of *EventFilterSpecRecursionOption_enum*. To be used in *Any* placeholders.
    ArrayOfEventFilterSpecRecursionOption(Vec<super::enums::EventFilterSpecRecursionOptionEnum>),
    /// A boxed *FtIssuesOnHostHostSelectionType_enum*. To be used in *Any* placeholders.
    FtIssuesOnHostHostSelectionType(super::enums::FtIssuesOnHostHostSelectionTypeEnum),
    /// A boxed array of *FtIssuesOnHostHostSelectionType_enum*. To be used in *Any* placeholders.
    ArrayOfFtIssuesOnHostHostSelectionType(Vec<super::enums::FtIssuesOnHostHostSelectionTypeEnum>),
    /// A boxed *AutoStartWaitHeartbeatSetting_enum*. To be used in *Any* placeholders.
    AutoStartWaitHeartbeatSetting(super::enums::AutoStartWaitHeartbeatSettingEnum),
    /// A boxed array of *AutoStartWaitHeartbeatSetting_enum*. To be used in *Any* placeholders.
    ArrayOfAutoStartWaitHeartbeatSetting(Vec<super::enums::AutoStartWaitHeartbeatSettingEnum>),
    /// A boxed *FibreChannelPortType_enum*. To be used in *Any* placeholders.
    FibreChannelPortType(super::enums::FibreChannelPortTypeEnum),
    /// A boxed array of *FibreChannelPortType_enum*. To be used in *Any* placeholders.
    ArrayOfFibreChannelPortType(Vec<super::enums::FibreChannelPortTypeEnum>),
    /// A boxed *HostAccessMode_enum*. To be used in *Any* placeholders.
    HostAccessMode(super::enums::HostAccessModeEnum),
    /// A boxed array of *HostAccessMode_enum*. To be used in *Any* placeholders.
    ArrayOfHostAccessMode(Vec<super::enums::HostAccessModeEnum>),
    /// A boxed *HostLockdownMode_enum*. To be used in *Any* placeholders.
    HostLockdownMode(super::enums::HostLockdownModeEnum),
    /// A boxed array of *HostLockdownMode_enum*. To be used in *Any* placeholders.
    ArrayOfHostLockdownMode(Vec<super::enums::HostLockdownModeEnum>),
    /// A boxed *HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationType_enum*. To be used in *Any* placeholders.
    HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationType(super::enums::HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationTypeEnum),
    /// A boxed array of *HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationType_enum*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaIscsiIpv6AddressAddressConfigurationType(Vec<super::enums::HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationTypeEnum>),
    /// A boxed *HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperation_enum*. To be used in *Any* placeholders.
    HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperation(super::enums::HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperationEnum),
    /// A boxed array of *HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperation_enum*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperation(Vec<super::enums::HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperationEnum>),
    /// A boxed *HostInternetScsiHbaNetworkBindingSupportType_enum*. To be used in *Any* placeholders.
    HostInternetScsiHbaNetworkBindingSupportType(super::enums::HostInternetScsiHbaNetworkBindingSupportTypeEnum),
    /// A boxed array of *HostInternetScsiHbaNetworkBindingSupportType_enum*. To be used in *Any* placeholders.
    ArrayOfHostInternetScsiHbaNetworkBindingSupportType(Vec<super::enums::HostInternetScsiHbaNetworkBindingSupportTypeEnum>),
    /// A boxed *HostFirewallRuleDirection_enum*. To be used in *Any* placeholders.
    HostFirewallRuleDirection(super::enums::HostFirewallRuleDirectionEnum),
    /// A boxed array of *HostFirewallRuleDirection_enum*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRuleDirection(Vec<super::enums::HostFirewallRuleDirectionEnum>),
    /// A boxed *HostFirewallRulePortType_enum*. To be used in *Any* placeholders.
    HostFirewallRulePortType(super::enums::HostFirewallRulePortTypeEnum),
    /// A boxed array of *HostFirewallRulePortType_enum*. To be used in *Any* placeholders.
    ArrayOfHostFirewallRulePortType(Vec<super::enums::HostFirewallRulePortTypeEnum>),
    /// A boxed *HostSnmpAgentCapability_enum*. To be used in *Any* placeholders.
    HostSnmpAgentCapability(super::enums::HostSnmpAgentCapabilityEnum),
    /// A boxed array of *HostSnmpAgentCapability_enum*. To be used in *Any* placeholders.
    ArrayOfHostSnmpAgentCapability(Vec<super::enums::HostSnmpAgentCapabilityEnum>),
    /// A boxed *HostTpmAttestationInfoAcceptanceStatus_enum*. To be used in *Any* placeholders.
    HostTpmAttestationInfoAcceptanceStatus(super::enums::HostTpmAttestationInfoAcceptanceStatusEnum),
    /// A boxed array of *HostTpmAttestationInfoAcceptanceStatus_enum*. To be used in *Any* placeholders.
    ArrayOfHostTpmAttestationInfoAcceptanceStatus(Vec<super::enums::HostTpmAttestationInfoAcceptanceStatusEnum>),
    /// A boxed *ArrayUpdateOperation_enum*. To be used in *Any* placeholders.
    ArrayUpdateOperation(super::enums::ArrayUpdateOperationEnum),
    /// A boxed array of *ArrayUpdateOperation_enum*. To be used in *Any* placeholders.
    ArrayOfArrayUpdateOperation(Vec<super::enums::ArrayUpdateOperationEnum>),
    /// A boxed *ProfileNumericComparator_enum*. To be used in *Any* placeholders.
    ProfileNumericComparator(super::enums::ProfileNumericComparatorEnum),
    /// A boxed array of *ProfileNumericComparator_enum*. To be used in *Any* placeholders.
    ArrayOfProfileNumericComparator(Vec<super::enums::ProfileNumericComparatorEnum>),
    /// A boxed *DayOfWeek_enum*. To be used in *Any* placeholders.
    DayOfWeek(super::enums::DayOfWeekEnum),
    /// A boxed array of *DayOfWeek_enum*. To be used in *Any* placeholders.
    ArrayOfDayOfWeek(Vec<super::enums::DayOfWeekEnum>),
    /// A boxed *WeekOfMonth_enum*. To be used in *Any* placeholders.
    WeekOfMonth(super::enums::WeekOfMonthEnum),
    /// A boxed array of *WeekOfMonth_enum*. To be used in *Any* placeholders.
    ArrayOfWeekOfMonth(Vec<super::enums::WeekOfMonthEnum>),
    /// A boxed *VirtualMachineCloneSpecTpmProvisionPolicy_enum*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    VirtualMachineCloneSpecTpmProvisionPolicy(super::enums::VirtualMachineCloneSpecTpmProvisionPolicyEnum),
    /// A boxed array of *VirtualMachineCloneSpecTpmProvisionPolicy_enum*. To be used in *Any* placeholders.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    ArrayOfVirtualMachineCloneSpecTpmProvisionPolicy(Vec<super::enums::VirtualMachineCloneSpecTpmProvisionPolicyEnum>),
    /// A boxed *VirtualMachineConfigInfoNpivWwnType_enum*. To be used in *Any* placeholders.
    VirtualMachineConfigInfoNpivWwnType(super::enums::VirtualMachineConfigInfoNpivWwnTypeEnum),
    /// A boxed array of *VirtualMachineConfigInfoNpivWwnType_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineConfigInfoNpivWwnType(Vec<super::enums::VirtualMachineConfigInfoNpivWwnTypeEnum>),
    /// A boxed *VirtualMachineToolsStatus_enum*. To be used in *Any* placeholders.
    VirtualMachineToolsStatus(super::enums::VirtualMachineToolsStatusEnum),
    /// A boxed array of *VirtualMachineToolsStatus_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineToolsStatus(Vec<super::enums::VirtualMachineToolsStatusEnum>),
    /// A boxed *GuestQuiesceEndGuestQuiesceError_enum*. To be used in *Any* placeholders.
    GuestQuiesceEndGuestQuiesceError(super::enums::GuestQuiesceEndGuestQuiesceErrorEnum),
    /// A boxed array of *GuestQuiesceEndGuestQuiesceError_enum*. To be used in *Any* placeholders.
    ArrayOfGuestQuiesceEndGuestQuiesceError(Vec<super::enums::GuestQuiesceEndGuestQuiesceErrorEnum>),
    /// A boxed *VirtualMachineMetadataManagerVmMetadataOp_enum*. To be used in *Any* placeholders.
    VirtualMachineMetadataManagerVmMetadataOp(super::enums::VirtualMachineMetadataManagerVmMetadataOpEnum),
    /// A boxed array of *VirtualMachineMetadataManagerVmMetadataOp_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineMetadataManagerVmMetadataOp(Vec<super::enums::VirtualMachineMetadataManagerVmMetadataOpEnum>),
    /// A boxed *VirtualMachineRelocateTransformation_enum*. To be used in *Any* placeholders.
    VirtualMachineRelocateTransformation(super::enums::VirtualMachineRelocateTransformationEnum),
    /// A boxed array of *VirtualMachineRelocateTransformation_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualMachineRelocateTransformation(Vec<super::enums::VirtualMachineRelocateTransformationEnum>),
    /// A boxed *CustomizationNetBIOSMode_enum*. To be used in *Any* placeholders.
    #[serde(rename = "CustomizationNetBIOSMode")]
    CustomizationNetBiosMode(super::enums::CustomizationNetBiosModeEnum),
    /// A boxed array of *CustomizationNetBIOSMode_enum*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfCustomizationNetBIOSMode")]
    ArrayOfCustomizationNetBiosMode(Vec<super::enums::CustomizationNetBiosModeEnum>),
    /// A boxed *CustomizationLicenseDataMode_enum*. To be used in *Any* placeholders.
    CustomizationLicenseDataMode(super::enums::CustomizationLicenseDataModeEnum),
    /// A boxed array of *CustomizationLicenseDataMode_enum*. To be used in *Any* placeholders.
    ArrayOfCustomizationLicenseDataMode(Vec<super::enums::CustomizationLicenseDataModeEnum>),
    /// A boxed *CustomizationSysprepRebootOption_enum*. To be used in *Any* placeholders.
    CustomizationSysprepRebootOption(super::enums::CustomizationSysprepRebootOptionEnum),
    /// A boxed array of *CustomizationSysprepRebootOption_enum*. To be used in *Any* placeholders.
    ArrayOfCustomizationSysprepRebootOption(Vec<super::enums::CustomizationSysprepRebootOptionEnum>),
    /// A boxed *VirtualDeviceConfigSpecFileOperation_enum*. To be used in *Any* placeholders.
    VirtualDeviceConfigSpecFileOperation(super::enums::VirtualDeviceConfigSpecFileOperationEnum),
    /// A boxed array of *VirtualDeviceConfigSpecFileOperation_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConfigSpecFileOperation(Vec<super::enums::VirtualDeviceConfigSpecFileOperationEnum>),
    /// A boxed *VirtualDeviceConfigSpecOperation_enum*. To be used in *Any* placeholders.
    VirtualDeviceConfigSpecOperation(super::enums::VirtualDeviceConfigSpecOperationEnum),
    /// A boxed array of *VirtualDeviceConfigSpecOperation_enum*. To be used in *Any* placeholders.
    ArrayOfVirtualDeviceConfigSpecOperation(Vec<super::enums::VirtualDeviceConfigSpecOperationEnum>),
    /// A boxed *VirtualSCSISharing_enum*. To be used in *Any* placeholders.
    #[serde(rename = "VirtualSCSISharing")]
    VirtualScsiSharing(super::enums::VirtualScsiSharingEnum),
    /// A boxed array of *VirtualSCSISharing_enum*. To be used in *Any* placeholders.
    #[serde(rename = "ArrayOfVirtualSCSISharing")]
    ArrayOfVirtualScsiSharing(Vec<super::enums::VirtualScsiSharingEnum>),
    /// A boxed *VsanHostDiskResultState_enum*. To be used in *Any* placeholders.
    VsanHostDiskResultState(super::enums::VsanHostDiskResultStateEnum),
    /// A boxed array of *VsanHostDiskResultState_enum*. To be used in *Any* placeholders.
    ArrayOfVsanHostDiskResultState(Vec<super::enums::VsanHostDiskResultStateEnum>),
    /// A boxed *VsanHostHealthState_enum*. To be used in *Any* placeholders.
    VsanHostHealthState(super::enums::VsanHostHealthStateEnum),
    /// A boxed array of *VsanHostHealthState_enum*. To be used in *Any* placeholders.
    ArrayOfVsanHostHealthState(Vec<super::enums::VsanHostHealthStateEnum>),
    /// A boxed *VsanHostNodeState_enum*. To be used in *Any* placeholders.
    VsanHostNodeState(super::enums::VsanHostNodeStateEnum),
    /// A boxed array of *VsanHostNodeState_enum*. To be used in *Any* placeholders.
    ArrayOfVsanHostNodeState(Vec<super::enums::VsanHostNodeStateEnum>),
    /// A boxed *PropertyChangeOp_enum*. To be used in *Any* placeholders.
    PropertyChangeOp(super::enums::PropertyChangeOpEnum),
    /// A boxed array of *PropertyChangeOp_enum*. To be used in *Any* placeholders.
    ArrayOfPropertyChangeOp(Vec<super::enums::PropertyChangeOpEnum>),
    /// A boxed *ObjectUpdateKind_enum*. To be used in *Any* placeholders.
    ObjectUpdateKind(super::enums::ObjectUpdateKindEnum),
    /// A boxed array of *ObjectUpdateKind_enum*. To be used in *Any* placeholders.
    ArrayOfObjectUpdateKind(Vec<super::enums::ObjectUpdateKindEnum>),
}
