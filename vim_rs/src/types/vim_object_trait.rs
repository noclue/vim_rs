use super::as_any::AsAny;
use super::dyn_serialize;
use super::struct_enum::StructType;
use super::structs::*;

/// Base trait of all VIM (Virtual Infrastructure Management) objects.
/// This trait is used to obtain the actual data type of object even
/// when used through a trait reference. The other use of this trait is
/// to upcast a trait reference to a VimObjectTrait reference needed by
/// common library functionality.
pub trait VimObjectTrait: AsAny + std::fmt::Debug + Send + Sync {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait;
    fn data_type(&self) -> StructType;
}

impl serde::Serialize for dyn VimObjectTrait {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        dyn_serialize::serialize_polymorphic(self, serializer)
    }
}

impl VimObjectTrait for ManagedObjectReference {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ManagedObjectReference
    }
}

impl VimObjectTrait for DataObject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DataObject
    }
}

impl VimObjectTrait for AboutInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AboutInfo
    }
}

impl VimObjectTrait for AuthorizationDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthorizationDescription
    }
}

impl VimObjectTrait for EntityPrivilege {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityPrivilege
    }
}

impl VimObjectTrait for Permission {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Permission
    }
}

impl VimObjectTrait for AuthorizationPrivilege {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthorizationPrivilege
    }
}

impl VimObjectTrait for PrivilegeAvailability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PrivilegeAvailability
    }
}

impl VimObjectTrait for AuthorizationRole {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthorizationRole
    }
}

impl VimObjectTrait for UserPrivilegeResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserPrivilegeResult
    }
}

impl VimObjectTrait for BatchResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BatchResult
    }
}

impl VimObjectTrait for Capability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Capability
    }
}

impl VimObjectTrait for ClusterComputeResourceClusterConfigResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceClusterConfigResult
    }
}

impl VimObjectTrait for ClusterComputeResourceDvsSetting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceDvsSetting
    }
}

impl VimObjectTrait for ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping
    }
}

impl VimObjectTrait for ClusterComputeResourceDvsProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceDvsProfile
    }
}

impl VimObjectTrait for ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping
    }
}

impl VimObjectTrait for ClusterComputeResourceHciConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHciConfigInfo
    }
}

impl VimObjectTrait for ClusterComputeResourceHciConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHciConfigSpec
    }
}

impl VimObjectTrait for ClusterComputeResourceHostConfigurationInput {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHostConfigurationInput
    }
}

impl VimObjectTrait for ClusterComputeResourceHostConfigurationProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHostConfigurationProfile
    }
}

impl VimObjectTrait for ClusterComputeResourceHostVmkNicInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHostVmkNicInfo
    }
}

impl VimObjectTrait for ClusterComputeResourceVcProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceVcProfile
    }
}

impl VimObjectTrait for ClusterComputeResourceValidationResultBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceValidationResultBase
    }
}

impl VimObjectTrait for ClusterComputeResourceDvsConfigurationValidation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceDvsConfigurationValidation
    }
}

impl VimObjectTrait for ClusterComputeResourceHostConfigurationValidation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHostConfigurationValidation
    }
}

impl VimObjectTrait for ClusterComputeResourceVcsSlots {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceVcsSlots
    }
}

impl VimObjectTrait for ComputeResourceConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceConfigInfo
    }
}

impl VimObjectTrait for ClusterConfigInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterConfigInfoEx
    }
}

impl VimObjectTrait for ComputeResourceConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceConfigSpec
    }
}

impl VimObjectTrait for ClusterConfigSpecEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterConfigSpecEx
    }
}

impl VimObjectTrait for ComputeResourceHostSpbmLicenseInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceHostSpbmLicenseInfo
    }
}

impl VimObjectTrait for ComputeResourceSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceSummary
    }
}

impl VimObjectTrait for ClusterComputeResourceSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceSummary
    }
}

impl VimObjectTrait for CustomFieldDef {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldDef
    }
}

impl VimObjectTrait for CustomFieldValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldValue
    }
}

impl VimObjectTrait for CustomFieldStringValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldStringValue
    }
}

impl VimObjectTrait for CustomizationSpecInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSpecInfo
    }
}

impl VimObjectTrait for CustomizationSpecItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSpecItem
    }
}

impl VimObjectTrait for DatacenterBasicConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterBasicConnectInfo
    }
}

impl VimObjectTrait for DatacenterConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterConfigInfo
    }
}

impl VimObjectTrait for DatacenterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterConfigSpec
    }
}

impl VimObjectTrait for DatastoreCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreCapability
    }
}

impl VimObjectTrait for DatastoreHostMount {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreHostMount
    }
}

impl VimObjectTrait for DatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreInfo
    }
}

impl VimObjectTrait for LocalDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalDatastoreInfo
    }
}

impl VimObjectTrait for NasDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasDatastoreInfo
    }
}

impl VimObjectTrait for PMemDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PMemDatastoreInfo
    }
}

impl VimObjectTrait for VmfsDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreInfo
    }
}

impl VimObjectTrait for VsanDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreInfo
    }
}

impl VimObjectTrait for VvolDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VvolDatastoreInfo
    }
}

impl VimObjectTrait for DatastoreMountPathDatastorePair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreMountPathDatastorePair
    }
}

impl VimObjectTrait for DatastoreSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreSummary
    }
}

impl VimObjectTrait for DatastoreVVolContainerFailoverPair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreVVolContainerFailoverPair
    }
}

impl VimObjectTrait for DatastoreNamespaceManagerDirectoryInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreNamespaceManagerDirectoryInfo
    }
}

impl VimObjectTrait for Description {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Description
    }
}

impl VimObjectTrait for ElementDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ElementDescription
    }
}

impl VimObjectTrait for EvcMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcMode
    }
}

impl VimObjectTrait for ExtendedElementDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtendedElementDescription
    }
}

impl VimObjectTrait for FeatureEvcMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FeatureEvcMode
    }
}

impl VimObjectTrait for OptionDef {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OptionDef
    }
}

impl VimObjectTrait for ExtendedDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtendedDescription
    }
}

impl VimObjectTrait for MethodDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodDescription
    }
}

impl VimObjectTrait for TypeDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TypeDescription
    }
}

impl VimObjectTrait for ScheduledTaskDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskDetail
    }
}

impl VimObjectTrait for DesiredSoftwareSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DesiredSoftwareSpec
    }
}

impl VimObjectTrait for DesiredSoftwareSpecBaseImageSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DesiredSoftwareSpecBaseImageSpec
    }
}

impl VimObjectTrait for DesiredSoftwareSpecComponentSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DesiredSoftwareSpecComponentSpec
    }
}

impl VimObjectTrait for DesiredSoftwareSpecVendorAddOnSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DesiredSoftwareSpecVendorAddOnSpec
    }
}

impl VimObjectTrait for DiagnosticManagerAuditRecordResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiagnosticManagerAuditRecordResult
    }
}

impl VimObjectTrait for DiagnosticManagerBundleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiagnosticManagerBundleInfo
    }
}

impl VimObjectTrait for DiagnosticManagerLogDescriptor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiagnosticManagerLogDescriptor
    }
}

impl VimObjectTrait for DiagnosticManagerLogHeader {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiagnosticManagerLogHeader
    }
}

impl VimObjectTrait for DvsBackupRestoreCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsBackupRestoreCapability
    }
}

impl VimObjectTrait for DvsCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsCapability
    }
}

impl VimObjectTrait for DvsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsConfigInfo
    }
}

impl VimObjectTrait for VMwareDvsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsConfigInfo
    }
}

impl VimObjectTrait for DvsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsConfigSpec
    }
}

impl VimObjectTrait for VMwareDvsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsConfigSpec
    }
}

impl VimObjectTrait for DvsContactInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsContactInfo
    }
}

impl VimObjectTrait for DvsCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsCreateSpec
    }
}

impl VimObjectTrait for DvsFeatureCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFeatureCapability
    }
}

impl VimObjectTrait for VMwareDvsFeatureCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsFeatureCapability
    }
}

impl VimObjectTrait for DvsHealthCheckConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHealthCheckConfig
    }
}

impl VimObjectTrait for VMwareDvsHealthCheckConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsHealthCheckConfig
    }
}

impl VimObjectTrait for VMwareDvsTeamingHealthCheckConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsTeamingHealthCheckConfig
    }
}

impl VimObjectTrait for VMwareDvsVlanMtuHealthCheckConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsVlanMtuHealthCheckConfig
    }
}

impl VimObjectTrait for DvsHealthCheckCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHealthCheckCapability
    }
}

impl VimObjectTrait for VMwareDvsHealthCheckCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsHealthCheckCapability
    }
}

impl VimObjectTrait for DvsHostInfrastructureTrafficResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostInfrastructureTrafficResource
    }
}

impl VimObjectTrait for DvsHostInfrastructureTrafficResourceAllocation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostInfrastructureTrafficResourceAllocation
    }
}

impl VimObjectTrait for DvsNetworkResourceManagementCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkResourceManagementCapability
    }
}

impl VimObjectTrait for DvsResourceRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsResourceRuntimeInfo
    }
}

impl VimObjectTrait for DvsRollbackCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsRollbackCapability
    }
}

impl VimObjectTrait for DvsRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsRuntimeInfo
    }
}

impl VimObjectTrait for DvsSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsSummary
    }
}

impl VimObjectTrait for DvsPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPolicy
    }
}

impl VimObjectTrait for DvsUplinkPortPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUplinkPortPolicy
    }
}

impl VimObjectTrait for DvsNameArrayUplinkPortPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNameArrayUplinkPortPolicy
    }
}

impl VimObjectTrait for EnumDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnumDescription
    }
}

impl VimObjectTrait for EnvironmentBrowserConfigOptionQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnvironmentBrowserConfigOptionQuerySpec
    }
}

impl VimObjectTrait for Extension {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Extension
    }
}

impl VimObjectTrait for ExtensionClientInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionClientInfo
    }
}

impl VimObjectTrait for ExtensionEventTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionEventTypeInfo
    }
}

impl VimObjectTrait for ExtensionFaultTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionFaultTypeInfo
    }
}

impl VimObjectTrait for ExtensionHealthInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionHealthInfo
    }
}

impl VimObjectTrait for ExtensionOvfConsumerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionOvfConsumerInfo
    }
}

impl VimObjectTrait for ExtensionPrivilegeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionPrivilegeInfo
    }
}

impl VimObjectTrait for ExtensionResourceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionResourceInfo
    }
}

impl VimObjectTrait for ExtensionServerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionServerInfo
    }
}

impl VimObjectTrait for ExtensionTaskTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionTaskTypeInfo
    }
}

impl VimObjectTrait for ExtensionManagerIpAllocationUsage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensionManagerIpAllocationUsage
    }
}

impl VimObjectTrait for FaultsByHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultsByHost
    }
}

impl VimObjectTrait for FaultsByVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultsByVm
    }
}

impl VimObjectTrait for FileLockInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileLockInfo
    }
}

impl VimObjectTrait for FileLockInfoResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileLockInfoResult
    }
}

impl VimObjectTrait for FolderBatchAddHostsToClusterResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderBatchAddHostsToClusterResult
    }
}

impl VimObjectTrait for FolderBatchAddStandaloneHostsResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderBatchAddStandaloneHostsResult
    }
}

impl VimObjectTrait for FolderFailedHostResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderFailedHostResult
    }
}

impl VimObjectTrait for FolderNewHostSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderNewHostSpec
    }
}

impl VimObjectTrait for HbrManagerReplicationVmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrManagerReplicationVmInfo
    }
}

impl VimObjectTrait for ReplicationVmProgressInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationVmProgressInfo
    }
}

impl VimObjectTrait for HbrManagerVmReplicationCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrManagerVmReplicationCapability
    }
}

impl VimObjectTrait for HealthUpdate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HealthUpdate
    }
}

impl VimObjectTrait for HealthUpdateInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HealthUpdateInfo
    }
}

impl VimObjectTrait for PerfInterval {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfInterval
    }
}

impl VimObjectTrait for HostServiceTicket {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostServiceTicket
    }
}

impl VimObjectTrait for HostSystemComplianceCheckState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemComplianceCheckState
    }
}

impl VimObjectTrait for HostSystemReconnectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemReconnectSpec
    }
}

impl VimObjectTrait for HostSystemRemediationState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemRemediationState
    }
}

impl VimObjectTrait for HttpNfcLeaseCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseCapabilities
    }
}

impl VimObjectTrait for HttpNfcLeaseDatastoreLeaseInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseDatastoreLeaseInfo
    }
}

impl VimObjectTrait for HttpNfcLeaseDeviceUrl {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseDeviceUrl
    }
}

impl VimObjectTrait for HttpNfcLeaseHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseHostInfo
    }
}

impl VimObjectTrait for HttpNfcLeaseInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseInfo
    }
}

impl VimObjectTrait for HttpNfcLeaseManifestEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseManifestEntry
    }
}

impl VimObjectTrait for HttpNfcLeaseProbeResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseProbeResult
    }
}

impl VimObjectTrait for HttpNfcLeaseSourceFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpNfcLeaseSourceFile
    }
}

impl VimObjectTrait for ImportSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ImportSpec
    }
}

impl VimObjectTrait for VirtualAppImportSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualAppImportSpec
    }
}

impl VimObjectTrait for VirtualMachineImportSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineImportSpec
    }
}

impl VimObjectTrait for InheritablePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InheritablePolicy
    }
}

impl VimObjectTrait for BoolPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BoolPolicy
    }
}

impl VimObjectTrait for IntPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntPolicy
    }
}

impl VimObjectTrait for LongPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LongPolicy
    }
}

impl VimObjectTrait for StringPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StringPolicy
    }
}

impl VimObjectTrait for DvsFilterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterConfig
    }
}

impl VimObjectTrait for DvsFilterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterConfigSpec
    }
}

impl VimObjectTrait for DvsTrafficFilterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsTrafficFilterConfig
    }
}

impl VimObjectTrait for DvsTrafficFilterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsTrafficFilterConfigSpec
    }
}

impl VimObjectTrait for DvsFilterPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterPolicy
    }
}

impl VimObjectTrait for DvsTrafficShapingPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsTrafficShapingPolicy
    }
}

impl VimObjectTrait for DvsVendorSpecificConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVendorSpecificConfig
    }
}

impl VimObjectTrait for DvsFailureCriteria {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFailureCriteria
    }
}

impl VimObjectTrait for DvsMacLearningPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsMacLearningPolicy
    }
}

impl VimObjectTrait for DvsMacManagementPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsMacManagementPolicy
    }
}

impl VimObjectTrait for DvsSecurityPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsSecurityPolicy
    }
}

impl VimObjectTrait for VMwareUplinkLacpPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareUplinkLacpPolicy
    }
}

impl VimObjectTrait for VMwareUplinkPortOrderPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareUplinkPortOrderPolicy
    }
}

impl VimObjectTrait for VmwareUplinkPortTeamingPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareUplinkPortTeamingPolicy
    }
}

impl VimObjectTrait for VmwareDistributedVirtualSwitchVlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchVlanSpec
    }
}

impl VimObjectTrait for VmwareDistributedVirtualSwitchPvlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchPvlanSpec
    }
}

impl VimObjectTrait for VmwareDistributedVirtualSwitchTrunkVlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec
    }
}

impl VimObjectTrait for VmwareDistributedVirtualSwitchVlanIdSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchVlanIdSpec
    }
}

impl VimObjectTrait for IoFilterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IoFilterInfo
    }
}

impl VimObjectTrait for ClusterIoFilterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterIoFilterInfo
    }
}

impl VimObjectTrait for HostIoFilterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIoFilterInfo
    }
}

impl VimObjectTrait for IoFilterQueryIssueResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IoFilterQueryIssueResult
    }
}

impl VimObjectTrait for IoFilterHostIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IoFilterHostIssue
    }
}

impl VimObjectTrait for IpPoolManagerIpAllocation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpPoolManagerIpAllocation
    }
}

impl VimObjectTrait for KeyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KeyValue
    }
}

impl VimObjectTrait for LatencySensitivity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LatencySensitivity
    }
}

impl VimObjectTrait for LicenseAssignmentManagerLicenseAssignment {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseAssignmentManagerLicenseAssignment
    }
}

impl VimObjectTrait for LicenseAvailabilityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseAvailabilityInfo
    }
}

impl VimObjectTrait for LicenseDiagnostics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseDiagnostics
    }
}

impl VimObjectTrait for LicenseManagerEvaluationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseManagerEvaluationInfo
    }
}

impl VimObjectTrait for LicenseFeatureInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseFeatureInfo
    }
}

impl VimObjectTrait for HostLicensableResourceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLicensableResourceInfo
    }
}

impl VimObjectTrait for LicenseManagerLicenseInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseManagerLicenseInfo
    }
}

impl VimObjectTrait for LicenseSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseSource
    }
}

impl VimObjectTrait for EvaluationLicenseSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvaluationLicenseSource
    }
}

impl VimObjectTrait for LicenseServerSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseServerSource
    }
}

impl VimObjectTrait for LocalLicenseSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalLicenseSource
    }
}

impl VimObjectTrait for LicenseUsageInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseUsageInfo
    }
}

impl VimObjectTrait for LicenseReservationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseReservationInfo
    }
}

impl VimObjectTrait for LocalizationManagerMessageCatalog {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalizationManagerMessageCatalog
    }
}

impl VimObjectTrait for NegatableExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NegatableExpression
    }
}

impl VimObjectTrait for IntExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntExpression
    }
}

impl VimObjectTrait for IpAddress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpAddress
    }
}

impl VimObjectTrait for IpRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpRange
    }
}

impl VimObjectTrait for SingleIp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SingleIp
    }
}

impl VimObjectTrait for MacAddress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MacAddress
    }
}

impl VimObjectTrait for MacRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MacRange
    }
}

impl VimObjectTrait for SingleMac {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SingleMac
    }
}

impl VimObjectTrait for StringExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StringExpression
    }
}

impl VimObjectTrait for DvsIpPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsIpPort
    }
}

impl VimObjectTrait for DvsIpPortRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsIpPortRange
    }
}

impl VimObjectTrait for DvsSingleIpPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsSingleIpPort
    }
}

impl VimObjectTrait for NetworkSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkSummary
    }
}

impl VimObjectTrait for OpaqueNetworkSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OpaqueNetworkSummary
    }
}

impl VimObjectTrait for NumericRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumericRange
    }
}

impl VimObjectTrait for OpaqueNetworkCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OpaqueNetworkCapability
    }
}

impl VimObjectTrait for OvfConsumerOstNode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerOstNode
    }
}

impl VimObjectTrait for OvfConsumerOvfSection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerOvfSection
    }
}

impl VimObjectTrait for OvfManagerCommonParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfManagerCommonParams
    }
}

impl VimObjectTrait for OvfCreateImportSpecParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCreateImportSpecParams
    }
}

impl VimObjectTrait for OvfParseDescriptorParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfParseDescriptorParams
    }
}

impl VimObjectTrait for OvfValidateHostParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfValidateHostParams
    }
}

impl VimObjectTrait for OvfCreateDescriptorParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCreateDescriptorParams
    }
}

impl VimObjectTrait for OvfCreateDescriptorResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCreateDescriptorResult
    }
}

impl VimObjectTrait for OvfCreateImportSpecResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCreateImportSpecResult
    }
}

impl VimObjectTrait for OvfDeploymentOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDeploymentOption
    }
}

impl VimObjectTrait for OvfFileItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfFileItem
    }
}

impl VimObjectTrait for OvfNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNetworkInfo
    }
}

impl VimObjectTrait for OvfNetworkMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNetworkMapping
    }
}

impl VimObjectTrait for OvfFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfFile
    }
}

impl VimObjectTrait for OvfOptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfOptionInfo
    }
}

impl VimObjectTrait for OvfParseDescriptorResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfParseDescriptorResult
    }
}

impl VimObjectTrait for OvfResourceMap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfResourceMap
    }
}

impl VimObjectTrait for OvfValidateHostResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfValidateHostResult
    }
}

impl VimObjectTrait for PasswordField {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PasswordField
    }
}

impl VimObjectTrait for PerformanceDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerformanceDescription
    }
}

impl VimObjectTrait for PerfCompositeMetric {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfCompositeMetric
    }
}

impl VimObjectTrait for PerfCounterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfCounterInfo
    }
}

impl VimObjectTrait for PerformanceManagerCounterLevelMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerformanceManagerCounterLevelMapping
    }
}

impl VimObjectTrait for PerfEntityMetricBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfEntityMetricBase
    }
}

impl VimObjectTrait for PerfEntityMetric {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfEntityMetric
    }
}

impl VimObjectTrait for PerfEntityMetricCsv {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfEntityMetricCsv
    }
}

impl VimObjectTrait for PerfMetricId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfMetricId
    }
}

impl VimObjectTrait for PerfMetricSeries {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfMetricSeries
    }
}

impl VimObjectTrait for PerfMetricIntSeries {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfMetricIntSeries
    }
}

impl VimObjectTrait for PerfMetricSeriesCsv {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfMetricSeriesCsv
    }
}

impl VimObjectTrait for PerfProviderSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfProviderSummary
    }
}

impl VimObjectTrait for PerfQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfQuerySpec
    }
}

impl VimObjectTrait for PerfSampleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerfSampleInfo
    }
}

impl VimObjectTrait for PrivilegePolicyDef {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PrivilegePolicyDef
    }
}

impl VimObjectTrait for ResourceAllocationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceAllocationInfo
    }
}

impl VimObjectTrait for ResourceAllocationOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceAllocationOption
    }
}

impl VimObjectTrait for ResourceConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceConfigOption
    }
}

impl VimObjectTrait for ResourceConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceConfigSpec
    }
}

impl VimObjectTrait for DatabaseSizeEstimate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatabaseSizeEstimate
    }
}

impl VimObjectTrait for DatabaseSizeParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatabaseSizeParam
    }
}

impl VimObjectTrait for InventoryDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InventoryDescription
    }
}

impl VimObjectTrait for PerformanceStatisticsDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerformanceStatisticsDescription
    }
}

impl VimObjectTrait for ResourcePoolResourceUsage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolResourceUsage
    }
}

impl VimObjectTrait for ResourcePoolRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolRuntimeInfo
    }
}

impl VimObjectTrait for ResourcePoolSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolSummary
    }
}

impl VimObjectTrait for VirtualAppSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualAppSummary
    }
}

impl VimObjectTrait for ResourcePoolQuickStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolQuickStats
    }
}

impl VimObjectTrait for SddcBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SddcBase
    }
}

impl VimObjectTrait for SelectionSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SelectionSet
    }
}

impl VimObjectTrait for DvPortgroupSelection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupSelection
    }
}

impl VimObjectTrait for DvsSelection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsSelection
    }
}

impl VimObjectTrait for HostVMotionCompatibility {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionCompatibility
    }
}

impl VimObjectTrait for ProductComponentInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProductComponentInfo
    }
}

impl VimObjectTrait for ServiceContent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceContent
    }
}

impl VimObjectTrait for ServiceLocator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceLocator
    }
}

impl VimObjectTrait for ServiceLocatorCredential {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceLocatorCredential
    }
}

impl VimObjectTrait for ServiceLocatorNamePassword {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceLocatorNamePassword
    }
}

impl VimObjectTrait for ServiceLocatorSamlCredential {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceLocatorSamlCredential
    }
}

impl VimObjectTrait for ServiceManagerServiceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceManagerServiceInfo
    }
}

impl VimObjectTrait for SessionManagerGenericServiceTicket {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionManagerGenericServiceTicket
    }
}

impl VimObjectTrait for SessionManagerLocalTicket {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionManagerLocalTicket
    }
}

impl VimObjectTrait for SessionManagerServiceRequestSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionManagerServiceRequestSpec
    }
}

impl VimObjectTrait for SessionManagerHttpServiceRequestSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionManagerHttpServiceRequestSpec
    }
}

impl VimObjectTrait for SessionManagerVmomiServiceRequestSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionManagerVmomiServiceRequestSpec
    }
}

impl VimObjectTrait for SharesInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SharesInfo
    }
}

impl VimObjectTrait for SharesOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SharesOption
    }
}

impl VimObjectTrait for SiteInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SiteInfo
    }
}

impl VimObjectTrait for StoragePodSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePodSummary
    }
}

impl VimObjectTrait for StorageIoAllocationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageIoAllocationInfo
    }
}

impl VimObjectTrait for StorageIoAllocationOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageIoAllocationOption
    }
}

impl VimObjectTrait for StorageIormInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageIormInfo
    }
}

impl VimObjectTrait for StorageIormConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageIormConfigOption
    }
}

impl VimObjectTrait for StorageIormConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageIormConfigSpec
    }
}

impl VimObjectTrait for PodStorageDrsEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PodStorageDrsEntry
    }
}

impl VimObjectTrait for StoragePerformanceSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePerformanceSummary
    }
}

impl VimObjectTrait for StorageResourceManagerStorageProfileStatistics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageResourceManagerStorageProfileStatistics
    }
}

impl VimObjectTrait for Tag {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Tag
    }
}

impl VimObjectTrait for TaskDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskDescription
    }
}

impl VimObjectTrait for TaskFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskFilterSpec
    }
}

impl VimObjectTrait for TaskFilterSpecByEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskFilterSpecByEntity
    }
}

impl VimObjectTrait for TaskFilterSpecByTime {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskFilterSpecByTime
    }
}

impl VimObjectTrait for TaskFilterSpecByUsername {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskFilterSpecByUsername
    }
}

impl VimObjectTrait for TaskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskInfo
    }
}

impl VimObjectTrait for TaskReason {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskReason
    }
}

impl VimObjectTrait for TaskReasonAlarm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskReasonAlarm
    }
}

impl VimObjectTrait for TaskReasonSchedule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskReasonSchedule
    }
}

impl VimObjectTrait for TaskReasonSystem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskReasonSystem
    }
}

impl VimObjectTrait for TaskReasonUser {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskReasonUser
    }
}

impl VimObjectTrait for UpdateVirtualMachineFilesResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UpdateVirtualMachineFilesResult
    }
}

impl VimObjectTrait for UpdateVirtualMachineFilesResultFailedVmFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UpdateVirtualMachineFilesResultFailedVmFileInfo
    }
}

impl VimObjectTrait for UserSearchResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserSearchResult
    }
}

impl VimObjectTrait for PosixUserSearchResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PosixUserSearchResult
    }
}

impl VimObjectTrait for UserSession {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserSession
    }
}

impl VimObjectTrait for VVolVmConfigFileUpdateResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VVolVmConfigFileUpdateResult
    }
}

impl VimObjectTrait for VVolVmConfigFileUpdateResultFailedVmConfigFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo
    }
}

impl VimObjectTrait for VasaStorageArray {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaStorageArray
    }
}

impl VimObjectTrait for VasaStorageArrayDiscoveryFcTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaStorageArrayDiscoveryFcTransport
    }
}

impl VimObjectTrait for VasaStorageArrayDiscoveryIpTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaStorageArrayDiscoveryIpTransport
    }
}

impl VimObjectTrait for VasaStorageArrayDiscoverySvcInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaStorageArrayDiscoverySvcInfo
    }
}

impl VimObjectTrait for VasaProviderContainerSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaProviderContainerSpec
    }
}

impl VimObjectTrait for VimVasaProvider {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVasaProvider
    }
}

impl VimObjectTrait for VimVasaProviderStatePerArray {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVasaProviderStatePerArray
    }
}

impl VimObjectTrait for VimVasaProviderVirtualHostConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVasaProviderVirtualHostConfig
    }
}

impl VimObjectTrait for VimVasaProviderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVasaProviderInfo
    }
}

impl VimObjectTrait for VirtualAppLinkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualAppLinkInfo
    }
}

impl VimObjectTrait for VirtualDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSpec
    }
}

impl VimObjectTrait for DeviceBackedVirtualDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceBackedVirtualDiskSpec
    }
}

impl VimObjectTrait for FileBackedVirtualDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileBackedVirtualDiskSpec
    }
}

impl VimObjectTrait for SeSparseVirtualDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SeSparseVirtualDiskSpec
    }
}

impl VimObjectTrait for VirtualMachineConnection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConnection
    }
}

impl VimObjectTrait for VirtualMachineMksConnection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMksConnection
    }
}

impl VimObjectTrait for DiskChangeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskChangeInfo
    }
}

impl VimObjectTrait for DiskChangeExtent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskChangeExtent
    }
}

impl VimObjectTrait for VirtualMachineDisplayTopology {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDisplayTopology
    }
}

impl VimObjectTrait for VirtualMachineMksTicket {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMksTicket
    }
}

impl VimObjectTrait for StorageRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageRequirement
    }
}

impl VimObjectTrait for VirtualMachineTicket {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineTicket
    }
}

impl VimObjectTrait for VirtualMachineWipeResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineWipeResult
    }
}

impl VimObjectTrait for VsanUpgradeSystemNetworkPartitionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemNetworkPartitionInfo
    }
}

impl VimObjectTrait for VsanUpgradeSystemPreflightCheckIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemPreflightCheckIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemApiBrokenIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemApiBrokenIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemAutoClaimEnabledOnHostsIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemHostsDisconnectedIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemHostsDisconnectedIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemMissingHostsInClusterIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemMissingHostsInClusterIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemNetworkPartitionIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemNetworkPartitionIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemNotEnoughFreeCapacityIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemRogueHostsInClusterIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemRogueHostsInClusterIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemWrongEsxVersionIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemWrongEsxVersionIssue
    }
}

impl VimObjectTrait for VsanUpgradeSystemPreflightCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemPreflightCheckResult
    }
}

impl VimObjectTrait for VsanUpgradeSystemUpgradeHistoryItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemUpgradeHistoryItem
    }
}

impl VimObjectTrait for VsanUpgradeSystemUpgradeHistoryDiskGroupOp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp
    }
}

impl VimObjectTrait for VsanUpgradeSystemUpgradeHistoryPreflightFail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail
    }
}

impl VimObjectTrait for VsanUpgradeSystemUpgradeStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemUpgradeStatus
    }
}

impl VimObjectTrait for Action {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Action
    }
}

impl VimObjectTrait for CreateTaskAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CreateTaskAction
    }
}

impl VimObjectTrait for MethodAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodAction
    }
}

impl VimObjectTrait for RunScriptAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RunScriptAction
    }
}

impl VimObjectTrait for SendEmailAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SendEmailAction
    }
}

impl VimObjectTrait for SendSnmpAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SendSnmpAction
    }
}

impl VimObjectTrait for MethodActionArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodActionArgument
    }
}

impl VimObjectTrait for AlarmAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmAction
    }
}

impl VimObjectTrait for AlarmTriggeringAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmTriggeringAction
    }
}

impl VimObjectTrait for GroupAlarmAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GroupAlarmAction
    }
}

impl VimObjectTrait for AlarmDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmDescription
    }
}

impl VimObjectTrait for AlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmExpression
    }
}

impl VimObjectTrait for AndAlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AndAlarmExpression
    }
}

impl VimObjectTrait for EventAlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventAlarmExpression
    }
}

impl VimObjectTrait for MetricAlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MetricAlarmExpression
    }
}

impl VimObjectTrait for OrAlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OrAlarmExpression
    }
}

impl VimObjectTrait for StateAlarmExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StateAlarmExpression
    }
}

impl VimObjectTrait for AlarmFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmFilterSpec
    }
}

impl VimObjectTrait for AlarmSetting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmSetting
    }
}

impl VimObjectTrait for AlarmSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmSpec
    }
}

impl VimObjectTrait for AlarmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmInfo
    }
}

impl VimObjectTrait for AlarmState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmState
    }
}

impl VimObjectTrait for AlarmTriggeringActionTransitionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmTriggeringActionTransitionSpec
    }
}

impl VimObjectTrait for EventAlarmExpressionComparison {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventAlarmExpressionComparison
    }
}

impl VimObjectTrait for ClusterAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAction
    }
}

impl VimObjectTrait for ClusterClusterInitialPlacementAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterClusterInitialPlacementAction
    }
}

impl VimObjectTrait for ClusterHostInfraUpdateHaModeAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterHostInfraUpdateHaModeAction
    }
}

impl VimObjectTrait for ClusterHostPowerAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterHostPowerAction
    }
}

impl VimObjectTrait for ClusterInitialPlacementAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterInitialPlacementAction
    }
}

impl VimObjectTrait for ClusterMigrationAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterMigrationAction
    }
}

impl VimObjectTrait for PlacementAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementAction
    }
}

impl VimObjectTrait for HbrDiskMigrationAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrDiskMigrationAction
    }
}

impl VimObjectTrait for StorageMigrationAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageMigrationAction
    }
}

impl VimObjectTrait for StoragePlacementAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePlacementAction
    }
}

impl VimObjectTrait for ClusterActionHistory {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterActionHistory
    }
}

impl VimObjectTrait for ClusterAttemptedVmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAttemptedVmInfo
    }
}

impl VimObjectTrait for ClusterConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterConfigInfo
    }
}

impl VimObjectTrait for ClusterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterConfigSpec
    }
}

impl VimObjectTrait for ClusterCryptoConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterCryptoConfigInfo
    }
}

impl VimObjectTrait for ClusterDasAamNodeState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAamNodeState
    }
}

impl VimObjectTrait for ClusterDasAdmissionControlInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAdmissionControlInfo
    }
}

impl VimObjectTrait for ClusterFailoverHostAdmissionControlInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverHostAdmissionControlInfo
    }
}

impl VimObjectTrait for ClusterFailoverLevelAdmissionControlInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverLevelAdmissionControlInfo
    }
}

impl VimObjectTrait for ClusterFailoverResourcesAdmissionControlInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverResourcesAdmissionControlInfo
    }
}

impl VimObjectTrait for ClusterDasAdmissionControlPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAdmissionControlPolicy
    }
}

impl VimObjectTrait for ClusterFailoverHostAdmissionControlPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverHostAdmissionControlPolicy
    }
}

impl VimObjectTrait for ClusterFailoverLevelAdmissionControlPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverLevelAdmissionControlPolicy
    }
}

impl VimObjectTrait for ClusterFailoverResourcesAdmissionControlPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverResourcesAdmissionControlPolicy
    }
}

impl VimObjectTrait for ClusterDasAdvancedRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAdvancedRuntimeInfo
    }
}

impl VimObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo
    }
}

impl VimObjectTrait for DasHeartbeatDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasHeartbeatDatastoreInfo
    }
}

impl VimObjectTrait for ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo
    }
}

impl VimObjectTrait for ClusterDasConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasConfigInfo
    }
}

impl VimObjectTrait for ClusterDasData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasData
    }
}

impl VimObjectTrait for ClusterDasDataSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasDataSummary
    }
}

impl VimObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots
    }
}

impl VimObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo
    }
}

impl VimObjectTrait for ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots
    }
}

impl VimObjectTrait for ClusterDasFdmHostState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasFdmHostState
    }
}

impl VimObjectTrait for ClusterDasHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasHostInfo
    }
}

impl VimObjectTrait for ClusterDasAamHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasAamHostInfo
    }
}

impl VimObjectTrait for ClusterDasHostRecommendation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasHostRecommendation
    }
}

impl VimObjectTrait for ClusterDasVmConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasVmConfigInfo
    }
}

impl VimObjectTrait for ClusterDasVmSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasVmSettings
    }
}

impl VimObjectTrait for ClusterDpmConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDpmConfigInfo
    }
}

impl VimObjectTrait for ClusterDpmHostConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDpmHostConfigInfo
    }
}

impl VimObjectTrait for ClusterDrsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsConfigInfo
    }
}

impl VimObjectTrait for ClusterDrsFaults {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsFaults
    }
}

impl VimObjectTrait for ClusterDrsFaultsFaultsByVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsFaultsFaultsByVm
    }
}

impl VimObjectTrait for ClusterDrsFaultsFaultsByVirtualDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsFaultsFaultsByVirtualDisk
    }
}

impl VimObjectTrait for ClusterDrsMigration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsMigration
    }
}

impl VimObjectTrait for ClusterDrsRecommendation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsRecommendation
    }
}

impl VimObjectTrait for ClusterDrsVmConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsVmConfigInfo
    }
}

impl VimObjectTrait for ClusterEvcManagerCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterEvcManagerCheckResult
    }
}

impl VimObjectTrait for ClusterEvcManagerEvcState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterEvcManagerEvcState
    }
}

impl VimObjectTrait for ClusterEnterMaintenanceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterEnterMaintenanceResult
    }
}

impl VimObjectTrait for ClusterFailoverHostAdmissionControlInfoHostStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFailoverHostAdmissionControlInfoHostStatus
    }
}

impl VimObjectTrait for ClusterGroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterGroupInfo
    }
}

impl VimObjectTrait for ClusterHostGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterHostGroup
    }
}

impl VimObjectTrait for ClusterVmGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmGroup
    }
}

impl VimObjectTrait for ClusterHostRecommendation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterHostRecommendation
    }
}

impl VimObjectTrait for ClusterInfraUpdateHaConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterInfraUpdateHaConfigInfo
    }
}

impl VimObjectTrait for ClusterNotAttemptedVmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterNotAttemptedVmInfo
    }
}

impl VimObjectTrait for ClusterOrchestrationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterOrchestrationInfo
    }
}

impl VimObjectTrait for PlacementResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementResult
    }
}

impl VimObjectTrait for PlacementSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementSpec
    }
}

impl VimObjectTrait for ClusterPowerOnVmResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterPowerOnVmResult
    }
}

impl VimObjectTrait for ClusterPreemptibleVmPairInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterPreemptibleVmPairInfo
    }
}

impl VimObjectTrait for ClusterProactiveDrsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProactiveDrsConfigInfo
    }
}

impl VimObjectTrait for ClusterRecommendation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterRecommendation
    }
}

impl VimObjectTrait for ClusterResourceUsageSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterResourceUsageSummary
    }
}

impl VimObjectTrait for ClusterRuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterRuleInfo
    }
}

impl VimObjectTrait for ClusterAffinityRuleSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAffinityRuleSpec
    }
}

impl VimObjectTrait for ClusterAntiAffinityRuleSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAntiAffinityRuleSpec
    }
}

impl VimObjectTrait for ClusterDependencyRuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDependencyRuleInfo
    }
}

impl VimObjectTrait for ClusterVmHostRuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmHostRuleInfo
    }
}

impl VimObjectTrait for VirtualDiskAntiAffinityRuleSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskAntiAffinityRuleSpec
    }
}

impl VimObjectTrait for VirtualDiskRuleSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskRuleSpec
    }
}

impl VimObjectTrait for ClusterSlotPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterSlotPolicy
    }
}

impl VimObjectTrait for ClusterFixedSizeSlotPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFixedSizeSlotPolicy
    }
}

impl VimObjectTrait for ClusterSystemVMsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterSystemVMsConfigInfo
    }
}

impl VimObjectTrait for ClusterSystemVMsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterSystemVMsConfigSpec
    }
}

impl VimObjectTrait for ClusterUsageSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterUsageSummary
    }
}

impl VimObjectTrait for ClusterVmComponentProtectionSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmComponentProtectionSettings
    }
}

impl VimObjectTrait for ClusterVmOrchestrationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmOrchestrationInfo
    }
}

impl VimObjectTrait for ClusterVmReadiness {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmReadiness
    }
}

impl VimObjectTrait for ClusterVmToolsMonitoringSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmToolsMonitoringSettings
    }
}

impl VimObjectTrait for DistributedVirtualPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualPort
    }
}

impl VimObjectTrait for DvPortConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortConfigInfo
    }
}

impl VimObjectTrait for DvPortConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortConfigSpec
    }
}

impl VimObjectTrait for DvsFilterParameter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterParameter
    }
}

impl VimObjectTrait for DvsHostLocalPortInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostLocalPortInfo
    }
}

impl VimObjectTrait for DvPortStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortStatus
    }
}

impl VimObjectTrait for DvPortSetting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortSetting
    }
}

impl VimObjectTrait for VMwareDvsPortSetting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsPortSetting
    }
}

impl VimObjectTrait for DvPortState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortState
    }
}

impl VimObjectTrait for DvPortgroupConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupConfigInfo
    }
}

impl VimObjectTrait for DvPortgroupConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupConfigSpec
    }
}

impl VimObjectTrait for DistributedVirtualPortgroupNsxPortgroupOperationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualPortgroupNsxPortgroupOperationResult
    }
}

impl VimObjectTrait for DvPortgroupPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupPolicy
    }
}

impl VimObjectTrait for VMwareDvsPortgroupPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsPortgroupPolicy
    }
}

impl VimObjectTrait for DistributedVirtualPortgroupProblem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualPortgroupProblem
    }
}

impl VimObjectTrait for DistributedVirtualPortgroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualPortgroupInfo
    }
}

impl VimObjectTrait for DistributedVirtualSwitchInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchInfo
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerCompatibilityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerCompatibilityResult
    }
}

impl VimObjectTrait for DvsManagerDvsConfigTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsManagerDvsConfigTarget
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerDvsProductSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerDvsProductSpec
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerHostContainer {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerHostContainer
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerHostDvsFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerHostArrayFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerHostArrayFilter
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerHostContainerFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerHostContainerFilter
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerHostDvsMembershipFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter
    }
}

impl VimObjectTrait for DistributedVirtualSwitchManagerImportResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerImportResult
    }
}

impl VimObjectTrait for DvsManagerPhysicalNicsList {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsManagerPhysicalNicsList
    }
}

impl VimObjectTrait for EntityBackup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityBackup
    }
}

impl VimObjectTrait for EntityBackupConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityBackupConfig
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMember {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMember
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberBacking
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberPnicBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberPnicBacking
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberConfigInfo
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberConfigSpec
    }
}

impl VimObjectTrait for HostMemberHealthCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemberHealthCheckResult
    }
}

impl VimObjectTrait for HostMemberUplinkHealthCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemberUplinkHealthCheckResult
    }
}

impl VimObjectTrait for VMwareDvsMtuHealthCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsMtuHealthCheckResult
    }
}

impl VimObjectTrait for VMwareDvsVlanHealthCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsVlanHealthCheckResult
    }
}

impl VimObjectTrait for VMwareDvsTeamingHealthCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsTeamingHealthCheckResult
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberPnicSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberPnicSpec
    }
}

impl VimObjectTrait for HostMemberRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemberRuntimeInfo
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberRuntimeState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberRuntimeState
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberTransportZoneInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberTransportZoneInfo
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostProductSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostProductSpec
    }
}

impl VimObjectTrait for DistributedVirtualSwitchKeyedOpaqueBlob {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchKeyedOpaqueBlob
    }
}

impl VimObjectTrait for DistributedVirtualSwitchNetworkOffloadSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchNetworkOffloadSpec
    }
}

impl VimObjectTrait for DvsNetworkResourcePool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkResourcePool
    }
}

impl VimObjectTrait for DvsNetworkResourcePoolAllocationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkResourcePoolAllocationInfo
    }
}

impl VimObjectTrait for DvsNetworkResourcePoolConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkResourcePoolConfigSpec
    }
}

impl VimObjectTrait for DistributedVirtualSwitchPortConnectee {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchPortConnectee
    }
}

impl VimObjectTrait for DistributedVirtualSwitchPortConnection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchPortConnection
    }
}

impl VimObjectTrait for DistributedVirtualSwitchPortCriteria {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchPortCriteria
    }
}

impl VimObjectTrait for DistributedVirtualSwitchPortStatistics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchPortStatistics
    }
}

impl VimObjectTrait for DistributedVirtualSwitchProductSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchProductSpec
    }
}

impl VimObjectTrait for DvsTrafficRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsTrafficRule
    }
}

impl VimObjectTrait for DvsNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkRuleAction
    }
}

impl VimObjectTrait for DvsAcceptNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsAcceptNetworkRuleAction
    }
}

impl VimObjectTrait for DvsCopyNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsCopyNetworkRuleAction
    }
}

impl VimObjectTrait for DvsDropNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsDropNetworkRuleAction
    }
}

impl VimObjectTrait for DvsGreEncapNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsGreEncapNetworkRuleAction
    }
}

impl VimObjectTrait for DvsLogNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsLogNetworkRuleAction
    }
}

impl VimObjectTrait for DvsMacRewriteNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsMacRewriteNetworkRuleAction
    }
}

impl VimObjectTrait for DvsPuntNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPuntNetworkRuleAction
    }
}

impl VimObjectTrait for DvsRateLimitNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsRateLimitNetworkRuleAction
    }
}

impl VimObjectTrait for DvsUpdateTagNetworkRuleAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUpdateTagNetworkRuleAction
    }
}

impl VimObjectTrait for DvsNetworkRuleQualifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNetworkRuleQualifier
    }
}

impl VimObjectTrait for DvsIpNetworkRuleQualifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsIpNetworkRuleQualifier
    }
}

impl VimObjectTrait for DvsMacNetworkRuleQualifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsMacNetworkRuleQualifier
    }
}

impl VimObjectTrait for DvsSystemTrafficNetworkRuleQualifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsSystemTrafficNetworkRuleQualifier
    }
}

impl VimObjectTrait for DvsTrafficRuleset {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsTrafficRuleset
    }
}

impl VimObjectTrait for DvsVmVnicNetworkResourcePool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVmVnicNetworkResourcePool
    }
}

impl VimObjectTrait for DvsVmVnicResourcePoolConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVmVnicResourcePoolConfigSpec
    }
}

impl VimObjectTrait for DvsVmVnicResourceAllocation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVmVnicResourceAllocation
    }
}

impl VimObjectTrait for DvsVmVnicNetworkResourcePoolRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVmVnicNetworkResourcePoolRuntimeInfo
    }
}

impl VimObjectTrait for DvsVnicAllocatedResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVnicAllocatedResource
    }
}

impl VimObjectTrait for VMwareDvsDpuCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsDpuCapability
    }
}

impl VimObjectTrait for VMwareIpfixConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareIpfixConfig
    }
}

impl VimObjectTrait for VMwareDvsIpfixCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsIpfixCapability
    }
}

impl VimObjectTrait for VMwareDvsLacpCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsLacpCapability
    }
}

impl VimObjectTrait for VMwareDvsLacpGroupConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsLacpGroupConfig
    }
}

impl VimObjectTrait for VMwareDvsLacpGroupSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsLacpGroupSpec
    }
}

impl VimObjectTrait for VMwareDvsLagIpfixConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsLagIpfixConfig
    }
}

impl VimObjectTrait for VMwareDvsLagVlanConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsLagVlanConfig
    }
}

impl VimObjectTrait for VMwareDvsMtuCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsMtuCapability
    }
}

impl VimObjectTrait for VMwareDvsPvlanConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsPvlanConfigSpec
    }
}

impl VimObjectTrait for VMwareDvsPvlanMapEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsPvlanMapEntry
    }
}

impl VimObjectTrait for VMwareDvsVspanConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsVspanConfigSpec
    }
}

impl VimObjectTrait for VMwareDvsVspanCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsVspanCapability
    }
}

impl VimObjectTrait for VMwareVspanPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareVspanPort
    }
}

impl VimObjectTrait for VMwareVspanSession {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareVspanSession
    }
}

impl VimObjectTrait for CryptoKeyId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoKeyId
    }
}

impl VimObjectTrait for CryptoKeyPlain {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoKeyPlain
    }
}

impl VimObjectTrait for CryptoKeyResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoKeyResult
    }
}

impl VimObjectTrait for CryptoManagerHostKeyStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerHostKeyStatus
    }
}

impl VimObjectTrait for CryptoManagerKmipCertSignRequest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCertSignRequest
    }
}

impl VimObjectTrait for CryptoManagerKmipCertificateInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCertificateInfo
    }
}

impl VimObjectTrait for CryptoManagerKmipClusterStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipClusterStatus
    }
}

impl VimObjectTrait for CryptoManagerKmipCryptoKeyStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCryptoKeyStatus
    }
}

impl VimObjectTrait for CryptoManagerKmipCustomAttributeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCustomAttributeSpec
    }
}

impl VimObjectTrait for CryptoManagerKmipServerCertInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipServerCertInfo
    }
}

impl VimObjectTrait for CryptoManagerKmipServerStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipServerStatus
    }
}

impl VimObjectTrait for CryptoSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpec
    }
}

impl VimObjectTrait for CryptoSpecDecrypt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecDecrypt
    }
}

impl VimObjectTrait for CryptoSpecDeepRecrypt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecDeepRecrypt
    }
}

impl VimObjectTrait for CryptoSpecEncrypt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecEncrypt
    }
}

impl VimObjectTrait for CryptoSpecNoOp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecNoOp
    }
}

impl VimObjectTrait for CryptoSpecRegister {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecRegister
    }
}

impl VimObjectTrait for CryptoSpecShallowRecrypt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoSpecShallowRecrypt
    }
}

impl VimObjectTrait for KeyProviderId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KeyProviderId
    }
}

impl VimObjectTrait for KmipClusterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipClusterInfo
    }
}

impl VimObjectTrait for KmipServerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerInfo
    }
}

impl VimObjectTrait for KmipServerSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerSpec
    }
}

impl VimObjectTrait for KmipServerStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerStatus
    }
}

impl VimObjectTrait for ChangesInfoEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ChangesInfoEventArgument
    }
}

impl VimObjectTrait for DvsOutOfSyncHostArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsOutOfSyncHostArgument
    }
}

impl VimObjectTrait for Event {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Event
    }
}

impl VimObjectTrait for AlarmEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmEvent
    }
}

impl VimObjectTrait for AlarmAcknowledgedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmAcknowledgedEvent
    }
}

impl VimObjectTrait for AlarmActionTriggeredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmActionTriggeredEvent
    }
}

impl VimObjectTrait for AlarmClearedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmClearedEvent
    }
}

impl VimObjectTrait for AlarmCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmCreatedEvent
    }
}

impl VimObjectTrait for AlarmEmailCompletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmEmailCompletedEvent
    }
}

impl VimObjectTrait for AlarmEmailFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmEmailFailedEvent
    }
}

impl VimObjectTrait for AlarmReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmReconfiguredEvent
    }
}

impl VimObjectTrait for AlarmRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmRemovedEvent
    }
}

impl VimObjectTrait for AlarmScriptCompleteEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmScriptCompleteEvent
    }
}

impl VimObjectTrait for AlarmScriptFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmScriptFailedEvent
    }
}

impl VimObjectTrait for AlarmSnmpCompletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmSnmpCompletedEvent
    }
}

impl VimObjectTrait for AlarmSnmpFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmSnmpFailedEvent
    }
}

impl VimObjectTrait for AlarmStatusChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmStatusChangedEvent
    }
}

impl VimObjectTrait for AuthorizationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthorizationEvent
    }
}

impl VimObjectTrait for PermissionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PermissionEvent
    }
}

impl VimObjectTrait for PermissionAddedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PermissionAddedEvent
    }
}

impl VimObjectTrait for PermissionRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PermissionRemovedEvent
    }
}

impl VimObjectTrait for PermissionUpdatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PermissionUpdatedEvent
    }
}

impl VimObjectTrait for RoleEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RoleEvent
    }
}

impl VimObjectTrait for RoleAddedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RoleAddedEvent
    }
}

impl VimObjectTrait for RoleRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RoleRemovedEvent
    }
}

impl VimObjectTrait for RoleUpdatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RoleUpdatedEvent
    }
}

impl VimObjectTrait for ClusterEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterEvent
    }
}

impl VimObjectTrait for ClusterComplianceCheckedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComplianceCheckedEvent
    }
}

impl VimObjectTrait for ClusterCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterCreatedEvent
    }
}

impl VimObjectTrait for ClusterDestroyedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDestroyedEvent
    }
}

impl VimObjectTrait for ClusterOvercommittedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterOvercommittedEvent
    }
}

impl VimObjectTrait for HostOvercommittedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostOvercommittedEvent
    }
}

impl VimObjectTrait for ClusterReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterReconfiguredEvent
    }
}

impl VimObjectTrait for ClusterStatusChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterStatusChangedEvent
    }
}

impl VimObjectTrait for HostStatusChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStatusChangedEvent
    }
}

impl VimObjectTrait for DasAdmissionControlDisabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasAdmissionControlDisabledEvent
    }
}

impl VimObjectTrait for DasAdmissionControlEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasAdmissionControlEnabledEvent
    }
}

impl VimObjectTrait for DasAgentFoundEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasAgentFoundEvent
    }
}

impl VimObjectTrait for DasAgentUnavailableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasAgentUnavailableEvent
    }
}

impl VimObjectTrait for DasClusterIsolatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasClusterIsolatedEvent
    }
}

impl VimObjectTrait for DasDisabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasDisabledEvent
    }
}

impl VimObjectTrait for DasEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasEnabledEvent
    }
}

impl VimObjectTrait for DasHostFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasHostFailedEvent
    }
}

impl VimObjectTrait for DasHostIsolatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasHostIsolatedEvent
    }
}

impl VimObjectTrait for DrsDisabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsDisabledEvent
    }
}

impl VimObjectTrait for DrsEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsEnabledEvent
    }
}

impl VimObjectTrait for DrsInvocationFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsInvocationFailedEvent
    }
}

impl VimObjectTrait for DrsRecoveredFromFailureEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsRecoveredFromFailureEvent
    }
}

impl VimObjectTrait for FailoverLevelRestored {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailoverLevelRestored
    }
}

impl VimObjectTrait for HostMonitoringStateChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMonitoringStateChangedEvent
    }
}

impl VimObjectTrait for InsufficientFailoverResourcesEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientFailoverResourcesEvent
    }
}

impl VimObjectTrait for VmHealthMonitoringStateChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmHealthMonitoringStateChangedEvent
    }
}

impl VimObjectTrait for CustomFieldEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldEvent
    }
}

impl VimObjectTrait for CustomFieldDefEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldDefEvent
    }
}

impl VimObjectTrait for CustomFieldDefAddedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldDefAddedEvent
    }
}

impl VimObjectTrait for CustomFieldDefRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldDefRemovedEvent
    }
}

impl VimObjectTrait for CustomFieldDefRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldDefRenamedEvent
    }
}

impl VimObjectTrait for CustomFieldValueChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomFieldValueChangedEvent
    }
}

impl VimObjectTrait for DvPortgroupEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupEvent
    }
}

impl VimObjectTrait for DvPortgroupCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupCreatedEvent
    }
}

impl VimObjectTrait for DvPortgroupDestroyedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupDestroyedEvent
    }
}

impl VimObjectTrait for DvPortgroupReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupReconfiguredEvent
    }
}

impl VimObjectTrait for DvPortgroupRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupRenamedEvent
    }
}

impl VimObjectTrait for DvpgImportEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvpgImportEvent
    }
}

impl VimObjectTrait for DvpgRestoreEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvpgRestoreEvent
    }
}

impl VimObjectTrait for DatacenterEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterEvent
    }
}

impl VimObjectTrait for DatacenterCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterCreatedEvent
    }
}

impl VimObjectTrait for DatacenterRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterRenamedEvent
    }
}

impl VimObjectTrait for DatastoreEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreEvent
    }
}

impl VimObjectTrait for DatastoreCapacityIncreasedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreCapacityIncreasedEvent
    }
}

impl VimObjectTrait for DatastoreDestroyedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreDestroyedEvent
    }
}

impl VimObjectTrait for DatastoreDuplicatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreDuplicatedEvent
    }
}

impl VimObjectTrait for DatastoreFileEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreFileEvent
    }
}

impl VimObjectTrait for DatastoreFileCopiedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreFileCopiedEvent
    }
}

impl VimObjectTrait for DatastoreFileDeletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreFileDeletedEvent
    }
}

impl VimObjectTrait for DatastoreFileMovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreFileMovedEvent
    }
}

impl VimObjectTrait for DatastoreIormReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreIormReconfiguredEvent
    }
}

impl VimObjectTrait for DatastoreRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreRenamedEvent
    }
}

impl VimObjectTrait for NonViWorkloadDetectedOnDatastoreEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NonViWorkloadDetectedOnDatastoreEvent
    }
}

impl VimObjectTrait for DvsEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsEvent
    }
}

impl VimObjectTrait for DvsCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsCreatedEvent
    }
}

impl VimObjectTrait for DvsDestroyedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsDestroyedEvent
    }
}

impl VimObjectTrait for DvsHostBackInSyncEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostBackInSyncEvent
    }
}

impl VimObjectTrait for DvsHostJoinedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostJoinedEvent
    }
}

impl VimObjectTrait for DvsHostLeftEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostLeftEvent
    }
}

impl VimObjectTrait for DvsHostStatusUpdated {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostStatusUpdated
    }
}

impl VimObjectTrait for DvsHostWentOutOfSyncEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostWentOutOfSyncEvent
    }
}

impl VimObjectTrait for DvsImportEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsImportEvent
    }
}

impl VimObjectTrait for DvsMergedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsMergedEvent
    }
}

impl VimObjectTrait for DvsPortBlockedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortBlockedEvent
    }
}

impl VimObjectTrait for DvsPortConnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortConnectedEvent
    }
}

impl VimObjectTrait for DvsPortCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortCreatedEvent
    }
}

impl VimObjectTrait for DvsPortDeletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortDeletedEvent
    }
}

impl VimObjectTrait for DvsPortDisconnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortDisconnectedEvent
    }
}

impl VimObjectTrait for DvsPortEnteredPassthruEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortEnteredPassthruEvent
    }
}

impl VimObjectTrait for DvsPortExitedPassthruEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortExitedPassthruEvent
    }
}

impl VimObjectTrait for DvsPortJoinPortgroupEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortJoinPortgroupEvent
    }
}

impl VimObjectTrait for DvsPortLeavePortgroupEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortLeavePortgroupEvent
    }
}

impl VimObjectTrait for DvsPortLinkDownEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortLinkDownEvent
    }
}

impl VimObjectTrait for DvsPortLinkUpEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortLinkUpEvent
    }
}

impl VimObjectTrait for DvsPortReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortReconfiguredEvent
    }
}

impl VimObjectTrait for DvsPortRuntimeChangeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortRuntimeChangeEvent
    }
}

impl VimObjectTrait for DvsPortUnblockedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortUnblockedEvent
    }
}

impl VimObjectTrait for DvsPortVendorSpecificStateChangeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsPortVendorSpecificStateChangeEvent
    }
}

impl VimObjectTrait for DvsReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsReconfiguredEvent
    }
}

impl VimObjectTrait for DvsRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsRenamedEvent
    }
}

impl VimObjectTrait for DvsRestoreEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsRestoreEvent
    }
}

impl VimObjectTrait for DvsUpgradeAvailableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUpgradeAvailableEvent
    }
}

impl VimObjectTrait for DvsUpgradeInProgressEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUpgradeInProgressEvent
    }
}

impl VimObjectTrait for DvsUpgradeRejectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUpgradeRejectedEvent
    }
}

impl VimObjectTrait for DvsUpgradedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsUpgradedEvent
    }
}

impl VimObjectTrait for HostLocalPortCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLocalPortCreatedEvent
    }
}

impl VimObjectTrait for OutOfSyncDvsHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OutOfSyncDvsHost
    }
}

impl VimObjectTrait for RecoveryEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecoveryEvent
    }
}

impl VimObjectTrait for RollbackEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RollbackEvent
    }
}

impl VimObjectTrait for VmVnicPoolReservationViolationClearEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmVnicPoolReservationViolationClearEvent
    }
}

impl VimObjectTrait for VmVnicPoolReservationViolationRaiseEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmVnicPoolReservationViolationRaiseEvent
    }
}

impl VimObjectTrait for EventEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventEx
    }
}

impl VimObjectTrait for GeneralEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralEvent
    }
}

impl VimObjectTrait for ExtendedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtendedEvent
    }
}

impl VimObjectTrait for GeneralHostErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralHostErrorEvent
    }
}

impl VimObjectTrait for GeneralHostInfoEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralHostInfoEvent
    }
}

impl VimObjectTrait for GeneralHostWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralHostWarningEvent
    }
}

impl VimObjectTrait for GeneralUserEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralUserEvent
    }
}

impl VimObjectTrait for GeneralVmErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralVmErrorEvent
    }
}

impl VimObjectTrait for GeneralVmInfoEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralVmInfoEvent
    }
}

impl VimObjectTrait for GeneralVmWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GeneralVmWarningEvent
    }
}

impl VimObjectTrait for HealthStatusChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HealthStatusChangedEvent
    }
}

impl VimObjectTrait for HostEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostEvent
    }
}

impl VimObjectTrait for AccountCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AccountCreatedEvent
    }
}

impl VimObjectTrait for AccountRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AccountRemovedEvent
    }
}

impl VimObjectTrait for AccountUpdatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AccountUpdatedEvent
    }
}

impl VimObjectTrait for AdminPasswordNotChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AdminPasswordNotChangedEvent
    }
}

impl VimObjectTrait for CanceledHostOperationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CanceledHostOperationEvent
    }
}

impl VimObjectTrait for DatastoreDiscoveredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreDiscoveredEvent
    }
}

impl VimObjectTrait for DatastorePrincipalConfigured {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastorePrincipalConfigured
    }
}

impl VimObjectTrait for DatastoreRemovedOnHostEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreRemovedOnHostEvent
    }
}

impl VimObjectTrait for DatastoreRenamedOnHostEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreRenamedOnHostEvent
    }
}

impl VimObjectTrait for DrsResourceConfigureFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsResourceConfigureFailedEvent
    }
}

impl VimObjectTrait for DrsResourceConfigureSyncedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsResourceConfigureSyncedEvent
    }
}

impl VimObjectTrait for DuplicateIpDetectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DuplicateIpDetectedEvent
    }
}

impl VimObjectTrait for DvsHealthStatusChangeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHealthStatusChangeEvent
    }
}

impl VimObjectTrait for MtuMatchEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MtuMatchEvent
    }
}

impl VimObjectTrait for MtuMismatchEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MtuMismatchEvent
    }
}

impl VimObjectTrait for TeamingMatchEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TeamingMatchEvent
    }
}

impl VimObjectTrait for TeamingMisMatchEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TeamingMisMatchEvent
    }
}

impl VimObjectTrait for UplinkPortMtuNotSupportEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UplinkPortMtuNotSupportEvent
    }
}

impl VimObjectTrait for UplinkPortMtuSupportEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UplinkPortMtuSupportEvent
    }
}

impl VimObjectTrait for UplinkPortVlanTrunkedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UplinkPortVlanTrunkedEvent
    }
}

impl VimObjectTrait for UplinkPortVlanUntrunkedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UplinkPortVlanUntrunkedEvent
    }
}

impl VimObjectTrait for EnteredMaintenanceModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnteredMaintenanceModeEvent
    }
}

impl VimObjectTrait for EnteredStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnteredStandbyModeEvent
    }
}

impl VimObjectTrait for DrsEnteredStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsEnteredStandbyModeEvent
    }
}

impl VimObjectTrait for EnteringMaintenanceModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnteringMaintenanceModeEvent
    }
}

impl VimObjectTrait for EnteringStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EnteringStandbyModeEvent
    }
}

impl VimObjectTrait for DrsEnteringStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsEnteringStandbyModeEvent
    }
}

impl VimObjectTrait for ExitMaintenanceModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExitMaintenanceModeEvent
    }
}

impl VimObjectTrait for ExitStandbyModeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExitStandbyModeFailedEvent
    }
}

impl VimObjectTrait for DrsExitStandbyModeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsExitStandbyModeFailedEvent
    }
}

impl VimObjectTrait for ExitedStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExitedStandbyModeEvent
    }
}

impl VimObjectTrait for DrsExitedStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsExitedStandbyModeEvent
    }
}

impl VimObjectTrait for ExitingStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExitingStandbyModeEvent
    }
}

impl VimObjectTrait for DrsExitingStandbyModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsExitingStandbyModeEvent
    }
}

impl VimObjectTrait for GhostDvsProxySwitchDetectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GhostDvsProxySwitchDetectedEvent
    }
}

impl VimObjectTrait for GhostDvsProxySwitchRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GhostDvsProxySwitchRemovedEvent
    }
}

impl VimObjectTrait for HostAddFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAddFailedEvent
    }
}

impl VimObjectTrait for HostAddedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAddedEvent
    }
}

impl VimObjectTrait for HostAdminDisableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAdminDisableEvent
    }
}

impl VimObjectTrait for HostAdminEnableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAdminEnableEvent
    }
}

impl VimObjectTrait for HostCnxFailedAccountFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedAccountFailedEvent
    }
}

impl VimObjectTrait for HostCnxFailedAlreadyManagedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedAlreadyManagedEvent
    }
}

impl VimObjectTrait for HostCnxFailedBadCcagentEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedBadCcagentEvent
    }
}

impl VimObjectTrait for HostCnxFailedBadUsernameEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedBadUsernameEvent
    }
}

impl VimObjectTrait for HostCnxFailedBadVersionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedBadVersionEvent
    }
}

impl VimObjectTrait for HostCnxFailedCcagentUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedCcagentUpgradeEvent
    }
}

impl VimObjectTrait for HostCnxFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedEvent
    }
}

impl VimObjectTrait for HostCnxFailedNetworkErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedNetworkErrorEvent
    }
}

impl VimObjectTrait for HostCnxFailedNoAccessEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedNoAccessEvent
    }
}

impl VimObjectTrait for HostCnxFailedNoConnectionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedNoConnectionEvent
    }
}

impl VimObjectTrait for HostCnxFailedNoLicenseEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedNoLicenseEvent
    }
}

impl VimObjectTrait for HostCnxFailedNotFoundEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedNotFoundEvent
    }
}

impl VimObjectTrait for HostCnxFailedTimeoutEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCnxFailedTimeoutEvent
    }
}

impl VimObjectTrait for HostComplianceCheckedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostComplianceCheckedEvent
    }
}

impl VimObjectTrait for HostCompliantEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCompliantEvent
    }
}

impl VimObjectTrait for HostConfigAppliedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigAppliedEvent
    }
}

impl VimObjectTrait for HostConnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectedEvent
    }
}

impl VimObjectTrait for HostConnectionLostEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectionLostEvent
    }
}

impl VimObjectTrait for HostDasDisabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasDisabledEvent
    }
}

impl VimObjectTrait for HostDasDisablingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasDisablingEvent
    }
}

impl VimObjectTrait for HostDasEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasEnabledEvent
    }
}

impl VimObjectTrait for HostDasEnablingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasEnablingEvent
    }
}

impl VimObjectTrait for HostDasErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasErrorEvent
    }
}

impl VimObjectTrait for HostDasEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasEvent
    }
}

impl VimObjectTrait for HostExtraNetworksEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostExtraNetworksEvent
    }
}

impl VimObjectTrait for HostIsolationIpPingFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIsolationIpPingFailedEvent
    }
}

impl VimObjectTrait for HostMissingNetworksEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMissingNetworksEvent
    }
}

impl VimObjectTrait for HostNoAvailableNetworksEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNoAvailableNetworksEvent
    }
}

impl VimObjectTrait for HostNoHaEnabledPortGroupsEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNoHaEnabledPortGroupsEvent
    }
}

impl VimObjectTrait for HostNoRedundantManagementNetworkEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNoRedundantManagementNetworkEvent
    }
}

impl VimObjectTrait for HostNotInClusterEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNotInClusterEvent
    }
}

impl VimObjectTrait for HostPrimaryAgentNotShortNameEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPrimaryAgentNotShortNameEvent
    }
}

impl VimObjectTrait for HostShortNameInconsistentEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostShortNameInconsistentEvent
    }
}

impl VimObjectTrait for HostDasOkEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDasOkEvent
    }
}

impl VimObjectTrait for HostDisconnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDisconnectedEvent
    }
}

impl VimObjectTrait for HostEnableAdminFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostEnableAdminFailedEvent
    }
}

impl VimObjectTrait for HostGetShortNameFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostGetShortNameFailedEvent
    }
}

impl VimObjectTrait for HostInAuditModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInAuditModeEvent
    }
}

impl VimObjectTrait for HostIpChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpChangedEvent
    }
}

impl VimObjectTrait for HostIpInconsistentEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpInconsistentEvent
    }
}

impl VimObjectTrait for HostIpToShortNameFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpToShortNameFailedEvent
    }
}

impl VimObjectTrait for HostNonCompliantEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNonCompliantEvent
    }
}

impl VimObjectTrait for HostProfileAppliedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileAppliedEvent
    }
}

impl VimObjectTrait for HostReconnectionFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostReconnectionFailedEvent
    }
}

impl VimObjectTrait for HostRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRemovedEvent
    }
}

impl VimObjectTrait for HostShortNameToIpFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostShortNameToIpFailedEvent
    }
}

impl VimObjectTrait for HostShutdownEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostShutdownEvent
    }
}

impl VimObjectTrait for HostSpecificationChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpecificationChangedEvent
    }
}

impl VimObjectTrait for HostSpecificationRequireEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpecificationRequireEvent
    }
}

impl VimObjectTrait for HostSpecificationUpdateEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpecificationUpdateEvent
    }
}

impl VimObjectTrait for HostSubSpecificationDeleteEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSubSpecificationDeleteEvent
    }
}

impl VimObjectTrait for HostSubSpecificationUpdateEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSubSpecificationUpdateEvent
    }
}

impl VimObjectTrait for HostSyncFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSyncFailedEvent
    }
}

impl VimObjectTrait for HostUpgradeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUpgradeFailedEvent
    }
}

impl VimObjectTrait for HostUserWorldSwapNotEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUserWorldSwapNotEnabledEvent
    }
}

impl VimObjectTrait for HostVnicConnectedToCustomizedDvPortEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVnicConnectedToCustomizedDvPortEvent
    }
}

impl VimObjectTrait for HostWwnChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostWwnChangedEvent
    }
}

impl VimObjectTrait for HostWwnConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostWwnConflictEvent
    }
}

impl VimObjectTrait for LocalDatastoreCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalDatastoreCreatedEvent
    }
}

impl VimObjectTrait for LocalTsmEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalTsmEnabledEvent
    }
}

impl VimObjectTrait for NasDatastoreCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasDatastoreCreatedEvent
    }
}

impl VimObjectTrait for NoDatastoresConfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDatastoresConfiguredEvent
    }
}

impl VimObjectTrait for RemoteTsmEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoteTsmEnabledEvent
    }
}

impl VimObjectTrait for TimedOutHostOperationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TimedOutHostOperationEvent
    }
}

impl VimObjectTrait for UpdatedAgentBeingRestartedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UpdatedAgentBeingRestartedEvent
    }
}

impl VimObjectTrait for UserAssignedToGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserAssignedToGroup
    }
}

impl VimObjectTrait for UserPasswordChanged {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserPasswordChanged
    }
}

impl VimObjectTrait for UserUnassignedFromGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserUnassignedFromGroup
    }
}

impl VimObjectTrait for VmfsDatastoreCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreCreatedEvent
    }
}

impl VimObjectTrait for VmfsDatastoreExpandedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreExpandedEvent
    }
}

impl VimObjectTrait for VmfsDatastoreExtendedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreExtendedEvent
    }
}

impl VimObjectTrait for VcAgentUninstallFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcAgentUninstallFailedEvent
    }
}

impl VimObjectTrait for VcAgentUninstalledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcAgentUninstalledEvent
    }
}

impl VimObjectTrait for VcAgentUpgradeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcAgentUpgradeFailedEvent
    }
}

impl VimObjectTrait for VcAgentUpgradedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcAgentUpgradedEvent
    }
}

impl VimObjectTrait for VimAccountPasswordChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimAccountPasswordChangedEvent
    }
}

impl VimObjectTrait for IScsiBootFailureEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IScsiBootFailureEvent
    }
}

impl VimObjectTrait for HostInventoryUnreadableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInventoryUnreadableEvent
    }
}

impl VimObjectTrait for LicenseEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseEvent
    }
}

impl VimObjectTrait for AllVirtualMachinesLicensedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AllVirtualMachinesLicensedEvent
    }
}

impl VimObjectTrait for HostInventoryFullEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInventoryFullEvent
    }
}

impl VimObjectTrait for HostLicenseExpiredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLicenseExpiredEvent
    }
}

impl VimObjectTrait for IncorrectHostInformationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncorrectHostInformationEvent
    }
}

impl VimObjectTrait for InvalidEditionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidEditionEvent
    }
}

impl VimObjectTrait for LicenseNonComplianceEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseNonComplianceEvent
    }
}

impl VimObjectTrait for LicenseRestrictedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseRestrictedEvent
    }
}

impl VimObjectTrait for LicenseServerAvailableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseServerAvailableEvent
    }
}

impl VimObjectTrait for LicenseServerUnavailableEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseServerUnavailableEvent
    }
}

impl VimObjectTrait for NoLicenseEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoLicenseEvent
    }
}

impl VimObjectTrait for ServerLicenseExpiredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServerLicenseExpiredEvent
    }
}

impl VimObjectTrait for UnlicensedVirtualMachinesEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnlicensedVirtualMachinesEvent
    }
}

impl VimObjectTrait for UnlicensedVirtualMachinesFoundEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnlicensedVirtualMachinesFoundEvent
    }
}

impl VimObjectTrait for VMotionLicenseExpiredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionLicenseExpiredEvent
    }
}

impl VimObjectTrait for LicenseExpiredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseExpiredEvent
    }
}

impl VimObjectTrait for LockerMisconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LockerMisconfiguredEvent
    }
}

impl VimObjectTrait for LockerReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LockerReconfiguredEvent
    }
}

impl VimObjectTrait for NetworkRollbackEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkRollbackEvent
    }
}

impl VimObjectTrait for ProfileEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileEvent
    }
}

impl VimObjectTrait for ProfileAssociatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileAssociatedEvent
    }
}

impl VimObjectTrait for ProfileChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileChangedEvent
    }
}

impl VimObjectTrait for ProfileCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileCreatedEvent
    }
}

impl VimObjectTrait for ProfileDissociatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileDissociatedEvent
    }
}

impl VimObjectTrait for ProfileReferenceHostChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileReferenceHostChangedEvent
    }
}

impl VimObjectTrait for ProfileRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileRemovedEvent
    }
}

impl VimObjectTrait for ResourcePoolEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolEvent
    }
}

impl VimObjectTrait for ResourcePoolCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolCreatedEvent
    }
}

impl VimObjectTrait for ResourcePoolDestroyedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolDestroyedEvent
    }
}

impl VimObjectTrait for ResourcePoolMovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolMovedEvent
    }
}

impl VimObjectTrait for ResourcePoolReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolReconfiguredEvent
    }
}

impl VimObjectTrait for ResourceViolatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceViolatedEvent
    }
}

impl VimObjectTrait for ScheduledTaskEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskEvent
    }
}

impl VimObjectTrait for ScheduledTaskCompletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskCompletedEvent
    }
}

impl VimObjectTrait for ScheduledTaskCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskCreatedEvent
    }
}

impl VimObjectTrait for ScheduledTaskEmailCompletedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskEmailCompletedEvent
    }
}

impl VimObjectTrait for ScheduledTaskEmailFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskEmailFailedEvent
    }
}

impl VimObjectTrait for ScheduledTaskFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskFailedEvent
    }
}

impl VimObjectTrait for ScheduledTaskReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskReconfiguredEvent
    }
}

impl VimObjectTrait for ScheduledTaskRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskRemovedEvent
    }
}

impl VimObjectTrait for ScheduledTaskStartedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskStartedEvent
    }
}

impl VimObjectTrait for SessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionEvent
    }
}

impl VimObjectTrait for AlreadyAuthenticatedSessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlreadyAuthenticatedSessionEvent
    }
}

impl VimObjectTrait for BadUsernameSessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BadUsernameSessionEvent
    }
}

impl VimObjectTrait for GlobalMessageChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GlobalMessageChangedEvent
    }
}

impl VimObjectTrait for NoAccessUserEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoAccessUserEvent
    }
}

impl VimObjectTrait for ServerStartedSessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServerStartedSessionEvent
    }
}

impl VimObjectTrait for SessionTerminatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SessionTerminatedEvent
    }
}

impl VimObjectTrait for UserLoginSessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserLoginSessionEvent
    }
}

impl VimObjectTrait for UserLogoutSessionEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserLogoutSessionEvent
    }
}

impl VimObjectTrait for TaskEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskEvent
    }
}

impl VimObjectTrait for TaskTimeoutEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskTimeoutEvent
    }
}

impl VimObjectTrait for TemplateUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateUpgradeEvent
    }
}

impl VimObjectTrait for TemplateBeingUpgradedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateBeingUpgradedEvent
    }
}

impl VimObjectTrait for TemplateUpgradeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateUpgradeFailedEvent
    }
}

impl VimObjectTrait for TemplateUpgradedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateUpgradedEvent
    }
}

impl VimObjectTrait for UpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UpgradeEvent
    }
}

impl VimObjectTrait for ErrorUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ErrorUpgradeEvent
    }
}

impl VimObjectTrait for InfoUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InfoUpgradeEvent
    }
}

impl VimObjectTrait for UserUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserUpgradeEvent
    }
}

impl VimObjectTrait for WarningUpgradeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WarningUpgradeEvent
    }
}

impl VimObjectTrait for VmEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmEvent
    }
}

impl VimObjectTrait for CustomizationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationEvent
    }
}

impl VimObjectTrait for CustomizationFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationFailed
    }
}

impl VimObjectTrait for CustomizationLinuxIdentityFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationLinuxIdentityFailed
    }
}

impl VimObjectTrait for CustomizationNetworkSetupFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationNetworkSetupFailed
    }
}

impl VimObjectTrait for CustomizationSysprepFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSysprepFailed
    }
}

impl VimObjectTrait for CustomizationUnknownFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationUnknownFailure
    }
}

impl VimObjectTrait for CustomizationStartedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationStartedEvent
    }
}

impl VimObjectTrait for CustomizationSucceeded {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSucceeded
    }
}

impl VimObjectTrait for DrsRuleComplianceEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsRuleComplianceEvent
    }
}

impl VimObjectTrait for DrsRuleViolationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsRuleViolationEvent
    }
}

impl VimObjectTrait for DrsSoftRuleViolationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsSoftRuleViolationEvent
    }
}

impl VimObjectTrait for MigrationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationEvent
    }
}

impl VimObjectTrait for MigrationErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationErrorEvent
    }
}

impl VimObjectTrait for MigrationHostErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationHostErrorEvent
    }
}

impl VimObjectTrait for MigrationHostWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationHostWarningEvent
    }
}

impl VimObjectTrait for MigrationResourceErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationResourceErrorEvent
    }
}

impl VimObjectTrait for MigrationResourceWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationResourceWarningEvent
    }
}

impl VimObjectTrait for MigrationWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationWarningEvent
    }
}

impl VimObjectTrait for NoMaintenanceModeDrsRecommendationForVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoMaintenanceModeDrsRecommendationForVm
    }
}

impl VimObjectTrait for NotEnoughResourcesToStartVmEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotEnoughResourcesToStartVmEvent
    }
}

impl VimObjectTrait for VmAcquiredMksTicketEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmAcquiredMksTicketEvent
    }
}

impl VimObjectTrait for VmAcquiredTicketEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmAcquiredTicketEvent
    }
}

impl VimObjectTrait for VmAutoRenameEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmAutoRenameEvent
    }
}

impl VimObjectTrait for VmBeingCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingCreatedEvent
    }
}

impl VimObjectTrait for VmBeingDeployedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingDeployedEvent
    }
}

impl VimObjectTrait for VmBeingHotMigratedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingHotMigratedEvent
    }
}

impl VimObjectTrait for VmBeingMigratedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingMigratedEvent
    }
}

impl VimObjectTrait for VmCloneEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmCloneEvent
    }
}

impl VimObjectTrait for VmBeingClonedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingClonedEvent
    }
}

impl VimObjectTrait for VmBeingClonedNoFolderEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingClonedNoFolderEvent
    }
}

impl VimObjectTrait for VmCloneFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmCloneFailedEvent
    }
}

impl VimObjectTrait for VmClonedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmClonedEvent
    }
}

impl VimObjectTrait for VmConfigMissingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigMissingEvent
    }
}

impl VimObjectTrait for VmConnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConnectedEvent
    }
}

impl VimObjectTrait for VmCreatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmCreatedEvent
    }
}

impl VimObjectTrait for VmDasBeingResetEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDasBeingResetEvent
    }
}

impl VimObjectTrait for VmDasBeingResetWithScreenshotEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDasBeingResetWithScreenshotEvent
    }
}

impl VimObjectTrait for VmDasResetFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDasResetFailedEvent
    }
}

impl VimObjectTrait for VmDasUpdateErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDasUpdateErrorEvent
    }
}

impl VimObjectTrait for VmDasUpdateOkEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDasUpdateOkEvent
    }
}

impl VimObjectTrait for VmDateRolledBackEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDateRolledBackEvent
    }
}

impl VimObjectTrait for VmDeployFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDeployFailedEvent
    }
}

impl VimObjectTrait for VmDeployedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDeployedEvent
    }
}

impl VimObjectTrait for VmDisconnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDisconnectedEvent
    }
}

impl VimObjectTrait for VmDiscoveredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiscoveredEvent
    }
}

impl VimObjectTrait for VmDiskFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFailedEvent
    }
}

impl VimObjectTrait for VmEmigratingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmEmigratingEvent
    }
}

impl VimObjectTrait for VmEndRecordingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmEndRecordingEvent
    }
}

impl VimObjectTrait for VmEndReplayingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmEndReplayingEvent
    }
}

impl VimObjectTrait for VmFailedMigrateEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedMigrateEvent
    }
}

impl VimObjectTrait for VmFailedRelayoutEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedRelayoutEvent
    }
}

impl VimObjectTrait for VmFailedRelayoutOnVmfs2DatastoreEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedRelayoutOnVmfs2DatastoreEvent
    }
}

impl VimObjectTrait for VmFailedStartingSecondaryEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedStartingSecondaryEvent
    }
}

impl VimObjectTrait for VmFailedToPowerOffEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToPowerOffEvent
    }
}

impl VimObjectTrait for VmFailedToPowerOnEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToPowerOnEvent
    }
}

impl VimObjectTrait for VmFailedToRebootGuestEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToRebootGuestEvent
    }
}

impl VimObjectTrait for VmFailedToResetEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToResetEvent
    }
}

impl VimObjectTrait for VmFailedToShutdownGuestEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToShutdownGuestEvent
    }
}

impl VimObjectTrait for VmFailedToStandbyGuestEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToStandbyGuestEvent
    }
}

impl VimObjectTrait for VmFailedToSuspendEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedToSuspendEvent
    }
}

impl VimObjectTrait for VmFailedUpdatingSecondaryConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailedUpdatingSecondaryConfig
    }
}

impl VimObjectTrait for VmFailoverFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFailoverFailed
    }
}

impl VimObjectTrait for VmFaultToleranceStateChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceStateChangedEvent
    }
}

impl VimObjectTrait for VmFaultToleranceTurnedOffEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceTurnedOffEvent
    }
}

impl VimObjectTrait for VmFaultToleranceVmTerminatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceVmTerminatedEvent
    }
}

impl VimObjectTrait for VmGuestOsCrashedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmGuestOsCrashedEvent
    }
}

impl VimObjectTrait for VmGuestRebootEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmGuestRebootEvent
    }
}

impl VimObjectTrait for VmGuestShutdownEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmGuestShutdownEvent
    }
}

impl VimObjectTrait for VmGuestStandbyEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmGuestStandbyEvent
    }
}

impl VimObjectTrait for VmInstanceUuidAssignedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmInstanceUuidAssignedEvent
    }
}

impl VimObjectTrait for VmInstanceUuidChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmInstanceUuidChangedEvent
    }
}

impl VimObjectTrait for VmInstanceUuidConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmInstanceUuidConflictEvent
    }
}

impl VimObjectTrait for VmMacAssignedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMacAssignedEvent
    }
}

impl VimObjectTrait for VmMacChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMacChangedEvent
    }
}

impl VimObjectTrait for VmMacConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMacConflictEvent
    }
}

impl VimObjectTrait for VmMaxFtRestartCountReached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMaxFtRestartCountReached
    }
}

impl VimObjectTrait for VmMaxRestartCountReached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMaxRestartCountReached
    }
}

impl VimObjectTrait for VmMessageErrorEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMessageErrorEvent
    }
}

impl VimObjectTrait for VmMessageEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMessageEvent
    }
}

impl VimObjectTrait for VmMessageWarningEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMessageWarningEvent
    }
}

impl VimObjectTrait for VmMigratedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMigratedEvent
    }
}

impl VimObjectTrait for DrsVmMigratedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsVmMigratedEvent
    }
}

impl VimObjectTrait for VmNoCompatibleHostForSecondaryEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmNoCompatibleHostForSecondaryEvent
    }
}

impl VimObjectTrait for VmNoNetworkAccessEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmNoNetworkAccessEvent
    }
}

impl VimObjectTrait for VmOrphanedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmOrphanedEvent
    }
}

impl VimObjectTrait for VmPoweredOffEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPoweredOffEvent
    }
}

impl VimObjectTrait for VmPowerOffOnIsolationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPowerOffOnIsolationEvent
    }
}

impl VimObjectTrait for VmShutdownOnIsolationEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmShutdownOnIsolationEvent
    }
}

impl VimObjectTrait for VmPoweredOnEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPoweredOnEvent
    }
}

impl VimObjectTrait for DrsVmPoweredOnEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsVmPoweredOnEvent
    }
}

impl VimObjectTrait for VmRestartedOnAlternateHostEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRestartedOnAlternateHostEvent
    }
}

impl VimObjectTrait for VmPoweringOnWithCustomizedDvPortEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPoweringOnWithCustomizedDvPortEvent
    }
}

impl VimObjectTrait for VmPrimaryFailoverEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPrimaryFailoverEvent
    }
}

impl VimObjectTrait for VmReconfiguredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmReconfiguredEvent
    }
}

impl VimObjectTrait for VmRegisteredEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRegisteredEvent
    }
}

impl VimObjectTrait for VmRelayoutSuccessfulEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRelayoutSuccessfulEvent
    }
}

impl VimObjectTrait for VmRelayoutUpToDateEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRelayoutUpToDateEvent
    }
}

impl VimObjectTrait for VmReloadFromPathEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmReloadFromPathEvent
    }
}

impl VimObjectTrait for VmReloadFromPathFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmReloadFromPathFailedEvent
    }
}

impl VimObjectTrait for VmRelocateSpecEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRelocateSpecEvent
    }
}

impl VimObjectTrait for VmBeingRelocatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmBeingRelocatedEvent
    }
}

impl VimObjectTrait for VmRelocateFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRelocateFailedEvent
    }
}

impl VimObjectTrait for VmRelocatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRelocatedEvent
    }
}

impl VimObjectTrait for VmRemoteConsoleConnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRemoteConsoleConnectedEvent
    }
}

impl VimObjectTrait for VmRemoteConsoleDisconnectedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRemoteConsoleDisconnectedEvent
    }
}

impl VimObjectTrait for VmRemovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRemovedEvent
    }
}

impl VimObjectTrait for VmRenamedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRenamedEvent
    }
}

impl VimObjectTrait for VmRequirementsExceedCurrentEvcModeEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRequirementsExceedCurrentEvcModeEvent
    }
}

impl VimObjectTrait for VmResettingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmResettingEvent
    }
}

impl VimObjectTrait for VmResourcePoolMovedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmResourcePoolMovedEvent
    }
}

impl VimObjectTrait for VmResourceReallocatedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmResourceReallocatedEvent
    }
}

impl VimObjectTrait for VmResumingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmResumingEvent
    }
}

impl VimObjectTrait for VmSecondaryAddedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSecondaryAddedEvent
    }
}

impl VimObjectTrait for VmSecondaryDisabledBySystemEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSecondaryDisabledBySystemEvent
    }
}

impl VimObjectTrait for VmSecondaryDisabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSecondaryDisabledEvent
    }
}

impl VimObjectTrait for VmSecondaryEnabledEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSecondaryEnabledEvent
    }
}

impl VimObjectTrait for VmSecondaryStartedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSecondaryStartedEvent
    }
}

impl VimObjectTrait for VmStartRecordingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStartRecordingEvent
    }
}

impl VimObjectTrait for VmStartReplayingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStartReplayingEvent
    }
}

impl VimObjectTrait for VmStartingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStartingEvent
    }
}

impl VimObjectTrait for VmUnsupportedStartingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUnsupportedStartingEvent
    }
}

impl VimObjectTrait for VmStartingSecondaryEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStartingSecondaryEvent
    }
}

impl VimObjectTrait for VmStaticMacConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStaticMacConflictEvent
    }
}

impl VimObjectTrait for VmStoppingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmStoppingEvent
    }
}

impl VimObjectTrait for VmSuspendedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSuspendedEvent
    }
}

impl VimObjectTrait for VmSuspendingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSuspendingEvent
    }
}

impl VimObjectTrait for VmTimedoutStartingSecondaryEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmTimedoutStartingSecondaryEvent
    }
}

impl VimObjectTrait for VmUpgradeCompleteEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUpgradeCompleteEvent
    }
}

impl VimObjectTrait for VmUpgradeFailedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUpgradeFailedEvent
    }
}

impl VimObjectTrait for VmUpgradingEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUpgradingEvent
    }
}

impl VimObjectTrait for VmUuidAssignedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUuidAssignedEvent
    }
}

impl VimObjectTrait for VmUuidChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUuidChangedEvent
    }
}

impl VimObjectTrait for VmUuidConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmUuidConflictEvent
    }
}

impl VimObjectTrait for VmWwnAssignedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWwnAssignedEvent
    }
}

impl VimObjectTrait for VmWwnChangedEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWwnChangedEvent
    }
}

impl VimObjectTrait for VmWwnConflictEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWwnConflictEvent
    }
}

impl VimObjectTrait for EventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventArgument
    }
}

impl VimObjectTrait for EntityEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityEventArgument
    }
}

impl VimObjectTrait for AlarmEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmEventArgument
    }
}

impl VimObjectTrait for ComputeResourceEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceEventArgument
    }
}

impl VimObjectTrait for DatacenterEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterEventArgument
    }
}

impl VimObjectTrait for DatastoreEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreEventArgument
    }
}

impl VimObjectTrait for DvsEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsEventArgument
    }
}

impl VimObjectTrait for FolderEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderEventArgument
    }
}

impl VimObjectTrait for HostEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostEventArgument
    }
}

impl VimObjectTrait for ManagedEntityEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ManagedEntityEventArgument
    }
}

impl VimObjectTrait for NetworkEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkEventArgument
    }
}

impl VimObjectTrait for ResourcePoolEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourcePoolEventArgument
    }
}

impl VimObjectTrait for ScheduledTaskEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskEventArgument
    }
}

impl VimObjectTrait for VmEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmEventArgument
    }
}

impl VimObjectTrait for ProfileEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileEventArgument
    }
}

impl VimObjectTrait for RoleEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RoleEventArgument
    }
}

impl VimObjectTrait for EventDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventDescription
    }
}

impl VimObjectTrait for EventArgDesc {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventArgDesc
    }
}

impl VimObjectTrait for EventDescriptionEventDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventDescriptionEventDetail
    }
}

impl VimObjectTrait for EventFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventFilterSpec
    }
}

impl VimObjectTrait for EventFilterSpecByEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventFilterSpecByEntity
    }
}

impl VimObjectTrait for EventFilterSpecByTime {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventFilterSpecByTime
    }
}

impl VimObjectTrait for EventFilterSpecByUsername {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventFilterSpecByUsername
    }
}

impl VimObjectTrait for ExtendedEventPair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtendedEventPair
    }
}

impl VimObjectTrait for VnicPortArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VnicPortArgument
    }
}

impl VimObjectTrait for ExtExtendedProductInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtExtendedProductInfo
    }
}

impl VimObjectTrait for ManagedByInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ManagedByInfo
    }
}

impl VimObjectTrait for ExtManagedEntityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtManagedEntityInfo
    }
}

impl VimObjectTrait for ExtSolutionManagerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtSolutionManagerInfo
    }
}

impl VimObjectTrait for ExtSolutionManagerInfoTabInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtSolutionManagerInfoTabInfo
    }
}

impl VimObjectTrait for AnswerFileUpdateFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileUpdateFailure
    }
}

impl VimObjectTrait for ConflictingConfigurationConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConflictingConfigurationConfig
    }
}

impl VimObjectTrait for DatacenterMismatchArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterMismatchArgument
    }
}

impl VimObjectTrait for DvsApplyOperationFaultFaultOnObject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsApplyOperationFaultFaultOnObject
    }
}

impl VimObjectTrait for DvsOperationBulkFaultFaultOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsOperationBulkFaultFaultOnHost
    }
}

impl VimObjectTrait for ImportOperationBulkFaultFaultOnImport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ImportOperationBulkFaultFaultOnImport
    }
}

impl VimObjectTrait for MultipleCertificatesVerifyFaultThumbprintData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MultipleCertificatesVerifyFaultThumbprintData
    }
}

impl VimObjectTrait for NoPermissionEntityPrivileges {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPermissionEntityPrivileges
    }
}

impl VimObjectTrait for ProfileUpdateFailedUpdateFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileUpdateFailedUpdateFailure
    }
}

impl VimObjectTrait for HostActiveDirectory {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostActiveDirectory
    }
}

impl VimObjectTrait for HostActiveDirectorySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostActiveDirectorySpec
    }
}

impl VimObjectTrait for HostAssignableHardwareBinding {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAssignableHardwareBinding
    }
}

impl VimObjectTrait for HostAssignableHardwareConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAssignableHardwareConfig
    }
}

impl VimObjectTrait for HostAssignableHardwareConfigAttributeOverride {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAssignableHardwareConfigAttributeOverride
    }
}

impl VimObjectTrait for HostAuthenticationManagerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAuthenticationManagerInfo
    }
}

impl VimObjectTrait for HostAuthenticationStoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAuthenticationStoreInfo
    }
}

impl VimObjectTrait for HostDirectoryStoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDirectoryStoreInfo
    }
}

impl VimObjectTrait for HostActiveDirectoryInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostActiveDirectoryInfo
    }
}

impl VimObjectTrait for HostLocalAuthenticationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLocalAuthenticationInfo
    }
}

impl VimObjectTrait for AutoStartPowerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AutoStartPowerInfo
    }
}

impl VimObjectTrait for HostAutoStartManagerConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAutoStartManagerConfig
    }
}

impl VimObjectTrait for AutoStartDefaults {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AutoStartDefaults
    }
}

impl VimObjectTrait for HostBiosInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostBiosInfo
    }
}

impl VimObjectTrait for HostBootDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostBootDeviceInfo
    }
}

impl VimObjectTrait for HostBootDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostBootDevice
    }
}

impl VimObjectTrait for HostCacheConfigurationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCacheConfigurationInfo
    }
}

impl VimObjectTrait for HostCacheConfigurationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCacheConfigurationSpec
    }
}

impl VimObjectTrait for HostCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCapability
    }
}

impl VimObjectTrait for HostCertificateManagerCertificateInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCertificateManagerCertificateInfo
    }
}

impl VimObjectTrait for HostCertificateManagerCertificateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCertificateManagerCertificateSpec
    }
}

impl VimObjectTrait for HostConfigChange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigChange
    }
}

impl VimObjectTrait for HostConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigInfo
    }
}

impl VimObjectTrait for HostConfigManager {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigManager
    }
}

impl VimObjectTrait for HostConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigSpec
    }
}

impl VimObjectTrait for HostConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectInfo
    }
}

impl VimObjectTrait for HostDatastoreConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreConnectInfo
    }
}

impl VimObjectTrait for HostDatastoreExistsConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreExistsConnectInfo
    }
}

impl VimObjectTrait for HostDatastoreNameConflictConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreNameConflictConnectInfo
    }
}

impl VimObjectTrait for HostLicenseConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLicenseConnectInfo
    }
}

impl VimObjectTrait for HostConnectInfoNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectInfoNetworkInfo
    }
}

impl VimObjectTrait for HostNewNetworkConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNewNetworkConnectInfo
    }
}

impl VimObjectTrait for HostConnectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectSpec
    }
}

impl VimObjectTrait for HostCpuIdInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCpuIdInfo
    }
}

impl VimObjectTrait for HostCpuInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCpuInfo
    }
}

impl VimObjectTrait for HostCpuPackage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCpuPackage
    }
}

impl VimObjectTrait for HostCpuPowerManagementInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCpuPowerManagementInfo
    }
}

impl VimObjectTrait for HostHyperThreadScheduleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHyperThreadScheduleInfo
    }
}

impl VimObjectTrait for HostDataTransportConnectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDataTransportConnectionInfo
    }
}

impl VimObjectTrait for HostNfcConnectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNfcConnectionInfo
    }
}

impl VimObjectTrait for FileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileInfo
    }
}

impl VimObjectTrait for FloppyImageFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FloppyImageFileInfo
    }
}

impl VimObjectTrait for FolderFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderFileInfo
    }
}

impl VimObjectTrait for IsoImageFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IsoImageFileInfo
    }
}

impl VimObjectTrait for VmConfigFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFileInfo
    }
}

impl VimObjectTrait for TemplateConfigFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateConfigFileInfo
    }
}

impl VimObjectTrait for VmDiskFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFileInfo
    }
}

impl VimObjectTrait for VmLogFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmLogFileInfo
    }
}

impl VimObjectTrait for VmNvramFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmNvramFileInfo
    }
}

impl VimObjectTrait for VmSnapshotFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSnapshotFileInfo
    }
}

impl VimObjectTrait for FileQueryFlags {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileQueryFlags
    }
}

impl VimObjectTrait for FileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileQuery
    }
}

impl VimObjectTrait for FloppyImageFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FloppyImageFileQuery
    }
}

impl VimObjectTrait for FolderFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderFileQuery
    }
}

impl VimObjectTrait for IsoImageFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IsoImageFileQuery
    }
}

impl VimObjectTrait for VmConfigFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFileQuery
    }
}

impl VimObjectTrait for TemplateConfigFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TemplateConfigFileQuery
    }
}

impl VimObjectTrait for VmDiskFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFileQuery
    }
}

impl VimObjectTrait for VmLogFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmLogFileQuery
    }
}

impl VimObjectTrait for VmNvramFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmNvramFileQuery
    }
}

impl VimObjectTrait for VmSnapshotFileQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSnapshotFileQuery
    }
}

impl VimObjectTrait for HostDatastoreBrowserSearchResults {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreBrowserSearchResults
    }
}

impl VimObjectTrait for HostDatastoreBrowserSearchSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreBrowserSearchSpec
    }
}

impl VimObjectTrait for VmConfigFileEncryptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFileEncryptionInfo
    }
}

impl VimObjectTrait for VmConfigFileQueryFlags {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFileQueryFlags
    }
}

impl VimObjectTrait for VmConfigFileQueryFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFileQueryFilter
    }
}

impl VimObjectTrait for VmDiskFileEncryptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFileEncryptionInfo
    }
}

impl VimObjectTrait for VmDiskFileQueryFlags {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFileQueryFlags
    }
}

impl VimObjectTrait for VmDiskFileQueryFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDiskFileQueryFilter
    }
}

impl VimObjectTrait for HostDatastoreSystemCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreSystemCapabilities
    }
}

impl VimObjectTrait for HostDatastoreSystemDatastoreResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreSystemDatastoreResult
    }
}

impl VimObjectTrait for HostDatastoreSystemVvolDatastoreSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDatastoreSystemVvolDatastoreSpec
    }
}

impl VimObjectTrait for HostDateTimeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDateTimeConfig
    }
}

impl VimObjectTrait for HostDateTimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDateTimeInfo
    }
}

impl VimObjectTrait for HostDateTimeSystemServiceTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDateTimeSystemServiceTestResult
    }
}

impl VimObjectTrait for HostDateTimeSystemTimeZone {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDateTimeSystemTimeZone
    }
}

impl VimObjectTrait for HostDeploymentInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDeploymentInfo
    }
}

impl VimObjectTrait for HostDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDevice
    }
}

impl VimObjectTrait for ScsiLun {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScsiLun
    }
}

impl VimObjectTrait for HostScsiDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiDisk
    }
}

impl VimObjectTrait for HostDhcpService {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDhcpService
    }
}

impl VimObjectTrait for HostDhcpServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDhcpServiceConfig
    }
}

impl VimObjectTrait for HostDhcpServiceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDhcpServiceSpec
    }
}

impl VimObjectTrait for HostDiagnosticPartition {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiagnosticPartition
    }
}

impl VimObjectTrait for HostDiagnosticPartitionCreateDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiagnosticPartitionCreateDescription
    }
}

impl VimObjectTrait for HostDiagnosticPartitionCreateOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiagnosticPartitionCreateOption
    }
}

impl VimObjectTrait for HostDiagnosticPartitionCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiagnosticPartitionCreateSpec
    }
}

impl VimObjectTrait for HostDigestInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDigestInfo
    }
}

impl VimObjectTrait for HostTpmDigestInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmDigestInfo
    }
}

impl VimObjectTrait for HostDiskConfigurationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskConfigurationResult
    }
}

impl VimObjectTrait for HostDiskDimensions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskDimensions
    }
}

impl VimObjectTrait for HostDiskDimensionsChs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskDimensionsChs
    }
}

impl VimObjectTrait for HostDiskDimensionsLba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskDimensionsLba
    }
}

impl VimObjectTrait for HostDiskPartitionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskPartitionInfo
    }
}

impl VimObjectTrait for HostDiskPartitionBlockRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskPartitionBlockRange
    }
}

impl VimObjectTrait for HostDiskPartitionLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskPartitionLayout
    }
}

impl VimObjectTrait for HostDiskPartitionAttributes {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskPartitionAttributes
    }
}

impl VimObjectTrait for HostDiskPartitionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskPartitionSpec
    }
}

impl VimObjectTrait for HostDnsConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDnsConfig
    }
}

impl VimObjectTrait for HostDnsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDnsConfigSpec
    }
}

impl VimObjectTrait for HostDvxClass {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDvxClass
    }
}

impl VimObjectTrait for HostEnterMaintenanceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostEnterMaintenanceResult
    }
}

impl VimObjectTrait for HostEsxAgentHostManagerConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostEsxAgentHostManagerConfigInfo
    }
}

impl VimObjectTrait for HostFaultToleranceManagerComponentHealthInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFaultToleranceManagerComponentHealthInfo
    }
}

impl VimObjectTrait for FcoeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeConfig
    }
}

impl VimObjectTrait for FcoeConfigFcoeCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeConfigFcoeCapabilities
    }
}

impl VimObjectTrait for FcoeConfigFcoeSpecification {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeConfigFcoeSpecification
    }
}

impl VimObjectTrait for FcoeConfigVlanRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeConfigVlanRange
    }
}

impl VimObjectTrait for HostFeatureCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFeatureCapability
    }
}

impl VimObjectTrait for HostFeatureMask {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFeatureMask
    }
}

impl VimObjectTrait for HostFeatureVersionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFeatureVersionInfo
    }
}

impl VimObjectTrait for HostFibreChannelOverEthernetHbaLinkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFibreChannelOverEthernetHbaLinkInfo
    }
}

impl VimObjectTrait for HostFileAccess {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFileAccess
    }
}

impl VimObjectTrait for ModeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ModeInfo
    }
}

impl VimObjectTrait for HostFileSystemMountInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFileSystemMountInfo
    }
}

impl VimObjectTrait for HostFileSystemVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFileSystemVolume
    }
}

impl VimObjectTrait for HostLocalFileSystemVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLocalFileSystemVolume
    }
}

impl VimObjectTrait for HostNasVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNasVolume
    }
}

impl VimObjectTrait for HostPMemVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPMemVolume
    }
}

impl VimObjectTrait for HostVfatVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVfatVolume
    }
}

impl VimObjectTrait for HostVffsVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVffsVolume
    }
}

impl VimObjectTrait for HostVmfsVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVmfsVolume
    }
}

impl VimObjectTrait for HostVvolVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVvolVolume
    }
}

impl VimObjectTrait for HostFileSystemVolumeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFileSystemVolumeInfo
    }
}

impl VimObjectTrait for HostFirewallConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallConfig
    }
}

impl VimObjectTrait for HostFirewallConfigRuleSetConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallConfigRuleSetConfig
    }
}

impl VimObjectTrait for HostFirewallInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallInfo
    }
}

impl VimObjectTrait for HostFirewallDefaultPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallDefaultPolicy
    }
}

impl VimObjectTrait for HostFlagInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFlagInfo
    }
}

impl VimObjectTrait for HostForceMountedInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostForceMountedInfo
    }
}

impl VimObjectTrait for HostFru {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFru
    }
}

impl VimObjectTrait for HostGatewaySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostGatewaySpec
    }
}

impl VimObjectTrait for HostGraphicsConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostGraphicsConfig
    }
}

impl VimObjectTrait for HostGraphicsConfigDeviceType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostGraphicsConfigDeviceType
    }
}

impl VimObjectTrait for HostGraphicsInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostGraphicsInfo
    }
}

impl VimObjectTrait for HostHardwareInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHardwareInfo
    }
}

impl VimObjectTrait for HostHardwareStatusInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHardwareStatusInfo
    }
}

impl VimObjectTrait for DpuStatusInfoOperationalInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DpuStatusInfoOperationalInfo
    }
}

impl VimObjectTrait for HostHardwareElementInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHardwareElementInfo
    }
}

impl VimObjectTrait for DpuStatusInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DpuStatusInfo
    }
}

impl VimObjectTrait for HostStorageElementInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageElementInfo
    }
}

impl VimObjectTrait for HostStorageOperationalInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageOperationalInfo
    }
}

impl VimObjectTrait for HostHbaCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHbaCreateSpec
    }
}

impl VimObjectTrait for HostTcpHbaCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTcpHbaCreateSpec
    }
}

impl VimObjectTrait for HealthSystemRuntime {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HealthSystemRuntime
    }
}

impl VimObjectTrait for HostAccessControlEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAccessControlEntry
    }
}

impl VimObjectTrait for HostHostBusAdapter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHostBusAdapter
    }
}

impl VimObjectTrait for HostBlockHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostBlockHba
    }
}

impl VimObjectTrait for HostFibreChannelHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFibreChannelHba
    }
}

impl VimObjectTrait for HostFibreChannelOverEthernetHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFibreChannelOverEthernetHba
    }
}

impl VimObjectTrait for HostInternetScsiHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHba
    }
}

impl VimObjectTrait for HostParallelScsiHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostParallelScsiHba
    }
}

impl VimObjectTrait for HostPcieHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPcieHba
    }
}

impl VimObjectTrait for HostRdmaHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaHba
    }
}

impl VimObjectTrait for HostSerialAttachedHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSerialAttachedHba
    }
}

impl VimObjectTrait for HostTcpHba {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTcpHba
    }
}

impl VimObjectTrait for HostProxySwitch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProxySwitch
    }
}

impl VimObjectTrait for HostProxySwitchConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProxySwitchConfig
    }
}

impl VimObjectTrait for HostProxySwitchEnsInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProxySwitchEnsInfo
    }
}

impl VimObjectTrait for HostProxySwitchHostLagConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProxySwitchHostLagConfig
    }
}

impl VimObjectTrait for HostProxySwitchSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProxySwitchSpec
    }
}

impl VimObjectTrait for HostImageProfileSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostImageProfileSummary
    }
}

impl VimObjectTrait for HostInternetScsiHbaAuthenticationCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaAuthenticationCapabilities
    }
}

impl VimObjectTrait for HostInternetScsiHbaAuthenticationProperties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaAuthenticationProperties
    }
}

impl VimObjectTrait for HostInternetScsiHbaDigestCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaDigestCapabilities
    }
}

impl VimObjectTrait for HostInternetScsiHbaDigestProperties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaDigestProperties
    }
}

impl VimObjectTrait for HostInternetScsiHbaDiscoveryCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaDiscoveryCapabilities
    }
}

impl VimObjectTrait for HostInternetScsiHbaDiscoveryProperties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaDiscoveryProperties
    }
}

impl VimObjectTrait for HostInternetScsiHbaIpCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaIpCapabilities
    }
}

impl VimObjectTrait for HostInternetScsiHbaIpProperties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaIpProperties
    }
}

impl VimObjectTrait for HostInternetScsiHbaIPv6Properties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaIPv6Properties
    }
}

impl VimObjectTrait for HostInternetScsiHbaIscsiIpv6Address {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaIscsiIpv6Address
    }
}

impl VimObjectTrait for HostInternetScsiHbaSendTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaSendTarget
    }
}

impl VimObjectTrait for HostInternetScsiHbaStaticTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaStaticTarget
    }
}

impl VimObjectTrait for HostInternetScsiHbaTargetSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaTargetSet
    }
}

impl VimObjectTrait for HostIpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpConfig
    }
}

impl VimObjectTrait for HostIpConfigIpV6Address {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpConfigIpV6Address
    }
}

impl VimObjectTrait for HostIpConfigIpV6AddressConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpConfigIpV6AddressConfiguration
    }
}

impl VimObjectTrait for HostIpRouteConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteConfig
    }
}

impl VimObjectTrait for HostIpRouteConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteConfigSpec
    }
}

impl VimObjectTrait for HostIpRouteEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteEntry
    }
}

impl VimObjectTrait for HostIpRouteOp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteOp
    }
}

impl VimObjectTrait for HostIpRouteTableConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteTableConfig
    }
}

impl VimObjectTrait for HostIpRouteTableInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpRouteTableInfo
    }
}

impl VimObjectTrait for HostIpmiInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIpmiInfo
    }
}

impl VimObjectTrait for IscsiDependencyEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiDependencyEntity
    }
}

impl VimObjectTrait for IscsiMigrationDependency {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiMigrationDependency
    }
}

impl VimObjectTrait for IscsiPortInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiPortInfo
    }
}

impl VimObjectTrait for IscsiStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiStatus
    }
}

impl VimObjectTrait for KernelModuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KernelModuleInfo
    }
}

impl VimObjectTrait for KernelModuleSectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KernelModuleSectionInfo
    }
}

impl VimObjectTrait for HostLicenseSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLicenseSpec
    }
}

impl VimObjectTrait for LinkDiscoveryProtocolConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LinkDiscoveryProtocolConfig
    }
}

impl VimObjectTrait for HostAccountSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAccountSpec
    }
}

impl VimObjectTrait for HostPosixAccountSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPosixAccountSpec
    }
}

impl VimObjectTrait for HostLocalFileSystemVolumeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLocalFileSystemVolumeSpec
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerDiskLayoutSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerDiskLayoutSpec
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerFileDeleteResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerFileDeleteResult
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerFileDeleteSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerFileDeleteSpec
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerFileReserveResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerFileReserveResult
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerFileReserveSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerFileReserveSpec
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerSnapshotLayoutSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerSnapshotLayoutSpec
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerVmMigrationStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerVmMigrationStatus
    }
}

impl VimObjectTrait for HostLowLevelProvisioningManagerVmRecoveryInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostLowLevelProvisioningManagerVmRecoveryInfo
    }
}

impl VimObjectTrait for HostMaintenanceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMaintenanceSpec
    }
}

impl VimObjectTrait for ServiceConsoleReservationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceConsoleReservationInfo
    }
}

impl VimObjectTrait for VirtualMachineMemoryReservationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMemoryReservationInfo
    }
}

impl VimObjectTrait for VirtualMachineMemoryReservationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMemoryReservationSpec
    }
}

impl VimObjectTrait for HostMemorySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemorySpec
    }
}

impl VimObjectTrait for HostMemoryTierInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemoryTierInfo
    }
}

impl VimObjectTrait for HostMountInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMountInfo
    }
}

impl VimObjectTrait for HostMultipathInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfo
    }
}

impl VimObjectTrait for HostMultipathInfoLogicalUnit {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoLogicalUnit
    }
}

impl VimObjectTrait for HostMultipathInfoLogicalUnitPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoLogicalUnitPolicy
    }
}

impl VimObjectTrait for HostMultipathInfoFixedLogicalUnitPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoFixedLogicalUnitPolicy
    }
}

impl VimObjectTrait for HostMultipathInfoHppLogicalUnitPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoHppLogicalUnitPolicy
    }
}

impl VimObjectTrait for HostMultipathInfoLogicalUnitStorageArrayTypePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoLogicalUnitStorageArrayTypePolicy
    }
}

impl VimObjectTrait for HostMultipathInfoPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathInfoPath
    }
}

impl VimObjectTrait for HostMultipathStateInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathStateInfo
    }
}

impl VimObjectTrait for HostMultipathStateInfoPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMultipathStateInfoPath
    }
}

impl VimObjectTrait for HostNasVolumeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNasVolumeConfig
    }
}

impl VimObjectTrait for HostNasVolumeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNasVolumeSpec
    }
}

impl VimObjectTrait for HostNasVolumeUserInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNasVolumeUserInfo
    }
}

impl VimObjectTrait for HostNatService {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNatService
    }
}

impl VimObjectTrait for HostNatServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNatServiceConfig
    }
}

impl VimObjectTrait for HostNatServiceNameServiceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNatServiceNameServiceSpec
    }
}

impl VimObjectTrait for HostNatServicePortForwardSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNatServicePortForwardSpec
    }
}

impl VimObjectTrait for HostNatServiceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNatServiceSpec
    }
}

impl VimObjectTrait for HostNetCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetCapabilities
    }
}

impl VimObjectTrait for HostNetOffloadCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetOffloadCapabilities
    }
}

impl VimObjectTrait for HostNetStackInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetStackInstance
    }
}

impl VimObjectTrait for HostNetworkConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkConfig
    }
}

impl VimObjectTrait for HostNetworkConfigNetStackSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkConfigNetStackSpec
    }
}

impl VimObjectTrait for HostNetworkConfigResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkConfigResult
    }
}

impl VimObjectTrait for HostNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkInfo
    }
}

impl VimObjectTrait for HostNetworkPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkPolicy
    }
}

impl VimObjectTrait for HostNicFailureCriteria {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNicFailureCriteria
    }
}

impl VimObjectTrait for HostNicOrderPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNicOrderPolicy
    }
}

impl VimObjectTrait for HostNicTeamingPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNicTeamingPolicy
    }
}

impl VimObjectTrait for HostNetworkSecurityPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkSecurityPolicy
    }
}

impl VimObjectTrait for HostNetworkTrafficShapingPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkTrafficShapingPolicy
    }
}

impl VimObjectTrait for HostNtpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNtpConfig
    }
}

impl VimObjectTrait for HostNumaInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNumaInfo
    }
}

impl VimObjectTrait for HostNumaNode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNumaNode
    }
}

impl VimObjectTrait for HostNumericSensorInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNumericSensorInfo
    }
}

impl VimObjectTrait for NvdimmDimmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmDimmInfo
    }
}

impl VimObjectTrait for NvdimmGuid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmGuid
    }
}

impl VimObjectTrait for NvdimmHealthInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmHealthInfo
    }
}

impl VimObjectTrait for NvdimmInterleaveSetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmInterleaveSetInfo
    }
}

impl VimObjectTrait for NvdimmNamespaceCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmNamespaceCreateSpec
    }
}

impl VimObjectTrait for NvdimmNamespaceDeleteSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmNamespaceDeleteSpec
    }
}

impl VimObjectTrait for NvdimmNamespaceDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmNamespaceDetails
    }
}

impl VimObjectTrait for NvdimmNamespaceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmNamespaceInfo
    }
}

impl VimObjectTrait for NvdimmSystemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmSystemInfo
    }
}

impl VimObjectTrait for NvdimmPMemNamespaceCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmPMemNamespaceCreateSpec
    }
}

impl VimObjectTrait for NvdimmRegionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmRegionInfo
    }
}

impl VimObjectTrait for NvdimmSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NvdimmSummary
    }
}

impl VimObjectTrait for HostNvmeController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeController
    }
}

impl VimObjectTrait for HostNvmeDisconnectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeDisconnectSpec
    }
}

impl VimObjectTrait for HostNvmeDiscoveryLog {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeDiscoveryLog
    }
}

impl VimObjectTrait for HostNvmeDiscoveryLogEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeDiscoveryLogEntry
    }
}

impl VimObjectTrait for HostNvmeNamespace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeNamespace
    }
}

impl VimObjectTrait for HostNvmeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeSpec
    }
}

impl VimObjectTrait for HostNvmeConnectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeConnectSpec
    }
}

impl VimObjectTrait for HostNvmeDiscoverSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeDiscoverSpec
    }
}

impl VimObjectTrait for HostNvmeTopology {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeTopology
    }
}

impl VimObjectTrait for HostNvmeTopologyInterface {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeTopologyInterface
    }
}

impl VimObjectTrait for HostNvmeTransportParameters {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeTransportParameters
    }
}

impl VimObjectTrait for HostNvmeOpaqueTransportParameters {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeOpaqueTransportParameters
    }
}

impl VimObjectTrait for HostNvmeOverFibreChannelParameters {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeOverFibreChannelParameters
    }
}

impl VimObjectTrait for HostNvmeOverRdmaParameters {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeOverRdmaParameters
    }
}

impl VimObjectTrait for HostNvmeOverTcpParameters {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNvmeOverTcpParameters
    }
}

impl VimObjectTrait for HostOpaqueNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostOpaqueNetworkInfo
    }
}

impl VimObjectTrait for HostOpaqueSwitch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostOpaqueSwitch
    }
}

impl VimObjectTrait for HostOpaqueSwitchPhysicalNicZone {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostOpaqueSwitchPhysicalNicZone
    }
}

impl VimObjectTrait for HostPatchManagerLocator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPatchManagerLocator
    }
}

impl VimObjectTrait for HostPatchManagerPatchManagerOperationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPatchManagerPatchManagerOperationSpec
    }
}

impl VimObjectTrait for HostPatchManagerResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPatchManagerResult
    }
}

impl VimObjectTrait for HostPatchManagerStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPatchManagerStatus
    }
}

impl VimObjectTrait for HostPatchManagerStatusPrerequisitePatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPatchManagerStatusPrerequisitePatch
    }
}

impl VimObjectTrait for HostPathSelectionPolicyOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPathSelectionPolicyOption
    }
}

impl VimObjectTrait for HostPciDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPciDevice
    }
}

impl VimObjectTrait for HostPciPassthruConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPciPassthruConfig
    }
}

impl VimObjectTrait for HostSriovConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSriovConfig
    }
}

impl VimObjectTrait for HostPciPassthruInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPciPassthruInfo
    }
}

impl VimObjectTrait for HostSriovInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSriovInfo
    }
}

impl VimObjectTrait for HostPersistentMemoryInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPersistentMemoryInfo
    }
}

impl VimObjectTrait for PhysicalNic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNic
    }
}

impl VimObjectTrait for PhysicalNicCdpDeviceCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicCdpDeviceCapability
    }
}

impl VimObjectTrait for PhysicalNicCdpInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicCdpInfo
    }
}

impl VimObjectTrait for PhysicalNicConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicConfig
    }
}

impl VimObjectTrait for PhysicalNicLinkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicLinkInfo
    }
}

impl VimObjectTrait for LinkLayerDiscoveryProtocolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LinkLayerDiscoveryProtocolInfo
    }
}

impl VimObjectTrait for PhysicalNicHintInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicHintInfo
    }
}

impl VimObjectTrait for PhysicalNicHint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicHint
    }
}

impl VimObjectTrait for PhysicalNicIpHint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicIpHint
    }
}

impl VimObjectTrait for PhysicalNicNameHint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicNameHint
    }
}

impl VimObjectTrait for PhysicalNicSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicSpec
    }
}

impl VimObjectTrait for HostPlugStoreTopology {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopology
    }
}

impl VimObjectTrait for HostPlugStoreTopologyAdapter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopologyAdapter
    }
}

impl VimObjectTrait for HostPlugStoreTopologyDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopologyDevice
    }
}

impl VimObjectTrait for HostPlugStoreTopologyPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopologyPath
    }
}

impl VimObjectTrait for HostPlugStoreTopologyPlugin {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopologyPlugin
    }
}

impl VimObjectTrait for HostPlugStoreTopologyTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlugStoreTopologyTarget
    }
}

impl VimObjectTrait for HostPortGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPortGroup
    }
}

impl VimObjectTrait for HostPortGroupConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPortGroupConfig
    }
}

impl VimObjectTrait for HostPortGroupPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPortGroupPort
    }
}

impl VimObjectTrait for HostPortGroupSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPortGroupSpec
    }
}

impl VimObjectTrait for PowerSystemCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PowerSystemCapability
    }
}

impl VimObjectTrait for PowerSystemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PowerSystemInfo
    }
}

impl VimObjectTrait for HostPowerPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPowerPolicy
    }
}

impl VimObjectTrait for HostProtocolEndpoint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProtocolEndpoint
    }
}

impl VimObjectTrait for HostPtpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPtpConfig
    }
}

impl VimObjectTrait for HostPtpConfigPtpPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPtpConfigPtpPort
    }
}

impl VimObjectTrait for HostQualifiedName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostQualifiedName
    }
}

impl VimObjectTrait for HostRdmaDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaDevice
    }
}

impl VimObjectTrait for HostRdmaDeviceBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaDeviceBacking
    }
}

impl VimObjectTrait for HostRdmaDevicePnicBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaDevicePnicBacking
    }
}

impl VimObjectTrait for HostRdmaDeviceCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaDeviceCapability
    }
}

impl VimObjectTrait for HostRdmaDeviceConnectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaDeviceConnectionInfo
    }
}

impl VimObjectTrait for HostReliableMemoryInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostReliableMemoryInfo
    }
}

impl VimObjectTrait for HostResignatureRescanResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostResignatureRescanResult
    }
}

impl VimObjectTrait for HostFirewallRuleset {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallRuleset
    }
}

impl VimObjectTrait for HostFirewallRulesetIpList {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallRulesetIpList
    }
}

impl VimObjectTrait for HostFirewallRulesetIpNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallRulesetIpNetwork
    }
}

impl VimObjectTrait for HostFirewallRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallRule
    }
}

impl VimObjectTrait for HostFirewallRulesetRulesetSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFirewallRulesetRulesetSpec
    }
}

impl VimObjectTrait for HostRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRuntimeInfo
    }
}

impl VimObjectTrait for HostRuntimeInfoNetStackInstanceRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRuntimeInfoNetStackInstanceRuntimeInfo
    }
}

impl VimObjectTrait for HostNetworkResourceRuntime {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNetworkResourceRuntime
    }
}

impl VimObjectTrait for HostRuntimeInfoNetworkRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRuntimeInfoNetworkRuntimeInfo
    }
}

impl VimObjectTrait for HostPlacedVirtualNicIdentifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPlacedVirtualNicIdentifier
    }
}

impl VimObjectTrait for HostPnicNetworkResourceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPnicNetworkResourceInfo
    }
}

impl VimObjectTrait for HostRuntimeInfoStateEncryptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRuntimeInfoStateEncryptionInfo
    }
}

impl VimObjectTrait for HostScsiDiskPartition {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiDiskPartition
    }
}

impl VimObjectTrait for ScsiLunCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScsiLunCapabilities
    }
}

impl VimObjectTrait for ScsiLunDescriptor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScsiLunDescriptor
    }
}

impl VimObjectTrait for ScsiLunDurableName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScsiLunDurableName
    }
}

impl VimObjectTrait for HostScsiTopology {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiTopology
    }
}

impl VimObjectTrait for HostScsiTopologyInterface {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiTopologyInterface
    }
}

impl VimObjectTrait for HostScsiTopologyLun {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiTopologyLun
    }
}

impl VimObjectTrait for HostScsiTopologyTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostScsiTopologyTarget
    }
}

impl VimObjectTrait for HostSecuritySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSecuritySpec
    }
}

impl VimObjectTrait for HostService {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostService
    }
}

impl VimObjectTrait for HostServiceSourcePackage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostServiceSourcePackage
    }
}

impl VimObjectTrait for HostServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostServiceConfig
    }
}

impl VimObjectTrait for HostServiceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostServiceInfo
    }
}

impl VimObjectTrait for HostSevInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSevInfo
    }
}

impl VimObjectTrait for HostSgxInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSgxInfo
    }
}

impl VimObjectTrait for HostSgxRegistrationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSgxRegistrationInfo
    }
}

impl VimObjectTrait for HostSharedGpuCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSharedGpuCapabilities
    }
}

impl VimObjectTrait for HostSnmpSystemAgentLimits {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSnmpSystemAgentLimits
    }
}

impl VimObjectTrait for HostSnmpConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSnmpConfigSpec
    }
}

impl VimObjectTrait for HostSnmpDestination {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSnmpDestination
    }
}

impl VimObjectTrait for SoftwarePackage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SoftwarePackage
    }
}

impl VimObjectTrait for SoftwarePackageCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SoftwarePackageCapability
    }
}

impl VimObjectTrait for Relation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Relation
    }
}

impl VimObjectTrait for HostSriovDevicePoolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSriovDevicePoolInfo
    }
}

impl VimObjectTrait for HostSriovNetworkDevicePoolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSriovNetworkDevicePoolInfo
    }
}

impl VimObjectTrait for HostSslThumbprintInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSslThumbprintInfo
    }
}

impl VimObjectTrait for HostStorageArrayTypePolicyOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageArrayTypePolicyOption
    }
}

impl VimObjectTrait for HostStorageDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageDeviceInfo
    }
}

impl VimObjectTrait for HostStorageSystemDiskLocatorLedResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageSystemDiskLocatorLedResult
    }
}

impl VimObjectTrait for HostStorageSystemScsiLunResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageSystemScsiLunResult
    }
}

impl VimObjectTrait for HostStorageSystemVmfsVolumeResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostStorageSystemVmfsVolumeResult
    }
}

impl VimObjectTrait for HostListSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostListSummary
    }
}

impl VimObjectTrait for HostConfigSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigSummary
    }
}

impl VimObjectTrait for HostListSummaryGatewaySummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostListSummaryGatewaySummary
    }
}

impl VimObjectTrait for HostHardwareSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHardwareSummary
    }
}

impl VimObjectTrait for HostListSummaryQuickStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostListSummaryQuickStats
    }
}

impl VimObjectTrait for SystemEventInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SystemEventInfo
    }
}

impl VimObjectTrait for HostSystemHealthInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemHealthInfo
    }
}

impl VimObjectTrait for HostSystemIdentificationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemIdentificationInfo
    }
}

impl VimObjectTrait for HostSystemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemInfo
    }
}

impl VimObjectTrait for HostSystemResourceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemResourceInfo
    }
}

impl VimObjectTrait for HostSystemSwapConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfiguration
    }
}

impl VimObjectTrait for HostSystemSwapConfigurationSystemSwapOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfigurationSystemSwapOption
    }
}

impl VimObjectTrait for HostSystemSwapConfigurationDatastoreOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfigurationDatastoreOption
    }
}

impl VimObjectTrait for HostSystemSwapConfigurationDisabledOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfigurationDisabledOption
    }
}

impl VimObjectTrait for HostSystemSwapConfigurationHostCacheOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfigurationHostCacheOption
    }
}

impl VimObjectTrait for HostSystemSwapConfigurationHostLocalSwapOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSystemSwapConfigurationHostLocalSwapOption
    }
}

impl VimObjectTrait for HostTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTargetTransport
    }
}

impl VimObjectTrait for HostBlockAdapterTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostBlockAdapterTargetTransport
    }
}

impl VimObjectTrait for HostFibreChannelTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFibreChannelTargetTransport
    }
}

impl VimObjectTrait for HostFibreChannelOverEthernetTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostFibreChannelOverEthernetTargetTransport
    }
}

impl VimObjectTrait for HostInternetScsiTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiTargetTransport
    }
}

impl VimObjectTrait for HostParallelScsiTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostParallelScsiTargetTransport
    }
}

impl VimObjectTrait for HostPcieTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPcieTargetTransport
    }
}

impl VimObjectTrait for HostRdmaTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostRdmaTargetTransport
    }
}

impl VimObjectTrait for HostSerialAttachedTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSerialAttachedTargetTransport
    }
}

impl VimObjectTrait for HostTcpTargetTransport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTcpTargetTransport
    }
}

impl VimObjectTrait for HostTpmAttestationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmAttestationInfo
    }
}

impl VimObjectTrait for HostTpmAttestationReport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmAttestationReport
    }
}

impl VimObjectTrait for HostTpmEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmEventDetails
    }
}

impl VimObjectTrait for HostTpmBootCompleteEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmBootCompleteEventDetails
    }
}

impl VimObjectTrait for HostTpmBootSecurityOptionEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmBootSecurityOptionEventDetails
    }
}

impl VimObjectTrait for HostTpmNvTagEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmNvTagEventDetails
    }
}

impl VimObjectTrait for HostTpmSignerEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmSignerEventDetails
    }
}

impl VimObjectTrait for HostTpmCommandEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmCommandEventDetails
    }
}

impl VimObjectTrait for HostTpmOptionEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmOptionEventDetails
    }
}

impl VimObjectTrait for HostTpmSoftwareComponentEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmSoftwareComponentEventDetails
    }
}

impl VimObjectTrait for HostTpmVersionEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmVersionEventDetails
    }
}

impl VimObjectTrait for HostTpmEventLogEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmEventLogEntry
    }
}

impl VimObjectTrait for HostTrustAuthorityAttestationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTrustAuthorityAttestationInfo
    }
}

impl VimObjectTrait for HostUnresolvedVmfsExtent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsExtent
    }
}

impl VimObjectTrait for HostUnresolvedVmfsResignatureSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsResignatureSpec
    }
}

impl VimObjectTrait for HostUnresolvedVmfsResolutionResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsResolutionResult
    }
}

impl VimObjectTrait for HostUnresolvedVmfsResolutionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsResolutionSpec
    }
}

impl VimObjectTrait for HostUnresolvedVmfsVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsVolume
    }
}

impl VimObjectTrait for HostUnresolvedVmfsVolumeResolveStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostUnresolvedVmfsVolumeResolveStatus
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashCacheConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashCacheConfigInfo
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashCacheConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashCacheConfigSpec
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashConfigInfo
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashResourceConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashResourceConfigInfo
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashResourceConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashResourceConfigSpec
    }
}

impl VimObjectTrait for HostVFlashManagerVFlashResourceRunTimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashManagerVFlashResourceRunTimeInfo
    }
}

impl VimObjectTrait for HostVFlashResourceConfigurationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVFlashResourceConfigurationResult
    }
}

impl VimObjectTrait for HostVMotionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionConfig
    }
}

impl VimObjectTrait for HostVMotionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionInfo
    }
}

impl VimObjectTrait for HostVMotionManagerDstInstantCloneResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionManagerDstInstantCloneResult
    }
}

impl VimObjectTrait for HostVMotionManagerSrcInstantCloneResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionManagerSrcInstantCloneResult
    }
}

impl VimObjectTrait for HostVMotionNetConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVMotionNetConfig
    }
}

impl VimObjectTrait for HostVffsSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVffsSpec
    }
}

impl VimObjectTrait for HostVirtualNic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNic
    }
}

impl VimObjectTrait for HostVirtualNicConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicConfig
    }
}

impl VimObjectTrait for HostVirtualNicIpRouteSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicIpRouteSpec
    }
}

impl VimObjectTrait for HostVirtualNicOpaqueNetworkSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicOpaqueNetworkSpec
    }
}

impl VimObjectTrait for HostVirtualNicSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicSpec
    }
}

impl VimObjectTrait for HostVirtualNicConnection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicConnection
    }
}

impl VimObjectTrait for VirtualNicManagerNetConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNicManagerNetConfig
    }
}

impl VimObjectTrait for HostVirtualNicManagerNicTypeSelection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicManagerNicTypeSelection
    }
}

impl VimObjectTrait for HostVirtualNicManagerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualNicManagerInfo
    }
}

impl VimObjectTrait for HostVirtualSwitch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitch
    }
}

impl VimObjectTrait for HostVirtualSwitchBeaconConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchBeaconConfig
    }
}

impl VimObjectTrait for HostVirtualSwitchBridge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchBridge
    }
}

impl VimObjectTrait for HostVirtualSwitchAutoBridge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchAutoBridge
    }
}

impl VimObjectTrait for HostVirtualSwitchBondBridge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchBondBridge
    }
}

impl VimObjectTrait for HostVirtualSwitchSimpleBridge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchSimpleBridge
    }
}

impl VimObjectTrait for HostVirtualSwitchConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchConfig
    }
}

impl VimObjectTrait for HostVirtualSwitchSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVirtualSwitchSpec
    }
}

impl VimObjectTrait for HostVmciAccessManagerAccessSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVmciAccessManagerAccessSpec
    }
}

impl VimObjectTrait for VmfsDatastoreOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreOption
    }
}

impl VimObjectTrait for VmfsDatastoreBaseOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreBaseOption
    }
}

impl VimObjectTrait for VmfsDatastoreMultipleExtentOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreMultipleExtentOption
    }
}

impl VimObjectTrait for VmfsDatastoreSingleExtentOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreSingleExtentOption
    }
}

impl VimObjectTrait for VmfsDatastoreAllExtentOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreAllExtentOption
    }
}

impl VimObjectTrait for VmfsDatastoreSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreSpec
    }
}

impl VimObjectTrait for VmfsDatastoreCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreCreateSpec
    }
}

impl VimObjectTrait for VmfsDatastoreExpandSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreExpandSpec
    }
}

impl VimObjectTrait for VmfsDatastoreExtendSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsDatastoreExtendSpec
    }
}

impl VimObjectTrait for HostVmfsRescanResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVmfsRescanResult
    }
}

impl VimObjectTrait for VmfsConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsConfigOption
    }
}

impl VimObjectTrait for HostVmfsSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVmfsSpec
    }
}

impl VimObjectTrait for VmfsUnmapBandwidthSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsUnmapBandwidthSpec
    }
}

impl VimObjectTrait for HostVsanInternalSystemCmmdsQuery {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVsanInternalSystemCmmdsQuery
    }
}

impl VimObjectTrait for HostVsanInternalSystemDeleteVsanObjectsResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVsanInternalSystemDeleteVsanObjectsResult
    }
}

impl VimObjectTrait for VsanNewPolicyBatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNewPolicyBatch
    }
}

impl VimObjectTrait for VsanPolicyChangeBatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPolicyChangeBatch
    }
}

impl VimObjectTrait for VsanPolicyCost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPolicyCost
    }
}

impl VimObjectTrait for VsanPolicySatisfiability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPolicySatisfiability
    }
}

impl VimObjectTrait for HostVsanInternalSystemVsanObjectOperationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVsanInternalSystemVsanObjectOperationResult
    }
}

impl VimObjectTrait for HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult
    }
}

impl VimObjectTrait for HostVvolNqn {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVvolNqn
    }
}

impl VimObjectTrait for VVolHostPe {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VVolHostPe
    }
}

impl VimObjectTrait for HostVvolVolumeHostVvolNqn {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVvolVolumeHostVvolNqn
    }
}

impl VimObjectTrait for HostVvolVolumeSpecification {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostVvolVolumeSpecification
    }
}

impl VimObjectTrait for NetDhcpConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDhcpConfigInfo
    }
}

impl VimObjectTrait for NetDhcpConfigInfoDhcpOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDhcpConfigInfoDhcpOptions
    }
}

impl VimObjectTrait for NetDhcpConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDhcpConfigSpec
    }
}

impl VimObjectTrait for NetDhcpConfigSpecDhcpOptionsSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDhcpConfigSpecDhcpOptionsSpec
    }
}

impl VimObjectTrait for NetDnsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDnsConfigInfo
    }
}

impl VimObjectTrait for NetDnsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetDnsConfigSpec
    }
}

impl VimObjectTrait for NetIpConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpConfigInfo
    }
}

impl VimObjectTrait for NetIpConfigInfoIpAddress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpConfigInfoIpAddress
    }
}

impl VimObjectTrait for NetIpConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpConfigSpec
    }
}

impl VimObjectTrait for NetIpConfigSpecIpAddressSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpConfigSpecIpAddressSpec
    }
}

impl VimObjectTrait for NetIpRouteConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigInfo
    }
}

impl VimObjectTrait for NetIpRouteConfigInfoGateway {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigInfoGateway
    }
}

impl VimObjectTrait for NetIpRouteConfigInfoIpRoute {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigInfoIpRoute
    }
}

impl VimObjectTrait for NetIpRouteConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigSpec
    }
}

impl VimObjectTrait for NetIpRouteConfigSpecGatewaySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigSpecGatewaySpec
    }
}

impl VimObjectTrait for NetIpRouteConfigSpecIpRouteSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpRouteConfigSpecIpRouteSpec
    }
}

impl VimObjectTrait for NetIpStackInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpStackInfo
    }
}

impl VimObjectTrait for NetIpStackInfoDefaultRouter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpStackInfoDefaultRouter
    }
}

impl VimObjectTrait for NetIpStackInfoNetToMedia {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetIpStackInfoNetToMedia
    }
}

impl VimObjectTrait for NetBiosConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetBiosConfigInfo
    }
}

impl VimObjectTrait for WinNetBiosConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WinNetBiosConfigInfo
    }
}

impl VimObjectTrait for ArrayUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ArrayUpdateSpec
    }
}

impl VimObjectTrait for ClusterDasVmConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDasVmConfigSpec
    }
}

impl VimObjectTrait for ClusterDatastoreUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDatastoreUpdateSpec
    }
}

impl VimObjectTrait for ClusterDpmHostConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDpmHostConfigSpec
    }
}

impl VimObjectTrait for ClusterDrsVmConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterDrsVmConfigSpec
    }
}

impl VimObjectTrait for ClusterGroupSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterGroupSpec
    }
}

impl VimObjectTrait for ClusterPreemptibleVmPairSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterPreemptibleVmPairSpec
    }
}

impl VimObjectTrait for ClusterRuleSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterRuleSpec
    }
}

impl VimObjectTrait for ClusterTagCategoryUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterTagCategoryUpdateSpec
    }
}

impl VimObjectTrait for ClusterVmOrchestrationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterVmOrchestrationSpec
    }
}

impl VimObjectTrait for StorageDrsOptionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsOptionSpec
    }
}

impl VimObjectTrait for StorageDrsVmConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsVmConfigSpec
    }
}

impl VimObjectTrait for VAppOvfSectionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppOvfSectionSpec
    }
}

impl VimObjectTrait for VAppProductSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppProductSpec
    }
}

impl VimObjectTrait for VAppPropertySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppPropertySpec
    }
}

impl VimObjectTrait for VirtualMachineCpuIdInfoSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineCpuIdInfoSpec
    }
}

impl VimObjectTrait for OptionType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OptionType
    }
}

impl VimObjectTrait for BoolOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BoolOption
    }
}

impl VimObjectTrait for ChoiceOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ChoiceOption
    }
}

impl VimObjectTrait for FloatOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FloatOption
    }
}

impl VimObjectTrait for IntOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntOption
    }
}

impl VimObjectTrait for LongOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LongOption
    }
}

impl VimObjectTrait for StringOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StringOption
    }
}

impl VimObjectTrait for OptionValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OptionValue
    }
}

impl VimObjectTrait for HostInternetScsiHbaParamValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInternetScsiHbaParamValue
    }
}

impl VimObjectTrait for ApplyProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ApplyProfile
    }
}

impl VimObjectTrait for ProfileApplyProfileElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileApplyProfileElement
    }
}

impl VimObjectTrait for ActiveDirectoryProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ActiveDirectoryProfile
    }
}

impl VimObjectTrait for AuthenticationProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthenticationProfile
    }
}

impl VimObjectTrait for DateTimeProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DateTimeProfile
    }
}

impl VimObjectTrait for DvsProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsProfile
    }
}

impl VimObjectTrait for DvsVNicProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsVNicProfile
    }
}

impl VimObjectTrait for DvsHostVNicProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsHostVNicProfile
    }
}

impl VimObjectTrait for DvsServiceConsoleVNicProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsServiceConsoleVNicProfile
    }
}

impl VimObjectTrait for FirewallProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FirewallProfile
    }
}

impl VimObjectTrait for FirewallProfileRulesetProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FirewallProfileRulesetProfile
    }
}

impl VimObjectTrait for HostApplyProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostApplyProfile
    }
}

impl VimObjectTrait for HostMemoryProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMemoryProfile
    }
}

impl VimObjectTrait for IpAddressProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpAddressProfile
    }
}

impl VimObjectTrait for IpRouteProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpRouteProfile
    }
}

impl VimObjectTrait for NasStorageProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasStorageProfile
    }
}

impl VimObjectTrait for NetStackInstanceProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetStackInstanceProfile
    }
}

impl VimObjectTrait for NetworkPolicyProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkPolicyProfile
    }
}

impl VimObjectTrait for NetworkProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkProfile
    }
}

impl VimObjectTrait for NetworkProfileDnsConfigProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkProfileDnsConfigProfile
    }
}

impl VimObjectTrait for NsxHostVNicProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NsxHostVNicProfile
    }
}

impl VimObjectTrait for OpaqueSwitchProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OpaqueSwitchProfile
    }
}

impl VimObjectTrait for OptionProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OptionProfile
    }
}

impl VimObjectTrait for PermissionProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PermissionProfile
    }
}

impl VimObjectTrait for PhysicalNicProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysicalNicProfile
    }
}

impl VimObjectTrait for PnicUplinkProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PnicUplinkProfile
    }
}

impl VimObjectTrait for PortGroupProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PortGroupProfile
    }
}

impl VimObjectTrait for HostPortGroupProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPortGroupProfile
    }
}

impl VimObjectTrait for ServiceConsolePortGroupProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceConsolePortGroupProfile
    }
}

impl VimObjectTrait for VmPortGroupProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPortGroupProfile
    }
}

impl VimObjectTrait for VirtualSwitchSelectionProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSwitchSelectionProfile
    }
}

impl VimObjectTrait for VlanProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VlanProfile
    }
}

impl VimObjectTrait for SecurityProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecurityProfile
    }
}

impl VimObjectTrait for ServiceProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ServiceProfile
    }
}

impl VimObjectTrait for StaticRouteProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StaticRouteProfile
    }
}

impl VimObjectTrait for StorageProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageProfile
    }
}

impl VimObjectTrait for UserGroupProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserGroupProfile
    }
}

impl VimObjectTrait for UserProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserProfile
    }
}

impl VimObjectTrait for VirtualSwitchProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSwitchProfile
    }
}

impl VimObjectTrait for LinkProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LinkProfile
    }
}

impl VimObjectTrait for NumPortsProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumPortsProfile
    }
}

impl VimObjectTrait for ProfileApplyProfileProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileApplyProfileProperty
    }
}

impl VimObjectTrait for ComplianceLocator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComplianceLocator
    }
}

impl VimObjectTrait for ComplianceProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComplianceProfile
    }
}

impl VimObjectTrait for ComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComplianceResult
    }
}

impl VimObjectTrait for ComplianceFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComplianceFailure
    }
}

impl VimObjectTrait for ComplianceFailureComplianceFailureValues {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComplianceFailureComplianceFailureValues
    }
}

impl VimObjectTrait for ProfileDeferredPolicyOptionParameter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileDeferredPolicyOptionParameter
    }
}

impl VimObjectTrait for ProfileExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileExpression
    }
}

impl VimObjectTrait for ProfileCompositeExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileCompositeExpression
    }
}

impl VimObjectTrait for ProfileSimpleExpression {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileSimpleExpression
    }
}

impl VimObjectTrait for ProfileExpressionMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileExpressionMetadata
    }
}

impl VimObjectTrait for ProfileParameterMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileParameterMetadata
    }
}

impl VimObjectTrait for ProfileParameterMetadataParameterRelationMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileParameterMetadataParameterRelationMetadata
    }
}

impl VimObjectTrait for ProfilePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfilePolicy
    }
}

impl VimObjectTrait for ProfilePolicyMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfilePolicyMetadata
    }
}

impl VimObjectTrait for PolicyOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PolicyOption
    }
}

impl VimObjectTrait for CompositePolicyOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CompositePolicyOption
    }
}

impl VimObjectTrait for ProfilePolicyOptionMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfilePolicyOptionMetadata
    }
}

impl VimObjectTrait for ProfileCompositePolicyOptionMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileCompositePolicyOptionMetadata
    }
}

impl VimObjectTrait for UserInputRequiredParameterMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserInputRequiredParameterMetadata
    }
}

impl VimObjectTrait for ProfileConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileConfigInfo
    }
}

impl VimObjectTrait for ClusterProfileConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProfileConfigInfo
    }
}

impl VimObjectTrait for HostProfileConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileConfigInfo
    }
}

impl VimObjectTrait for ProfileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileCreateSpec
    }
}

impl VimObjectTrait for ProfileSerializedCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileSerializedCreateSpec
    }
}

impl VimObjectTrait for HostProfileSerializedHostProfileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileSerializedHostProfileSpec
    }
}

impl VimObjectTrait for ClusterProfileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProfileCreateSpec
    }
}

impl VimObjectTrait for ClusterProfileConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProfileConfigSpec
    }
}

impl VimObjectTrait for ClusterProfileCompleteConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProfileCompleteConfigSpec
    }
}

impl VimObjectTrait for ClusterProfileConfigServiceCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterProfileConfigServiceCreateSpec
    }
}

impl VimObjectTrait for HostProfileConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileConfigSpec
    }
}

impl VimObjectTrait for HostProfileCompleteConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileCompleteConfigSpec
    }
}

impl VimObjectTrait for HostProfileHostBasedConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileHostBasedConfigSpec
    }
}

impl VimObjectTrait for ProfileDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileDescription
    }
}

impl VimObjectTrait for ProfileDescriptionSection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileDescriptionSection
    }
}

impl VimObjectTrait for ProfileMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileMetadata
    }
}

impl VimObjectTrait for ProfileMetadataProfileOperationMessage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileMetadataProfileOperationMessage
    }
}

impl VimObjectTrait for ProfileMetadataProfileSortSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileMetadataProfileSortSpec
    }
}

impl VimObjectTrait for ProfilePropertyPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfilePropertyPath
    }
}

impl VimObjectTrait for ProfileProfileStructure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileProfileStructure
    }
}

impl VimObjectTrait for ProfileProfileStructureProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileProfileStructureProperty
    }
}

impl VimObjectTrait for AnswerFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFile
    }
}

impl VimObjectTrait for AnswerFileStatusResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileStatusResult
    }
}

impl VimObjectTrait for AnswerFileStatusError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileStatusError
    }
}

impl VimObjectTrait for ProfileExecuteResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileExecuteResult
    }
}

impl VimObjectTrait for ApplyHostProfileConfigurationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ApplyHostProfileConfigurationSpec
    }
}

impl VimObjectTrait for ProfileExecuteError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileExecuteError
    }
}

impl VimObjectTrait for HostProfileValidationFailureInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileValidationFailureInfo
    }
}

impl VimObjectTrait for HostSpecification {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpecification
    }
}

impl VimObjectTrait for HostSubSpecification {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSubSpecification
    }
}

impl VimObjectTrait for AnswerFileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileCreateSpec
    }
}

impl VimObjectTrait for AnswerFileOptionsCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileOptionsCreateSpec
    }
}

impl VimObjectTrait for AnswerFileSerializedCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileSerializedCreateSpec
    }
}

impl VimObjectTrait for ApplyHostProfileConfigurationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ApplyHostProfileConfigurationResult
    }
}

impl VimObjectTrait for HostProfileManagerCompositionResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerCompositionResult
    }
}

impl VimObjectTrait for HostProfileManagerCompositionResultResultElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerCompositionResultResultElement
    }
}

impl VimObjectTrait for HostProfileManagerCompositionValidationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerCompositionValidationResult
    }
}

impl VimObjectTrait for HostProfileManagerCompositionValidationResultResultElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerCompositionValidationResultResultElement
    }
}

impl VimObjectTrait for HostProfileManagerConfigTaskList {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerConfigTaskList
    }
}

impl VimObjectTrait for HostProfilesEntityCustomizations {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfilesEntityCustomizations
    }
}

impl VimObjectTrait for StructuredCustomizations {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StructuredCustomizations
    }
}

impl VimObjectTrait for HostProfileManagerHostToConfigSpecMap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostProfileManagerHostToConfigSpecMap
    }
}

impl VimObjectTrait for ScheduledTaskDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskDescription
    }
}

impl VimObjectTrait for ScheduledTaskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskSpec
    }
}

impl VimObjectTrait for ScheduledTaskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledTaskInfo
    }
}

impl VimObjectTrait for TaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskScheduler
    }
}

impl VimObjectTrait for AfterStartupTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AfterStartupTaskScheduler
    }
}

impl VimObjectTrait for OnceTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OnceTaskScheduler
    }
}

impl VimObjectTrait for RecurrentTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecurrentTaskScheduler
    }
}

impl VimObjectTrait for HourlyTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HourlyTaskScheduler
    }
}

impl VimObjectTrait for DailyTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DailyTaskScheduler
    }
}

impl VimObjectTrait for MonthlyTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MonthlyTaskScheduler
    }
}

impl VimObjectTrait for MonthlyByDayTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MonthlyByDayTaskScheduler
    }
}

impl VimObjectTrait for MonthlyByWeekdayTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MonthlyByWeekdayTaskScheduler
    }
}

impl VimObjectTrait for WeeklyTaskScheduler {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WeeklyTaskScheduler
    }
}

impl VimObjectTrait for ApplyStorageRecommendationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ApplyStorageRecommendationResult
    }
}

impl VimObjectTrait for StorageDrsAutomationConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsAutomationConfig
    }
}

impl VimObjectTrait for StorageDrsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsConfigInfo
    }
}

impl VimObjectTrait for StorageDrsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsConfigSpec
    }
}

impl VimObjectTrait for StorageDrsIoLoadBalanceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsIoLoadBalanceConfig
    }
}

impl VimObjectTrait for PlacementAffinityRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementAffinityRule
    }
}

impl VimObjectTrait for PlacementRankResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementRankResult
    }
}

impl VimObjectTrait for PlacementRankSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlacementRankSpec
    }
}

impl VimObjectTrait for StorageDrsPlacementRankVmSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsPlacementRankVmSpec
    }
}

impl VimObjectTrait for StorageDrsPodConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsPodConfigInfo
    }
}

impl VimObjectTrait for StorageDrsPodConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsPodConfigSpec
    }
}

impl VimObjectTrait for StorageDrsPodSelectionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsPodSelectionSpec
    }
}

impl VimObjectTrait for PodDiskLocator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PodDiskLocator
    }
}

impl VimObjectTrait for VmPodConfigForPlacement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPodConfigForPlacement
    }
}

impl VimObjectTrait for StorageDrsSpaceLoadBalanceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsSpaceLoadBalanceConfig
    }
}

impl VimObjectTrait for StoragePlacementResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePlacementResult
    }
}

impl VimObjectTrait for StoragePlacementSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePlacementSpec
    }
}

impl VimObjectTrait for StorageDrsVmConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsVmConfigInfo
    }
}

impl VimObjectTrait for VAppCloneSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppCloneSpec
    }
}

impl VimObjectTrait for VAppCloneSpecNetworkMappingPair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppCloneSpecNetworkMappingPair
    }
}

impl VimObjectTrait for VAppCloneSpecResourceMap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppCloneSpecResourceMap
    }
}

impl VimObjectTrait for VAppEntityConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppEntityConfigInfo
    }
}

impl VimObjectTrait for VAppIpAssignmentInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppIpAssignmentInfo
    }
}

impl VimObjectTrait for IpPool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpPool
    }
}

impl VimObjectTrait for IpPoolAssociation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpPoolAssociation
    }
}

impl VimObjectTrait for IpPoolIpPoolConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpPoolIpPoolConfigInfo
    }
}

impl VimObjectTrait for VAppOvfSectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppOvfSectionInfo
    }
}

impl VimObjectTrait for VAppProductInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppProductInfo
    }
}

impl VimObjectTrait for VAppPropertyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppPropertyInfo
    }
}

impl VimObjectTrait for VmConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigInfo
    }
}

impl VimObjectTrait for VAppConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppConfigInfo
    }
}

impl VimObjectTrait for VmConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigSpec
    }
}

impl VimObjectTrait for VAppConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppConfigSpec
    }
}

impl VimObjectTrait for ClusterNetworkConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterNetworkConfigSpec
    }
}

impl VimObjectTrait for FailoverNodeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailoverNodeInfo
    }
}

impl VimObjectTrait for NodeDeploymentSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NodeDeploymentSpec
    }
}

impl VimObjectTrait for PassiveNodeDeploymentSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PassiveNodeDeploymentSpec
    }
}

impl VimObjectTrait for NodeNetworkSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NodeNetworkSpec
    }
}

impl VimObjectTrait for PassiveNodeNetworkSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PassiveNodeNetworkSpec
    }
}

impl VimObjectTrait for SourceNodeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SourceNodeSpec
    }
}

impl VimObjectTrait for VchaClusterConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterConfigInfo
    }
}

impl VimObjectTrait for VchaClusterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterConfigSpec
    }
}

impl VimObjectTrait for VchaClusterDeploymentSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterDeploymentSpec
    }
}

impl VimObjectTrait for VchaClusterNetworkSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterNetworkSpec
    }
}

impl VimObjectTrait for WitnessNodeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WitnessNodeInfo
    }
}

impl VimObjectTrait for VchaClusterHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterHealth
    }
}

impl VimObjectTrait for VchaClusterRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaClusterRuntimeInfo
    }
}

impl VimObjectTrait for VchaNodeRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VchaNodeRuntimeInfo
    }
}

impl VimObjectTrait for VirtualMachineAffinityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineAffinityInfo
    }
}

impl VimObjectTrait for VirtualMachineBaseIndependentFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBaseIndependentFilterSpec
    }
}

impl VimObjectTrait for VirtualMachineEmptyIndependentFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineEmptyIndependentFilterSpec
    }
}

impl VimObjectTrait for VirtualMachineIndependentFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineIndependentFilterSpec
    }
}

impl VimObjectTrait for VirtualMachineBootOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptions
    }
}

impl VimObjectTrait for VirtualMachineBootOptionsBootableDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptionsBootableDevice
    }
}

impl VimObjectTrait for VirtualMachineBootOptionsBootableCdromDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptionsBootableCdromDevice
    }
}

impl VimObjectTrait for VirtualMachineBootOptionsBootableDiskDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptionsBootableDiskDevice
    }
}

impl VimObjectTrait for VirtualMachineBootOptionsBootableEthernetDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptionsBootableEthernetDevice
    }
}

impl VimObjectTrait for VirtualMachineBootOptionsBootableFloppyDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineBootOptionsBootableFloppyDevice
    }
}

impl VimObjectTrait for VirtualMachineCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineCapability
    }
}

impl VimObjectTrait for VirtualMachineCertThumbprint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineCertThumbprint
    }
}

impl VimObjectTrait for VirtualMachineCloneSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineCloneSpec
    }
}

impl VimObjectTrait for VirtualMachineConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigInfo
    }
}

impl VimObjectTrait for VirtualMachineConfigInfoDatastoreUrlPair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigInfoDatastoreUrlPair
    }
}

impl VimObjectTrait for VirtualMachineConfigInfoOverheadInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigInfoOverheadInfo
    }
}

impl VimObjectTrait for VirtualMachineConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigOption
    }
}

impl VimObjectTrait for VirtualMachineConfigOptionDescriptor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigOptionDescriptor
    }
}

impl VimObjectTrait for VirtualMachineConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigSpec
    }
}

impl VimObjectTrait for ConfigTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConfigTarget
    }
}

impl VimObjectTrait for VirtualMachineConsolePreferences {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConsolePreferences
    }
}

impl VimObjectTrait for VirtualMachineContentLibraryItemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineContentLibraryItemInfo
    }
}

impl VimObjectTrait for DatastoreOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreOption
    }
}

impl VimObjectTrait for VirtualMachineDatastoreVolumeOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDatastoreVolumeOption
    }
}

impl VimObjectTrait for VirtualMachineDefaultPowerOpInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDefaultPowerOpInfo
    }
}

impl VimObjectTrait for VirtualMachineDeviceRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDeviceRuntimeInfo
    }
}

impl VimObjectTrait for VirtualMachineDeviceRuntimeInfoDeviceRuntimeState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState
    }
}

impl VimObjectTrait for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState
    }
}

impl VimObjectTrait for VirtualMachineDvxClassInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDvxClassInfo
    }
}

impl VimObjectTrait for FaultToleranceConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceConfigInfo
    }
}

impl VimObjectTrait for FaultTolerancePrimaryConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultTolerancePrimaryConfigInfo
    }
}

impl VimObjectTrait for FaultToleranceSecondaryConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceSecondaryConfigInfo
    }
}

impl VimObjectTrait for FaultToleranceConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceConfigSpec
    }
}

impl VimObjectTrait for FaultToleranceMetaSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceMetaSpec
    }
}

impl VimObjectTrait for FaultToleranceSecondaryOpResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceSecondaryOpResult
    }
}

impl VimObjectTrait for FaultToleranceVmConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceVmConfigSpec
    }
}

impl VimObjectTrait for FaultToleranceDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceDiskSpec
    }
}

impl VimObjectTrait for VirtualMachineFeatureRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFeatureRequirement
    }
}

impl VimObjectTrait for VirtualMachineFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileInfo
    }
}

impl VimObjectTrait for VirtualMachineFileLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayout
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutDiskLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutDiskLayout
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutSnapshotLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutSnapshotLayout
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutEx
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutExDiskLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutExDiskLayout
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutExDiskUnit {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutExDiskUnit
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutExFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutExFileInfo
    }
}

impl VimObjectTrait for VirtualMachineFileLayoutExSnapshotLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFileLayoutExSnapshotLayout
    }
}

impl VimObjectTrait for VirtualMachineFlagInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFlagInfo
    }
}

impl VimObjectTrait for VirtualMachineForkConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineForkConfigInfo
    }
}

impl VimObjectTrait for GuestInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestInfo
    }
}

impl VimObjectTrait for GuestInfoCustomizationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestInfoCustomizationInfo
    }
}

impl VimObjectTrait for GuestDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestDiskInfo
    }
}

impl VimObjectTrait for GuestInfoNamespaceGenerationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestInfoNamespaceGenerationInfo
    }
}

impl VimObjectTrait for GuestNicInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestNicInfo
    }
}

impl VimObjectTrait for GuestScreenInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestScreenInfo
    }
}

impl VimObjectTrait for GuestStackInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestStackInfo
    }
}

impl VimObjectTrait for GuestInfoVirtualDiskMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestInfoVirtualDiskMapping
    }
}

impl VimObjectTrait for VirtualMachineGuestIntegrityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineGuestIntegrityInfo
    }
}

impl VimObjectTrait for VirtualMachineGuestMonitoringModeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineGuestMonitoringModeInfo
    }
}

impl VimObjectTrait for GuestOsDescriptor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestOsDescriptor
    }
}

impl VimObjectTrait for VirtualMachineGuestQuiesceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineGuestQuiesceSpec
    }
}

impl VimObjectTrait for VirtualMachineWindowsQuiesceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineWindowsQuiesceSpec
    }
}

impl VimObjectTrait for VirtualMachineIdeDiskDevicePartitionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineIdeDiskDevicePartitionInfo
    }
}

impl VimObjectTrait for VirtualMachineInstantCloneSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineInstantCloneSpec
    }
}

impl VimObjectTrait for VirtualMachineLegacyNetworkSwitchInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineLegacyNetworkSwitchInfo
    }
}

impl VimObjectTrait for VirtualMachineMessage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMessage
    }
}

impl VimObjectTrait for VirtualMachineMetadataManagerVmMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMetadataManagerVmMetadata
    }
}

impl VimObjectTrait for VirtualMachineMetadataManagerVmMetadataInput {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMetadataManagerVmMetadataInput
    }
}

impl VimObjectTrait for VirtualMachineMetadataManagerVmMetadataOwner {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMetadataManagerVmMetadataOwner
    }
}

impl VimObjectTrait for VirtualMachineMetadataManagerVmMetadataResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMetadataManagerVmMetadataResult
    }
}

impl VimObjectTrait for VirtualMachineNetworkShaperInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineNetworkShaperInfo
    }
}

impl VimObjectTrait for VirtualMachineProfileDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineProfileDetails
    }
}

impl VimObjectTrait for VirtualMachineProfileDetailsDiskProfileDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineProfileDetailsDiskProfileDetails
    }
}

impl VimObjectTrait for VirtualMachineProfileRawData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineProfileRawData
    }
}

impl VimObjectTrait for VirtualMachineProfileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineProfileSpec
    }
}

impl VimObjectTrait for VirtualMachineDefaultProfileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDefaultProfileSpec
    }
}

impl VimObjectTrait for VirtualMachineDefinedProfileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDefinedProfileSpec
    }
}

impl VimObjectTrait for VirtualMachineEmptyProfileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineEmptyProfileSpec
    }
}

impl VimObjectTrait for VirtualMachinePropertyRelation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachinePropertyRelation
    }
}

impl VimObjectTrait for VirtualMachineQuestionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineQuestionInfo
    }
}

impl VimObjectTrait for VirtualMachineRelocateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineRelocateSpec
    }
}

impl VimObjectTrait for VirtualMachineRelocateSpecDiskLocator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineRelocateSpecDiskLocator
    }
}

impl VimObjectTrait for VirtualMachineRelocateSpecDiskLocatorBackingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineRelocateSpecDiskLocatorBackingSpec
    }
}

impl VimObjectTrait for ReplicationConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationConfigSpec
    }
}

impl VimObjectTrait for ReplicationInfoDiskSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationInfoDiskSettings
    }
}

impl VimObjectTrait for VirtualMachineRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineRuntimeInfo
    }
}

impl VimObjectTrait for VirtualMachineRuntimeInfoDasProtectionState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineRuntimeInfoDasProtectionState
    }
}

impl VimObjectTrait for ScheduledHardwareUpgradeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ScheduledHardwareUpgradeInfo
    }
}

impl VimObjectTrait for VirtualMachineSgxInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSgxInfo
    }
}

impl VimObjectTrait for VirtualMachineSnapshotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSnapshotInfo
    }
}

impl VimObjectTrait for VirtualMachineSnapshotTree {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSnapshotTree
    }
}

impl VimObjectTrait for VirtualMachineSriovDevicePoolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSriovDevicePoolInfo
    }
}

impl VimObjectTrait for VirtualMachineSriovNetworkDevicePoolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSriovNetworkDevicePoolInfo
    }
}

impl VimObjectTrait for VirtualMachineStorageInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineStorageInfo
    }
}

impl VimObjectTrait for VirtualMachineUsageOnDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineUsageOnDatastore
    }
}

impl VimObjectTrait for VirtualMachineSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSummary
    }
}

impl VimObjectTrait for VirtualMachineConfigSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineConfigSummary
    }
}

impl VimObjectTrait for VirtualMachineGuestSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineGuestSummary
    }
}

impl VimObjectTrait for VirtualMachineQuickStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineQuickStats
    }
}

impl VimObjectTrait for VirtualMachineQuickStatsMemoryTierStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineQuickStatsMemoryTierStats
    }
}

impl VimObjectTrait for VirtualMachineStorageSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineStorageSummary
    }
}

impl VimObjectTrait for VirtualMachineTargetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineTargetInfo
    }
}

impl VimObjectTrait for VirtualMachineCdromInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineCdromInfo
    }
}

impl VimObjectTrait for VirtualMachineDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDatastoreInfo
    }
}

impl VimObjectTrait for VirtualMachineDiskDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDiskDeviceInfo
    }
}

impl VimObjectTrait for VirtualMachineIdeDiskDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineIdeDiskDeviceInfo
    }
}

impl VimObjectTrait for VirtualMachineScsiDiskDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineScsiDiskDeviceInfo
    }
}

impl VimObjectTrait for VirtualMachineDynamicPassthroughInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineDynamicPassthroughInfo
    }
}

impl VimObjectTrait for VirtualMachineFloppyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFloppyInfo
    }
}

impl VimObjectTrait for VirtualMachineNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineNetworkInfo
    }
}

impl VimObjectTrait for OpaqueNetworkTargetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OpaqueNetworkTargetInfo
    }
}

impl VimObjectTrait for VirtualMachineParallelInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineParallelInfo
    }
}

impl VimObjectTrait for VirtualMachinePciPassthroughInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachinePciPassthroughInfo
    }
}

impl VimObjectTrait for VirtualMachineSriovInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSriovInfo
    }
}

impl VimObjectTrait for VirtualMachinePciSharedGpuPassthroughInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachinePciSharedGpuPassthroughInfo
    }
}

impl VimObjectTrait for VirtualMachinePrecisionClockInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachinePrecisionClockInfo
    }
}

impl VimObjectTrait for VirtualMachineScsiPassthroughInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineScsiPassthroughInfo
    }
}

impl VimObjectTrait for VirtualMachineSerialInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSerialInfo
    }
}

impl VimObjectTrait for VirtualMachineSgxTargetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSgxTargetInfo
    }
}

impl VimObjectTrait for VirtualMachineSoundInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineSoundInfo
    }
}

impl VimObjectTrait for VirtualMachineUsbInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineUsbInfo
    }
}

impl VimObjectTrait for VirtualMachineVFlashModuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVFlashModuleInfo
    }
}

impl VimObjectTrait for VirtualMachineVMotionStunTimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVMotionStunTimeInfo
    }
}

impl VimObjectTrait for VirtualMachineVendorDeviceGroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVendorDeviceGroupInfo
    }
}

impl VimObjectTrait for VirtualMachineVgpuDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVgpuDeviceInfo
    }
}

impl VimObjectTrait for VirtualMachineVgpuProfileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVgpuProfileInfo
    }
}

impl VimObjectTrait for ToolsConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsConfigInfo
    }
}

impl VimObjectTrait for ToolsConfigInfoToolsLastInstallInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsConfigInfoToolsLastInstallInfo
    }
}

impl VimObjectTrait for UsbScanCodeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UsbScanCodeSpec
    }
}

impl VimObjectTrait for UsbScanCodeSpecKeyEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UsbScanCodeSpecKeyEvent
    }
}

impl VimObjectTrait for UsbScanCodeSpecModifierType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UsbScanCodeSpecModifierType
    }
}

impl VimObjectTrait for VirtualMachineVcpuConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVcpuConfig
    }
}

impl VimObjectTrait for VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo
    }
}

impl VimObjectTrait for VirtualMachineVirtualDeviceGroups {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualDeviceGroups
    }
}

impl VimObjectTrait for VirtualMachineVirtualDeviceGroupsDeviceGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup
    }
}

impl VimObjectTrait for VirtualMachineVirtualDeviceGroupsVendorDeviceGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup
    }
}

impl VimObjectTrait for VirtualMachineVirtualDeviceSwap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualDeviceSwap
    }
}

impl VimObjectTrait for VirtualMachineVirtualDeviceSwapDeviceSwapInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualDeviceSwapDeviceSwapInfo
    }
}

impl VimObjectTrait for VirtualHardware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHardware
    }
}

impl VimObjectTrait for VirtualHardwareOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHardwareOption
    }
}

impl VimObjectTrait for VirtualMachineVirtualNuma {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualNuma
    }
}

impl VimObjectTrait for VirtualMachineVirtualNumaInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualNumaInfo
    }
}

impl VimObjectTrait for VirtualMachineVirtualPMem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVirtualPMem
    }
}

impl VimObjectTrait for CheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CheckResult
    }
}

impl VimObjectTrait for CustomizationAdapterMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationAdapterMapping
    }
}

impl VimObjectTrait for CustomizationGlobalIpSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationGlobalIpSettings
    }
}

impl VimObjectTrait for CustomizationGuiRunOnce {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationGuiRunOnce
    }
}

impl VimObjectTrait for CustomizationGuiUnattended {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationGuiUnattended
    }
}

impl VimObjectTrait for CustomizationIpSettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIpSettings
    }
}

impl VimObjectTrait for CustomizationIpSettingsIpV6AddressSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIpSettingsIpV6AddressSpec
    }
}

impl VimObjectTrait for CustomizationIdentification {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIdentification
    }
}

impl VimObjectTrait for CustomizationIdentitySettings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIdentitySettings
    }
}

impl VimObjectTrait for CustomizationCloudinitPrep {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationCloudinitPrep
    }
}

impl VimObjectTrait for CustomizationLinuxPrep {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationLinuxPrep
    }
}

impl VimObjectTrait for CustomizationSysprep {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSysprep
    }
}

impl VimObjectTrait for CustomizationSysprepText {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSysprepText
    }
}

impl VimObjectTrait for CustomizationIpGenerator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIpGenerator
    }
}

impl VimObjectTrait for CustomizationCustomIpGenerator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationCustomIpGenerator
    }
}

impl VimObjectTrait for CustomizationDhcpIpGenerator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationDhcpIpGenerator
    }
}

impl VimObjectTrait for CustomizationFixedIp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationFixedIp
    }
}

impl VimObjectTrait for CustomizationUnknownIpGenerator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationUnknownIpGenerator
    }
}

impl VimObjectTrait for CustomizationIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationIpV6Generator
    }
}

impl VimObjectTrait for CustomizationAutoIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationAutoIpV6Generator
    }
}

impl VimObjectTrait for CustomizationCustomIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationCustomIpV6Generator
    }
}

impl VimObjectTrait for CustomizationDhcpIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationDhcpIpV6Generator
    }
}

impl VimObjectTrait for CustomizationFixedIpV6 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationFixedIpV6
    }
}

impl VimObjectTrait for CustomizationStatelessIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationStatelessIpV6Generator
    }
}

impl VimObjectTrait for CustomizationUnknownIpV6Generator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationUnknownIpV6Generator
    }
}

impl VimObjectTrait for CustomizationLicenseFilePrintData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationLicenseFilePrintData
    }
}

impl VimObjectTrait for CustomizationName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationName
    }
}

impl VimObjectTrait for CustomizationCustomName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationCustomName
    }
}

impl VimObjectTrait for CustomizationFixedName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationFixedName
    }
}

impl VimObjectTrait for CustomizationPrefixName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationPrefixName
    }
}

impl VimObjectTrait for CustomizationUnknownName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationUnknownName
    }
}

impl VimObjectTrait for CustomizationVirtualMachineName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationVirtualMachineName
    }
}

impl VimObjectTrait for CustomizationOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationOptions
    }
}

impl VimObjectTrait for CustomizationLinuxOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationLinuxOptions
    }
}

impl VimObjectTrait for CustomizationWinOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationWinOptions
    }
}

impl VimObjectTrait for CustomizationPassword {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationPassword
    }
}

impl VimObjectTrait for CustomizationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationSpec
    }
}

impl VimObjectTrait for CustomizationUserData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationUserData
    }
}

impl VimObjectTrait for HostDiskMappingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskMappingInfo
    }
}

impl VimObjectTrait for HostDiskMappingPartitionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskMappingPartitionInfo
    }
}

impl VimObjectTrait for HostDiskMappingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskMappingOption
    }
}

impl VimObjectTrait for HostDiskMappingPartitionOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostDiskMappingPartitionOption
    }
}

impl VimObjectTrait for VirtualDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDevice
    }
}

impl VimObjectTrait for VirtualCdrom {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdrom
    }
}

impl VimObjectTrait for VirtualController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualController
    }
}

impl VimObjectTrait for VirtualIdeController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualIdeController
    }
}

impl VimObjectTrait for VirtualNvdimmController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvdimmController
    }
}

impl VimObjectTrait for VirtualNvmeController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvmeController
    }
}

impl VimObjectTrait for VirtualPciController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciController
    }
}

impl VimObjectTrait for VirtualPs2Controller {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPs2Controller
    }
}

impl VimObjectTrait for VirtualSataController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSataController
    }
}

impl VimObjectTrait for VirtualAhciController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualAhciController
    }
}

impl VimObjectTrait for VirtualScsiController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiController
    }
}

impl VimObjectTrait for ParaVirtualScsiController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ParaVirtualScsiController
    }
}

impl VimObjectTrait for VirtualBusLogicController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualBusLogicController
    }
}

impl VimObjectTrait for VirtualLsiLogicController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualLsiLogicController
    }
}

impl VimObjectTrait for VirtualLsiLogicSasController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualLsiLogicSasController
    }
}

impl VimObjectTrait for VirtualSioController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSioController
    }
}

impl VimObjectTrait for VirtualUsbController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbController
    }
}

impl VimObjectTrait for VirtualUsbxhciController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbxhciController
    }
}

impl VimObjectTrait for VirtualDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDisk
    }
}

impl VimObjectTrait for VirtualEthernetCard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCard
    }
}

impl VimObjectTrait for VirtualE1000 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualE1000
    }
}

impl VimObjectTrait for VirtualE1000E {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualE1000E
    }
}

impl VimObjectTrait for VirtualPcNet32 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPcNet32
    }
}

impl VimObjectTrait for VirtualSriovEthernetCard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSriovEthernetCard
    }
}

impl VimObjectTrait for VirtualVmxnet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet
    }
}

impl VimObjectTrait for VirtualVmxnet2 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet2
    }
}

impl VimObjectTrait for VirtualVmxnet3 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3
    }
}

impl VimObjectTrait for VirtualVmxnet3Vrdma {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3Vrdma
    }
}

impl VimObjectTrait for VirtualFloppy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppy
    }
}

impl VimObjectTrait for VirtualKeyboard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualKeyboard
    }
}

impl VimObjectTrait for VirtualNvdimm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvdimm
    }
}

impl VimObjectTrait for VirtualPciPassthrough {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthrough
    }
}

impl VimObjectTrait for VirtualParallelPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPort
    }
}

impl VimObjectTrait for VirtualPointingDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPointingDevice
    }
}

impl VimObjectTrait for VirtualPrecisionClock {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPrecisionClock
    }
}

impl VimObjectTrait for VirtualScsiPassthrough {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiPassthrough
    }
}

impl VimObjectTrait for VirtualSerialPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPort
    }
}

impl VimObjectTrait for VirtualSoundCard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundCard
    }
}

impl VimObjectTrait for VirtualEnsoniq1371 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEnsoniq1371
    }
}

impl VimObjectTrait for VirtualHdAudioCard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHdAudioCard
    }
}

impl VimObjectTrait for VirtualSoundBlaster16 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundBlaster16
    }
}

impl VimObjectTrait for VirtualTpm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualTpm
    }
}

impl VimObjectTrait for VirtualUsb {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsb
    }
}

impl VimObjectTrait for VirtualMachineVmciDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmciDevice
    }
}

impl VimObjectTrait for VirtualMachineVmirom {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmirom
    }
}

impl VimObjectTrait for VirtualMachineVideoCard {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVideoCard
    }
}

impl VimObjectTrait for VirtualWdt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualWdt
    }
}

impl VimObjectTrait for VirtualDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualDeviceDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualCdromAtapiBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromAtapiBackingInfo
    }
}

impl VimObjectTrait for VirtualCdromPassthroughBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromPassthroughBackingInfo
    }
}

impl VimObjectTrait for VirtualDiskRawDiskVer2BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskRawDiskVer2BackingInfo
    }
}

impl VimObjectTrait for VirtualDiskPartitionedRawDiskVer2BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo
    }
}

impl VimObjectTrait for VirtualEthernetCardLegacyNetworkBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardLegacyNetworkBackingInfo
    }
}

impl VimObjectTrait for VirtualEthernetCardNetworkBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardNetworkBackingInfo
    }
}

impl VimObjectTrait for VirtualFloppyDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualPciPassthroughDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualPciPassthroughDynamicBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDynamicBackingInfo
    }
}

impl VimObjectTrait for VirtualParallelPortDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPortDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualPointingDeviceDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPointingDeviceDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualScsiPassthroughDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiPassthroughDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualSerialPortDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualSoundCardDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundCardDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualUsbRemoteHostBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbRemoteHostBackingInfo
    }
}

impl VimObjectTrait for VirtualUsbusbBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbusbBackingInfo
    }
}

impl VimObjectTrait for VirtualDeviceFileBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceFileBackingInfo
    }
}

impl VimObjectTrait for VirtualCdromIsoBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromIsoBackingInfo
    }
}

impl VimObjectTrait for VirtualDiskFlatVer1BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskFlatVer1BackingInfo
    }
}

impl VimObjectTrait for VirtualDiskFlatVer2BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskFlatVer2BackingInfo
    }
}

impl VimObjectTrait for VirtualDiskLocalPMemBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskLocalPMemBackingInfo
    }
}

impl VimObjectTrait for VirtualDiskRawDiskMappingVer1BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskRawDiskMappingVer1BackingInfo
    }
}

impl VimObjectTrait for VirtualDiskSeSparseBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSeSparseBackingInfo
    }
}

impl VimObjectTrait for VirtualDiskSparseVer1BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSparseVer1BackingInfo
    }
}

impl VimObjectTrait for VirtualDiskSparseVer2BackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSparseVer2BackingInfo
    }
}

impl VimObjectTrait for VirtualFloppyImageBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyImageBackingInfo
    }
}

impl VimObjectTrait for VirtualNvdimmBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvdimmBackingInfo
    }
}

impl VimObjectTrait for VirtualParallelPortFileBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPortFileBackingInfo
    }
}

impl VimObjectTrait for VirtualSerialPortFileBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortFileBackingInfo
    }
}

impl VimObjectTrait for VirtualDevicePipeBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDevicePipeBackingInfo
    }
}

impl VimObjectTrait for VirtualSerialPortPipeBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortPipeBackingInfo
    }
}

impl VimObjectTrait for VirtualDeviceRemoteDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceRemoteDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualCdromRemoteAtapiBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromRemoteAtapiBackingInfo
    }
}

impl VimObjectTrait for VirtualCdromRemotePassthroughBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromRemotePassthroughBackingInfo
    }
}

impl VimObjectTrait for VirtualFloppyRemoteDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyRemoteDeviceBackingInfo
    }
}

impl VimObjectTrait for VirtualUsbRemoteClientBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbRemoteClientBackingInfo
    }
}

impl VimObjectTrait for VirtualDeviceUriBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceUriBackingInfo
    }
}

impl VimObjectTrait for VirtualSerialPortUriBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortUriBackingInfo
    }
}

impl VimObjectTrait for VirtualEthernetCardDistributedVirtualPortBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo
    }
}

impl VimObjectTrait for VirtualEthernetCardOpaqueNetworkBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardOpaqueNetworkBackingInfo
    }
}

impl VimObjectTrait for VirtualPciPassthroughDvxBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDvxBackingInfo
    }
}

impl VimObjectTrait for VirtualPciPassthroughPluginBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughPluginBackingInfo
    }
}

impl VimObjectTrait for VirtualPciPassthroughVmiopBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughVmiopBackingInfo
    }
}

impl VimObjectTrait for VirtualPrecisionClockSystemClockBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPrecisionClockSystemClockBackingInfo
    }
}

impl VimObjectTrait for VirtualSerialPortThinPrintBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortThinPrintBackingInfo
    }
}

impl VimObjectTrait for VirtualSriovEthernetCardSriovBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSriovEthernetCardSriovBackingInfo
    }
}

impl VimObjectTrait for VirtualDeviceBusSlotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceBusSlotInfo
    }
}

impl VimObjectTrait for VirtualDevicePciBusSlotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDevicePciBusSlotInfo
    }
}

impl VimObjectTrait for VirtualUsbControllerPciBusSlotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbControllerPciBusSlotInfo
    }
}

impl VimObjectTrait for VirtualDeviceConnectInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceConnectInfo
    }
}

impl VimObjectTrait for VirtualDeviceDeviceGroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceDeviceGroupInfo
    }
}

impl VimObjectTrait for VirtualDeviceOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceOption
    }
}

impl VimObjectTrait for VirtualCdromOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromOption
    }
}

impl VimObjectTrait for VirtualControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualControllerOption
    }
}

impl VimObjectTrait for VirtualIdeControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualIdeControllerOption
    }
}

impl VimObjectTrait for VirtualNvdimmControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvdimmControllerOption
    }
}

impl VimObjectTrait for VirtualNvmeControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvmeControllerOption
    }
}

impl VimObjectTrait for VirtualPciControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciControllerOption
    }
}

impl VimObjectTrait for VirtualPs2ControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPs2ControllerOption
    }
}

impl VimObjectTrait for VirtualSataControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSataControllerOption
    }
}

impl VimObjectTrait for VirtualAhciControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualAhciControllerOption
    }
}

impl VimObjectTrait for VirtualScsiControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiControllerOption
    }
}

impl VimObjectTrait for ParaVirtualScsiControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ParaVirtualScsiControllerOption
    }
}

impl VimObjectTrait for VirtualBusLogicControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualBusLogicControllerOption
    }
}

impl VimObjectTrait for VirtualLsiLogicControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualLsiLogicControllerOption
    }
}

impl VimObjectTrait for VirtualLsiLogicSasControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualLsiLogicSasControllerOption
    }
}

impl VimObjectTrait for VirtualSioControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSioControllerOption
    }
}

impl VimObjectTrait for VirtualUsbControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbControllerOption
    }
}

impl VimObjectTrait for VirtualUsbxhciControllerOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbxhciControllerOption
    }
}

impl VimObjectTrait for VirtualDiskOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskOption
    }
}

impl VimObjectTrait for VirtualEthernetCardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardOption
    }
}

impl VimObjectTrait for VirtualE1000Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualE1000Option
    }
}

impl VimObjectTrait for VirtualE1000EOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualE1000EOption
    }
}

impl VimObjectTrait for VirtualPcNet32Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPcNet32Option
    }
}

impl VimObjectTrait for VirtualSriovEthernetCardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSriovEthernetCardOption
    }
}

impl VimObjectTrait for VirtualVmxnetOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnetOption
    }
}

impl VimObjectTrait for VirtualVmxnet2Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet2Option
    }
}

impl VimObjectTrait for VirtualVmxnet3Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3Option
    }
}

impl VimObjectTrait for VirtualVmxnet3VrdmaOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3VrdmaOption
    }
}

impl VimObjectTrait for VirtualFloppyOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyOption
    }
}

impl VimObjectTrait for VirtualKeyboardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualKeyboardOption
    }
}

impl VimObjectTrait for VirtualNvdimmOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualNvdimmOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughOption
    }
}

impl VimObjectTrait for VirtualParallelPortOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPortOption
    }
}

impl VimObjectTrait for VirtualPointingDeviceOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPointingDeviceOption
    }
}

impl VimObjectTrait for VirtualPrecisionClockOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPrecisionClockOption
    }
}

impl VimObjectTrait for VirtualScsiPassthroughOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiPassthroughOption
    }
}

impl VimObjectTrait for VirtualSerialPortOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortOption
    }
}

impl VimObjectTrait for VirtualSoundCardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundCardOption
    }
}

impl VimObjectTrait for VirtualEnsoniq1371Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEnsoniq1371Option
    }
}

impl VimObjectTrait for VirtualHdAudioCardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHdAudioCardOption
    }
}

impl VimObjectTrait for VirtualSoundBlaster16Option {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundBlaster16Option
    }
}

impl VimObjectTrait for VirtualTpmOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualTpmOption
    }
}

impl VimObjectTrait for VirtualUsbOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbOption
    }
}

impl VimObjectTrait for VirtualMachineVmciDeviceOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmciDeviceOption
    }
}

impl VimObjectTrait for VirtualVmiromOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmiromOption
    }
}

impl VimObjectTrait for VirtualVideoCardOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVideoCardOption
    }
}

impl VimObjectTrait for VirtualWdtOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualWdtOption
    }
}

impl VimObjectTrait for VirtualDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualDeviceDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualCdromAtapiBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromAtapiBackingOption
    }
}

impl VimObjectTrait for VirtualCdromPassthroughBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromPassthroughBackingOption
    }
}

impl VimObjectTrait for VirtualCdromRemoteAtapiBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromRemoteAtapiBackingOption
    }
}

impl VimObjectTrait for VirtualDiskRawDiskMappingVer1BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskRawDiskMappingVer1BackingOption
    }
}

impl VimObjectTrait for VirtualDiskRawDiskVer2BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskRawDiskVer2BackingOption
    }
}

impl VimObjectTrait for VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskPartitionedRawDiskVer2BackingOption
    }
}

impl VimObjectTrait for VirtualEthernetCardLegacyNetworkBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardLegacyNetworkBackingOption
    }
}

impl VimObjectTrait for VirtualEthernetCardNetworkBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardNetworkBackingOption
    }
}

impl VimObjectTrait for VirtualFloppyDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughDynamicBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDynamicBackingOption
    }
}

impl VimObjectTrait for VirtualParallelPortDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPortDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualPointingDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPointingDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualScsiPassthroughDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualScsiPassthroughDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualSerialPortDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualSoundCardDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSoundCardDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualUsbRemoteHostBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbRemoteHostBackingOption
    }
}

impl VimObjectTrait for VirtualUsbusbBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbusbBackingOption
    }
}

impl VimObjectTrait for VirtualDeviceFileBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceFileBackingOption
    }
}

impl VimObjectTrait for VirtualCdromIsoBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromIsoBackingOption
    }
}

impl VimObjectTrait for VirtualDiskFlatVer1BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskFlatVer1BackingOption
    }
}

impl VimObjectTrait for VirtualDiskFlatVer2BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskFlatVer2BackingOption
    }
}

impl VimObjectTrait for VirtualDiskLocalPMemBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskLocalPMemBackingOption
    }
}

impl VimObjectTrait for VirtualDiskSeSparseBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSeSparseBackingOption
    }
}

impl VimObjectTrait for VirtualDiskSparseVer1BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSparseVer1BackingOption
    }
}

impl VimObjectTrait for VirtualDiskSparseVer2BackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskSparseVer2BackingOption
    }
}

impl VimObjectTrait for VirtualFloppyImageBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyImageBackingOption
    }
}

impl VimObjectTrait for VirtualParallelPortFileBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualParallelPortFileBackingOption
    }
}

impl VimObjectTrait for VirtualSerialPortFileBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortFileBackingOption
    }
}

impl VimObjectTrait for VirtualDevicePipeBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDevicePipeBackingOption
    }
}

impl VimObjectTrait for VirtualSerialPortPipeBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortPipeBackingOption
    }
}

impl VimObjectTrait for VirtualDeviceRemoteDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceRemoteDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualCdromRemotePassthroughBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualCdromRemotePassthroughBackingOption
    }
}

impl VimObjectTrait for VirtualFloppyRemoteDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualFloppyRemoteDeviceBackingOption
    }
}

impl VimObjectTrait for VirtualUsbRemoteClientBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualUsbRemoteClientBackingOption
    }
}

impl VimObjectTrait for VirtualDeviceUriBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceUriBackingOption
    }
}

impl VimObjectTrait for VirtualSerialPortUriBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortUriBackingOption
    }
}

impl VimObjectTrait for VirtualEthernetCardDvPortBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardDvPortBackingOption
    }
}

impl VimObjectTrait for VirtualEthernetCardOpaqueNetworkBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardOpaqueNetworkBackingOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughDvxBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughDvxBackingOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughPluginBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughPluginBackingOption
    }
}

impl VimObjectTrait for VirtualPciPassthroughVmiopBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughVmiopBackingOption
    }
}

impl VimObjectTrait for VirtualPrecisionClockSystemClockBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPrecisionClockSystemClockBackingOption
    }
}

impl VimObjectTrait for VirtualSerialPortThinPrintBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSerialPortThinPrintBackingOption
    }
}

impl VimObjectTrait for VirtualSriovEthernetCardSriovBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualSriovEthernetCardSriovBackingOption
    }
}

impl VimObjectTrait for VirtualDeviceBusSlotOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceBusSlotOption
    }
}

impl VimObjectTrait for VirtualDeviceConnectOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceConnectOption
    }
}

impl VimObjectTrait for VirtualDeviceConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceConfigSpec
    }
}

impl VimObjectTrait for VirtualDiskConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskConfigSpec
    }
}

impl VimObjectTrait for VirtualDeviceConfigSpecBackingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDeviceConfigSpecBackingSpec
    }
}

impl VimObjectTrait for VirtualDiskVFlashCacheConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskVFlashCacheConfigInfo
    }
}

impl VimObjectTrait for VirtualDiskId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskId
    }
}

impl VimObjectTrait for VirtualDiskDeltaDiskFormatsSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskDeltaDiskFormatsSupported
    }
}

impl VimObjectTrait for VirtualDiskOptionVFlashCacheConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskOptionVFlashCacheConfigOption
    }
}

impl VimObjectTrait for VirtualEthernetCardResourceAllocation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardResourceAllocation
    }
}

impl VimObjectTrait for VirtualPciPassthroughAllowedDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualPciPassthroughAllowedDevice
    }
}

impl VimObjectTrait for VirtualMachineVmciDeviceFilterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmciDeviceFilterInfo
    }
}

impl VimObjectTrait for VirtualMachineVmciDeviceFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmciDeviceFilterSpec
    }
}

impl VimObjectTrait for VirtualMachineVmciDeviceOptionFilterSpecOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineVmciDeviceOptionFilterSpecOption
    }
}

impl VimObjectTrait for GuestAliases {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAliases
    }
}

impl VimObjectTrait for GuestAuthAliasInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthAliasInfo
    }
}

impl VimObjectTrait for GuestAuthSubject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthSubject
    }
}

impl VimObjectTrait for GuestAuthAnySubject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthAnySubject
    }
}

impl VimObjectTrait for GuestAuthNamedSubject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthNamedSubject
    }
}

impl VimObjectTrait for GuestMappedAliases {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestMappedAliases
    }
}

impl VimObjectTrait for GuestFileAttributes {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestFileAttributes
    }
}

impl VimObjectTrait for GuestPosixFileAttributes {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestPosixFileAttributes
    }
}

impl VimObjectTrait for GuestWindowsFileAttributes {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestWindowsFileAttributes
    }
}

impl VimObjectTrait for GuestFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestFileInfo
    }
}

impl VimObjectTrait for FileTransferInformation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileTransferInformation
    }
}

impl VimObjectTrait for GuestListFileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestListFileInfo
    }
}

impl VimObjectTrait for GuestAuthentication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthentication
    }
}

impl VimObjectTrait for NamePasswordAuthentication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NamePasswordAuthentication
    }
}

impl VimObjectTrait for SamlTokenAuthentication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SamlTokenAuthentication
    }
}

impl VimObjectTrait for SspiAuthentication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SspiAuthentication
    }
}

impl VimObjectTrait for TicketedSessionAuthentication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TicketedSessionAuthentication
    }
}

impl VimObjectTrait for GuestProcessInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestProcessInfo
    }
}

impl VimObjectTrait for GuestProgramSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestProgramSpec
    }
}

impl VimObjectTrait for GuestWindowsProgramSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestWindowsProgramSpec
    }
}

impl VimObjectTrait for GuestRegKeySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegKeySpec
    }
}

impl VimObjectTrait for GuestRegKeyNameSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegKeyNameSpec
    }
}

impl VimObjectTrait for GuestRegKeyRecordSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegKeyRecordSpec
    }
}

impl VimObjectTrait for GuestRegValueSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueSpec
    }
}

impl VimObjectTrait for GuestRegValueDataSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueDataSpec
    }
}

impl VimObjectTrait for GuestRegValueBinarySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueBinarySpec
    }
}

impl VimObjectTrait for GuestRegValueDwordSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueDwordSpec
    }
}

impl VimObjectTrait for GuestRegValueExpandStringSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueExpandStringSpec
    }
}

impl VimObjectTrait for GuestRegValueMultiStringSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueMultiStringSpec
    }
}

impl VimObjectTrait for GuestRegValueQwordSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueQwordSpec
    }
}

impl VimObjectTrait for GuestRegValueStringSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueStringSpec
    }
}

impl VimObjectTrait for GuestRegValueNameSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegValueNameSpec
    }
}

impl VimObjectTrait for DeviceGroupId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceGroupId
    }
}

impl VimObjectTrait for FaultDomainId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultDomainId
    }
}

impl VimObjectTrait for ReplicationGroupId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationGroupId
    }
}

impl VimObjectTrait for ReplicationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationSpec
    }
}

impl VimObjectTrait for VsanClusterConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterConfigInfo
    }
}

impl VimObjectTrait for VsanClusterConfigInfoHostDefaultInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterConfigInfoHostDefaultInfo
    }
}

impl VimObjectTrait for VsanHostClusterStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClusterStatus
    }
}

impl VimObjectTrait for VsanHostClusterStatusState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClusterStatusState
    }
}

impl VimObjectTrait for VsanHostClusterStatusStateCompletionEstimate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClusterStatusStateCompletionEstimate
    }
}

impl VimObjectTrait for VsanHostConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfo
    }
}

impl VimObjectTrait for VsanHostConfigInfoClusterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfoClusterInfo
    }
}

impl VimObjectTrait for VsanHostFaultDomainInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostFaultDomainInfo
    }
}

impl VimObjectTrait for VsanHostConfigInfoNetworkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfoNetworkInfo
    }
}

impl VimObjectTrait for VsanHostConfigInfoNetworkInfoPortConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfoNetworkInfoPortConfig
    }
}

impl VimObjectTrait for VsanHostConfigInfoStorageInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfoStorageInfo
    }
}

impl VimObjectTrait for VsanHostDecommissionMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDecommissionMode
    }
}

impl VimObjectTrait for VsanHostDiskMapInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDiskMapInfo
    }
}

impl VimObjectTrait for VsanHostDiskMapResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDiskMapResult
    }
}

impl VimObjectTrait for VsanHostDiskMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDiskMapping
    }
}

impl VimObjectTrait for VsanHostDiskResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDiskResult
    }
}

impl VimObjectTrait for VsanHostIpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostIpConfig
    }
}

impl VimObjectTrait for VsanHostMembershipInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostMembershipInfo
    }
}

impl VimObjectTrait for VsanHostVsanDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostVsanDiskInfo
    }
}

impl VimObjectTrait for VsanHostRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostRuntimeInfo
    }
}

impl VimObjectTrait for VsanHostRuntimeInfoDiskIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostRuntimeInfoDiskIssue
    }
}

impl VimObjectTrait for BaseConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BaseConfigInfo
    }
}

impl VimObjectTrait for VStorageObjectConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectConfigInfo
    }
}

impl VimObjectTrait for BaseConfigInfoBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BaseConfigInfoBackingInfo
    }
}

impl VimObjectTrait for BaseConfigInfoFileBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BaseConfigInfoFileBackingInfo
    }
}

impl VimObjectTrait for BaseConfigInfoDiskFileBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BaseConfigInfoDiskFileBackingInfo
    }
}

impl VimObjectTrait for BaseConfigInfoRawDiskMappingBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BaseConfigInfoRawDiskMappingBackingInfo
    }
}

impl VimObjectTrait for VslmCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmCreateSpec
    }
}

impl VimObjectTrait for VslmCreateSpecBackingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmCreateSpecBackingSpec
    }
}

impl VimObjectTrait for VslmCreateSpecDiskFileBackingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmCreateSpecDiskFileBackingSpec
    }
}

impl VimObjectTrait for VslmCreateSpecRawDiskMappingBackingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmCreateSpecRawDiskMappingBackingSpec
    }
}

impl VimObjectTrait for DiskCryptoSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskCryptoSpec
    }
}

impl VimObjectTrait for Id {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Id
    }
}

impl VimObjectTrait for VslmInfrastructureObjectPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmInfrastructureObjectPolicy
    }
}

impl VimObjectTrait for VslmInfrastructureObjectPolicySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmInfrastructureObjectPolicySpec
    }
}

impl VimObjectTrait for VslmMigrateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmMigrateSpec
    }
}

impl VimObjectTrait for VslmCloneSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmCloneSpec
    }
}

impl VimObjectTrait for VslmRelocateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmRelocateSpec
    }
}

impl VimObjectTrait for VStorageObjectStateInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectStateInfo
    }
}

impl VimObjectTrait for VslmTagEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTagEntry
    }
}

impl VimObjectTrait for VslmVClockInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVClockInfo
    }
}

impl VimObjectTrait for VStorageObject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObject
    }
}

impl VimObjectTrait for VStorageObjectSnapshot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectSnapshot
    }
}

impl VimObjectTrait for VStorageObjectSnapshotDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectSnapshotDetails
    }
}

impl VimObjectTrait for VStorageObjectSnapshotInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectSnapshotInfo
    }
}

impl VimObjectTrait for VStorageObjectSnapshotInfoVStorageObjectSnapshot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectSnapshotInfoVStorageObjectSnapshot
    }
}

impl VimObjectTrait for RetrieveVStorageObjSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RetrieveVStorageObjSpec
    }
}

impl VimObjectTrait for VStorageObjectAssociations {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectAssociations
    }
}

impl VimObjectTrait for VStorageObjectAssociationsVmDiskAssociations {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectAssociationsVmDiskAssociations
    }
}

impl VimObjectTrait for DynamicArray {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DynamicArray
    }
}

impl VimObjectTrait for DynamicProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DynamicProperty
    }
}

impl VimObjectTrait for KeyAnyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KeyAnyValue
    }
}

impl VimObjectTrait for LocalizableMessage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalizableMessage
    }
}

impl VimObjectTrait for LocalizedMethodFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LocalizedMethodFault
    }
}

impl VimObjectTrait for PropertyChange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PropertyChange
    }
}

impl VimObjectTrait for PropertyFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PropertyFilterSpec
    }
}

impl VimObjectTrait for PropertyFilterUpdate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PropertyFilterUpdate
    }
}

impl VimObjectTrait for MissingObject {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingObject
    }
}

impl VimObjectTrait for MissingProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingProperty
    }
}

impl VimObjectTrait for ObjectContent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ObjectContent
    }
}

impl VimObjectTrait for ObjectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ObjectSpec
    }
}

impl VimObjectTrait for ObjectUpdate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ObjectUpdate
    }
}

impl VimObjectTrait for PropertySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PropertySpec
    }
}

impl VimObjectTrait for RetrieveOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RetrieveOptions
    }
}

impl VimObjectTrait for RetrieveResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RetrieveResult
    }
}

impl VimObjectTrait for SelectionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SelectionSpec
    }
}

impl VimObjectTrait for TraversalSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TraversalSpec
    }
}

impl VimObjectTrait for UpdateSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UpdateSet
    }
}

impl VimObjectTrait for WaitOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WaitOptions
    }
}

impl VimObjectTrait for MethodFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodFault
    }
}

impl VimObjectTrait for VimFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimFault
    }
}

impl VimObjectTrait for ActiveDirectoryFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ActiveDirectoryFault
    }
}

impl VimObjectTrait for DomainNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DomainNotFound
    }
}

impl VimObjectTrait for InvalidCamServer {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidCamServer
    }
}

impl VimObjectTrait for CamServerRefusedConnection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CamServerRefusedConnection
    }
}

impl VimObjectTrait for InvalidCamCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidCamCertificate
    }
}

impl VimObjectTrait for NoPermissionOnAd {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPermissionOnAd
    }
}

impl VimObjectTrait for NonAdUserRequired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NonAdUserRequired
    }
}

impl VimObjectTrait for AlreadyExists {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlreadyExists
    }
}

impl VimObjectTrait for AlreadyUpgraded {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlreadyUpgraded
    }
}

impl VimObjectTrait for AnswerFileUpdateFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AnswerFileUpdateFailed
    }
}

impl VimObjectTrait for AuthMinimumAdminPermission {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthMinimumAdminPermission
    }
}

impl VimObjectTrait for CannotAccessLocalSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessLocalSource
    }
}

impl VimObjectTrait for CannotDisconnectHostWithFaultToleranceVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotDisconnectHostWithFaultToleranceVm
    }
}

impl VimObjectTrait for CannotEnableVmcpForCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotEnableVmcpForCluster
    }
}

impl VimObjectTrait for CannotMoveFaultToleranceVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotMoveFaultToleranceVm
    }
}

impl VimObjectTrait for CannotMoveHostWithFaultToleranceVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotMoveHostWithFaultToleranceVm
    }
}

impl VimObjectTrait for CannotPlaceWithoutPrerequisiteMoves {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotPlaceWithoutPrerequisiteMoves
    }
}

impl VimObjectTrait for ConcurrentAccess {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConcurrentAccess
    }
}

impl VimObjectTrait for CustomizationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationFault
    }
}

impl VimObjectTrait for CannotDecryptPasswords {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotDecryptPasswords
    }
}

impl VimObjectTrait for CustomizationPending {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationPending
    }
}

impl VimObjectTrait for IpHostnameGeneratorError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IpHostnameGeneratorError
    }
}

impl VimObjectTrait for LinuxVolumeNotClean {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LinuxVolumeNotClean
    }
}

impl VimObjectTrait for MissingLinuxCustResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingLinuxCustResources
    }
}

impl VimObjectTrait for MissingWindowsCustResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingWindowsCustResources
    }
}

impl VimObjectTrait for MountError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MountError
    }
}

impl VimObjectTrait for NicSettingMismatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NicSettingMismatch
    }
}

impl VimObjectTrait for NoDisksToCustomize {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDisksToCustomize
    }
}

impl VimObjectTrait for UncustomizableGuest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UncustomizableGuest
    }
}

impl VimObjectTrait for UnexpectedCustomizationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnexpectedCustomizationFault
    }
}

impl VimObjectTrait for VolumeEditorError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VolumeEditorError
    }
}

impl VimObjectTrait for DasConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DasConfigFault
    }
}

impl VimObjectTrait for DrsDisabledOnVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsDisabledOnVm
    }
}

impl VimObjectTrait for DuplicateName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DuplicateName
    }
}

impl VimObjectTrait for DvsFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFault
    }
}

impl VimObjectTrait for BackupBlobReadFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BackupBlobReadFailure
    }
}

impl VimObjectTrait for BackupBlobWriteFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BackupBlobWriteFailure
    }
}

impl VimObjectTrait for CollectorAddressUnset {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CollectorAddressUnset
    }
}

impl VimObjectTrait for ConflictingConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConflictingConfiguration
    }
}

impl VimObjectTrait for DvsApplyOperationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsApplyOperationFault
    }
}

impl VimObjectTrait for DvsNotAuthorized {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsNotAuthorized
    }
}

impl VimObjectTrait for DvsOperationBulkFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsOperationBulkFault
    }
}

impl VimObjectTrait for DvsScopeViolated {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsScopeViolated
    }
}

impl VimObjectTrait for ImportHostAddFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ImportHostAddFailure
    }
}

impl VimObjectTrait for ImportOperationBulkFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ImportOperationBulkFault
    }
}

impl VimObjectTrait for InvalidIpfixConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidIpfixConfig
    }
}

impl VimObjectTrait for RollbackFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RollbackFailure
    }
}

impl VimObjectTrait for SwitchIpUnset {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SwitchIpUnset
    }
}

impl VimObjectTrait for SwitchNotInUpgradeMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SwitchNotInUpgradeMode
    }
}

impl VimObjectTrait for VspanDestPortConflict {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanDestPortConflict
    }
}

impl VimObjectTrait for VspanPortConflict {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPortConflict
    }
}

impl VimObjectTrait for VspanPortMoveFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPortMoveFault
    }
}

impl VimObjectTrait for VspanPortPromiscChangeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPortPromiscChangeFault
    }
}

impl VimObjectTrait for VspanPortgroupPromiscChangeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPortgroupPromiscChangeFault
    }
}

impl VimObjectTrait for VspanPortgroupTypeChangeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPortgroupTypeChangeFault
    }
}

impl VimObjectTrait for VspanPromiscuousPortNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanPromiscuousPortNotSupported
    }
}

impl VimObjectTrait for VspanSameSessionPortConflict {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VspanSameSessionPortConflict
    }
}

impl VimObjectTrait for EvcConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcConfigFault
    }
}

impl VimObjectTrait for ActiveVMsBlockingEvc {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ActiveVMsBlockingEvc
    }
}

impl VimObjectTrait for DisconnectedHostsBlockingEvc {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisconnectedHostsBlockingEvc
    }
}

impl VimObjectTrait for EvcModeIllegalByVendor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcModeIllegalByVendor
    }
}

impl VimObjectTrait for EvcModeUnsupportedByHosts {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcModeUnsupportedByHosts
    }
}

impl VimObjectTrait for EvcUnsupportedByHostHardware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcUnsupportedByHostHardware
    }
}

impl VimObjectTrait for EvcUnsupportedByHostSoftware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcUnsupportedByHostSoftware
    }
}

impl VimObjectTrait for HeterogenousHostsBlockingEvc {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HeterogenousHostsBlockingEvc
    }
}

impl VimObjectTrait for ExtendedFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtendedFault
    }
}

impl VimObjectTrait for FaultToleranceVmNotDasProtected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceVmNotDasProtected
    }
}

impl VimObjectTrait for FcoeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeFault
    }
}

impl VimObjectTrait for FcoeFaultPnicHasNoPortSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeFaultPnicHasNoPortSet
    }
}

impl VimObjectTrait for FileFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileFault
    }
}

impl VimObjectTrait for CannotAccessFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessFile
    }
}

impl VimObjectTrait for CannotCreateFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotCreateFile
    }
}

impl VimObjectTrait for CannotDeleteFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotDeleteFile
    }
}

impl VimObjectTrait for DirectoryNotEmpty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectoryNotEmpty
    }
}

impl VimObjectTrait for FileAlreadyExists {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileAlreadyExists
    }
}

impl VimObjectTrait for FileLocked {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileLocked
    }
}

impl VimObjectTrait for FileNameTooLong {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileNameTooLong
    }
}

impl VimObjectTrait for FileNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileNotFound
    }
}

impl VimObjectTrait for FileNotWritable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileNotWritable
    }
}

impl VimObjectTrait for FileTooLarge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileTooLarge
    }
}

impl VimObjectTrait for IncorrectFileType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncorrectFileType
    }
}

impl VimObjectTrait for NetworkCopyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkCopyFault
    }
}

impl VimObjectTrait for NoDiskSpace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDiskSpace
    }
}

impl VimObjectTrait for NotADirectory {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotADirectory
    }
}

impl VimObjectTrait for NotAFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotAFile
    }
}

impl VimObjectTrait for TooManyConcurrentNativeClones {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyConcurrentNativeClones
    }
}

impl VimObjectTrait for TooManyNativeCloneLevels {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyNativeCloneLevels
    }
}

impl VimObjectTrait for TooManyNativeClonesOnFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyNativeClonesOnFile
    }
}

impl VimObjectTrait for GenericDrsFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GenericDrsFault
    }
}

impl VimObjectTrait for GuestOperationsFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestOperationsFault
    }
}

impl VimObjectTrait for GuestAuthenticationChallenge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestAuthenticationChallenge
    }
}

impl VimObjectTrait for GuestComponentsOutOfDate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestComponentsOutOfDate
    }
}

impl VimObjectTrait for GuestMultipleMappings {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestMultipleMappings
    }
}

impl VimObjectTrait for GuestOperationsUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestOperationsUnavailable
    }
}

impl VimObjectTrait for GuestPermissionDenied {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestPermissionDenied
    }
}

impl VimObjectTrait for GuestProcessNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestProcessNotFound
    }
}

impl VimObjectTrait for GuestRegistryFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryFault
    }
}

impl VimObjectTrait for GuestRegistryKeyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryKeyFault
    }
}

impl VimObjectTrait for GuestRegistryKeyAlreadyExists {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryKeyAlreadyExists
    }
}

impl VimObjectTrait for GuestRegistryKeyHasSubkeys {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryKeyHasSubkeys
    }
}

impl VimObjectTrait for GuestRegistryKeyInvalid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryKeyInvalid
    }
}

impl VimObjectTrait for GuestRegistryKeyParentVolatile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryKeyParentVolatile
    }
}

impl VimObjectTrait for GuestRegistryValueFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryValueFault
    }
}

impl VimObjectTrait for GuestRegistryValueNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GuestRegistryValueNotFound
    }
}

impl VimObjectTrait for InvalidGuestLogin {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidGuestLogin
    }
}

impl VimObjectTrait for OperationDisabledByGuest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OperationDisabledByGuest
    }
}

impl VimObjectTrait for OperationNotSupportedByGuest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OperationNotSupportedByGuest
    }
}

impl VimObjectTrait for TooManyGuestLogons {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyGuestLogons
    }
}

impl VimObjectTrait for HostConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigFault
    }
}

impl VimObjectTrait for AdminDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AdminDisabled
    }
}

impl VimObjectTrait for AdminNotDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AdminNotDisabled
    }
}

impl VimObjectTrait for BlockedByFirewall {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BlockedByFirewall
    }
}

impl VimObjectTrait for ClockSkew {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClockSkew
    }
}

impl VimObjectTrait for DisableAdminNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisableAdminNotSupported
    }
}

impl VimObjectTrait for HostConfigFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConfigFailed
    }
}

impl VimObjectTrait for HostInDomain {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInDomain
    }
}

impl VimObjectTrait for InvalidHostName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidHostName
    }
}

impl VimObjectTrait for NasConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasConfigFault
    }
}

impl VimObjectTrait for InvalidNasCredentials {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidNasCredentials
    }
}

impl VimObjectTrait for InvalidNetworkResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidNetworkResource
    }
}

impl VimObjectTrait for NasConnectionLimitReached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasConnectionLimitReached
    }
}

impl VimObjectTrait for NasSessionCredentialConflict {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasSessionCredentialConflict
    }
}

impl VimObjectTrait for NasVolumeNotMounted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NasVolumeNotMounted
    }
}

impl VimObjectTrait for NetworkInaccessible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkInaccessible
    }
}

impl VimObjectTrait for NoPermissionOnNasVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPermissionOnNasVolume
    }
}

impl VimObjectTrait for NoGateway {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoGateway
    }
}

impl VimObjectTrait for NoVirtualNic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoVirtualNic
    }
}

impl VimObjectTrait for PlatformConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlatformConfigFault
    }
}

impl VimObjectTrait for InvalidBundle {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidBundle
    }
}

impl VimObjectTrait for PatchInstallFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchInstallFailed
    }
}

impl VimObjectTrait for PatchIntegrityError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchIntegrityError
    }
}

impl VimObjectTrait for VmfsMountFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsMountFault
    }
}

impl VimObjectTrait for VmfsAlreadyMounted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsAlreadyMounted
    }
}

impl VimObjectTrait for VmfsAmbiguousMount {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmfsAmbiguousMount
    }
}

impl VimObjectTrait for HostConnectFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostConnectFault
    }
}

impl VimObjectTrait for AgentInstallFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentInstallFailed
    }
}

impl VimObjectTrait for AlreadyBeingManaged {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlreadyBeingManaged
    }
}

impl VimObjectTrait for AlreadyConnected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlreadyConnected
    }
}

impl VimObjectTrait for CannotAddHostWithFtVmAsStandalone {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAddHostWithFtVmAsStandalone
    }
}

impl VimObjectTrait for CannotAddHostWithFtVmToDifferentCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAddHostWithFtVmToDifferentCluster
    }
}

impl VimObjectTrait for CannotAddHostWithFtVmToNonHaCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAddHostWithFtVmToNonHaCluster
    }
}

impl VimObjectTrait for GatewayConnectFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayConnectFault
    }
}

impl VimObjectTrait for GatewayNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayNotFound
    }
}

impl VimObjectTrait for GatewayNotReachable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayNotReachable
    }
}

impl VimObjectTrait for GatewayOperationRefused {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayOperationRefused
    }
}

impl VimObjectTrait for GatewayToHostConnectFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayToHostConnectFault
    }
}

impl VimObjectTrait for GatewayHostNotReachable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayHostNotReachable
    }
}

impl VimObjectTrait for GatewayToHostAuthFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayToHostAuthFault
    }
}

impl VimObjectTrait for GatewayToHostTrustVerifyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GatewayToHostTrustVerifyFault
    }
}

impl VimObjectTrait for MultipleCertificatesVerifyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MultipleCertificatesVerifyFault
    }
}

impl VimObjectTrait for NoHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoHost
    }
}

impl VimObjectTrait for NoPermissionOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPermissionOnHost
    }
}

impl VimObjectTrait for NotSupportedHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHost
    }
}

impl VimObjectTrait for NonVmwareOuiMacNotSupportedHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NonVmwareOuiMacNotSupportedHost
    }
}

impl VimObjectTrait for NotSupportedHostForVFlash {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostForVFlash
    }
}

impl VimObjectTrait for NotSupportedHostForVmcp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostForVmcp
    }
}

impl VimObjectTrait for NotSupportedHostForVmemFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostForVmemFile
    }
}

impl VimObjectTrait for NotSupportedHostForVsan {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostForVsan
    }
}

impl VimObjectTrait for NotSupportedHostInCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostInCluster
    }
}

impl VimObjectTrait for EvcAdmissionFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailed
    }
}

impl VimObjectTrait for EvcAdmissionFailedCpuFeaturesForMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedCpuFeaturesForMode
    }
}

impl VimObjectTrait for EvcAdmissionFailedCpuModel {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedCpuModel
    }
}

impl VimObjectTrait for EvcAdmissionFailedCpuModelForMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedCpuModelForMode
    }
}

impl VimObjectTrait for EvcAdmissionFailedCpuVendor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedCpuVendor
    }
}

impl VimObjectTrait for EvcAdmissionFailedCpuVendorUnknown {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedCpuVendorUnknown
    }
}

impl VimObjectTrait for EvcAdmissionFailedHostDisconnected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedHostDisconnected
    }
}

impl VimObjectTrait for EvcAdmissionFailedHostSoftware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedHostSoftware
    }
}

impl VimObjectTrait for EvcAdmissionFailedHostSoftwareForMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedHostSoftwareForMode
    }
}

impl VimObjectTrait for EvcAdmissionFailedVmActive {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EvcAdmissionFailedVmActive
    }
}

impl VimObjectTrait for NotSupportedHostInDvs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostInDvs
    }
}

impl VimObjectTrait for NotSupportedHostInHaCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostInHaCluster
    }
}

impl VimObjectTrait for ReadHostResourcePoolTreeFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReadHostResourcePoolTreeFailed
    }
}

impl VimObjectTrait for SslDisabledFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SslDisabledFault
    }
}

impl VimObjectTrait for SslVerifyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SslVerifyFault
    }
}

impl VimObjectTrait for TooManyHosts {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyHosts
    }
}

impl VimObjectTrait for HostHasComponentFailure {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostHasComponentFailure
    }
}

impl VimObjectTrait for HostIncompatibleForRecordReplay {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIncompatibleForRecordReplay
    }
}

impl VimObjectTrait for HostPowerOpFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPowerOpFailed
    }
}

impl VimObjectTrait for NoPeerHostFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPeerHostFound
    }
}

impl VimObjectTrait for VmotionInterfaceNotEnabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmotionInterfaceNotEnabled
    }
}

impl VimObjectTrait for WakeOnLanNotSupportedByVmotionNic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WakeOnLanNotSupportedByVmotionNic
    }
}

impl VimObjectTrait for HostSpecificationOperationFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpecificationOperationFailed
    }
}

impl VimObjectTrait for HttpFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HttpFault
    }
}

impl VimObjectTrait for IormNotSupportedHostOnDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IormNotSupportedHostOnDatastore
    }
}

impl VimObjectTrait for InaccessibleVFlashSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InaccessibleVFlashSource
    }
}

impl VimObjectTrait for InsufficientResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientResourcesFault
    }
}

impl VimObjectTrait for InsufficientAgentVmsDeployed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientAgentVmsDeployed
    }
}

impl VimObjectTrait for InsufficientCpuResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientCpuResourcesFault
    }
}

impl VimObjectTrait for InsufficientFailoverResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientFailoverResourcesFault
    }
}

impl VimObjectTrait for InsufficientGraphicsResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientGraphicsResourcesFault
    }
}

impl VimObjectTrait for InsufficientHostCapacityFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientHostCapacityFault
    }
}

impl VimObjectTrait for InsufficientHostCpuCapacityFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientHostCpuCapacityFault
    }
}

impl VimObjectTrait for InsufficientHostMemoryCapacityFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientHostMemoryCapacityFault
    }
}

impl VimObjectTrait for InsufficientPerCpuCapacity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientPerCpuCapacity
    }
}

impl VimObjectTrait for InsufficientMemoryResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientMemoryResourcesFault
    }
}

impl VimObjectTrait for InsufficientNetworkCapacity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientNetworkCapacity
    }
}

impl VimObjectTrait for InsufficientNetworkResourcePoolCapacity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientNetworkResourcePoolCapacity
    }
}

impl VimObjectTrait for InsufficientStandbyResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientStandbyResource
    }
}

impl VimObjectTrait for InsufficientStandbyCpuResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientStandbyCpuResource
    }
}

impl VimObjectTrait for InsufficientStandbyMemoryResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientStandbyMemoryResource
    }
}

impl VimObjectTrait for InsufficientStorageSpace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientStorageSpace
    }
}

impl VimObjectTrait for InsufficientVFlashResourcesFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientVFlashResourcesFault
    }
}

impl VimObjectTrait for InvalidResourcePoolStructureFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidResourcePoolStructureFault
    }
}

impl VimObjectTrait for NumVirtualCpusExceedsLimit {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumVirtualCpusExceedsLimit
    }
}

impl VimObjectTrait for VmFaultToleranceTooManyFtVcpusOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceTooManyFtVcpusOnHost
    }
}

impl VimObjectTrait for VmFaultToleranceTooManyVMsOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceTooManyVMsOnHost
    }
}

impl VimObjectTrait for VmSmpFaultToleranceTooManyVMsOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSmpFaultToleranceTooManyVMsOnHost
    }
}

impl VimObjectTrait for InsufficientStorageIops {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientStorageIops
    }
}

impl VimObjectTrait for InvalidAffinitySettingFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidAffinitySettingFault
    }
}

impl VimObjectTrait for InvalidBmcRole {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidBmcRole
    }
}

impl VimObjectTrait for InvalidDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDatastore
    }
}

impl VimObjectTrait for DatastoreNotWritableOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreNotWritableOnHost
    }
}

impl VimObjectTrait for SwapDatastoreNotWritableOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SwapDatastoreNotWritableOnHost
    }
}

impl VimObjectTrait for InaccessibleDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InaccessibleDatastore
    }
}

impl VimObjectTrait for InaccessibleFtMetadataDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InaccessibleFtMetadataDatastore
    }
}

impl VimObjectTrait for InvalidDatastorePath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDatastorePath
    }
}

impl VimObjectTrait for InvalidEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidEvent
    }
}

impl VimObjectTrait for InvalidFolder {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidFolder
    }
}

impl VimObjectTrait for VmAlreadyExistsInDatacenter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmAlreadyExistsInDatacenter
    }
}

impl VimObjectTrait for InvalidIpmiLoginInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidIpmiLoginInfo
    }
}

impl VimObjectTrait for InvalidIpmiMacAddress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidIpmiMacAddress
    }
}

impl VimObjectTrait for InvalidLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidLicense
    }
}

impl VimObjectTrait for InvalidLocale {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidLocale
    }
}

impl VimObjectTrait for InvalidLogin {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidLogin
    }
}

impl VimObjectTrait for InvalidClientCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidClientCertificate
    }
}

impl VimObjectTrait for PasswordExpired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PasswordExpired
    }
}

impl VimObjectTrait for InvalidName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidName
    }
}

impl VimObjectTrait for InvalidPrivilege {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidPrivilege
    }
}

impl VimObjectTrait for InvalidState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidState
    }
}

impl VimObjectTrait for CannotPowerOffVmInCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotPowerOffVmInCluster
    }
}

impl VimObjectTrait for EncryptionKeyRequired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EncryptionKeyRequired
    }
}

impl VimObjectTrait for InvalidDatastoreState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDatastoreState
    }
}

impl VimObjectTrait for InvalidHostState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidHostState
    }
}

impl VimObjectTrait for InvalidHostConnectionState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidHostConnectionState
    }
}

impl VimObjectTrait for InvalidPowerState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidPowerState
    }
}

impl VimObjectTrait for InvalidVmState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidVmState
    }
}

impl VimObjectTrait for MksConnectionLimitReached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MksConnectionLimitReached
    }
}

impl VimObjectTrait for NoActiveHostInCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoActiveHostInCluster
    }
}

impl VimObjectTrait for OvfConsumerPowerOnFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerPowerOnFault
    }
}

impl VimObjectTrait for QuestionPending {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QuestionPending
    }
}

impl VimObjectTrait for VmPowerOnDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPowerOnDisabled
    }
}

impl VimObjectTrait for IscsiFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFault
    }
}

impl VimObjectTrait for IscsiFaultInvalidVnic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultInvalidVnic
    }
}

impl VimObjectTrait for IscsiFaultPnicInUse {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultPnicInUse
    }
}

impl VimObjectTrait for IscsiFaultVnicAlreadyBound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicAlreadyBound
    }
}

impl VimObjectTrait for IscsiFaultVnicHasActivePaths {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicHasActivePaths
    }
}

impl VimObjectTrait for IscsiFaultVnicHasMultipleUplinks {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicHasMultipleUplinks
    }
}

impl VimObjectTrait for IscsiFaultVnicHasNoUplinks {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicHasNoUplinks
    }
}

impl VimObjectTrait for IscsiFaultVnicHasWrongUplink {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicHasWrongUplink
    }
}

impl VimObjectTrait for IscsiFaultVnicInUse {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicInUse
    }
}

impl VimObjectTrait for IscsiFaultVnicIsLastPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicIsLastPath
    }
}

impl VimObjectTrait for IscsiFaultVnicNotBound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicNotBound
    }
}

impl VimObjectTrait for IscsiFaultVnicNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiFaultVnicNotFound
    }
}

impl VimObjectTrait for KeyNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KeyNotFound
    }
}

impl VimObjectTrait for LicenseEntityNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseEntityNotFound
    }
}

impl VimObjectTrait for LicenseServerUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseServerUnavailable
    }
}

impl VimObjectTrait for LimitExceeded {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LimitExceeded
    }
}

impl VimObjectTrait for LogBundlingFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LogBundlingFailed
    }
}

impl VimObjectTrait for MigrationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationFault
    }
}

impl VimObjectTrait for AffinityConfigured {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AffinityConfigured
    }
}

impl VimObjectTrait for CannotModifyConfigCpuRequirements {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotModifyConfigCpuRequirements
    }
}

impl VimObjectTrait for CannotMoveVmWithDeltaDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotMoveVmWithDeltaDisk
    }
}

impl VimObjectTrait for CannotMoveVmWithNativeDeltaDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotMoveVmWithNativeDeltaDisk
    }
}

impl VimObjectTrait for CloneFromSnapshotNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CloneFromSnapshotNotSupported
    }
}

impl VimObjectTrait for DatacenterMismatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatacenterMismatch
    }
}

impl VimObjectTrait for DisallowedMigrationDeviceAttached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisallowedMigrationDeviceAttached
    }
}

impl VimObjectTrait for DiskMoveTypeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskMoveTypeNotSupported
    }
}

impl VimObjectTrait for FaultToleranceAntiAffinityViolated {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceAntiAffinityViolated
    }
}

impl VimObjectTrait for FaultToleranceNeedsThickDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceNeedsThickDisk
    }
}

impl VimObjectTrait for FaultToleranceNotSameBuild {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceNotSameBuild
    }
}

impl VimObjectTrait for HaErrorsAtDest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HaErrorsAtDest
    }
}

impl VimObjectTrait for IncompatibleDefaultDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncompatibleDefaultDevice
    }
}

impl VimObjectTrait for LargeRdmConversionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LargeRdmConversionNotSupported
    }
}

impl VimObjectTrait for MaintenanceModeFileMove {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MaintenanceModeFileMove
    }
}

impl VimObjectTrait for MigrationDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationDisabled
    }
}

impl VimObjectTrait for MigrationFeatureNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationFeatureNotSupported
    }
}

impl VimObjectTrait for FullStorageVMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FullStorageVMotionNotSupported
    }
}

impl VimObjectTrait for IndependentDiskVMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IndependentDiskVMotionNotSupported
    }
}

impl VimObjectTrait for NonHomeRdmvMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NonHomeRdmvMotionNotSupported
    }
}

impl VimObjectTrait for StorageVMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageVMotionNotSupported
    }
}

impl VimObjectTrait for UnsharedSwapVMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnsharedSwapVMotionNotSupported
    }
}

impl VimObjectTrait for VMotionAcrossNetworkNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionAcrossNetworkNotSupported
    }
}

impl VimObjectTrait for MigrationNotReady {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MigrationNotReady
    }
}

impl VimObjectTrait for MismatchedNetworkPolicies {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MismatchedNetworkPolicies
    }
}

impl VimObjectTrait for MismatchedVMotionNetworkNames {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MismatchedVMotionNetworkNames
    }
}

impl VimObjectTrait for NetworksMayNotBeTheSame {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworksMayNotBeTheSame
    }
}

impl VimObjectTrait for NoGuestHeartbeat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoGuestHeartbeat
    }
}

impl VimObjectTrait for RdmConversionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RdmConversionNotSupported
    }
}

impl VimObjectTrait for RdmNotPreserved {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RdmNotPreserved
    }
}

impl VimObjectTrait for ReadOnlyDisksWithLegacyDestination {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReadOnlyDisksWithLegacyDestination
    }
}

impl VimObjectTrait for SnapshotCopyNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotCopyNotSupported
    }
}

impl VimObjectTrait for HotSnapshotMoveNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HotSnapshotMoveNotSupported
    }
}

impl VimObjectTrait for SnapshotCloneNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotCloneNotSupported
    }
}

impl VimObjectTrait for SnapshotMoveFromNonHomeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotMoveFromNonHomeNotSupported
    }
}

impl VimObjectTrait for SnapshotMoveNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotMoveNotSupported
    }
}

impl VimObjectTrait for SnapshotMoveToNonHomeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotMoveToNonHomeNotSupported
    }
}

impl VimObjectTrait for SnapshotRevertIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotRevertIssue
    }
}

impl VimObjectTrait for SuspendedRelocateNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SuspendedRelocateNotSupported
    }
}

impl VimObjectTrait for TooManyDisksOnLegacyHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyDisksOnLegacyHost
    }
}

impl VimObjectTrait for ToolsInstallationInProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsInstallationInProgress
    }
}

impl VimObjectTrait for UncommittedUndoableDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UncommittedUndoableDisk
    }
}

impl VimObjectTrait for VMotionInterfaceIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionInterfaceIssue
    }
}

impl VimObjectTrait for VMotionLinkCapacityLow {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionLinkCapacityLow
    }
}

impl VimObjectTrait for VMotionLinkDown {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionLinkDown
    }
}

impl VimObjectTrait for VMotionNotConfigured {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionNotConfigured
    }
}

impl VimObjectTrait for VMotionNotLicensed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionNotLicensed
    }
}

impl VimObjectTrait for VMotionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionNotSupported
    }
}

impl VimObjectTrait for VMotionProtocolIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMotionProtocolIncompatible
    }
}

impl VimObjectTrait for WillLoseHaProtection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WillLoseHaProtection
    }
}

impl VimObjectTrait for WillModifyConfigCpuRequirements {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WillModifyConfigCpuRequirements
    }
}

impl VimObjectTrait for WillResetSnapshotDirectory {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WillResetSnapshotDirectory
    }
}

impl VimObjectTrait for MismatchedBundle {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MismatchedBundle
    }
}

impl VimObjectTrait for MissingBmcSupport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingBmcSupport
    }
}

impl VimObjectTrait for NamespaceFull {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NamespaceFull
    }
}

impl VimObjectTrait for NamespaceLimitReached {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NamespaceLimitReached
    }
}

impl VimObjectTrait for NamespaceWriteProtected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NamespaceWriteProtected
    }
}

impl VimObjectTrait for NetworkDisruptedAndConfigRolledBack {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NetworkDisruptedAndConfigRolledBack
    }
}

impl VimObjectTrait for NoClientCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoClientCertificate
    }
}

impl VimObjectTrait for NoCompatibleDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCompatibleDatastore
    }
}

impl VimObjectTrait for NoCompatibleHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCompatibleHost
    }
}

impl VimObjectTrait for NoCompatibleHostWithAccessToDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCompatibleHostWithAccessToDevice
    }
}

impl VimObjectTrait for NoConnectedDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoConnectedDatastore
    }
}

impl VimObjectTrait for NoDiskFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDiskFound
    }
}

impl VimObjectTrait for NoSubjectName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoSubjectName
    }
}

impl VimObjectTrait for NotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotFound
    }
}

impl VimObjectTrait for NotSupportedHostForChecksum {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedHostForChecksum
    }
}

impl VimObjectTrait for OutOfBounds {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OutOfBounds
    }
}

impl VimObjectTrait for OvfFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfFault
    }
}

impl VimObjectTrait for OvfConsumerCallbackFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerCallbackFault
    }
}

impl VimObjectTrait for OvfConsumerCommunicationError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerCommunicationError
    }
}

impl VimObjectTrait for OvfConsumerFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerFault
    }
}

impl VimObjectTrait for OvfConsumerInvalidSection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerInvalidSection
    }
}

impl VimObjectTrait for OvfConsumerUndeclaredSection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerUndeclaredSection
    }
}

impl VimObjectTrait for OvfConsumerUndefinedPrefix {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerUndefinedPrefix
    }
}

impl VimObjectTrait for OvfExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfExport
    }
}

impl VimObjectTrait for ConnectedIso {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConnectedIso
    }
}

impl VimObjectTrait for OvfDuplicatedPropertyIdExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDuplicatedPropertyIdExport
    }
}

impl VimObjectTrait for OvfDuplicatedPropertyIdImport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDuplicatedPropertyIdImport
    }
}

impl VimObjectTrait for OvfExportFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfExportFailed
    }
}

impl VimObjectTrait for OvfHardwareExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfHardwareExport
    }
}

impl VimObjectTrait for OvfConnectedDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConnectedDevice
    }
}

impl VimObjectTrait for OvfConnectedDeviceFloppy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConnectedDeviceFloppy
    }
}

impl VimObjectTrait for OvfConnectedDeviceIso {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConnectedDeviceIso
    }
}

impl VimObjectTrait for OvfUnableToExportDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnableToExportDisk
    }
}

impl VimObjectTrait for OvfUnknownDeviceBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnknownDeviceBacking
    }
}

impl VimObjectTrait for OvfUnsupportedDeviceExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedDeviceExport
    }
}

impl VimObjectTrait for OvfPropertyExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyExport
    }
}

impl VimObjectTrait for OvfPropertyNetworkExport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyNetworkExport
    }
}

impl VimObjectTrait for OvfImport {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfImport
    }
}

impl VimObjectTrait for OvfCpuCompatibility {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCpuCompatibility
    }
}

impl VimObjectTrait for OvfCpuCompatibilityCheckNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfCpuCompatibilityCheckNotSupported
    }
}

impl VimObjectTrait for OvfHardwareCheck {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfHardwareCheck
    }
}

impl VimObjectTrait for OvfImportFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfImportFailed
    }
}

impl VimObjectTrait for OvfMappedOsId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfMappedOsId
    }
}

impl VimObjectTrait for OvfMissingHardware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfMissingHardware
    }
}

impl VimObjectTrait for OvfNetworkMappingNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNetworkMappingNotSupported
    }
}

impl VimObjectTrait for OvfUnsupportedDiskProvisioning {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedDiskProvisioning
    }
}

impl VimObjectTrait for OvfInvalidPackage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidPackage
    }
}

impl VimObjectTrait for OvfAttribute {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfAttribute
    }
}

impl VimObjectTrait for OvfInvalidValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidValue
    }
}

impl VimObjectTrait for OvfInvalidValueConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidValueConfiguration
    }
}

impl VimObjectTrait for OvfInvalidValueEmpty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidValueEmpty
    }
}

impl VimObjectTrait for OvfInvalidValueFormatMalformed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidValueFormatMalformed
    }
}

impl VimObjectTrait for OvfInvalidValueReference {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidValueReference
    }
}

impl VimObjectTrait for OvfMissingAttribute {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfMissingAttribute
    }
}

impl VimObjectTrait for OvfConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConstraint
    }
}

impl VimObjectTrait for OvfDiskOrderConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDiskOrderConstraint
    }
}

impl VimObjectTrait for OvfHostResourceConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfHostResourceConstraint
    }
}

impl VimObjectTrait for OvfElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfElement
    }
}

impl VimObjectTrait for OvfDuplicateElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDuplicateElement
    }
}

impl VimObjectTrait for OvfDuplicatedElementBoundary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDuplicatedElementBoundary
    }
}

impl VimObjectTrait for OvfElementInvalidValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfElementInvalidValue
    }
}

impl VimObjectTrait for OvfMissingElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfMissingElement
    }
}

impl VimObjectTrait for OvfMissingElementNormalBoundary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfMissingElementNormalBoundary
    }
}

impl VimObjectTrait for OvfUnexpectedElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnexpectedElement
    }
}

impl VimObjectTrait for OvfWrongElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfWrongElement
    }
}

impl VimObjectTrait for OvfProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfProperty
    }
}

impl VimObjectTrait for OvfPropertyNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyNetwork
    }
}

impl VimObjectTrait for OvfPropertyQualifier {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyQualifier
    }
}

impl VimObjectTrait for OvfPropertyQualifierDuplicate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyQualifierDuplicate
    }
}

impl VimObjectTrait for OvfPropertyQualifierIgnored {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyQualifierIgnored
    }
}

impl VimObjectTrait for OvfPropertyType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyType
    }
}

impl VimObjectTrait for OvfPropertyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfPropertyValue
    }
}

impl VimObjectTrait for OvfWrongNamespace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfWrongNamespace
    }
}

impl VimObjectTrait for OvfXmlFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfXmlFormat
    }
}

impl VimObjectTrait for OvfSystemFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfSystemFault
    }
}

impl VimObjectTrait for OvfDiskMappingNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDiskMappingNotFound
    }
}

impl VimObjectTrait for OvfHostValueNotParsed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfHostValueNotParsed
    }
}

impl VimObjectTrait for OvfInternalError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInternalError
    }
}

impl VimObjectTrait for OvfToXmlUnsupportedElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfToXmlUnsupportedElement
    }
}

impl VimObjectTrait for OvfUnknownDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnknownDevice
    }
}

impl VimObjectTrait for OvfUnknownEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnknownEntity
    }
}

impl VimObjectTrait for OvfUnsupportedDeviceBackingInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedDeviceBackingInfo
    }
}

impl VimObjectTrait for OvfUnsupportedDeviceBackingOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedDeviceBackingOption
    }
}

impl VimObjectTrait for OvfUnsupportedPackage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedPackage
    }
}

impl VimObjectTrait for OvfInvalidVmName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidVmName
    }
}

impl VimObjectTrait for OvfNoHostNic {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNoHostNic
    }
}

impl VimObjectTrait for OvfNoSupportedHardwareFamily {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNoSupportedHardwareFamily
    }
}

impl VimObjectTrait for OvfUnsupportedAttribute {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedAttribute
    }
}

impl VimObjectTrait for OvfUnsupportedAttributeValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedAttributeValue
    }
}

impl VimObjectTrait for OvfUnsupportedElement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedElement
    }
}

impl VimObjectTrait for OvfNoSpaceOnController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfNoSpaceOnController
    }
}

impl VimObjectTrait for OvfUnsupportedElementValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedElementValue
    }
}

impl VimObjectTrait for OvfUnsupportedSection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedSection
    }
}

impl VimObjectTrait for OvfUnsupportedSubType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedSubType
    }
}

impl VimObjectTrait for OvfUnsupportedType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfUnsupportedType
    }
}

impl VimObjectTrait for PatchBinariesNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchBinariesNotFound
    }
}

impl VimObjectTrait for PatchMetadataInvalid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchMetadataInvalid
    }
}

impl VimObjectTrait for PatchMetadataCorrupted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchMetadataCorrupted
    }
}

impl VimObjectTrait for PatchMetadataNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchMetadataNotFound
    }
}

impl VimObjectTrait for PatchNotApplicable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchNotApplicable
    }
}

impl VimObjectTrait for PatchAlreadyInstalled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchAlreadyInstalled
    }
}

impl VimObjectTrait for PatchMissingDependencies {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchMissingDependencies
    }
}

impl VimObjectTrait for PatchSuperseded {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PatchSuperseded
    }
}

impl VimObjectTrait for ProfileUpdateFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ProfileUpdateFailed
    }
}

impl VimObjectTrait for RebootRequired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RebootRequired
    }
}

impl VimObjectTrait for RecordReplayDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecordReplayDisabled
    }
}

impl VimObjectTrait for RemoveFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoveFailed
    }
}

impl VimObjectTrait for ReplicationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationFault
    }
}

impl VimObjectTrait for IncompatibleHostForVmReplication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncompatibleHostForVmReplication
    }
}

impl VimObjectTrait for ReplicationConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationConfigFault
    }
}

impl VimObjectTrait for ReplicationDiskConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationDiskConfigFault
    }
}

impl VimObjectTrait for ReplicationVmConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationVmConfigFault
    }
}

impl VimObjectTrait for ReplicationIncompatibleWithFt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationIncompatibleWithFt
    }
}

impl VimObjectTrait for ReplicationInvalidOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationInvalidOptions
    }
}

impl VimObjectTrait for ReplicationNotSupportedOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationNotSupportedOnHost
    }
}

impl VimObjectTrait for ReplicationVmFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationVmFault
    }
}

impl VimObjectTrait for ReplicationVmInProgressFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationVmInProgressFault
    }
}

impl VimObjectTrait for ResourceInUse {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceInUse
    }
}

impl VimObjectTrait for FilterInUse {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FilterInUse
    }
}

impl VimObjectTrait for QuiesceDatastoreIoForHaFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QuiesceDatastoreIoForHaFailed
    }
}

impl VimObjectTrait for ResourceNotAvailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResourceNotAvailable
    }
}

impl VimObjectTrait for SspiChallenge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SspiChallenge
    }
}

impl VimObjectTrait for ShrinkDiskFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ShrinkDiskFault
    }
}

impl VimObjectTrait for SnapshotFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotFault
    }
}

impl VimObjectTrait for ApplicationQuiesceFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ApplicationQuiesceFault
    }
}

impl VimObjectTrait for FilesystemQuiesceFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FilesystemQuiesceFault
    }
}

impl VimObjectTrait for MemorySnapshotOnIndependentDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemorySnapshotOnIndependentDisk
    }
}

impl VimObjectTrait for MultipleSnapshotsNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MultipleSnapshotsNotSupported
    }
}

impl VimObjectTrait for SnapshotDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotDisabled
    }
}

impl VimObjectTrait for SnapshotIncompatibleDeviceInVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotIncompatibleDeviceInVm
    }
}

impl VimObjectTrait for SnapshotLocked {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotLocked
    }
}

impl VimObjectTrait for SnapshotNoChange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotNoChange
    }
}

impl VimObjectTrait for TooManySnapshotLevels {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManySnapshotLevels
    }
}

impl VimObjectTrait for SsdDiskNotAvailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SsdDiskNotAvailable
    }
}

impl VimObjectTrait for StorageDrsCannotMoveDiskInMultiWriterMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveDiskInMultiWriterMode
    }
}

impl VimObjectTrait for StorageDrsCannotMoveFtVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveFtVm
    }
}

impl VimObjectTrait for StorageDrsCannotMoveIndependentDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveIndependentDisk
    }
}

impl VimObjectTrait for StorageDrsCannotMoveManuallyPlacedSwapFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveManuallyPlacedSwapFile
    }
}

impl VimObjectTrait for StorageDrsCannotMoveManuallyPlacedVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveManuallyPlacedVm
    }
}

impl VimObjectTrait for StorageDrsCannotMoveSharedDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveSharedDisk
    }
}

impl VimObjectTrait for StorageDrsCannotMoveTemplate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveTemplate
    }
}

impl VimObjectTrait for StorageDrsCannotMoveVmInUserFolder {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveVmInUserFolder
    }
}

impl VimObjectTrait for StorageDrsCannotMoveVmWithMountedCdrom {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveVmWithMountedCdrom
    }
}

impl VimObjectTrait for StorageDrsCannotMoveVmWithNoFilesInLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsCannotMoveVmWithNoFilesInLayout
    }
}

impl VimObjectTrait for StorageDrsDatacentersCannotShareDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsDatacentersCannotShareDatastore
    }
}

impl VimObjectTrait for StorageDrsDisabledOnVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsDisabledOnVm
    }
}

impl VimObjectTrait for StorageDrsHbrDiskNotMovable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsHbrDiskNotMovable
    }
}

impl VimObjectTrait for StorageDrsHmsMoveInProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsHmsMoveInProgress
    }
}

impl VimObjectTrait for StorageDrsHmsUnreachable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsHmsUnreachable
    }
}

impl VimObjectTrait for StorageDrsIolbDisabledInternally {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsIolbDisabledInternally
    }
}

impl VimObjectTrait for StorageDrsRelocateDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsRelocateDisabled
    }
}

impl VimObjectTrait for StorageDrsStaleHmsCollection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsStaleHmsCollection
    }
}

impl VimObjectTrait for StorageDrsUnableToMoveFiles {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageDrsUnableToMoveFiles
    }
}

impl VimObjectTrait for SwapDatastoreUnset {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SwapDatastoreUnset
    }
}

impl VimObjectTrait for TaskInProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskInProgress
    }
}

impl VimObjectTrait for VAppTaskInProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppTaskInProgress
    }
}

impl VimObjectTrait for Timedout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Timedout
    }
}

impl VimObjectTrait for PowerOnFtSecondaryTimedout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PowerOnFtSecondaryTimedout
    }
}

impl VimObjectTrait for TooManyConsecutiveOverrides {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyConsecutiveOverrides
    }
}

impl VimObjectTrait for ToolsUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsUnavailable
    }
}

impl VimObjectTrait for UnrecognizedHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnrecognizedHost
    }
}

impl VimObjectTrait for UnsupportedVimApiVersion {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnsupportedVimApiVersion
    }
}

impl VimObjectTrait for UserNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UserNotFound
    }
}

impl VimObjectTrait for VAppConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppConfigFault
    }
}

impl VimObjectTrait for MissingPowerOffConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingPowerOffConfiguration
    }
}

impl VimObjectTrait for MissingPowerOnConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingPowerOnConfiguration
    }
}

impl VimObjectTrait for NoVmInVApp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoVmInVApp
    }
}

impl VimObjectTrait for VFlashModuleVersionIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VFlashModuleVersionIncompatible
    }
}

impl VimObjectTrait for VmConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigFault
    }
}

impl VimObjectTrait for CannotAccessVmComponent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessVmComponent
    }
}

impl VimObjectTrait for CannotAccessVmConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessVmConfig
    }
}

impl VimObjectTrait for CannotAccessVmDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessVmDevice
    }
}

impl VimObjectTrait for CannotAccessNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessNetwork
    }
}

impl VimObjectTrait for DestinationSwitchFull {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DestinationSwitchFull
    }
}

impl VimObjectTrait for LegacyNetworkInterfaceInUse {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LegacyNetworkInterfaceInUse
    }
}

impl VimObjectTrait for VmOnConflictDvPort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmOnConflictDvPort
    }
}

impl VimObjectTrait for VmOnVirtualIntranet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmOnVirtualIntranet
    }
}

impl VimObjectTrait for CannotAccessVmDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessVmDisk
    }
}

impl VimObjectTrait for RdmPointsToInaccessibleDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RdmPointsToInaccessibleDisk
    }
}

impl VimObjectTrait for CannotDisableSnapshot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotDisableSnapshot
    }
}

impl VimObjectTrait for CannotUseNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotUseNetwork
    }
}

impl VimObjectTrait for CpuHotPlugNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CpuHotPlugNotSupported
    }
}

impl VimObjectTrait for DeltaDiskFormatNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeltaDiskFormatNotSupported
    }
}

impl VimObjectTrait for EightHostLimitViolated {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EightHostLimitViolated
    }
}

impl VimObjectTrait for FaultToleranceCannotEditMem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceCannotEditMem
    }
}

impl VimObjectTrait for GenericVmConfigFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GenericVmConfigFault
    }
}

impl VimObjectTrait for InvalidFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidFormat
    }
}

impl VimObjectTrait for InvalidDiskFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDiskFormat
    }
}

impl VimObjectTrait for InvalidSnapshotFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidSnapshotFormat
    }
}

impl VimObjectTrait for InvalidVmConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidVmConfig
    }
}

impl VimObjectTrait for InvalidDeviceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDeviceSpec
    }
}

impl VimObjectTrait for DeviceHotPlugNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceHotPlugNotSupported
    }
}

impl VimObjectTrait for DeviceNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceNotFound
    }
}

impl VimObjectTrait for DeviceUnsupportedForVmPlatform {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceUnsupportedForVmPlatform
    }
}

impl VimObjectTrait for DeviceUnsupportedForVmVersion {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceUnsupportedForVmVersion
    }
}

impl VimObjectTrait for DisallowedDiskModeChange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisallowedDiskModeChange
    }
}

impl VimObjectTrait for InvalidController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidController
    }
}

impl VimObjectTrait for InvalidDeviceBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDeviceBacking
    }
}

impl VimObjectTrait for InvalidDeviceOperation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDeviceOperation
    }
}

impl VimObjectTrait for MissingController {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingController
    }
}

impl VimObjectTrait for SwapPlacementOverrideNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SwapPlacementOverrideNotSupported
    }
}

impl VimObjectTrait for TooManyDevices {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TooManyDevices
    }
}

impl VimObjectTrait for UnsupportedGuest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnsupportedGuest
    }
}

impl VimObjectTrait for VmWwnConflict {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWwnConflict
    }
}

impl VimObjectTrait for LargeRdmNotSupportedOnDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LargeRdmNotSupportedOnDatastore
    }
}

impl VimObjectTrait for MemoryHotPlugNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemoryHotPlugNotSupported
    }
}

impl VimObjectTrait for NoCompatibleHardAffinityHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCompatibleHardAffinityHost
    }
}

impl VimObjectTrait for NoCompatibleSoftAffinityHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCompatibleSoftAffinityHost
    }
}

impl VimObjectTrait for NumVirtualCpusIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumVirtualCpusIncompatible
    }
}

impl VimObjectTrait for OvfConsumerValidationFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfConsumerValidationFault
    }
}

impl VimObjectTrait for QuarantineModeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QuarantineModeFault
    }
}

impl VimObjectTrait for RdmNotSupportedOnDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RdmNotSupportedOnDatastore
    }
}

impl VimObjectTrait for RuleViolation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RuleViolation
    }
}

impl VimObjectTrait for SoftRuleVioCorrectionDisallowed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SoftRuleVioCorrectionDisallowed
    }
}

impl VimObjectTrait for SoftRuleVioCorrectionImpact {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SoftRuleVioCorrectionImpact
    }
}

impl VimObjectTrait for UnsupportedDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnsupportedDatastore
    }
}

impl VimObjectTrait for MemoryFileFormatNotSupportedByDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemoryFileFormatNotSupportedByDatastore
    }
}

impl VimObjectTrait for UnSupportedDatastoreForVFlash {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnSupportedDatastoreForVFlash
    }
}

impl VimObjectTrait for UnsupportedVmxLocation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnsupportedVmxLocation
    }
}

impl VimObjectTrait for VAppNotRunning {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppNotRunning
    }
}

impl VimObjectTrait for VAppPropertyFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppPropertyFault
    }
}

impl VimObjectTrait for InvalidNetworkInType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidNetworkInType
    }
}

impl VimObjectTrait for InvalidPropertyType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidPropertyType
    }
}

impl VimObjectTrait for InvalidPropertyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidPropertyValue
    }
}

impl VimObjectTrait for UnconfiguredPropertyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnconfiguredPropertyValue
    }
}

impl VimObjectTrait for MissingIpPool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingIpPool
    }
}

impl VimObjectTrait for MissingNetworkIpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingNetworkIpConfig
    }
}

impl VimObjectTrait for NoAvailableIp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoAvailableIp
    }
}

impl VimObjectTrait for NoVcManagedIpConfigured {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoVcManagedIpConfigured
    }
}

impl VimObjectTrait for NotUserConfigurableProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotUserConfigurableProperty
    }
}

impl VimObjectTrait for VFlashCacheHotConfigNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VFlashCacheHotConfigNotSupported
    }
}

impl VimObjectTrait for VFlashModuleNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VFlashModuleNotSupported
    }
}

impl VimObjectTrait for VirtualHardwareCompatibilityIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHardwareCompatibilityIssue
    }
}

impl VimObjectTrait for CpuIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CpuIncompatible
    }
}

impl VimObjectTrait for CpuCompatibilityUnknown {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CpuCompatibilityUnknown
    }
}

impl VimObjectTrait for CpuIncompatible1Ecx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CpuIncompatible1Ecx
    }
}

impl VimObjectTrait for CpuIncompatible81Edx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CpuIncompatible81Edx
    }
}

impl VimObjectTrait for FaultToleranceCpuIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceCpuIncompatible
    }
}

impl VimObjectTrait for DeviceNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceNotSupported
    }
}

impl VimObjectTrait for DeviceBackingNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceBackingNotSupported
    }
}

impl VimObjectTrait for DvPortNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortNotSupported
    }
}

impl VimObjectTrait for UnusedVirtualDiskBlocksNotScrubbed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnusedVirtualDiskBlocksNotScrubbed
    }
}

impl VimObjectTrait for VirtualDiskBlocksNotFullyProvisioned {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskBlocksNotFullyProvisioned
    }
}

impl VimObjectTrait for DeviceControllerNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceControllerNotSupported
    }
}

impl VimObjectTrait for DigestNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DigestNotSupported
    }
}

impl VimObjectTrait for FileBackedPortNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileBackedPortNotSupported
    }
}

impl VimObjectTrait for MultiWriterNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MultiWriterNotSupported
    }
}

impl VimObjectTrait for NonPersistentDisksNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NonPersistentDisksNotSupported
    }
}

impl VimObjectTrait for RdmNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RdmNotSupported
    }
}

impl VimObjectTrait for PhysCompatRdmNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PhysCompatRdmNotSupported
    }
}

impl VimObjectTrait for RawDiskNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RawDiskNotSupported
    }
}

impl VimObjectTrait for RemoteDeviceNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoteDeviceNotSupported
    }
}

impl VimObjectTrait for SharedBusControllerNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SharedBusControllerNotSupported
    }
}

impl VimObjectTrait for VmiNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmiNotSupported
    }
}

impl VimObjectTrait for VirtualDiskModeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskModeNotSupported
    }
}

impl VimObjectTrait for VirtualEthernetCardNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualEthernetCardNotSupported
    }
}

impl VimObjectTrait for DiskNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskNotSupported
    }
}

impl VimObjectTrait for IdeDiskNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IdeDiskNotSupported
    }
}

impl VimObjectTrait for DrsVmotionIncompatibleFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsVmotionIncompatibleFault
    }
}

impl VimObjectTrait for FeatureRequirementsNotMet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FeatureRequirementsNotMet
    }
}

impl VimObjectTrait for MemorySizeNotRecommended {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemorySizeNotRecommended
    }
}

impl VimObjectTrait for MemorySizeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemorySizeNotSupported
    }
}

impl VimObjectTrait for MemorySizeNotSupportedByDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MemorySizeNotSupportedByDatastore
    }
}

impl VimObjectTrait for NotEnoughCpus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotEnoughCpus
    }
}

impl VimObjectTrait for NotEnoughLogicalCpus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotEnoughLogicalCpus
    }
}

impl VimObjectTrait for NumVirtualCoresPerSocketNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumVirtualCoresPerSocketNotSupported
    }
}

impl VimObjectTrait for NumVirtualCpusNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NumVirtualCpusNotSupported
    }
}

impl VimObjectTrait for StorageVmotionIncompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageVmotionIncompatible
    }
}

impl VimObjectTrait for VirtualHardwareVersionNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualHardwareVersionNotSupported
    }
}

impl VimObjectTrait for WakeOnLanNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WakeOnLanNotSupported
    }
}

impl VimObjectTrait for VmConfigIncompatibleForFaultTolerance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigIncompatibleForFaultTolerance
    }
}

impl VimObjectTrait for VmConfigIncompatibleForRecordReplay {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmConfigIncompatibleForRecordReplay
    }
}

impl VimObjectTrait for VmHostAffinityRuleViolation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmHostAffinityRuleViolation
    }
}

impl VimObjectTrait for VmFaultToleranceIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceIssue
    }
}

impl VimObjectTrait for CannotChangeDrsBehaviorForFtSecondary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotChangeDrsBehaviorForFtSecondary
    }
}

impl VimObjectTrait for CannotChangeHaSettingsForFtSecondary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotChangeHaSettingsForFtSecondary
    }
}

impl VimObjectTrait for CannotComputeFtCompatibleHosts {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotComputeFtCompatibleHosts
    }
}

impl VimObjectTrait for FaultToleranceNotLicensed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultToleranceNotLicensed
    }
}

impl VimObjectTrait for FaultTolerancePrimaryPowerOnNotAttempted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultTolerancePrimaryPowerOnNotAttempted
    }
}

impl VimObjectTrait for FtIssuesOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FtIssuesOnHost
    }
}

impl VimObjectTrait for HostIncompatibleForFaultTolerance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIncompatibleForFaultTolerance
    }
}

impl VimObjectTrait for IncompatibleHostForFtSecondary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncompatibleHostForFtSecondary
    }
}

impl VimObjectTrait for InvalidOperationOnSecondaryVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidOperationOnSecondaryVm
    }
}

impl VimObjectTrait for NoHostSuitableForFtSecondary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoHostSuitableForFtSecondary
    }
}

impl VimObjectTrait for NotSupportedDeviceForFt {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupportedDeviceForFt
    }
}

impl VimObjectTrait for PowerOnFtSecondaryFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PowerOnFtSecondaryFailed
    }
}

impl VimObjectTrait for SecondaryVmAlreadyDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecondaryVmAlreadyDisabled
    }
}

impl VimObjectTrait for SecondaryVmAlreadyEnabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecondaryVmAlreadyEnabled
    }
}

impl VimObjectTrait for SecondaryVmAlreadyRegistered {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecondaryVmAlreadyRegistered
    }
}

impl VimObjectTrait for SecondaryVmNotRegistered {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecondaryVmNotRegistered
    }
}

impl VimObjectTrait for VmFaultToleranceConfigIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceConfigIssue
    }
}

impl VimObjectTrait for VmFaultToleranceConfigIssueWrapper {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceConfigIssueWrapper
    }
}

impl VimObjectTrait for VmFaultToleranceInvalidFileBacking {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceInvalidFileBacking
    }
}

impl VimObjectTrait for VmFaultToleranceOpIssuesList {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmFaultToleranceOpIssuesList
    }
}

impl VimObjectTrait for VmMetadataManagerFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMetadataManagerFault
    }
}

impl VimObjectTrait for VmMonitorIncompatibleForFaultTolerance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMonitorIncompatibleForFaultTolerance
    }
}

impl VimObjectTrait for VmToolsUpgradeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmToolsUpgradeFault
    }
}

impl VimObjectTrait for ToolsAlreadyUpgraded {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsAlreadyUpgraded
    }
}

impl VimObjectTrait for ToolsAutoUpgradeNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsAutoUpgradeNotSupported
    }
}

impl VimObjectTrait for ToolsImageCopyFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsImageCopyFailed
    }
}

impl VimObjectTrait for ToolsImageNotAvailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsImageNotAvailable
    }
}

impl VimObjectTrait for ToolsImageSignatureCheckFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsImageSignatureCheckFailed
    }
}

impl VimObjectTrait for ToolsUpgradeCancelled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ToolsUpgradeCancelled
    }
}

impl VimObjectTrait for VmValidateMaxDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmValidateMaxDevice
    }
}

impl VimObjectTrait for VsanFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFault
    }
}

impl VimObjectTrait for CannotChangeVsanClusterUuid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotChangeVsanClusterUuid
    }
}

impl VimObjectTrait for CannotChangeVsanNodeUuid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotChangeVsanNodeUuid
    }
}

impl VimObjectTrait for CannotMoveVsanEnabledHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotMoveVsanEnabledHost
    }
}

impl VimObjectTrait for DestinationVsanDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DestinationVsanDisabled
    }
}

impl VimObjectTrait for VsanClusterUuidMismatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterUuidMismatch
    }
}

impl VimObjectTrait for CannotReconfigureVsanWhenHaEnabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotReconfigureVsanWhenHaEnabled
    }
}

impl VimObjectTrait for DuplicateVsanNetworkInterface {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DuplicateVsanNetworkInterface
    }
}

impl VimObjectTrait for VsanDiskFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskFault
    }
}

impl VimObjectTrait for DiskHasPartitions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskHasPartitions
    }
}

impl VimObjectTrait for DiskIsLastRemainingNonSsd {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskIsLastRemainingNonSsd
    }
}

impl VimObjectTrait for DiskIsNonLocal {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskIsNonLocal
    }
}

impl VimObjectTrait for DiskIsUsb {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskIsUsb
    }
}

impl VimObjectTrait for DiskTooSmall {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskTooSmall
    }
}

impl VimObjectTrait for DuplicateDisks {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DuplicateDisks
    }
}

impl VimObjectTrait for InsufficientDisks {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientDisks
    }
}

impl VimObjectTrait for VsanIncompatibleDiskMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIncompatibleDiskMapping
    }
}

impl VimObjectTrait for WipeDiskFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::WipeDiskFault
    }
}

impl VimObjectTrait for RuntimeFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RuntimeFault
    }
}

impl VimObjectTrait for CannotDisableDrsOnClustersWithVApps {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotDisableDrsOnClustersWithVApps
    }
}

impl VimObjectTrait for ConflictingDatastoreFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ConflictingDatastoreFound
    }
}

impl VimObjectTrait for DatabaseError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatabaseError
    }
}

impl VimObjectTrait for DisallowedChangeByService {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisallowedChangeByService
    }
}

impl VimObjectTrait for DisallowedOperationOnFailoverHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DisallowedOperationOnFailoverHost
    }
}

impl VimObjectTrait for FailToLockFaultToleranceVMs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailToLockFaultToleranceVMs
    }
}

impl VimObjectTrait for InvalidProfileReferenceHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidProfileReferenceHost
    }
}

impl VimObjectTrait for InvalidScheduledTask {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidScheduledTask
    }
}

impl VimObjectTrait for LicenseAssignmentFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseAssignmentFailed
    }
}

impl VimObjectTrait for MethodAlreadyDisabledFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodAlreadyDisabledFault
    }
}

impl VimObjectTrait for MethodDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodDisabled
    }
}

impl VimObjectTrait for OperationDisallowedOnHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OperationDisallowedOnHost
    }
}

impl VimObjectTrait for RestrictedByAdministrator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RestrictedByAdministrator
    }
}

impl VimObjectTrait for ThirdPartyLicenseAssignmentFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ThirdPartyLicenseAssignmentFailed
    }
}

impl VimObjectTrait for VAppOperationInProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VAppOperationInProgress
    }
}

impl VimObjectTrait for HostCommunication {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCommunication
    }
}

impl VimObjectTrait for HostNotConnected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNotConnected
    }
}

impl VimObjectTrait for HostNotReachable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostNotReachable
    }
}

impl VimObjectTrait for InvalidArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidArgument
    }
}

impl VimObjectTrait for IncompatibleSetting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncompatibleSetting
    }
}

impl VimObjectTrait for InvalidDasConfigArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDasConfigArgument
    }
}

impl VimObjectTrait for InvalidDasRestartPriorityForFtVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDasRestartPriorityForFtVm
    }
}

impl VimObjectTrait for InvalidDrsBehaviorForFtVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidDrsBehaviorForFtVm
    }
}

impl VimObjectTrait for InvalidIndexArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidIndexArgument
    }
}

impl VimObjectTrait for InvalidRequest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidRequest
    }
}

impl VimObjectTrait for InvalidType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidType
    }
}

impl VimObjectTrait for MethodNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MethodNotFound
    }
}

impl VimObjectTrait for ManagedObjectNotFound {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ManagedObjectNotFound
    }
}

impl VimObjectTrait for NotEnoughLicenses {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotEnoughLicenses
    }
}

impl VimObjectTrait for ExpiredFeatureLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExpiredFeatureLicense
    }
}

impl VimObjectTrait for ExpiredAddonLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExpiredAddonLicense
    }
}

impl VimObjectTrait for ExpiredEditionLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExpiredEditionLicense
    }
}

impl VimObjectTrait for FailToEnableSpbm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailToEnableSpbm
    }
}

impl VimObjectTrait for HostInventoryFull {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInventoryFull
    }
}

impl VimObjectTrait for InUseFeatureManipulationDisallowed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InUseFeatureManipulationDisallowed
    }
}

impl VimObjectTrait for IncorrectHostInformation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncorrectHostInformation
    }
}

impl VimObjectTrait for InvalidEditionLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidEditionLicense
    }
}

impl VimObjectTrait for InventoryHasStandardAloneHosts {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InventoryHasStandardAloneHosts
    }
}

impl VimObjectTrait for LicenseDowngradeDisallowed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseDowngradeDisallowed
    }
}

impl VimObjectTrait for LicenseExpired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseExpired
    }
}

impl VimObjectTrait for LicenseKeyEntityMismatch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseKeyEntityMismatch
    }
}

impl VimObjectTrait for LicenseRestricted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseRestricted
    }
}

impl VimObjectTrait for LicenseSourceUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LicenseSourceUnavailable
    }
}

impl VimObjectTrait for NoLicenseServerConfigured {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoLicenseServerConfigured
    }
}

impl VimObjectTrait for VmLimitLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmLimitLicense
    }
}

impl VimObjectTrait for VramLimitLicense {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VramLimitLicense
    }
}

impl VimObjectTrait for NotImplemented {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotImplemented
    }
}

impl VimObjectTrait for NotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotSupported
    }
}

impl VimObjectTrait for HostAccessRestrictedToManagementServer {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAccessRestrictedToManagementServer
    }
}

impl VimObjectTrait for RequestCanceled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RequestCanceled
    }
}

impl VimObjectTrait for SecurityError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SecurityError
    }
}

impl VimObjectTrait for NoPermission {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoPermission
    }
}

impl VimObjectTrait for NotAuthenticated {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NotAuthenticated
    }
}

impl VimObjectTrait for RestrictedVersion {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RestrictedVersion
    }
}

impl VimObjectTrait for SolutionUserRequired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionUserRequired
    }
}

impl VimObjectTrait for SystemError {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SystemError
    }
}

impl VimObjectTrait for UnexpectedFault {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnexpectedFault
    }
}

impl VimObjectTrait for InvalidCollectorVersion {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidCollectorVersion
    }
}

impl VimObjectTrait for InvalidProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidProperty
    }
}

