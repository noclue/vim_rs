use super::as_any::AsAny;
use super::struct_enum::StructType;
use super::structs::*;

/// Base trait of all VIM (Virtual Infrastructure Management) objects.
/// This trait is used to obtain the actual data type of object even
/// when used through a trait reference. The other use of this trait is
/// to upcast a trait reference to a VimObjectTrait reference needed by
/// common library functionality.
pub trait VimObjectTrait: AsAny + miniserde::Serialize + std::fmt::Debug + Send + Sync {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait;
    fn data_type(&self) -> StructType;
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

impl VimObjectTrait for AgencyConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyConfigInfo
    }
}

impl VimObjectTrait for AgencyScope {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyScope
    }
}

impl VimObjectTrait for AgencyComputeResourceScope {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyComputeResourceScope
    }
}

impl VimObjectTrait for AgencyVmFolder {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyVmFolder
    }
}

impl VimObjectTrait for AgencyVmResourcePool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyVmResourcePool
    }
}

impl VimObjectTrait for AgentConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentConfigInfo
    }
}

impl VimObjectTrait for AgentOvfEnvironmentInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentOvfEnvironmentInfo
    }
}

impl VimObjectTrait for AgentOvfEnvironmentInfoOvfProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentOvfEnvironmentInfoOvfProperty
    }
}

impl VimObjectTrait for AgentSslTrust {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentSslTrust
    }
}

impl VimObjectTrait for AgentAnyCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentAnyCertificate
    }
}

impl VimObjectTrait for AgentPinnedPemCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentPinnedPemCertificate
    }
}

impl VimObjectTrait for AgentStoragePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentStoragePolicy
    }
}

impl VimObjectTrait for AgentVsanStoragePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentVsanStoragePolicy
    }
}

impl VimObjectTrait for AgentVibMatchingRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentVibMatchingRule
    }
}

impl VimObjectTrait for AgentVmHook {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentVmHook
    }
}

impl VimObjectTrait for EamObjectRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EamObjectRuntimeInfo
    }
}

impl VimObjectTrait for AgentRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentRuntimeInfo
    }
}

impl VimObjectTrait for Issue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::Issue
    }
}

impl VimObjectTrait for AgencyIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyIssue
    }
}

impl VimObjectTrait for AgencyDisabled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgencyDisabled
    }
}

impl VimObjectTrait for AgentIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AgentIssue
    }
}

impl VimObjectTrait for EamCertificateNotTrusted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EamCertificateNotTrusted
    }
}

impl VimObjectTrait for HostInPartialMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInPartialMaintenanceMode
    }
}

impl VimObjectTrait for ManagedHostNotReachable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ManagedHostNotReachable
    }
}

impl VimObjectTrait for MissingDvFilterSwitch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingDvFilterSwitch
    }
}

impl VimObjectTrait for OvfInvalidProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidProperty
    }
}

impl VimObjectTrait for TransitionFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TransitionFailed
    }
}

impl VimObjectTrait for VibIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibIssue
    }
}

impl VimObjectTrait for ImmediateHostRebootRequired {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ImmediateHostRebootRequired
    }
}

impl VimObjectTrait for VibCannotPutHostInMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibCannotPutHostInMaintenanceMode
    }
}

impl VimObjectTrait for VibCannotPutHostOutOfMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibCannotPutHostOutOfMaintenanceMode
    }
}

impl VimObjectTrait for VibNotInstalled {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibNotInstalled
    }
}

impl VimObjectTrait for CannotAccessAgentVib {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessAgentVib
    }
}

impl VimObjectTrait for VibDependenciesNotMetByHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibDependenciesNotMetByHost
    }
}

impl VimObjectTrait for VibInvalidFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibInvalidFormat
    }
}

impl VimObjectTrait for VibRequirementsNotMetByHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibRequirementsNotMetByHost
    }
}

impl VimObjectTrait for VibRequiresHostInMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibRequiresHostInMaintenanceMode
    }
}

impl VimObjectTrait for VibRequiresHostReboot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibRequiresHostReboot
    }
}

impl VimObjectTrait for VibRequiresManualInstallation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibRequiresManualInstallation
    }
}

impl VimObjectTrait for VibRequiresManualUninstallation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibRequiresManualUninstallation
    }
}

impl VimObjectTrait for VmIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmIssue
    }
}

impl VimObjectTrait for InvalidConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InvalidConfig
    }
}

impl VimObjectTrait for VmCorrupted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmCorrupted
    }
}

impl VimObjectTrait for VmDeployed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmDeployed
    }
}

impl VimObjectTrait for HostInMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInMaintenanceMode
    }
}

impl VimObjectTrait for HostInStandbyMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostInStandbyMode
    }
}

impl VimObjectTrait for HostPoweredOff {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPoweredOff
    }
}

impl VimObjectTrait for VmHookFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmHookFailed
    }
}

impl VimObjectTrait for VmHookTimedout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmHookTimedout
    }
}

impl VimObjectTrait for VmInaccessible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmInaccessible
    }
}

impl VimObjectTrait for VmMarkedAsTemplate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmMarkedAsTemplate
    }
}

impl VimObjectTrait for VmOrphaned {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmOrphaned
    }
}

impl VimObjectTrait for VmPoweredOff {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPoweredOff
    }
}

impl VimObjectTrait for InsufficientIpAddresses {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientIpAddresses
    }
}

impl VimObjectTrait for MissingAgentIpPool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::MissingAgentIpPool
    }
}

impl VimObjectTrait for VmPoweredOn {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPoweredOn
    }
}

impl VimObjectTrait for VmProtected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmProtected
    }
}

impl VimObjectTrait for VmSuspended {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmSuspended
    }
}

impl VimObjectTrait for VmWrongFolder {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWrongFolder
    }
}

impl VimObjectTrait for VmWrongResourcePool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmWrongResourcePool
    }
}

impl VimObjectTrait for VmNotDeployed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmNotDeployed
    }
}

impl VimObjectTrait for CannotAccessAgentOvf {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CannotAccessAgentOvf
    }
}

impl VimObjectTrait for IncompatibleHostVersion {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IncompatibleHostVersion
    }
}

impl VimObjectTrait for InsufficientResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientResources
    }
}

impl VimObjectTrait for InsufficientSpace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::InsufficientSpace
    }
}

impl VimObjectTrait for NoAgentVmDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoAgentVmDatastore
    }
}

impl VimObjectTrait for NoCustomAgentVmDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCustomAgentVmDatastore
    }
}

impl VimObjectTrait for NoAgentVmNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoAgentVmNetwork
    }
}

impl VimObjectTrait for NoCustomAgentVmNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoCustomAgentVmNetwork
    }
}

impl VimObjectTrait for NoDiscoverableAgentVmDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDiscoverableAgentVmDatastore
    }
}

impl VimObjectTrait for NoDiscoverableAgentVmNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NoDiscoverableAgentVmNetwork
    }
}

impl VimObjectTrait for OvfInvalidFormat {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfInvalidFormat
    }
}

impl VimObjectTrait for VmRequiresHostOutOfMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmRequiresHostOutOfMaintenanceMode
    }
}

impl VimObjectTrait for PersonalityAgentPmIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgentPmIssue
    }
}

impl VimObjectTrait for PersonalityAgentAwaitingPmRemediation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgentAwaitingPmRemediation
    }
}

impl VimObjectTrait for PersonalityAgentBlockedByAgencyOperation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgentBlockedByAgencyOperation
    }
}

impl VimObjectTrait for OrphanedAgency {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OrphanedAgency
    }
}

impl VimObjectTrait for ClusterAgentAgentIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentAgentIssue
    }
}

impl VimObjectTrait for ClusterAgentOvfInvalidProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentOvfInvalidProperty
    }
}

impl VimObjectTrait for ClusterAgentTransitionFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentTransitionFailed
    }
}

impl VimObjectTrait for ClusterAgentVmIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmIssue
    }
}

impl VimObjectTrait for ClusterAgentHostInMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentHostInMaintenanceMode
    }
}

impl VimObjectTrait for ClusterAgentHostInPartialMaintenanceMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentHostInPartialMaintenanceMode
    }
}

impl VimObjectTrait for ClusterAgentInvalidConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentInvalidConfig
    }
}

impl VimObjectTrait for ClusterAgentVmHookFailed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmHookFailed
    }
}

impl VimObjectTrait for ClusterAgentVmHookTimedout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmHookTimedout
    }
}

impl VimObjectTrait for ClusterAgentVmInaccessible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmInaccessible
    }
}

impl VimObjectTrait for ClusterAgentVmNotRemoved {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmNotRemoved
    }
}

impl VimObjectTrait for ClusterAgentVmPoweredOff {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmPoweredOff
    }
}

impl VimObjectTrait for ClusterAgentInsufficientClusterResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentInsufficientClusterResources
    }
}

impl VimObjectTrait for ClusterAgentVmPoweredOn {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmPoweredOn
    }
}

impl VimObjectTrait for ClusterAgentVmProtected {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmProtected
    }
}

impl VimObjectTrait for ClusterAgentVmSuspended {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmSuspended
    }
}

impl VimObjectTrait for ClusterAgentVmNotDeployed {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentVmNotDeployed
    }
}

impl VimObjectTrait for ClusterAgentCertificateNotTrusted {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentCertificateNotTrusted
    }
}

impl VimObjectTrait for ClusterAgentInsufficientClusterSpace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentInsufficientClusterSpace
    }
}

impl VimObjectTrait for ClusterAgentMissingClusterVmDatastore {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentMissingClusterVmDatastore
    }
}

impl VimObjectTrait for ClusterAgentMissingClusterVmNetwork {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterAgentMissingClusterVmNetwork
    }
}

impl VimObjectTrait for IntegrityAgencyVumIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntegrityAgencyVumIssue
    }
}

impl VimObjectTrait for IntegrityAgencyCannotDeleteSoftware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntegrityAgencyCannotDeleteSoftware
    }
}

impl VimObjectTrait for IntegrityAgencyCannotStageSoftware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntegrityAgencyCannotStageSoftware
    }
}

impl VimObjectTrait for IntegrityAgencyVumUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IntegrityAgencyVumUnavailable
    }
}

impl VimObjectTrait for PersonalityAgencyPmIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyPmIssue
    }
}

impl VimObjectTrait for PersonalityAgencyCannotConfigureSolutions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyCannotConfigureSolutions
    }
}

impl VimObjectTrait for PersonalityAgencyDepotIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyDepotIssue
    }
}

impl VimObjectTrait for PersonalityAgencyCannotUploadDepot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyCannotUploadDepot
    }
}

impl VimObjectTrait for PersonalityAgencyInaccessibleDepot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyInaccessibleDepot
    }
}

impl VimObjectTrait for PersonalityAgencyInvalidDepot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyInvalidDepot
    }
}

impl VimObjectTrait for PersonalityAgencyPmUnavailable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PersonalityAgencyPmUnavailable
    }
}

impl VimObjectTrait for ExtensibleIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ExtensibleIssue
    }
}

impl VimObjectTrait for HostIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostIssue
    }
}

impl VimObjectTrait for OrphanedDvFilterSwitch {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OrphanedDvFilterSwitch
    }
}

impl VimObjectTrait for UnknownAgentVm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UnknownAgentVm
    }
}

impl VimObjectTrait for HooksHookListSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HooksHookListSpec
    }
}

impl VimObjectTrait for HooksMarkAsProcessedSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HooksMarkAsProcessedSpec
    }
}

impl VimObjectTrait for SolutionsApplySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsApplySpec
    }
}

impl VimObjectTrait for SolutionsClusterSolutionComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsClusterSolutionComplianceResult
    }
}

impl VimObjectTrait for SolutionsComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsComplianceResult
    }
}

impl VimObjectTrait for SolutionsComplianceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsComplianceSpec
    }
}

impl VimObjectTrait for SolutionsDeploymentUnitComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsDeploymentUnitComplianceResult
    }
}

impl VimObjectTrait for SolutionsHookAcknowledgeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsHookAcknowledgeConfig
    }
}

impl VimObjectTrait for SolutionsInteractiveHookAcknowledgeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsInteractiveHookAcknowledgeConfig
    }
}

impl VimObjectTrait for SolutionsHookConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsHookConfig
    }
}

impl VimObjectTrait for SolutionsHookInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsHookInfo
    }
}

impl VimObjectTrait for SolutionsHostComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsHostComplianceResult
    }
}

impl VimObjectTrait for SolutionsOvfProperty {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsOvfProperty
    }
}

impl VimObjectTrait for SolutionsSolutionComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsSolutionComplianceResult
    }
}

impl VimObjectTrait for SolutionsSolutionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsSolutionConfig
    }
}

impl VimObjectTrait for SolutionsSolutionValidationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsSolutionValidationResult
    }
}

impl VimObjectTrait for SolutionsStoragePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsStoragePolicy
    }
}

impl VimObjectTrait for SolutionsProfileIdStoragePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsProfileIdStoragePolicy
    }
}

impl VimObjectTrait for SolutionsTransitionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsTransitionSpec
    }
}

impl VimObjectTrait for SolutionsTypeSpecificSolutionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsTypeSpecificSolutionConfig
    }
}

impl VimObjectTrait for SolutionsClusterBoundSolutionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsClusterBoundSolutionConfig
    }
}

impl VimObjectTrait for SolutionsHostBoundSolutionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsHostBoundSolutionConfig
    }
}

impl VimObjectTrait for SolutionsVmNetworkMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsVmNetworkMapping
    }
}

impl VimObjectTrait for SolutionsVmSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsVmSource
    }
}

impl VimObjectTrait for SolutionsUrlVmSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsUrlVmSource
    }
}

impl VimObjectTrait for SolutionsValidateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsValidateSpec
    }
}

impl VimObjectTrait for SolutionsValidationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsValidationResult
    }
}

impl VimObjectTrait for SolutionsVmResourceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SolutionsVmResourceSpec
    }
}

impl VimObjectTrait for VibVibInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibVibInfo
    }
}

impl VimObjectTrait for VibVibInfoSoftwareTags {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibVibInfoSoftwareTags
    }
}

impl VimObjectTrait for VibVibServicesSslTrust {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibVibServicesSslTrust
    }
}

impl VimObjectTrait for VibVibServicesAnyCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibVibServicesAnyCertificate
    }
}

impl VimObjectTrait for VibVibServicesPinnedPemCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VibVibServicesPinnedPemCertificate
    }
}

impl VimObjectTrait for PbmAboutInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmAboutInfo
    }
}

impl VimObjectTrait for PbmExtendedElementDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmExtendedElementDescription
    }
}

impl VimObjectTrait for PbmLoggingConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmLoggingConfiguration
    }
}

impl VimObjectTrait for PbmServerObjectRef {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmServerObjectRef
    }
}

impl VimObjectTrait for PbmServiceInstanceContent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmServiceInstanceContent
    }
}

impl VimObjectTrait for PbmCapabilityInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityInstance
    }
}

impl VimObjectTrait for PbmCapabilityMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityMetadata
    }
}

impl VimObjectTrait for PbmCapabilityMetadataUniqueId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityMetadataUniqueId
    }
}

impl VimObjectTrait for PbmCapabilityConstraintInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityConstraintInstance
    }
}

impl VimObjectTrait for PbmCapabilityPropertyInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityPropertyInstance
    }
}

impl VimObjectTrait for PbmCapabilityPropertyMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityPropertyMetadata
    }
}

impl VimObjectTrait for PbmCapabilityTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityTypeInfo
    }
}

impl VimObjectTrait for PbmCapabilityGenericTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityGenericTypeInfo
    }
}

impl VimObjectTrait for PbmCapabilityMetadataPerCategory {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityMetadataPerCategory
    }
}

impl VimObjectTrait for PbmCapabilitySchema {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilitySchema
    }
}

impl VimObjectTrait for PbmCapabilityNamespaceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityNamespaceInfo
    }
}

impl VimObjectTrait for PbmCapabilitySchemaVendorInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilitySchemaVendorInfo
    }
}

impl VimObjectTrait for PbmCapabilityVendorNamespaceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityVendorNamespaceInfo
    }
}

impl VimObjectTrait for PbmCapabilityVendorResourceTypeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityVendorResourceTypeInfo
    }
}

impl VimObjectTrait for PbmLineOfServiceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmLineOfServiceInfo
    }
}

impl VimObjectTrait for PbmPersistenceBasedDataServiceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPersistenceBasedDataServiceInfo
    }
}

impl VimObjectTrait for PbmVaioDataServiceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmVaioDataServiceInfo
    }
}

impl VimObjectTrait for PbmCapabilityDescription {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityDescription
    }
}

impl VimObjectTrait for PbmCapabilityDiscreteSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityDiscreteSet
    }
}

impl VimObjectTrait for PbmCapabilityRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityRange
    }
}

impl VimObjectTrait for PbmCapabilityTimeSpan {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityTimeSpan
    }
}

impl VimObjectTrait for PbmComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmComplianceResult
    }
}

impl VimObjectTrait for PbmFetchEntityHealthStatusSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmFetchEntityHealthStatusSpec
    }
}

impl VimObjectTrait for PbmComplianceOperationalStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmComplianceOperationalStatus
    }
}

impl VimObjectTrait for PbmCompliancePolicyStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCompliancePolicyStatus
    }
}

impl VimObjectTrait for PbmRollupComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmRollupComplianceResult
    }
}

impl VimObjectTrait for PbmFaultNoPermissionEntityPrivileges {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmFaultNoPermissionEntityPrivileges
    }
}

impl VimObjectTrait for PbmPlacementCompatibilityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementCompatibilityResult
    }
}

impl VimObjectTrait for PbmPlacementMatchingResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementMatchingResources
    }
}

impl VimObjectTrait for PbmPlacementMatchingReplicationResources {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementMatchingReplicationResources
    }
}

impl VimObjectTrait for PbmPlacementHub {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementHub
    }
}

impl VimObjectTrait for PbmPlacementHubInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementHubInfo
    }
}

impl VimObjectTrait for PbmPlacementRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementRequirement
    }
}

impl VimObjectTrait for PbmPlacementCapabilityConstraintsRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementCapabilityConstraintsRequirement
    }
}

impl VimObjectTrait for PbmPlacementCapabilityProfileRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementCapabilityProfileRequirement
    }
}

impl VimObjectTrait for PbmPlacementZoneTopologyRequirement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementZoneTopologyRequirement
    }
}

impl VimObjectTrait for PbmPlacementResourceUtilization {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmPlacementResourceUtilization
    }
}

impl VimObjectTrait for PbmCapabilityProfileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityProfileCreateSpec
    }
}

impl VimObjectTrait for PbmCapabilityProfileUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityProfileUpdateSpec
    }
}

impl VimObjectTrait for PbmCapabilityConstraints {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityConstraints
    }
}

impl VimObjectTrait for PbmCapabilitySubProfileConstraints {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilitySubProfileConstraints
    }
}

impl VimObjectTrait for PbmDataServiceToPoliciesMap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmDataServiceToPoliciesMap
    }
}

impl VimObjectTrait for PbmDefaultProfileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmDefaultProfileInfo
    }
}

impl VimObjectTrait for PbmProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfile
    }
}

impl VimObjectTrait for PbmCapabilityProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilityProfile
    }
}

impl VimObjectTrait for PbmDefaultCapabilityProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmDefaultCapabilityProfile
    }
}

impl VimObjectTrait for PbmProfileId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfileId
    }
}

impl VimObjectTrait for PbmProfileK8SCompliantNameSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfileK8SCompliantNameSpec
    }
}

impl VimObjectTrait for PbmProfileOperationOutcome {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfileOperationOutcome
    }
}

impl VimObjectTrait for PbmProfileType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfileType
    }
}

impl VimObjectTrait for PbmQueryProfileResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmQueryProfileResult
    }
}

impl VimObjectTrait for PbmProfileResourceType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmProfileResourceType
    }
}

impl VimObjectTrait for PbmCapabilitySubProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmCapabilitySubProfile
    }
}

impl VimObjectTrait for PbmDatastoreSpaceStatistics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmDatastoreSpaceStatistics
    }
}

impl VimObjectTrait for PbmQueryReplicationGroupResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PbmQueryReplicationGroupResult
    }
}

impl VimObjectTrait for SmsAboutInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SmsAboutInfo
    }
}

impl VimObjectTrait for EntityReference {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityReference
    }
}

impl VimObjectTrait for FaultDomainFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultDomainFilter
    }
}

impl VimObjectTrait for ReplicationGroupFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationGroupFilter
    }
}

impl VimObjectTrait for SmsTaskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SmsTaskInfo
    }
}

impl VimObjectTrait for AlarmFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmFilter
    }
}

impl VimObjectTrait for AlarmResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AlarmResult
    }
}

impl VimObjectTrait for SmsProviderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SmsProviderInfo
    }
}

impl VimObjectTrait for VasaProviderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaProviderInfo
    }
}

impl VimObjectTrait for SmsProviderSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SmsProviderSpec
    }
}

impl VimObjectTrait for VasaProviderSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaProviderSpec
    }
}

impl VimObjectTrait for VasaProviderUpgradeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaProviderUpgradeSpec
    }
}

impl VimObjectTrait for RelatedStorageArray {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RelatedStorageArray
    }
}

impl VimObjectTrait for SupportedVendorModelMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SupportedVendorModelMapping
    }
}

impl VimObjectTrait for BackingConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BackingConfig
    }
}

impl VimObjectTrait for BackingStoragePool {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::BackingStoragePool
    }
}

impl VimObjectTrait for DatastoreBackingPoolMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastoreBackingPoolMapping
    }
}

impl VimObjectTrait for DatastorePair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DatastorePair
    }
}

impl VimObjectTrait for DrsMigrationCapabilityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DrsMigrationCapabilityResult
    }
}

impl VimObjectTrait for FaultDomainProviderMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultDomainProviderMapping
    }
}

impl VimObjectTrait for StorageFileSystemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageFileSystemInfo
    }
}

impl VimObjectTrait for LunHbaAssociation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LunHbaAssociation
    }
}

impl VimObjectTrait for NameValuePair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::NameValuePair
    }
}

impl VimObjectTrait for StorageAlarm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageAlarm
    }
}

impl VimObjectTrait for StorageArray {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageArray
    }
}

impl VimObjectTrait for StorageCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageCapability
    }
}

impl VimObjectTrait for StorageContainer {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageContainer
    }
}

impl VimObjectTrait for StorageContainerResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageContainerResult
    }
}

impl VimObjectTrait for StorageContainerSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageContainerSpec
    }
}

impl VimObjectTrait for StorageFileSystem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageFileSystem
    }
}

impl VimObjectTrait for StorageLun {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageLun
    }
}

impl VimObjectTrait for StoragePort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StoragePort
    }
}

impl VimObjectTrait for FcStoragePort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcStoragePort
    }
}

impl VimObjectTrait for FcoeStoragePort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FcoeStoragePort
    }
}

impl VimObjectTrait for IscsiStoragePort {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IscsiStoragePort
    }
}

impl VimObjectTrait for StorageProcessor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::StorageProcessor
    }
}

impl VimObjectTrait for DeviceId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DeviceId
    }
}

impl VimObjectTrait for VVolId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VVolId
    }
}

impl VimObjectTrait for VasaVirtualDiskId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VasaVirtualDiskId
    }
}

impl VimObjectTrait for VirtualDiskKey {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskKey
    }
}

impl VimObjectTrait for VirtualDiskMoId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualDiskMoId
    }
}

impl VimObjectTrait for VirtualMachineId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineId
    }
}

impl VimObjectTrait for VirtualMachineFilePath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineFilePath
    }
}

impl VimObjectTrait for VirtualMachineMoId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineMoId
    }
}

impl VimObjectTrait for VirtualMachineUuid {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineUuid
    }
}

impl VimObjectTrait for FailoverParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailoverParam
    }
}

impl VimObjectTrait for TestFailoverParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TestFailoverParam
    }
}

impl VimObjectTrait for PolicyAssociation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PolicyAssociation
    }
}

impl VimObjectTrait for ReplicationGroupData {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationGroupData
    }
}

impl VimObjectTrait for RecoveredDevice {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecoveredDevice
    }
}

impl VimObjectTrait for RecoveredDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecoveredDiskInfo
    }
}

impl VimObjectTrait for GroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GroupInfo
    }
}

impl VimObjectTrait for SourceGroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SourceGroupInfo
    }
}

impl VimObjectTrait for TargetGroupInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TargetGroupInfo
    }
}

impl VimObjectTrait for GroupOperationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GroupOperationResult
    }
}

impl VimObjectTrait for FailoverSuccessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FailoverSuccessResult
    }
}

impl VimObjectTrait for GroupErrorResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::GroupErrorResult
    }
}

impl VimObjectTrait for QueryPointInTimeReplicaSuccessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryPointInTimeReplicaSuccessResult
    }
}

impl VimObjectTrait for QueryPointInTimeReplicaSummaryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryPointInTimeReplicaSummaryResult
    }
}

impl VimObjectTrait for QueryReplicationGroupSuccessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryReplicationGroupSuccessResult
    }
}

impl VimObjectTrait for ReverseReplicationSuccessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReverseReplicationSuccessResult
    }
}

impl VimObjectTrait for SyncReplicationGroupSuccessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SyncReplicationGroupSuccessResult
    }
}

impl VimObjectTrait for PointInTimeReplicaId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PointInTimeReplicaId
    }
}

impl VimObjectTrait for PromoteParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PromoteParam
    }
}

impl VimObjectTrait for QueryPointInTimeReplicaParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryPointInTimeReplicaParam
    }
}

impl VimObjectTrait for ReplicaQueryIntervalParam {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicaQueryIntervalParam
    }
}

impl VimObjectTrait for PointInTimeReplicaInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PointInTimeReplicaInfo
    }
}

impl VimObjectTrait for ReplicaIntervalQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicaIntervalQueryResult
    }
}

impl VimObjectTrait for QueryReplicationPeerResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryReplicationPeerResult
    }
}

impl VimObjectTrait for ReplicaId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicaId
    }
}

impl VimObjectTrait for ReplicationTargetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ReplicationTargetInfo
    }
}

impl VimObjectTrait for SourceGroupMemberInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SourceGroupMemberInfo
    }
}

impl VimObjectTrait for TargetDeviceId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TargetDeviceId
    }
}

impl VimObjectTrait for TargetToSourceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TargetToSourceInfo
    }
}

impl VimObjectTrait for TargetGroupMemberInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TargetGroupMemberInfo
    }
}

impl VimObjectTrait for RecoveredTargetGroupMemberInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RecoveredTargetGroupMemberInfo
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

impl VimObjectTrait for ClusterComputeResourceCryptoModePolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceCryptoModePolicy
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

impl VimObjectTrait for ClusterComputeResourceHostEvacuationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceHostEvacuationInfo
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

impl VimObjectTrait for ClusterComputeResourceMaintenanceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterComputeResourceMaintenanceInfo
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

impl VimObjectTrait for VsanClusterConfigPrecheckItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterConfigPrecheckItem
    }
}

impl VimObjectTrait for VsanValidationItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanValidationItem
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

impl VimObjectTrait for ComputeResourceHostSeedSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceHostSeedSpec
    }
}

impl VimObjectTrait for ComputeResourceHostSeedSpecSingleHostSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ComputeResourceHostSeedSpecSingleHostSpec
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

impl VimObjectTrait for DirectPathProfileManagerCapacityQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityQuerySpec
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityQueryByDeviceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityQueryByDeviceConfig
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityQueryById {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityQueryById
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityQueryByName {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityQueryByName
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityResult
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityInfo
    }
}

impl VimObjectTrait for DirectPathProfileManagerCapacityUnknown {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCapacityUnknown
    }
}

impl VimObjectTrait for DirectPathProfileManagerCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerCreateSpec
    }
}

impl VimObjectTrait for DirectPathProfileManagerDirectPathConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerDirectPathConfig
    }
}

impl VimObjectTrait for DirectPathProfileManagerDvxDirectPathConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerDvxDirectPathConfig
    }
}

impl VimObjectTrait for DirectPathProfileManagerDynamicDirectPathConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerDynamicDirectPathConfig
    }
}

impl VimObjectTrait for DirectPathProfileManagerVirtualDeviceGroupDirectPathConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerVirtualDeviceGroupDirectPathConfig
    }
}

impl VimObjectTrait for DirectPathProfileManagerVmiopDirectPathConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerVmiopDirectPathConfig
    }
}

impl VimObjectTrait for DirectPathProfileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileInfo
    }
}

impl VimObjectTrait for DirectPathProfileManagerFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerFilterSpec
    }
}

impl VimObjectTrait for DirectPathProfileManagerTargetEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerTargetEntity
    }
}

impl VimObjectTrait for DirectPathProfileManagerTargetCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerTargetCluster
    }
}

impl VimObjectTrait for DirectPathProfileManagerTargetHost {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerTargetHost
    }
}

impl VimObjectTrait for DirectPathProfileManagerUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DirectPathProfileManagerUpdateSpec
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

impl VimObjectTrait for FolderExternallyManagedFolderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FolderExternallyManagedFolderInfo
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

impl VimObjectTrait for HbrReplicationTargetSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrReplicationTargetSpec
    }
}

impl VimObjectTrait for HbrTargetSpecReplacement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrTargetSpecReplacement
    }
}

impl VimObjectTrait for HbrTargetSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HbrTargetSpec
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

impl VimObjectTrait for IoFilterManagerSslTrust {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::IoFilterManagerSslTrust
    }
}

impl VimObjectTrait for PinnedCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PinnedCertificate
    }
}

impl VimObjectTrait for UntrustedCertificate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::UntrustedCertificate
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

impl VimObjectTrait for OvfImportParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfImportParams
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

impl VimObjectTrait for OvfDatastoreMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfDatastoreMapping
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

impl VimObjectTrait for OvfStorageProfileMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::OvfStorageProfileMapping
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

impl VimObjectTrait for PlaceVmsXClusterSpecCandidateNetworks {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PlaceVmsXClusterSpecCandidateNetworks
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

impl VimObjectTrait for VimVsanReconfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanReconfigSpec
    }
}

impl VimObjectTrait for SearchIndexFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexFilter
    }
}

impl VimObjectTrait for SearchIndexIterationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexIterationSpec
    }
}

impl VimObjectTrait for SearchIndexOptionalValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexOptionalValue
    }
}

impl VimObjectTrait for SearchIndexPredicate {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexPredicate
    }
}

impl VimObjectTrait for SearchIndexQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexQuerySpec
    }
}

impl VimObjectTrait for SearchIndexResourceItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexResourceItem
    }
}

impl VimObjectTrait for SearchIndexResultSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SearchIndexResultSet
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

impl VimObjectTrait for TagId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TagId
    }
}

impl VimObjectTrait for TagIdNameId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TagIdNameId
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

impl VimObjectTrait for TaskInfoFilterSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskInfoFilterSpec
    }
}

impl VimObjectTrait for TaskInfoFilterSpecFilterTaskResults {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskInfoFilterSpecFilterTaskResults
    }
}

impl VimObjectTrait for TaskManagerTaskViewSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskManagerTaskViewSpec
    }
}

impl VimObjectTrait for TaskManagerViewByStartId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TaskManagerViewByStartId
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

impl VimObjectTrait for TransitGatewayConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TransitGatewayConfigInfo
    }
}

impl VimObjectTrait for TransitGatewayConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TransitGatewayConfigSpec
    }
}

impl VimObjectTrait for TransitGatewayCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TransitGatewayCreateSpec
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

impl VimObjectTrait for VsanComparator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComparator
    }
}

impl VimObjectTrait for VsanJsonComparator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanJsonComparator
    }
}

impl VimObjectTrait for VsanNestJsonComparator {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNestJsonComparator
    }
}

impl VimObjectTrait for VsanDataObfuscationRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataObfuscationRule
    }
}

impl VimObjectTrait for VsanJsonFilterRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanJsonFilterRule
    }
}

impl VimObjectTrait for VsanMassCollectorPropertyParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMassCollectorPropertyParams
    }
}

impl VimObjectTrait for VsanMassCollectorSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMassCollectorSpec
    }
}

impl VimObjectTrait for VsanObjectTypeRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectTypeRule
    }
}

impl VimObjectTrait for VsanRegexBasedRule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRegexBasedRule
    }
}

impl VimObjectTrait for VsanResourceConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceConstraint
    }
}

impl VimObjectTrait for VsanCompositeConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCompositeConstraint
    }
}

impl VimObjectTrait for VsanPropertyConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPropertyConstraint
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

impl VimObjectTrait for VsanBrokenDiskChainIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanBrokenDiskChainIssue
    }
}

impl VimObjectTrait for VsanDisallowDataMovementIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDisallowDataMovementIssue
    }
}

impl VimObjectTrait for VsanDisallowEvacuateDataIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDisallowEvacuateDataIssue
    }
}

impl VimObjectTrait for VsanDiskUnhealthIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskUnhealthIssue
    }
}

impl VimObjectTrait for VsanHigherObjectsPresentDuringDowngradeIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHigherObjectsPresentDuringDowngradeIssue
    }
}

impl VimObjectTrait for VsanHostPropertyRetrieveIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostPropertyRetrieveIssue
    }
}

impl VimObjectTrait for VsanHostWithHybridDiskgroupIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostWithHybridDiskgroupIssue
    }
}

impl VimObjectTrait for VsanHostsCompressionOnlyNotSupported {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostsCompressionOnlyNotSupported
    }
}

impl VimObjectTrait for VsanMixedEsxVersionInClientIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMixedEsxVersionInClientIssue
    }
}

impl VimObjectTrait for VsanMixedEsxVersionIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMixedEsxVersionIssue
    }
}

impl VimObjectTrait for VsanObjectInaccessibleIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectInaccessibleIssue
    }
}

impl VimObjectTrait for VsanObjectPolicyIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectPolicyIssue
    }
}

impl VimObjectTrait for VsanRemoteClusterNotCompatible {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRemoteClusterNotCompatible
    }
}

impl VimObjectTrait for VsanUnknownScanIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUnknownScanIssue
    }
}

impl VimObjectTrait for VsanUnsupportedHighDiskVersionIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUnsupportedHighDiskVersionIssue
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

impl VimObjectTrait for VsanDiskFormatConversionCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskFormatConversionCheckResult
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

impl VimObjectTrait for VsanUpgradeSystemUpgradeHistoryStoragePoolOp {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeSystemUpgradeHistoryStoragePoolOp
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

impl VimObjectTrait for VsanUpgradeStatusEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpgradeStatusEx
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

impl VimObjectTrait for ClusterPowerContext {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterPowerContext
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

impl VimObjectTrait for PerformClusterPowerActionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PerformClusterPowerActionSpec
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

impl VimObjectTrait for QueryVsanManagedStorageSpaceUsageSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::QueryVsanManagedStorageSpaceUsageSpec
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

impl VimObjectTrait for ClusterFtVmHostRuleInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterFtVmHostRuleInfo
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

impl VimObjectTrait for VsanSiteFaultDomain {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteFaultDomain
    }
}

impl VimObjectTrait for VsanSiteFaultDomainConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteFaultDomainConfig
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

impl VimObjectTrait for VsanStorageComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStorageComplianceResult
    }
}

impl VimObjectTrait for VsanStorageOperationalStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStorageOperationalStatus
    }
}

impl VimObjectTrait for VsanStoragePolicyStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStoragePolicyStatus
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

impl VimObjectTrait for VimClusterVsanPreferredFaultDomainInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanPreferredFaultDomainInfo
    }
}

impl VimObjectTrait for VimClusterVsanStretchedClusterCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanStretchedClusterCapability
    }
}

impl VimObjectTrait for VimClusterVsanStretchedClusterFaultDomainConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanStretchedClusterFaultDomainConfig
    }
}

impl VimObjectTrait for VsanStretchedClusterHostVirtualApplianceStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStretchedClusterHostVirtualApplianceStatus
    }
}

impl VimObjectTrait for VimClusterVsanWitnessHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanWitnessHostInfo
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

impl VimObjectTrait for VsanAttachToSrOperation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanAttachToSrOperation
    }
}

impl VimObjectTrait for VsanCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCapability
    }
}

impl VimObjectTrait for VsanClientServerHciMeshDitEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClientServerHciMeshDitEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanClusterAdvCfgSyncHostResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterAdvCfgSyncHostResult
    }
}

impl VimObjectTrait for VsanClusterAdvCfgSyncResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterAdvCfgSyncResult
    }
}

impl VimObjectTrait for VsanClusterBalancePerDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterBalancePerDiskInfo
    }
}

impl VimObjectTrait for VsanClusterBalanceSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterBalanceSummary
    }
}

impl VimObjectTrait for VsanClusterClomdLivenessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterClomdLivenessResult
    }
}

impl VimObjectTrait for VsanClusterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterConfig
    }
}

impl VimObjectTrait for VsanClusterCreateVmHealthTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterCreateVmHealthTestResult
    }
}

impl VimObjectTrait for VsanClusterDitEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterDitEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanClusterEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanClusterFileServiceHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterFileServiceHealthSummary
    }
}

impl VimObjectTrait for VsanClusterGlobalDedupHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterGlobalDedupHealthSummary
    }
}

impl VimObjectTrait for VsanClusterHciMeshDitEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHciMeshDitEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanClusterHclInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHclInfo
    }
}

impl VimObjectTrait for VsanClusterHealthAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthAction
    }
}

impl VimObjectTrait for VsanClusterHealthCheckInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthCheckInfo
    }
}

impl VimObjectTrait for VsanClusterHealthConfigs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthConfigs
    }
}

impl VimObjectTrait for VsanClusterHealthGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthGroup
    }
}

impl VimObjectTrait for VsanClusterHealthLinkBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthLinkBase
    }
}

impl VimObjectTrait for VsanClusterHealthLink {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthLink
    }
}

impl VimObjectTrait for VsanClusterHealthQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthQuerySpec
    }
}

impl VimObjectTrait for VsanClusterHealthResultBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultBase
    }
}

impl VimObjectTrait for VsanClusterHealthResultTable {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultTable
    }
}

impl VimObjectTrait for VsanClusterHealthResultWithRemediation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultWithRemediation
    }
}

impl VimObjectTrait for VsanClusterHealthResultColumnInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultColumnInfo
    }
}

impl VimObjectTrait for VsanClusterHealthResultKeyValuePair {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultKeyValuePair
    }
}

impl VimObjectTrait for VsanClusterHealthResultRow {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthResultRow
    }
}

impl VimObjectTrait for VsanClusterHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthSummary
    }
}

impl VimObjectTrait for VsanClusterHealthSystemObjectsRepairResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthSystemObjectsRepairResult
    }
}

impl VimObjectTrait for VsanClusterHealthSystemStatusResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthSystemStatusResult
    }
}

impl VimObjectTrait for VsanClusterHealthSystemVersionResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthSystemVersionResult
    }
}

impl VimObjectTrait for VsanClusterHealthTest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHealthTest
    }
}

impl VimObjectTrait for VsanClusterHostVmknicMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterHostVmknicMapping
    }
}

impl VimObjectTrait for VsanClusterLimitHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterLimitHealthResult
    }
}

impl VimObjectTrait for VsanClusterNetworkHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterNetworkHealthResult
    }
}

impl VimObjectTrait for VsanClusterNetworkLoadTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterNetworkLoadTestResult
    }
}

impl VimObjectTrait for VsanClusterNetworkPartitionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterNetworkPartitionInfo
    }
}

impl VimObjectTrait for VsanClusterNetworkPerfTaskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterNetworkPerfTaskSpec
    }
}

impl VimObjectTrait for VsanClusterProactiveTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterProactiveTestResult
    }
}

impl VimObjectTrait for VsanClusterTelemetryProxyConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterTelemetryProxyConfig
    }
}

impl VimObjectTrait for VsanClusterVMsHealthOverallResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterVMsHealthOverallResult
    }
}

impl VimObjectTrait for VsanClusterVMsHealthSummaryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterVMsHealthSummaryResult
    }
}

impl VimObjectTrait for VsanClusterVmdkLoadTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterVmdkLoadTestResult
    }
}

impl VimObjectTrait for VsanClusterWhatifHostFailuresResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterWhatifHostFailuresResult
    }
}

impl VimObjectTrait for VsanComponentBasicInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComponentBasicInfo
    }
}

impl VimObjectTrait for VsanComponentPlacement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComponentPlacement
    }
}

impl VimObjectTrait for VsanConfigGeneration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanConfigGeneration
    }
}

impl VimObjectTrait for VsanDataDrivenApiAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataDrivenApiAction
    }
}

impl VimObjectTrait for VsanDiagnosticsThreshold {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiagnosticsThreshold
    }
}

impl VimObjectTrait for VsanDiskFormatConversionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskFormatConversionSpec
    }
}

impl VimObjectTrait for VimClusterVsanDiskMappingsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanDiskMappingsConfigSpec
    }
}

impl VimObjectTrait for VsanEffectiveSpaceUsage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEffectiveSpaceUsage
    }
}

impl VimObjectTrait for VsanEntitySpaceUsage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEntitySpaceUsage
    }
}

impl VimObjectTrait for VimClusterVsanFaultDomainSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanFaultDomainSpec
    }
}

impl VimObjectTrait for VsanFaultDomainDestroySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFaultDomainDestroySpec
    }
}

impl VimObjectTrait for VsanFaultDomainUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFaultDomainUpdateSpec
    }
}

impl VimObjectTrait for VimClusterVsanFaultDomainsConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanFaultDomainsConfigSpec
    }
}

impl VimObjectTrait for VsanHealthActionBase {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthActionBase
    }
}

impl VimObjectTrait for VsanHealthActionSteps {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthActionSteps
    }
}

impl VimObjectTrait for VsanHealthApiBasedAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthApiBasedAction
    }
}

impl VimObjectTrait for VsanHealthCmdBasedAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthCmdBasedAction
    }
}

impl VimObjectTrait for VsanHealthDataDrivenAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthDataDrivenAction
    }
}

impl VimObjectTrait for VsanHealthTxtBasedAction {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthTxtBasedAction
    }
}

impl VimObjectTrait for VsanHealthConfirmationDialog {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthConfirmationDialog
    }
}

impl VimObjectTrait for VsanHealthCorrelation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthCorrelation
    }
}

impl VimObjectTrait for VsanHealthExtMgmtPreCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthExtMgmtPreCheckResult
    }
}

impl VimObjectTrait for VsanHealthStatusCounts {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthStatusCounts
    }
}

impl VimObjectTrait for VsanHealthTroubleshooting {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthTroubleshooting
    }
}

impl VimObjectTrait for VsanHistoricalHealthQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHistoricalHealthQuerySpec
    }
}

impl VimObjectTrait for VsanHistoricalHealthTest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHistoricalHealthTest
    }
}

impl VimObjectTrait for VsanHostClomdLivenessResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClomdLivenessResult
    }
}

impl VimObjectTrait for VsanHostCreateVmHealthTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostCreateVmHealthTestResult
    }
}

impl VimObjectTrait for VimClusterVsanHostDiskMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanHostDiskMapping
    }
}

impl VimObjectTrait for VsanHostHealthSystemVersionResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHealthSystemVersionResult
    }
}

impl VimObjectTrait for VsanIoInsightInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoInsightInstance
    }
}

impl VimObjectTrait for VsanIoInsightInstanceQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoInsightInstanceQuerySpec
    }
}

impl VimObjectTrait for VsanIscsiHomeObjectSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiHomeObjectSpec
    }
}

impl VimObjectTrait for VsanIscsiInitiatorGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiInitiatorGroup
    }
}

impl VimObjectTrait for VsanIscsiLunCommonInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiLunCommonInfo
    }
}

impl VimObjectTrait for VsanIscsiLun {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiLun
    }
}

impl VimObjectTrait for VsanIscsiLunSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiLunSpec
    }
}

impl VimObjectTrait for VsanIscsiTargetAuthSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetAuthSpec
    }
}

impl VimObjectTrait for VsanIscsiTargetBasicInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetBasicInfo
    }
}

impl VimObjectTrait for VsanIscsiTargetCommonInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetCommonInfo
    }
}

impl VimObjectTrait for VsanIscsiTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTarget
    }
}

impl VimObjectTrait for VsanIscsiTargetSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetSpec
    }
}

impl VimObjectTrait for VsanIscsiTargetServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetServiceConfig
    }
}

impl VimObjectTrait for VsanIscsiTargetServiceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetServiceSpec
    }
}

impl VimObjectTrait for VsanIscsiTargetServiceDefaultConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiTargetServiceDefaultConfigSpec
    }
}

impl VimObjectTrait for VsanNetworkDiagnostics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkDiagnostics
    }
}

impl VimObjectTrait for VsanObjIdentityQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjIdentityQuerySpec
    }
}

impl VimObjectTrait for VsanClusterObjectExtAttrs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterObjectExtAttrs
    }
}

impl VimObjectTrait for VsanObjectExtraAttributes {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectExtraAttributes
    }
}

impl VimObjectTrait for VsanObjectIdentity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectIdentity
    }
}

impl VimObjectTrait for VsanObjectIdentityAndHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectIdentityAndHealth
    }
}

impl VimObjectTrait for VsanObjectInformation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectInformation
    }
}

impl VimObjectTrait for VsanObjectPlacement {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectPlacement
    }
}

impl VimObjectTrait for VsanObjectPlacementDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectPlacementDetails
    }
}

impl VimObjectTrait for VsanObjectQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectQuerySpec
    }
}

impl VimObjectTrait for VsanObjectSpaceSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectSpaceSummary
    }
}

impl VimObjectTrait for VsanPerfDiagnoseQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfDiagnoseQuerySpec
    }
}

impl VimObjectTrait for VsanPerfDiagnosticException {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfDiagnosticException
    }
}

impl VimObjectTrait for VsanPerfDiagnosticResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfDiagnosticResult
    }
}

impl VimObjectTrait for VsanPerfEntityMetricCsv {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfEntityMetricCsv
    }
}

impl VimObjectTrait for VsanPerfEntityType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfEntityType
    }
}

impl VimObjectTrait for VsanPerfGraph {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfGraph
    }
}

impl VimObjectTrait for VsanPerfHotspotEntitiesMetrics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfHotspotEntitiesMetrics
    }
}

impl VimObjectTrait for VsanPerfHotspotEntityType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfHotspotEntityType
    }
}

impl VimObjectTrait for VsanPerfHotspotQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfHotspotQuerySpec
    }
}

impl VimObjectTrait for VsanPerfMasterInformation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfMasterInformation
    }
}

impl VimObjectTrait for VsanPerfMemberInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfMemberInfo
    }
}

impl VimObjectTrait for VsanPerfMetricId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfMetricId
    }
}

impl VimObjectTrait for VsanPerfMetricSeriesCsv {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfMetricSeriesCsv
    }
}

impl VimObjectTrait for VsanPerfNodeInformation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfNodeInformation
    }
}

impl VimObjectTrait for VsanPerfQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfQuerySpec
    }
}

impl VimObjectTrait for VsanPerfThreshold {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfThreshold
    }
}

impl VimObjectTrait for VsanPerfTimeRange {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfTimeRange
    }
}

impl VimObjectTrait for VsanPerfTimeRangeQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfTimeRangeQuerySpec
    }
}

impl VimObjectTrait for VsanPerfTopEntities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfTopEntities
    }
}

impl VimObjectTrait for VsanPerfTopEntity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfTopEntity
    }
}

impl VimObjectTrait for VsanPerfTopQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfTopQuerySpec
    }
}

impl VimObjectTrait for VsanPerfsvcConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfsvcConfig
    }
}

impl VimObjectTrait for VsanQueryPhysicalPlacementSpecs {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanQueryPhysicalPlacementSpecs
    }
}

impl VimObjectTrait for VsanRemoteClusterQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRemoteClusterQuerySpec
    }
}

impl VimObjectTrait for VsanSnapshotSpace {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSnapshotSpace
    }
}

impl VimObjectTrait for VsanSpaceQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceQuerySpec
    }
}

impl VimObjectTrait for VsanSpaceUsage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceUsage
    }
}

impl VimObjectTrait for VsanSpaceUsageDetailResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceUsageDetailResult
    }
}

impl VimObjectTrait for VsanSpaceUsageWithDatastoreType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceUsageWithDatastoreType
    }
}

impl VimObjectTrait for VsanStorageWorkloadType {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStorageWorkloadType
    }
}

impl VimObjectTrait for VsanStretchedClusterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStretchedClusterConfig
    }
}

impl VimObjectTrait for VsanSyncingObjectFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSyncingObjectFilter
    }
}

impl VimObjectTrait for VsanUnicastAddressInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUnicastAddressInfo
    }
}

impl VimObjectTrait for VsanVcKmipServersHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcKmipServersHealth
    }
}

impl VimObjectTrait for VsanVcLifecycleCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcLifecycleCheckResult
    }
}

impl VimObjectTrait for VsanVcLifecycleCheckSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcLifecycleCheckSpec
    }
}

impl VimObjectTrait for VsanVsanClusterPcapGroup {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVsanClusterPcapGroup
    }
}

impl VimObjectTrait for VsanVsanClusterPcapResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVsanClusterPcapResult
    }
}

impl VimObjectTrait for VsanVumSystemConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVumSystemConfig
    }
}

impl VimObjectTrait for VsanWhatifCapacity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanWhatifCapacity
    }
}

impl VimObjectTrait for VimClusterVsanWitnessSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimClusterVsanWitnessSpec
    }
}

impl VimObjectTrait for CnsAccessControlSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsAccessControlSpec
    }
}

impl VimObjectTrait for CnsNfsAccessControlSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsNfsAccessControlSpec
    }
}

impl VimObjectTrait for CnsBackingObjectDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsBackingObjectDetails
    }
}

impl VimObjectTrait for CnsBlockBackingDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsBlockBackingDetails
    }
}

impl VimObjectTrait for CnsFileBackingDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsFileBackingDetails
    }
}

impl VimObjectTrait for CnsVsanFileShareBackingDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVsanFileShareBackingDetails
    }
}

impl VimObjectTrait for CnsBaseCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsBaseCreateSpec
    }
}

impl VimObjectTrait for CnsBlockCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsBlockCreateSpec
    }
}

impl VimObjectTrait for CnsFileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsFileCreateSpec
    }
}

impl VimObjectTrait for CnsVsanFileCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVsanFileCreateSpec
    }
}

impl VimObjectTrait for CnsContainerCluster {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsContainerCluster
    }
}

impl VimObjectTrait for CnsCursor {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsCursor
    }
}

impl VimObjectTrait for CnsEntityMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsEntityMetadata
    }
}

impl VimObjectTrait for CnsKubernetesEntityMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsKubernetesEntityMetadata
    }
}

impl VimObjectTrait for CnsKubernetesEntityReference {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsKubernetesEntityReference
    }
}

impl VimObjectTrait for CnsPlacementResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsPlacementResult
    }
}

impl VimObjectTrait for CnsQueryFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsQueryFilter
    }
}

impl VimObjectTrait for CnsKubernetesQueryFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsKubernetesQueryFilter
    }
}

impl VimObjectTrait for CnsQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsQueryResult
    }
}

impl VimObjectTrait for CnsQuerySelection {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsQuerySelection
    }
}

impl VimObjectTrait for CnsSnapshotCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsSnapshotCreateSpec
    }
}

impl VimObjectTrait for CnsSnapshotDeleteSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsSnapshotDeleteSpec
    }
}

impl VimObjectTrait for CnsSnapshotId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsSnapshotId
    }
}

impl VimObjectTrait for CnsSyncVolumeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsSyncVolumeSpec
    }
}

impl VimObjectTrait for CnsUnregisterVolumeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsUnregisterVolumeSpec
    }
}

impl VimObjectTrait for CnsVolume {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolume
    }
}

impl VimObjectTrait for CnsVolumeAclConfigureSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeAclConfigureSpec
    }
}

impl VimObjectTrait for CnsVolumeAttachDetachSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeAttachDetachSpec
    }
}

impl VimObjectTrait for CnsVolumeCreateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeCreateSpec
    }
}

impl VimObjectTrait for CnsVolumeCryptoUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeCryptoUpdateSpec
    }
}

impl VimObjectTrait for CnsVolumeExtendSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeExtendSpec
    }
}

impl VimObjectTrait for CnsVolumeId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeId
    }
}

impl VimObjectTrait for CnsVolumeMetadata {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeMetadata
    }
}

impl VimObjectTrait for CnsVolumeMetadataUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeMetadataUpdateSpec
    }
}

impl VimObjectTrait for CnsVolumeOperationBatchResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeOperationBatchResult
    }
}

impl VimObjectTrait for CnsVolumeOperationResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeOperationResult
    }
}

impl VimObjectTrait for CnsAsyncQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsAsyncQueryResult
    }
}

impl VimObjectTrait for CnsVolumeAttachResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeAttachResult
    }
}

impl VimObjectTrait for CnsVolumeCreateResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeCreateResult
    }
}

impl VimObjectTrait for CnsVolumePolicyReconfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumePolicyReconfigSpec
    }
}

impl VimObjectTrait for CnsVolumeRelocateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeRelocateSpec
    }
}

impl VimObjectTrait for CnsBlockVolumeRelocateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsBlockVolumeRelocateSpec
    }
}

impl VimObjectTrait for CnsVolumeSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsVolumeSource
    }
}

impl VimObjectTrait for CnsCloneVolumeSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsCloneVolumeSource
    }
}

impl VimObjectTrait for CnsSnapshotVolumeSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CnsSnapshotVolumeSource
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

impl VimObjectTrait for DvPortgroupNsxConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupNsxConfig
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

impl VimObjectTrait for DvPortgroupNsxSubnetAddressInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvPortgroupNsxSubnetAddressInfo
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

impl VimObjectTrait for DistributedVirtualSwitchManagerSpanInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchManagerSpanInfo
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

impl VimObjectTrait for DvsFilterSpecConnecteeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecConnecteeSpec
    }
}

impl VimObjectTrait for DvsFilterSpecPnicConnecteeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecPnicConnecteeSpec
    }
}

impl VimObjectTrait for DvsFilterSpecVmConnecteeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecVmConnecteeSpec
    }
}

impl VimObjectTrait for DvsFilterSpecVmknicConnecteeSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecVmknicConnecteeSpec
    }
}

impl VimObjectTrait for DvsFilterSpecVlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecVlanSpec
    }
}

impl VimObjectTrait for DvsFilterSpecPvlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecPvlanSpec
    }
}

impl VimObjectTrait for DvsFilterSpecTrunkVlanSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecTrunkVlanSpec
    }
}

impl VimObjectTrait for DvsFilterSpecVlanIdSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DvsFilterSpecVlanIdSpec
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

impl VimObjectTrait for DistributedVirtualSwitchHostMemberHostPerfNicOffloadState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberHostPerfNicOffloadState
    }
}

impl VimObjectTrait for DistributedVirtualSwitchHostMemberHostUplinkState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DistributedVirtualSwitchHostMemberHostUplinkState
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

impl VimObjectTrait for VmwareDistributedVirtualSwitchDpuFailoverPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchDpuFailoverPolicy
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

impl VimObjectTrait for VmwareDistributedVirtualSwitchNetworkOffloadConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchNetworkOffloadConfig
    }
}

impl VimObjectTrait for VMwareDvsPerfNicOffloadCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsPerfNicOffloadCapability
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

impl VimObjectTrait for VmwareDistributedVirtualSwitchRealTimeConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchRealTimeConfig
    }
}

impl VimObjectTrait for VmwareDistributedVirtualSwitchRealTimeLanAnnotation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmwareDistributedVirtualSwitchRealTimeLanAnnotation
    }
}

impl VimObjectTrait for VMwareDvsSystemTrafficCapabilities {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VMwareDvsSystemTrafficCapabilities
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

impl VimObjectTrait for CryptoManagerKmipCryptoKeyStatusKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCryptoKeyStatusKeyInfo
    }
}

impl VimObjectTrait for CryptoManagerKmipCryptoKeyStatusWrappingKeyIdKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCryptoKeyStatusWrappingKeyIdKeyInfo
    }
}

impl VimObjectTrait for CryptoManagerKmipCryptoKeyStatusWrappingRotationIntervalKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipCryptoKeyStatusWrappingRotationIntervalKeyInfo
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

impl VimObjectTrait for CryptoManagerKmipGenerateKeySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CryptoManagerKmipGenerateKeySpec
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

impl VimObjectTrait for KmipClusterInfoKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipClusterInfoKeyInfo
    }
}

impl VimObjectTrait for KmipClusterInfoWrappingKeyIdKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipClusterInfoWrappingKeyIdKeyInfo
    }
}

impl VimObjectTrait for KmipClusterInfoWrappingRotationIntervalKeyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipClusterInfoWrappingRotationIntervalKeyInfo
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

impl VimObjectTrait for KmipServerSpecKeySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerSpecKeySpec
    }
}

impl VimObjectTrait for KmipServerSpecWrappingKeyIdKeySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerSpecWrappingKeyIdKeySpec
    }
}

impl VimObjectTrait for KmipServerSpecWrappingRotationIntervalKeySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::KmipServerSpecWrappingRotationIntervalKeySpec
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

impl VimObjectTrait for TgwEventArgument {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TgwEventArgument
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

impl VimObjectTrait for EventManagerEventViewSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventManagerEventViewSpec
    }
}

impl VimObjectTrait for EventManagerViewByStartId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EventManagerViewByStartId
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

impl VimObjectTrait for HostAuthenticationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostAuthenticationInfo
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

impl VimObjectTrait for HostCpuSchedulerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostCpuSchedulerInfo
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

impl VimObjectTrait for DevicePciId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DevicePciId
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

impl VimObjectTrait for HostSpbmDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpbmDatastoreInfo
    }
}

impl VimObjectTrait for HostSpbmHashInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpbmHashInfo
    }
}

impl VimObjectTrait for HostSpbmPolicyBlobInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpbmPolicyBlobInfo
    }
}

impl VimObjectTrait for HostSpbmPolicyInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostSpbmPolicyInfo
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

impl VimObjectTrait for VsanFileServiceIpConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceIpConfig
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

impl VimObjectTrait for LacpInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LacpInfo
    }
}

impl VimObjectTrait for LagInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LagInfo
    }
}

impl VimObjectTrait for LagUplinkInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LagUplinkInfo
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

impl VimObjectTrait for HostMaintenanceSpecEvacuationMode {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostMaintenanceSpecEvacuationMode
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

impl VimObjectTrait for HostPartialMaintenanceModeRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPartialMaintenanceModeRuntimeInfo
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

impl VimObjectTrait for HostPciDeviceDirectPathInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPciDeviceDirectPathInfo
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

impl VimObjectTrait for HostPciPassthruInfoDirectPathState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostPciPassthruInfoDirectPathState
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

impl VimObjectTrait for PnicTsoInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PnicTsoInfo
    }
}

impl VimObjectTrait for PodVmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PodVmInfo
    }
}

impl VimObjectTrait for PodVmOverheadInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::PodVmOverheadInfo
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

impl VimObjectTrait for HostTdxInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTdxInfo
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

impl VimObjectTrait for HostTpmSystemVersionEventDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::HostTpmSystemVersionEventDetails
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

impl VimObjectTrait for VimHostVsanStretchedClusterHostCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimHostVsanStretchedClusterHostCapability
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

impl VimObjectTrait for VsanBasicDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanBasicDeviceInfo
    }
}

impl VimObjectTrait for VsanClusterMembershipInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterMembershipInfo
    }
}

impl VimObjectTrait for VsanDaemonHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDaemonHealth
    }
}

impl VimObjectTrait for VsanDiskEncryptionHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskEncryptionHealth
    }
}

impl VimObjectTrait for VsanDiskRebalanceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskRebalanceResult
    }
}

impl VimObjectTrait for VsanDitEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDitEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanFailedRepairObjectResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFailedRepairObjectResult
    }
}

impl VimObjectTrait for VsanFileServerHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServerHealthSummary
    }
}

impl VimObjectTrait for VsanFileServiceBalanceHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceBalanceHealth
    }
}

impl VimObjectTrait for VsanFileServiceHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceHealthSummary
    }
}

impl VimObjectTrait for VsanFileServiceRootFsHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceRootFsHealth
    }
}

impl VimObjectTrait for VsanFileServiceShareHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceShareHealthSummary
    }
}

impl VimObjectTrait for VsanHclCommonDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclCommonDeviceInfo
    }
}

impl VimObjectTrait for VsanHclNicInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclNicInfo
    }
}

impl VimObjectTrait for VsanHclComputeResource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclComputeResource
    }
}

impl VimObjectTrait for VsanHclControllerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclControllerInfo
    }
}

impl VimObjectTrait for VsanHclDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclDiskInfo
    }
}

impl VimObjectTrait for VsanHclFirmwareFile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclFirmwareFile
    }
}

impl VimObjectTrait for VsanHclFirmwareUpdateSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclFirmwareUpdateSpec
    }
}

impl VimObjectTrait for VsanHclMemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclMemInfo
    }
}

impl VimObjectTrait for VsanHealthObjectStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthObjectStats
    }
}

impl VimObjectTrait for VsanHealthQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthQuerySpec
    }
}

impl VimObjectTrait for VsanHostCimProviderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostCimProviderInfo
    }
}

impl VimObjectTrait for VsanHostEmmSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostEmmSummary
    }
}

impl VimObjectTrait for VsanHostFwComponent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostFwComponent
    }
}

impl VimObjectTrait for VsanHostGlobalDedupConfigHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostGlobalDedupConfigHealthSummary
    }
}

impl VimObjectTrait for VsanHostHciMeshDitEncryptionHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHciMeshDitEncryptionHealth
    }
}

impl VimObjectTrait for VsanHostHciMeshDitEncryptionHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHciMeshDitEncryptionHealthSummary
    }
}

impl VimObjectTrait for VsanHostHclInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHclInfo
    }
}

impl VimObjectTrait for VsanHostHealthSystemStatusResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHealthSystemStatusResult
    }
}

impl VimObjectTrait for VsanHostHwDeviceId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostHwDeviceId
    }
}

impl VimObjectTrait for VsanHostIoInsightInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostIoInsightInfo
    }
}

impl VimObjectTrait for VsanHostQueryCheckLimitsSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostQueryCheckLimitsSpec
    }
}

impl VimObjectTrait for VsanHostReference {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostReference
    }
}

impl VimObjectTrait for VsanHostVirtualApplianceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostVirtualApplianceInfo
    }
}

impl VimObjectTrait for VsanHostVmdkLoadTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostVmdkLoadTestResult
    }
}

impl VimObjectTrait for VsanHwToVcgInfoMapping {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHwToVcgInfoMapping
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

impl VimObjectTrait for VsanIoInsightInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoInsightInfo
    }
}

impl VimObjectTrait for VsanIperfClientSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIperfClientSpec
    }
}

impl VimObjectTrait for VsanKmsHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanKmsHealth
    }
}

impl VimObjectTrait for VsanLicensedDiskResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanLicensedDiskResult
    }
}

impl VimObjectTrait for VsanLimitHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanLimitHealthResult
    }
}

impl VimObjectTrait for VsanNetworkDiagnosticsHealthInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkDiagnosticsHealthInfo
    }
}

impl VimObjectTrait for VsanNetworkHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkHealthResult
    }
}

impl VimObjectTrait for VsanNetworkLoadTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkLoadTestResult
    }
}

impl VimObjectTrait for VsanNetworkPeerHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkPeerHealthResult
    }
}

impl VimObjectTrait for VsanNicRdmaInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNicRdmaInfo
    }
}

impl VimObjectTrait for VsanObjectHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectHealth
    }
}

impl VimObjectTrait for VsanObjectOverallHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectOverallHealth
    }
}

impl VimObjectTrait for VsanPhysicalDiskHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPhysicalDiskHealth
    }
}

impl VimObjectTrait for VsanPhysicalDiskHealthSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPhysicalDiskHealthSummary
    }
}

impl VimObjectTrait for VsanProactiveRebalanceInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanProactiveRebalanceInfoEx
    }
}

impl VimObjectTrait for VsanQueryResultHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanQueryResultHostInfo
    }
}

impl VimObjectTrait for VsanRepairObjectsResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRepairObjectsResult
    }
}

impl VimObjectTrait for VsanResourceHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceHealth
    }
}

impl VimObjectTrait for VsanServerClusterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanServerClusterInfo
    }
}

impl VimObjectTrait for VsanSmartDiskStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSmartDiskStats
    }
}

impl VimObjectTrait for VsanSmartParameter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSmartParameter
    }
}

impl VimObjectTrait for VsanSmartStatsHostSummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSmartStatsHostSummary
    }
}

impl VimObjectTrait for VsanVcgDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcgDeviceInfo
    }
}

impl VimObjectTrait for VsanVmdkIoLoadSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVmdkIoLoadSpec
    }
}

impl VimObjectTrait for VsanVmdkLoadTestResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVmdkLoadTestResult
    }
}

impl VimObjectTrait for VsanVmdkLoadTestSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVmdkLoadTestSpec
    }
}

impl VimObjectTrait for VsanVsanPcapResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVsanPcapResult
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

impl VimObjectTrait for TagSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::TagSpec
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

impl VimObjectTrait for VirtualMachineGuestRebootStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualMachineGuestRebootStatus
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

impl VimObjectTrait for SnapshotSelectionSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SnapshotSelectionSpec
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

impl VimObjectTrait for SubnetInfoFolderInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SubnetInfoFolderInfo
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

impl VimObjectTrait for SubnetInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SubnetInfo
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

impl VimObjectTrait for VmPlacementPolicy {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmPlacementPolicy
    }
}

impl VimObjectTrait for VmToVmGroupsAntiAffinity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmToVmGroupsAntiAffinity
    }
}

impl VimObjectTrait for VmVmAffinity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmVmAffinity
    }
}

impl VimObjectTrait for VmVmAntiAffinity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VmVmAntiAffinity
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

impl VimObjectTrait for CustomizationLinuxFlexPrep {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationLinuxFlexPrep
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

impl VimObjectTrait for CustomizationWindowsFlexPrep {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationWindowsFlexPrep
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

impl VimObjectTrait for CustomizationDisableIpV4 {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::CustomizationDisableIpV4
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

impl VimObjectTrait for VirtualVmxnet3StrictLatencyConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3StrictLatencyConfig
    }
}

impl VimObjectTrait for VirtualVmxnet3OptionStrictLatencyConfigOption {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VirtualVmxnet3OptionStrictLatencyConfigOption
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

impl VimObjectTrait for FaultDomainInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FaultDomainInfo
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

impl VimObjectTrait for VsanAutoRaidConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanAutoRaidConfig
    }
}

impl VimObjectTrait for VsanAutoRaidInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanAutoRaidInfo
    }
}

impl VimObjectTrait for VsanCapacityReservationInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCapacityReservationInfo
    }
}

impl VimObjectTrait for VsanClientUnicastConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClientUnicastConfig
    }
}

impl VimObjectTrait for ClusterRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ClusterRuntimeInfo
    }
}

impl VimObjectTrait for VsanCompatibilityCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCompatibilityCheckResult
    }
}

impl VimObjectTrait for VsanCyberRecoveryConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCyberRecoveryConfig
    }
}

impl VimObjectTrait for VimVsanDataEfficiencyCapacityState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanDataEfficiencyCapacityState
    }
}

impl VimObjectTrait for VsanDataEfficiencyConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataEfficiencyConfig
    }
}

impl VimObjectTrait for VsanDataEfficiencyConfigEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataEfficiencyConfigEx
    }
}

impl VimObjectTrait for VsanDataEncryptionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataEncryptionConfig
    }
}

impl VimObjectTrait for VsanDataInTransitEncryptionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDataInTransitEncryptionConfig
    }
}

impl VimObjectTrait for VsanDatastoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreConfig
    }
}

impl VimObjectTrait for VsanAdvancedDatastoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanAdvancedDatastoreConfig
    }
}

impl VimObjectTrait for VsanDatastoreSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreSpec
    }
}

impl VimObjectTrait for VsanClientDatastoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClientDatastoreConfig
    }
}

impl VimObjectTrait for VsanXvcClientConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcClientConfig
    }
}

impl VimObjectTrait for DefaultDatastorePolicySelectionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DefaultDatastorePolicySelectionInfo
    }
}

impl VimObjectTrait for VsanDirectoryServerConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDirectoryServerConfig
    }
}

impl VimObjectTrait for ActiveVsanDirectoryServerConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ActiveVsanDirectoryServerConfig
    }
}

impl VimObjectTrait for DiskClaimConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskClaimConfiguration
    }
}

impl VimObjectTrait for DiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DiskInfo
    }
}

impl VimObjectTrait for DpDaemonHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::DpDaemonHealth
    }
}

impl VimObjectTrait for VsanEntityCompatibilityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEntityCompatibilityResult
    }
}

impl VimObjectTrait for EntityResourceCheckDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::EntityResourceCheckDetails
    }
}

impl VimObjectTrait for VsanDiskGroupResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskGroupResourceCheckResult
    }
}

impl VimObjectTrait for VsanDiskResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskResourceCheckResult
    }
}

impl VimObjectTrait for VsanStoragePoolDiskResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStoragePoolDiskResourceCheckResult
    }
}

impl VimObjectTrait for VsanFaultDomainResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFaultDomainResourceCheckResult
    }
}

impl VimObjectTrait for VsanHostResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostResourceCheckResult
    }
}

impl VimObjectTrait for VsanResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckResult
    }
}

impl VimObjectTrait for VsanResourceCheckComponentResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckComponentResult
    }
}

impl VimObjectTrait for VsanResourceCheckDataPersistenceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckDataPersistenceResult
    }
}

impl VimObjectTrait for VsanResourceCheckVsanResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckVsanResult
    }
}

impl VimObjectTrait for VsanSiteMaintenanceResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenanceResourceCheckResult
    }
}

impl VimObjectTrait for VsanStoragePoolResourceCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStoragePoolResourceCheckResult
    }
}

impl VimObjectTrait for VsanFileServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceConfig
    }
}

impl VimObjectTrait for VsanFileServiceDomain {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceDomain
    }
}

impl VimObjectTrait for VsanFileServiceDomainConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceDomainConfig
    }
}

impl VimObjectTrait for VsanFileServiceDomainQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceDomainQuerySpec
    }
}

impl VimObjectTrait for VsanFileShare {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShare
    }
}

impl VimObjectTrait for VsanFileShareConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareConfig
    }
}

impl VimObjectTrait for VsanFileShareNetPermission {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareNetPermission
    }
}

impl VimObjectTrait for VsanFileShareQueryProperties {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareQueryProperties
    }
}

impl VimObjectTrait for FileShareQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::FileShareQueryResult
    }
}

impl VimObjectTrait for VsanFileShareQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareQuerySpec
    }
}

impl VimObjectTrait for VsanFileShareRuntimeInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareRuntimeInfo
    }
}

impl VimObjectTrait for VsanFileShareSmbOptions {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareSmbOptions
    }
}

impl VimObjectTrait for VsanFileShareSnapshot {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareSnapshot
    }
}

impl VimObjectTrait for VsanFileShareSnapshotConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareSnapshotConfig
    }
}

impl VimObjectTrait for VsanFileShareSnapshotQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareSnapshotQueryResult
    }
}

impl VimObjectTrait for VsanFileShareSnapshotQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileShareSnapshotQuerySpec
    }
}

impl VimObjectTrait for VsanHciMeshDatastoreSource {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHciMeshDatastoreSource
    }
}

impl VimObjectTrait for VsanHostSiteMaintenanceStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostSiteMaintenanceStatus
    }
}

impl VimObjectTrait for VsanIoDiagnosticsFailedCheck {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsFailedCheck
    }
}

impl VimObjectTrait for VsanIoDiagnosticsInstance {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsInstance
    }
}

impl VimObjectTrait for VsanIoDiagnosticsInstanceEvent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsInstanceEvent
    }
}

impl VimObjectTrait for VsanIoDiagnosticsInstanceQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsInstanceQuerySpec
    }
}

impl VimObjectTrait for VsanIoDiagnosticsObjectLayout {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsObjectLayout
    }
}

impl VimObjectTrait for VsanIoDiagnosticsPrecheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsPrecheckResult
    }
}

impl VimObjectTrait for VsanIoDiagnosticsStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsStats
    }
}

impl VimObjectTrait for VsanIoDiagnosticsTarget {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsTarget
    }
}

impl VimObjectTrait for VsanIoDiagnosticsTargetStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoDiagnosticsTargetStats
    }
}

impl VimObjectTrait for VsanIoLatency {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoLatency
    }
}

impl VimObjectTrait for VsanIoLatencyMetrics {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoLatencyMetrics
    }
}

impl VimObjectTrait for LifecycleConfigDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LifecycleConfigDetails
    }
}

impl VimObjectTrait for LifecycleFaultDomainDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LifecycleFaultDomainDetails
    }
}

impl VimObjectTrait for LifecyclePreCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LifecyclePreCheckResult
    }
}

impl VimObjectTrait for LifecycleWitnessDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::LifecycleWitnessDetails
    }
}

impl VimObjectTrait for VsanMetricProfile {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMetricProfile
    }
}

impl VimObjectTrait for VsanMetricsConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMetricsConfig
    }
}

impl VimObjectTrait for VsanMountPrecheckItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckItem
    }
}

impl VimObjectTrait for VsanDatastoreSourcePrecheckItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreSourcePrecheckItem
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkConnectivityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkConnectivityResult
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkLatencyResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkLatencyResult
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkConnectivity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkConnectivity
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkConnectivityDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkConnectivityDetail
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkLatency {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkLatency
    }
}

impl VimObjectTrait for VsanMountPrecheckNetworkLatencyDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckNetworkLatencyDetail
    }
}

impl VimObjectTrait for VsanMountPrecheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanMountPrecheckResult
    }
}

impl VimObjectTrait for VsanDatastoreSourcePrecheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreSourcePrecheckResult
    }
}

impl VimObjectTrait for VsanObjectHealthTelemetrySummary {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectHealthTelemetrySummary
    }
}

impl VimObjectTrait for VsanObjectIoStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectIoStats
    }
}

impl VimObjectTrait for VsanProactiveRebalanceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanProactiveRebalanceInfo
    }
}

impl VimObjectTrait for VsanRdmaConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRdmaConfig
    }
}

impl VimObjectTrait for VsanRemoteVcInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRemoteVcInfo
    }
}

impl VimObjectTrait for VsanRemoteVcInfoStandalone {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRemoteVcInfoStandalone
    }
}

impl VimObjectTrait for RemoteVsanSite {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoteVsanSite
    }
}

impl VimObjectTrait for RemoteVsanSiteAffinity {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoteVsanSiteAffinity
    }
}

impl VimObjectTrait for RepairTimerInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RepairTimerInfo
    }
}

impl VimObjectTrait for VsanResourceCheckSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckSpec
    }
}

impl VimObjectTrait for VsanResourceCheckStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckStatus
    }
}

impl VimObjectTrait for VsanResourceCheckTaskDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanResourceCheckTaskDetails
    }
}

impl VimObjectTrait for VsanDiskDataEvacuationResourceCheckTaskDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskDataEvacuationResourceCheckTaskDetails
    }
}

impl VimObjectTrait for VsanSiteMaintenanceCheckTaskDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenanceCheckTaskDetails
    }
}

impl VimObjectTrait for ResyncIopsInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::ResyncIopsInfo
    }
}

impl VimObjectTrait for VsanRuntimeStatsHostMap {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanRuntimeStatsHostMap
    }
}

impl VimObjectTrait for SsdEnduranceThresholdSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SsdEnduranceThresholdSpec
    }
}

impl VimObjectTrait for VsanServerHostUnicastInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanServerHostUnicastInfo
    }
}

impl VimObjectTrait for VsanSharedWitnessCompatibilityResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSharedWitnessCompatibilityResult
    }
}

impl VimObjectTrait for VsanSiteMaintenanceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenanceInfo
    }
}

impl VimObjectTrait for VsanSiteMaintenancePrecheckDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenancePrecheckDetail
    }
}

impl VimObjectTrait for VsanSiteMaintenancePrecheckStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenancePrecheckStatus
    }
}

impl VimObjectTrait for VsanSiteMaintenanceSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenanceSpec
    }
}

impl VimObjectTrait for VsanSnapServiceConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSnapServiceConfig
    }
}

impl VimObjectTrait for VsanSiteMaintenanceVmPowerOffInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteMaintenanceVmPowerOffInfo
    }
}

impl VimObjectTrait for VsanVbossConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVbossConfig
    }
}

impl VimObjectTrait for VsanVbossClusterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVbossClusterConfig
    }
}

impl VimObjectTrait for VsanVbossHostConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVbossHostConfig
    }
}

impl VimObjectTrait for VsanVbossObjectStoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVbossObjectStoreConfig
    }
}

impl VimObjectTrait for VcRemoteVsanServerClusterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcRemoteVsanServerClusterConfig
    }
}

impl VimObjectTrait for VcRemoteVsanServerClusterInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VcRemoteVsanServerClusterInfo
    }
}

impl VimObjectTrait for VsanIscsiVipConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiVipConfigSpec
    }
}

impl VimObjectTrait for VsanIscsiVipConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiVipConfig
    }
}

impl VimObjectTrait for VsanIscsiVipDVswitchConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiVipDVswitchConfig
    }
}

impl VimObjectTrait for VsanVipNetworkConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVipNetworkConfig
    }
}

impl VimObjectTrait for VsanIscsiVipVswitchConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIscsiVipVswitchConfig
    }
}

impl VimObjectTrait for VsanBurnInTest {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanBurnInTest
    }
}

impl VimObjectTrait for VsanBurnInTestCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanBurnInTestCheckResult
    }
}

impl VimObjectTrait for VsanCloudHealthStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCloudHealthStatus
    }
}

impl VimObjectTrait for VsanClusterBurnInTestResultList {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterBurnInTestResultList
    }
}

impl VimObjectTrait for VsanCompliantDriver {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCompliantDriver
    }
}

impl VimObjectTrait for VsanCompliantFirmware {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanCompliantFirmware
    }
}

impl VimObjectTrait for VsanConfigBaseIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanConfigBaseIssue
    }
}

impl VimObjectTrait for VsanConfigNotAllDisksClaimedIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanConfigNotAllDisksClaimedIssue
    }
}

impl VimObjectTrait for VsanConfigCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanConfigCheckResult
    }
}

impl VimObjectTrait for VsanDatastoreDefaultPolicySelectionConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDatastoreDefaultPolicySelectionConfig
    }
}

impl VimObjectTrait for VsanDeconvergedNetConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDeconvergedNetConfig
    }
}

impl VimObjectTrait for VsanDiskModelInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDiskModelInfo
    }
}

impl VimObjectTrait for VsanDownloadItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDownloadItem
    }
}

impl VimObjectTrait for VsanEsaConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEsaConfig
    }
}

impl VimObjectTrait for VsanEsaConfigInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEsaConfigInfo
    }
}

impl VimObjectTrait for VsanEsaDiskConfiguration {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanEsaDiskConfiguration
    }
}

impl VimObjectTrait for VsanExtendedConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanExtendedConfig
    }
}

impl VimObjectTrait for VsanFileServiceOvfSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServiceOvfSpec
    }
}

impl VimObjectTrait for VsanFileServicePreflightCheckResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanFileServicePreflightCheckResult
    }
}

impl VimObjectTrait for VsanGenericClusterBaseIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanGenericClusterBaseIssue
    }
}

impl VimObjectTrait for VsanGenericClusterBestPracticeHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanGenericClusterBestPracticeHealth
    }
}

impl VimObjectTrait for VsanHclDeviceConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclDeviceConstraint
    }
}

impl VimObjectTrait for VsanHclDiskConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclDiskConstraint
    }
}

impl VimObjectTrait for VsanHclDriverInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclDriverInfo
    }
}

impl VimObjectTrait for VsanHclMinFwConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclMinFwConstraint
    }
}

impl VimObjectTrait for VsanHclQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclQuerySpec
    }
}

impl VimObjectTrait for VsanHclReleaseConstraint {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHclReleaseConstraint
    }
}

impl VimObjectTrait for VsanHealthConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthConfigSpec
    }
}

impl VimObjectTrait for VsanHealthCustomizationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthCustomizationSpec
    }
}

impl VimObjectTrait for VsanHealthThreshold {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHealthThreshold
    }
}

impl VimObjectTrait for VsanHistoricalHealthConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHistoricalHealthConfig
    }
}

impl VimObjectTrait for VsanHostDeviceInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDeviceInfo
    }
}

impl VimObjectTrait for VsanHwToVcgInfoMappingSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHwToVcgInfoMappingSpec
    }
}

impl VimObjectTrait for VsanIoTripAnalyzerConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoTripAnalyzerConfig
    }
}

impl VimObjectTrait for VsanIoTripAnalyzerRecurrence {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanIoTripAnalyzerRecurrence
    }
}

impl VimObjectTrait for VsanInternalExtendedConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanInternalExtendedConfig
    }
}

impl VimObjectTrait for VsanNetworkConfigBaseIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigBaseIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigPnicSpeedInconsistencyIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigPnicSpeedInconsistencyIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigPortgroupWithNoRedundancyIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigPortgroupWithNoRedundancyIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigVdsScopeIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigVdsScopeIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigVsanNotOnVdsIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigVsanNotOnVdsIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigVswitchWithNoRedundancyIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigVswitchWithNoRedundancyIssue
    }
}

impl VimObjectTrait for VsanNetworkVMotionVmknicNotFountIssue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkVMotionVmknicNotFountIssue
    }
}

impl VimObjectTrait for VsanNetworkConfigBestPracticeHealth {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanNetworkConfigBestPracticeHealth
    }
}

impl VimObjectTrait for VsanObjSnapParams {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjSnapParams
    }
}

impl VimObjectTrait for VsanObjectDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectDetail
    }
}

impl VimObjectTrait for VsanObjectSnapshotId {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectSnapshotId
    }
}

impl VimObjectTrait for VimVsanVsanPMemConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanVsanPMemConfig
    }
}

impl VimObjectTrait for VsanPerfsvcHealthResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPerfsvcHealthResult
    }
}

impl VimObjectTrait for VsanPrepareVsanForVcsaSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPrepareVsanForVcsaSpec
    }
}

impl VimObjectTrait for VsanSnapshotDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSnapshotDetail
    }
}

impl VimObjectTrait for VsanSnapshotQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSnapshotQueryResult
    }
}

impl VimObjectTrait for VsanSnapshotQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSnapshotQuerySpec
    }
}

impl VimObjectTrait for VsanSpaceEfficiencyMetadataSize {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceEfficiencyMetadataSize
    }
}

impl VimObjectTrait for VsanSpaceEfficiencyRatio {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSpaceEfficiencyRatio
    }
}

impl VimObjectTrait for VsanUnmapConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUnmapConfig
    }
}

impl VimObjectTrait for VsanUpdateItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanUpdateItem
    }
}

impl VimObjectTrait for VsanVcPostDeployConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcPostDeployConfigSpec
    }
}

impl VimObjectTrait for VsanVcStretchedClusterConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcStretchedClusterConfigSpec
    }
}

impl VimObjectTrait for VsanVcsaDeploymentProgress {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVcsaDeploymentProgress
    }
}

impl VimObjectTrait for VsanVdsMigrationPlan {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVdsMigrationPlan
    }
}

impl VimObjectTrait for VsanVdsPgMigrationHostInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVdsPgMigrationHostInfo
    }
}

impl VimObjectTrait for VsanVdsPgMigrationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVdsPgMigrationSpec
    }
}

impl VimObjectTrait for VsanVdsPgMigrationVmInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVdsPgMigrationVmInfo
    }
}

impl VimObjectTrait for VsanVibInstallPreflightStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVibInstallPreflightStatus
    }
}

impl VimObjectTrait for VsanVibScanResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVibScanResult
    }
}

impl VimObjectTrait for VsanVibSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVibSpec
    }
}

impl VimObjectTrait for VsanVmVdsMigrationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVmVdsMigrationSpec
    }
}

impl VimObjectTrait for VsanVnicVdsMigrationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVnicVdsMigrationSpec
    }
}

impl VimObjectTrait for VsanVumConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanVumConfig
    }
}

impl VimObjectTrait for VsanWitnessHostConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanWitnessHostConfig
    }
}

impl VimObjectTrait for VsanXvcClientInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcClientInfo
    }
}

impl VimObjectTrait for VsanXvcDatastoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcDatastoreConfig
    }
}

impl VimObjectTrait for VsanXvcDatastoreInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcDatastoreInfo
    }
}

impl VimObjectTrait for VsanXvcClientInfoSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcClientInfoSpec
    }
}

impl VimObjectTrait for VsanXvcQueryCriteria {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcQueryCriteria
    }
}

impl VimObjectTrait for VsanXvcQueryFilter {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcQueryFilter
    }
}

impl VimObjectTrait for VsanXvcQueryPropertyValue {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcQueryPropertyValue
    }
}

impl VimObjectTrait for VsanXvcQueryResultSet {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcQueryResultSet
    }
}

impl VimObjectTrait for VsanXvcQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcQuerySpec
    }
}

impl VimObjectTrait for VsanXvcResultItem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanXvcResultItem
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

impl VimObjectTrait for VsanConfigInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanConfigInfoEx
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

impl VimObjectTrait for VsanClusterCoreConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterCoreConfig
    }
}

impl VimObjectTrait for VsanClusterCoreConfigSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanClusterCoreConfigSpec
    }
}

impl VimObjectTrait for VsanHostAbortWipeDiskStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostAbortWipeDiskStatus
    }
}

impl VimObjectTrait for VsanHostAboutInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostAboutInfoEx
    }
}

impl VimObjectTrait for VsanAddStoragePoolDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanAddStoragePoolDiskSpec
    }
}

impl VimObjectTrait for VsanHostClientClusterUnicastConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClientClusterUnicastConfig
    }
}

impl VimObjectTrait for VsanHostClientClusterUnicastInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClientClusterUnicastInfo
    }
}

impl VimObjectTrait for VsanHostClientHostUnicastInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostClientHostUnicastInfo
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

impl VimObjectTrait for VsanComplianceDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComplianceDetail
    }
}

impl VimObjectTrait for VsanComplianceResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComplianceResult
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

impl VimObjectTrait for VsanHostConfigInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostConfigInfoEx
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

impl VimObjectTrait for VsanHostPortConfigEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostPortConfigEx
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

impl VimObjectTrait for VsanHostCreateNativeKeyProviderSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostCreateNativeKeyProviderSpec
    }
}

impl VimObjectTrait for VsanInTransitEncryptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanInTransitEncryptionInfo
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

impl VimObjectTrait for VsanDeleteStoragePoolDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanDeleteStoragePoolDiskSpec
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

impl VimObjectTrait for VimVsanHostDiskMapInfoEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostDiskMapInfoEx
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

impl VimObjectTrait for VimVsanHostDiskMappingCreationSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostDiskMappingCreationSpec
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

impl VimObjectTrait for VimVsanHostDiskResultEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostDiskResultEx
    }
}

impl VimObjectTrait for VsanHostDrsStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostDrsStats
    }
}

impl VimObjectTrait for VsanHostEncryptionInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostEncryptionInfo
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

impl VimObjectTrait for VsanHostIpConfigEx {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostIpConfigEx
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

impl VimObjectTrait for VsanPolicyStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanPolicyStatus
    }
}

impl VimObjectTrait for VimVsanHostQueryVsanDisksSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostQueryVsanDisksSpec
    }
}

impl VimObjectTrait for RemoteVsanServerClusterConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::RemoteVsanServerClusterConfig
    }
}

impl VimObjectTrait for VsanHostRuntimeStats {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostRuntimeStats
    }
}

impl VimObjectTrait for VsanHostServerClusterUnicastConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostServerClusterUnicastConfig
    }
}

impl VimObjectTrait for VsanHostServerClusterUnicastInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostServerClusterUnicastInfo
    }
}

impl VimObjectTrait for SiteAffinityInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::SiteAffinityInfo
    }
}

impl VimObjectTrait for VsanSiteTakeoverConfig {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSiteTakeoverConfig
    }
}

impl VimObjectTrait for VsanStoragePoolDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanStoragePoolDisk
    }
}

impl VimObjectTrait for VimVsanHostStoragePoolDiskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostStoragePoolDiskInfo
    }
}

impl VimObjectTrait for VimVsanHostStoragePoolInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostStoragePoolInfo
    }
}

impl VimObjectTrait for VimVsanHostTrimDiskEntry {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostTrimDiskEntry
    }
}

impl VimObjectTrait for VimVsanHostTrimDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostTrimDiskSpec
    }
}

impl VimObjectTrait for VimVsanHostUpdateStoragePoolDiskSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostUpdateStoragePoolDiskSpec
    }
}

impl VimObjectTrait for VsanHostAssociatedObjects {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostAssociatedObjects
    }
}

impl VimObjectTrait for VsanHostAssociatedObjectsResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostAssociatedObjectsResult
    }
}

impl VimObjectTrait for VsanComplianceQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanComplianceQuerySpec
    }
}

impl VimObjectTrait for VsanHostComponentSyncState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostComponentSyncState
    }
}

impl VimObjectTrait for VimVsanHostVsanDirectStorage {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanDirectStorage
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

impl VimObjectTrait for VimVsanHostVsanDiskManagementSystemCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanDiskManagementSystemCapability
    }
}

impl VimObjectTrait for VimVsanHostVsanHostCapability {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanHostCapability
    }
}

impl VimObjectTrait for VimVsanHostVsanManagedDisksInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanManagedDisksInfo
    }
}

impl VimObjectTrait for VimVsanHostVsanManagedPMemInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanManagedPMemInfo
    }
}

impl VimObjectTrait for VsanObjectProfileInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanObjectProfileInfo
    }
}

impl VimObjectTrait for VsanHostVsanObjectSyncState {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostVsanObjectSyncState
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

impl VimObjectTrait for VimVsanHostVsanScsiDisk {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VimVsanHostVsanScsiDisk
    }
}

impl VimObjectTrait for VsanHostVsanObjectSyncQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostVsanObjectSyncQueryResult
    }
}

impl VimObjectTrait for VsanSyncingObjectRecoveryDetails {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanSyncingObjectRecoveryDetails
    }
}

impl VimObjectTrait for VsanWhatIfEvacDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanWhatIfEvacDetail
    }
}

impl VimObjectTrait for VsanWhatIfEvacResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanWhatIfEvacResult
    }
}

impl VimObjectTrait for VsanHostWipeDiskStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VsanHostWipeDiskStatus
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

impl VimObjectTrait for VStorageObjectReconcileResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectReconcileResult
    }
}

impl VimObjectTrait for VStorageObjectReconcileResultInvalidDiskPath {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectReconcileResultInvalidDiskPath
    }
}

impl VimObjectTrait for VStorageObjectReconcileResultReconcileDetail {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectReconcileResultReconcileDetail
    }
}

impl VimObjectTrait for VStorageObjectReconcileSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectReconcileSpec
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

impl VimObjectTrait for VStorageObjectAttachResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectAttachResult
    }
}

impl VimObjectTrait for VStorageObjectAttachSpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VStorageObjectAttachSpec
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

impl VimObjectTrait for AuthenticationRequiredChallenge {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::AuthenticationRequiredChallenge
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

impl VimObjectTrait for VslmAboutInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmAboutInfo
    }
}

impl VimObjectTrait for VslmQueryDatastoreInfoResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmQueryDatastoreInfoResult
    }
}

impl VimObjectTrait for VslmServiceInstanceContent {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmServiceInstanceContent
    }
}

impl VimObjectTrait for VslmTaskInfo {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskInfo
    }
}

impl VimObjectTrait for VslmTaskReason {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskReason
    }
}

impl VimObjectTrait for VslmTaskReasonAlarm {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskReasonAlarm
    }
}

impl VimObjectTrait for VslmTaskReasonSchedule {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskReasonSchedule
    }
}

impl VimObjectTrait for VslmTaskReasonSystem {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskReasonSystem
    }
}

impl VimObjectTrait for VslmTaskReasonUser {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmTaskReasonUser
    }
}

impl VimObjectTrait for VslmDatastoreSyncStatus {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmDatastoreSyncStatus
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectAssociations {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectAssociations
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectAssociationsVmDiskAssociation {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectAssociationsVmDiskAssociation
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectQueryResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectQueryResult
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectQuerySpec {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectQuerySpec
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectResult
    }
}

impl VimObjectTrait for VslmVsoVStorageObjectSnapshotResult {
    fn as_vim_object_ref<'a>(self: &'a Self) -> &'a dyn VimObjectTrait {
        self
    }
    
    fn data_type(&self) -> StructType {
        StructType::VslmVsoVStorageObjectSnapshotResult
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

