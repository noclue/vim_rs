//! GENERATED — do not edit. See `vim_build`.
#![cfg(feature = "xml")]

use super::api_field_types::ApiFieldType;
use super::data_type_aware::DataTypeAware;
use super::struct_enum::StructType;
use super::enums::*;
use super::structs::*;

impl DataTypeAware for ManagedObjectReference {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ManagedObjectReference)
    }
}

impl DataTypeAware for DataObject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DataObject)
    }
}

impl DataTypeAware for AgencyConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyConfigInfo)
    }
}

impl DataTypeAware for AgencyScope {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyScope)
    }
}

impl DataTypeAware for AgencyComputeResourceScope {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyComputeResourceScope)
    }
}

impl DataTypeAware for AgencyVmFolder {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyVmFolder)
    }
}

impl DataTypeAware for AgencyVmResourcePool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyVmResourcePool)
    }
}

impl DataTypeAware for AgentConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentConfigInfo)
    }
}

impl DataTypeAware for AgentOvfEnvironmentInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentOvfEnvironmentInfo)
    }
}

impl DataTypeAware for AgentOvfEnvironmentInfoOvfProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentOvfEnvironmentInfoOvfProperty)
    }
}

impl DataTypeAware for AgentSslTrust {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentSslTrust)
    }
}

impl DataTypeAware for AgentAnyCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentAnyCertificate)
    }
}

impl DataTypeAware for AgentPinnedPemCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentPinnedPemCertificate)
    }
}

impl DataTypeAware for AgentStoragePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentStoragePolicy)
    }
}

impl DataTypeAware for AgentVsanStoragePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentVsanStoragePolicy)
    }
}

impl DataTypeAware for AgentVibMatchingRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentVibMatchingRule)
    }
}

impl DataTypeAware for AgentVmHook {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentVmHook)
    }
}

impl DataTypeAware for EamObjectRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EamObjectRuntimeInfo)
    }
}

impl DataTypeAware for AgentRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentRuntimeInfo)
    }
}

impl DataTypeAware for Issue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Issue)
    }
}

impl DataTypeAware for AgencyIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyIssue)
    }
}

impl DataTypeAware for AgencyDisabled {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyDisabled)
    }
}

impl DataTypeAware for AgentIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentIssue)
    }
}

impl DataTypeAware for EamCertificateNotTrusted {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EamCertificateNotTrusted)
    }
}

impl DataTypeAware for HostInPartialMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInPartialMaintenanceMode)
    }
}

impl DataTypeAware for ManagedHostNotReachable {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ManagedHostNotReachable)
    }
}

impl DataTypeAware for MissingDvFilterSwitch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MissingDvFilterSwitch)
    }
}

impl DataTypeAware for OvfInvalidProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfInvalidProperty)
    }
}

impl DataTypeAware for TransitionFailed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TransitionFailed)
    }
}

impl DataTypeAware for VibIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibIssue)
    }
}

impl DataTypeAware for ImmediateHostRebootRequired {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ImmediateHostRebootRequired)
    }
}

impl DataTypeAware for VibCannotPutHostInMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibCannotPutHostInMaintenanceMode)
    }
}

impl DataTypeAware for VibCannotPutHostOutOfMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibCannotPutHostOutOfMaintenanceMode)
    }
}

impl DataTypeAware for VibNotInstalled {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibNotInstalled)
    }
}

impl DataTypeAware for CannotAccessAgentVib {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CannotAccessAgentVib)
    }
}

impl DataTypeAware for VibDependenciesNotMetByHost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibDependenciesNotMetByHost)
    }
}

impl DataTypeAware for VibInvalidFormat {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibInvalidFormat)
    }
}

impl DataTypeAware for VibRequirementsNotMetByHost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibRequirementsNotMetByHost)
    }
}

impl DataTypeAware for VibRequiresHostInMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibRequiresHostInMaintenanceMode)
    }
}

impl DataTypeAware for VibRequiresHostReboot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibRequiresHostReboot)
    }
}

impl DataTypeAware for VibRequiresManualInstallation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibRequiresManualInstallation)
    }
}

impl DataTypeAware for VibRequiresManualUninstallation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibRequiresManualUninstallation)
    }
}

impl DataTypeAware for VmIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmIssue)
    }
}

impl DataTypeAware for InvalidConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InvalidConfig)
    }
}

impl DataTypeAware for VmCorrupted {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmCorrupted)
    }
}

impl DataTypeAware for VmDeployed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDeployed)
    }
}

impl DataTypeAware for HostInMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInMaintenanceMode)
    }
}

impl DataTypeAware for HostInStandbyMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInStandbyMode)
    }
}

impl DataTypeAware for HostPoweredOff {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPoweredOff)
    }
}

impl DataTypeAware for VmHookFailed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmHookFailed)
    }
}

impl DataTypeAware for VmHookTimedout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmHookTimedout)
    }
}

impl DataTypeAware for VmInaccessible {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmInaccessible)
    }
}

impl DataTypeAware for VmMarkedAsTemplate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmMarkedAsTemplate)
    }
}

impl DataTypeAware for VmOrphaned {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmOrphaned)
    }
}

impl DataTypeAware for VmPoweredOff {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmPoweredOff)
    }
}

impl DataTypeAware for InsufficientIpAddresses {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InsufficientIpAddresses)
    }
}

impl DataTypeAware for MissingAgentIpPool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MissingAgentIpPool)
    }
}

impl DataTypeAware for VmPoweredOn {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmPoweredOn)
    }
}

impl DataTypeAware for VmProtected {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmProtected)
    }
}

impl DataTypeAware for VmSuspended {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmSuspended)
    }
}

impl DataTypeAware for VmWrongFolder {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmWrongFolder)
    }
}

impl DataTypeAware for VmWrongResourcePool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmWrongResourcePool)
    }
}

impl DataTypeAware for VmNotDeployed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmNotDeployed)
    }
}

impl DataTypeAware for CannotAccessAgentOvf {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CannotAccessAgentOvf)
    }
}

impl DataTypeAware for IncompatibleHostVersion {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IncompatibleHostVersion)
    }
}

impl DataTypeAware for InsufficientResources {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InsufficientResources)
    }
}

impl DataTypeAware for InsufficientSpace {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InsufficientSpace)
    }
}

impl DataTypeAware for NoAgentVmDatastore {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoAgentVmDatastore)
    }
}

impl DataTypeAware for NoCustomAgentVmDatastore {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoCustomAgentVmDatastore)
    }
}

impl DataTypeAware for NoAgentVmNetwork {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoAgentVmNetwork)
    }
}

impl DataTypeAware for NoCustomAgentVmNetwork {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoCustomAgentVmNetwork)
    }
}

impl DataTypeAware for NoDiscoverableAgentVmDatastore {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoDiscoverableAgentVmDatastore)
    }
}

impl DataTypeAware for NoDiscoverableAgentVmNetwork {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoDiscoverableAgentVmNetwork)
    }
}

impl DataTypeAware for OvfInvalidFormat {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfInvalidFormat)
    }
}

impl DataTypeAware for VmRequiresHostOutOfMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmRequiresHostOutOfMaintenanceMode)
    }
}

impl DataTypeAware for PersonalityAgentPmIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgentPmIssue)
    }
}

impl DataTypeAware for PersonalityAgentAwaitingPmRemediation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgentAwaitingPmRemediation)
    }
}

impl DataTypeAware for PersonalityAgentBlockedByAgencyOperation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgentBlockedByAgencyOperation)
    }
}

impl DataTypeAware for OrphanedAgency {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OrphanedAgency)
    }
}

impl DataTypeAware for ClusterAgentAgentIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentAgentIssue)
    }
}

impl DataTypeAware for ClusterAgentOvfInvalidProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentOvfInvalidProperty)
    }
}

impl DataTypeAware for ClusterAgentTransitionFailed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentTransitionFailed)
    }
}

impl DataTypeAware for ClusterAgentVmIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmIssue)
    }
}

impl DataTypeAware for ClusterAgentHostInMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentHostInMaintenanceMode)
    }
}

impl DataTypeAware for ClusterAgentHostInPartialMaintenanceMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentHostInPartialMaintenanceMode)
    }
}

impl DataTypeAware for ClusterAgentInvalidConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentInvalidConfig)
    }
}

impl DataTypeAware for ClusterAgentVmHookFailed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmHookFailed)
    }
}

impl DataTypeAware for ClusterAgentVmHookTimedout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmHookTimedout)
    }
}

impl DataTypeAware for ClusterAgentVmInaccessible {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmInaccessible)
    }
}

impl DataTypeAware for ClusterAgentVmNotRemoved {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmNotRemoved)
    }
}

impl DataTypeAware for ClusterAgentVmPoweredOff {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmPoweredOff)
    }
}

impl DataTypeAware for ClusterAgentInsufficientClusterResources {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentInsufficientClusterResources)
    }
}

impl DataTypeAware for ClusterAgentVmPoweredOn {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmPoweredOn)
    }
}

impl DataTypeAware for ClusterAgentVmProtected {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmProtected)
    }
}

impl DataTypeAware for ClusterAgentVmSuspended {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmSuspended)
    }
}

impl DataTypeAware for ClusterAgentVmNotDeployed {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmNotDeployed)
    }
}

impl DataTypeAware for ClusterAgentCertificateNotTrusted {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentCertificateNotTrusted)
    }
}

impl DataTypeAware for ClusterAgentInsufficientClusterSpace {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentInsufficientClusterSpace)
    }
}

impl DataTypeAware for ClusterAgentMissingClusterVmDatastore {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentMissingClusterVmDatastore)
    }
}

impl DataTypeAware for ClusterAgentMissingClusterVmNetwork {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentMissingClusterVmNetwork)
    }
}

impl DataTypeAware for IntegrityAgencyVumIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntegrityAgencyVumIssue)
    }
}

impl DataTypeAware for IntegrityAgencyCannotDeleteSoftware {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntegrityAgencyCannotDeleteSoftware)
    }
}

impl DataTypeAware for IntegrityAgencyCannotStageSoftware {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntegrityAgencyCannotStageSoftware)
    }
}

impl DataTypeAware for IntegrityAgencyVumUnavailable {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntegrityAgencyVumUnavailable)
    }
}

impl DataTypeAware for PersonalityAgencyPmIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyPmIssue)
    }
}

impl DataTypeAware for PersonalityAgencyCannotConfigureSolutions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyCannotConfigureSolutions)
    }
}

impl DataTypeAware for PersonalityAgencyDepotIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyDepotIssue)
    }
}

impl DataTypeAware for PersonalityAgencyCannotUploadDepot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyCannotUploadDepot)
    }
}

impl DataTypeAware for PersonalityAgencyInaccessibleDepot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyInaccessibleDepot)
    }
}

impl DataTypeAware for PersonalityAgencyInvalidDepot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyInvalidDepot)
    }
}

impl DataTypeAware for PersonalityAgencyPmUnavailable {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyPmUnavailable)
    }
}

impl DataTypeAware for ExtensibleIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensibleIssue)
    }
}

impl DataTypeAware for HostIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIssue)
    }
}

impl DataTypeAware for OrphanedDvFilterSwitch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OrphanedDvFilterSwitch)
    }
}

impl DataTypeAware for UnknownAgentVm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UnknownAgentVm)
    }
}

impl DataTypeAware for HooksHookListSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HooksHookListSpec)
    }
}

impl DataTypeAware for HooksMarkAsProcessedSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HooksMarkAsProcessedSpec)
    }
}

impl DataTypeAware for SolutionsApplySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsApplySpec)
    }
}

impl DataTypeAware for SolutionsClusterSolutionComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsClusterSolutionComplianceResult)
    }
}

impl DataTypeAware for SolutionsComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsComplianceResult)
    }
}

impl DataTypeAware for SolutionsComplianceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsComplianceSpec)
    }
}

impl DataTypeAware for SolutionsDeploymentUnitComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsDeploymentUnitComplianceResult)
    }
}

impl DataTypeAware for SolutionsHookAcknowledgeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHookAcknowledgeConfig)
    }
}

impl DataTypeAware for SolutionsInteractiveHookAcknowledgeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsInteractiveHookAcknowledgeConfig)
    }
}

impl DataTypeAware for SolutionsHookConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHookConfig)
    }
}

impl DataTypeAware for SolutionsHookInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHookInfo)
    }
}

impl DataTypeAware for SolutionsHostComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHostComplianceResult)
    }
}

impl DataTypeAware for SolutionsOvfProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsOvfProperty)
    }
}

impl DataTypeAware for SolutionsSolutionComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsSolutionComplianceResult)
    }
}

impl DataTypeAware for SolutionsSolutionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsSolutionConfig)
    }
}

impl DataTypeAware for SolutionsSolutionValidationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsSolutionValidationResult)
    }
}

impl DataTypeAware for SolutionsStoragePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsStoragePolicy)
    }
}

impl DataTypeAware for SolutionsProfileIdStoragePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsProfileIdStoragePolicy)
    }
}

impl DataTypeAware for SolutionsTransitionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsTransitionSpec)
    }
}

impl DataTypeAware for SolutionsTypeSpecificSolutionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsTypeSpecificSolutionConfig)
    }
}

impl DataTypeAware for SolutionsClusterBoundSolutionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsClusterBoundSolutionConfig)
    }
}

impl DataTypeAware for SolutionsHostBoundSolutionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHostBoundSolutionConfig)
    }
}

impl DataTypeAware for SolutionsVmNetworkMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsVmNetworkMapping)
    }
}

impl DataTypeAware for SolutionsVmSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsVmSource)
    }
}

impl DataTypeAware for SolutionsUrlVmSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsUrlVmSource)
    }
}

impl DataTypeAware for SolutionsValidateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsValidateSpec)
    }
}

impl DataTypeAware for SolutionsValidationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsValidationResult)
    }
}

impl DataTypeAware for SolutionsVmResourceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsVmResourceSpec)
    }
}

impl DataTypeAware for VibVibInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibInfo)
    }
}

impl DataTypeAware for VibVibInfoSoftwareTags {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibInfoSoftwareTags)
    }
}

impl DataTypeAware for VibVibServicesSslTrust {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibServicesSslTrust)
    }
}

impl DataTypeAware for VibVibServicesAnyCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibServicesAnyCertificate)
    }
}

impl DataTypeAware for VibVibServicesPinnedPemCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibServicesPinnedPemCertificate)
    }
}

impl DataTypeAware for PbmAboutInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmAboutInfo)
    }
}

impl DataTypeAware for PbmExtendedElementDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmExtendedElementDescription)
    }
}

impl DataTypeAware for PbmLoggingConfiguration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmLoggingConfiguration)
    }
}

impl DataTypeAware for PbmServerObjectRef {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmServerObjectRef)
    }
}

impl DataTypeAware for PbmServiceInstanceContent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmServiceInstanceContent)
    }
}

impl DataTypeAware for PbmCapabilityInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityInstance)
    }
}

impl DataTypeAware for PbmCapabilityMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityMetadata)
    }
}

impl DataTypeAware for PbmCapabilityMetadataUniqueId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityMetadataUniqueId)
    }
}

impl DataTypeAware for PbmCapabilityConstraintInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityConstraintInstance)
    }
}

impl DataTypeAware for PbmCapabilityPropertyInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityPropertyInstance)
    }
}

impl DataTypeAware for PbmCapabilityPropertyMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityPropertyMetadata)
    }
}

impl DataTypeAware for PbmCapabilityTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityTypeInfo)
    }
}

impl DataTypeAware for PbmCapabilityGenericTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityGenericTypeInfo)
    }
}

impl DataTypeAware for PbmCapabilityMetadataPerCategory {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityMetadataPerCategory)
    }
}

impl DataTypeAware for PbmCapabilitySchema {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilitySchema)
    }
}

impl DataTypeAware for PbmCapabilityNamespaceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityNamespaceInfo)
    }
}

impl DataTypeAware for PbmCapabilitySchemaVendorInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilitySchemaVendorInfo)
    }
}

impl DataTypeAware for PbmCapabilityVendorNamespaceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityVendorNamespaceInfo)
    }
}

impl DataTypeAware for PbmCapabilityVendorResourceTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityVendorResourceTypeInfo)
    }
}

impl DataTypeAware for PbmLineOfServiceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmLineOfServiceInfo)
    }
}

impl DataTypeAware for PbmPersistenceBasedDataServiceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPersistenceBasedDataServiceInfo)
    }
}

impl DataTypeAware for PbmVaioDataServiceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmVaioDataServiceInfo)
    }
}

impl DataTypeAware for PbmCapabilityDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityDescription)
    }
}

impl DataTypeAware for PbmCapabilityDiscreteSet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityDiscreteSet)
    }
}

impl DataTypeAware for PbmCapabilityRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityRange)
    }
}

impl DataTypeAware for PbmCapabilityTimeSpan {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityTimeSpan)
    }
}

impl DataTypeAware for PbmComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmComplianceResult)
    }
}

impl DataTypeAware for PbmFetchEntityHealthStatusSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmFetchEntityHealthStatusSpec)
    }
}

impl DataTypeAware for PbmComplianceOperationalStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmComplianceOperationalStatus)
    }
}

impl DataTypeAware for PbmCompliancePolicyStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCompliancePolicyStatus)
    }
}

impl DataTypeAware for PbmRollupComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmRollupComplianceResult)
    }
}

impl DataTypeAware for PbmFaultNoPermissionEntityPrivileges {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmFaultNoPermissionEntityPrivileges)
    }
}

impl DataTypeAware for PbmPlacementCompatibilityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementCompatibilityResult)
    }
}

impl DataTypeAware for PbmPlacementMatchingResources {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementMatchingResources)
    }
}

impl DataTypeAware for PbmPlacementMatchingReplicationResources {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementMatchingReplicationResources)
    }
}

impl DataTypeAware for PbmPlacementHub {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementHub)
    }
}

impl DataTypeAware for PbmPlacementRequirement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementRequirement)
    }
}

impl DataTypeAware for PbmPlacementCapabilityConstraintsRequirement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementCapabilityConstraintsRequirement)
    }
}

impl DataTypeAware for PbmPlacementCapabilityProfileRequirement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementCapabilityProfileRequirement)
    }
}

impl DataTypeAware for PbmPlacementResourceUtilization {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementResourceUtilization)
    }
}

impl DataTypeAware for PbmCapabilityProfileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityProfileCreateSpec)
    }
}

impl DataTypeAware for PbmCapabilityProfileUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityProfileUpdateSpec)
    }
}

impl DataTypeAware for PbmCapabilityConstraints {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityConstraints)
    }
}

impl DataTypeAware for PbmCapabilitySubProfileConstraints {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilitySubProfileConstraints)
    }
}

impl DataTypeAware for PbmDataServiceToPoliciesMap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmDataServiceToPoliciesMap)
    }
}

impl DataTypeAware for PbmDefaultProfileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmDefaultProfileInfo)
    }
}

impl DataTypeAware for PbmProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfile)
    }
}

impl DataTypeAware for PbmCapabilityProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityProfile)
    }
}

impl DataTypeAware for PbmDefaultCapabilityProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmDefaultCapabilityProfile)
    }
}

impl DataTypeAware for PbmProfileId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfileId)
    }
}

impl DataTypeAware for PbmProfileOperationOutcome {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfileOperationOutcome)
    }
}

impl DataTypeAware for PbmProfileType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfileType)
    }
}

impl DataTypeAware for PbmQueryProfileResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmQueryProfileResult)
    }
}

impl DataTypeAware for PbmProfileResourceType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfileResourceType)
    }
}

impl DataTypeAware for PbmCapabilitySubProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilitySubProfile)
    }
}

impl DataTypeAware for PbmDatastoreSpaceStatistics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmDatastoreSpaceStatistics)
    }
}

impl DataTypeAware for PbmQueryReplicationGroupResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmQueryReplicationGroupResult)
    }
}

impl DataTypeAware for SmsAboutInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsAboutInfo)
    }
}

impl DataTypeAware for EntityReference {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityReference)
    }
}

impl DataTypeAware for FaultDomainFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultDomainFilter)
    }
}

impl DataTypeAware for ReplicationGroupFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationGroupFilter)
    }
}

impl DataTypeAware for SmsTaskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsTaskInfo)
    }
}

impl DataTypeAware for AlarmFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmFilter)
    }
}

impl DataTypeAware for AlarmResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmResult)
    }
}

impl DataTypeAware for SmsProviderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsProviderInfo)
    }
}

impl DataTypeAware for VasaProviderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaProviderInfo)
    }
}

impl DataTypeAware for SmsProviderSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsProviderSpec)
    }
}

impl DataTypeAware for VasaProviderSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaProviderSpec)
    }
}

impl DataTypeAware for VasaProviderUpgradeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaProviderUpgradeSpec)
    }
}

impl DataTypeAware for RelatedStorageArray {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RelatedStorageArray)
    }
}

impl DataTypeAware for SupportedVendorModelMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SupportedVendorModelMapping)
    }
}

impl DataTypeAware for BackingConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BackingConfig)
    }
}

impl DataTypeAware for BackingStoragePool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BackingStoragePool)
    }
}

impl DataTypeAware for DatastoreBackingPoolMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreBackingPoolMapping)
    }
}

impl DataTypeAware for DatastorePair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastorePair)
    }
}

impl DataTypeAware for DrsMigrationCapabilityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DrsMigrationCapabilityResult)
    }
}

impl DataTypeAware for FaultDomainProviderMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultDomainProviderMapping)
    }
}

impl DataTypeAware for StorageFileSystemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageFileSystemInfo)
    }
}

impl DataTypeAware for LunHbaAssociation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LunHbaAssociation)
    }
}

impl DataTypeAware for NameValuePair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NameValuePair)
    }
}

impl DataTypeAware for StorageAlarm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageAlarm)
    }
}

impl DataTypeAware for StorageArray {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageArray)
    }
}

impl DataTypeAware for StorageCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageCapability)
    }
}

impl DataTypeAware for StorageContainer {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageContainer)
    }
}

impl DataTypeAware for StorageContainerResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageContainerResult)
    }
}

impl DataTypeAware for StorageContainerSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageContainerSpec)
    }
}

impl DataTypeAware for StorageFileSystem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageFileSystem)
    }
}

impl DataTypeAware for StorageLun {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageLun)
    }
}

impl DataTypeAware for StoragePort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePort)
    }
}

impl DataTypeAware for FcStoragePort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcStoragePort)
    }
}

impl DataTypeAware for FcoeStoragePort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcoeStoragePort)
    }
}

impl DataTypeAware for IscsiStoragePort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IscsiStoragePort)
    }
}

impl DataTypeAware for StorageProcessor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageProcessor)
    }
}

impl DataTypeAware for DeviceId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DeviceId)
    }
}

impl DataTypeAware for VVolId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VVolId)
    }
}

impl DataTypeAware for VasaVirtualDiskId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaVirtualDiskId)
    }
}

impl DataTypeAware for VirtualDiskKey {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskKey)
    }
}

impl DataTypeAware for VirtualDiskMoId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskMoId)
    }
}

impl DataTypeAware for VirtualMachineId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineId)
    }
}

impl DataTypeAware for VirtualMachineFilePath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFilePath)
    }
}

impl DataTypeAware for VirtualMachineMoId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMoId)
    }
}

impl DataTypeAware for VirtualMachineUuid {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineUuid)
    }
}

impl DataTypeAware for FailoverParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FailoverParam)
    }
}

impl DataTypeAware for TestFailoverParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TestFailoverParam)
    }
}

impl DataTypeAware for PolicyAssociation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PolicyAssociation)
    }
}

impl DataTypeAware for ReplicationGroupData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationGroupData)
    }
}

impl DataTypeAware for RecoveredDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RecoveredDevice)
    }
}

impl DataTypeAware for RecoveredDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RecoveredDiskInfo)
    }
}

impl DataTypeAware for GroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupInfo)
    }
}

impl DataTypeAware for SourceGroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SourceGroupInfo)
    }
}

impl DataTypeAware for TargetGroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TargetGroupInfo)
    }
}

impl DataTypeAware for GroupOperationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupOperationResult)
    }
}

impl DataTypeAware for FailoverSuccessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FailoverSuccessResult)
    }
}

impl DataTypeAware for GroupErrorResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupErrorResult)
    }
}

impl DataTypeAware for QueryPointInTimeReplicaSuccessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryPointInTimeReplicaSuccessResult)
    }
}

impl DataTypeAware for QueryPointInTimeReplicaSummaryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryPointInTimeReplicaSummaryResult)
    }
}

impl DataTypeAware for QueryReplicationGroupSuccessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryReplicationGroupSuccessResult)
    }
}

impl DataTypeAware for ReverseReplicationSuccessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReverseReplicationSuccessResult)
    }
}

impl DataTypeAware for SyncReplicationGroupSuccessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SyncReplicationGroupSuccessResult)
    }
}

impl DataTypeAware for PointInTimeReplicaId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PointInTimeReplicaId)
    }
}

impl DataTypeAware for PromoteParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PromoteParam)
    }
}

impl DataTypeAware for QueryPointInTimeReplicaParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryPointInTimeReplicaParam)
    }
}

impl DataTypeAware for ReplicaQueryIntervalParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicaQueryIntervalParam)
    }
}

impl DataTypeAware for PointInTimeReplicaInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PointInTimeReplicaInfo)
    }
}

impl DataTypeAware for ReplicaIntervalQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicaIntervalQueryResult)
    }
}

impl DataTypeAware for QueryReplicationPeerResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryReplicationPeerResult)
    }
}

impl DataTypeAware for ReplicaId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicaId)
    }
}

impl DataTypeAware for ReplicationTargetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationTargetInfo)
    }
}

impl DataTypeAware for SourceGroupMemberInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SourceGroupMemberInfo)
    }
}

impl DataTypeAware for TargetDeviceId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TargetDeviceId)
    }
}

impl DataTypeAware for TargetToSourceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TargetToSourceInfo)
    }
}

impl DataTypeAware for TargetGroupMemberInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TargetGroupMemberInfo)
    }
}

impl DataTypeAware for RecoveredTargetGroupMemberInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RecoveredTargetGroupMemberInfo)
    }
}

impl DataTypeAware for AboutInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AboutInfo)
    }
}

impl DataTypeAware for AuthorizationDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AuthorizationDescription)
    }
}

impl DataTypeAware for EntityPrivilege {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityPrivilege)
    }
}

impl DataTypeAware for Permission {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Permission)
    }
}

impl DataTypeAware for AuthorizationPrivilege {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AuthorizationPrivilege)
    }
}

impl DataTypeAware for PrivilegeAvailability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PrivilegeAvailability)
    }
}

impl DataTypeAware for AuthorizationRole {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AuthorizationRole)
    }
}

impl DataTypeAware for UserPrivilegeResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserPrivilegeResult)
    }
}

impl DataTypeAware for BatchResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BatchResult)
    }
}

impl DataTypeAware for Capability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Capability)
    }
}

impl DataTypeAware for ClusterComputeResourceClusterConfigResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceClusterConfigResult)
    }
}

impl DataTypeAware for ClusterComputeResourceCryptoModePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceCryptoModePolicy)
    }
}

impl DataTypeAware for ClusterComputeResourceDvsSetting {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceDvsSetting)
    }
}

impl DataTypeAware for ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceDvsSettingDvPortgroupToServiceMapping)
    }
}

impl DataTypeAware for ClusterComputeResourceDvsProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceDvsProfile)
    }
}

impl DataTypeAware for ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceDvsProfileDvPortgroupSpecToServiceMapping)
    }
}

impl DataTypeAware for ClusterComputeResourceHciConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHciConfigInfo)
    }
}

impl DataTypeAware for ClusterComputeResourceHciConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHciConfigSpec)
    }
}

impl DataTypeAware for ClusterComputeResourceHostConfigurationInput {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHostConfigurationInput)
    }
}

impl DataTypeAware for ClusterComputeResourceHostConfigurationProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHostConfigurationProfile)
    }
}

impl DataTypeAware for ClusterComputeResourceHostEvacuationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHostEvacuationInfo)
    }
}

impl DataTypeAware for ClusterComputeResourceHostVmkNicInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHostVmkNicInfo)
    }
}

impl DataTypeAware for ClusterComputeResourceMaintenanceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceMaintenanceInfo)
    }
}

impl DataTypeAware for ClusterComputeResourceVcProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceVcProfile)
    }
}

impl DataTypeAware for ClusterComputeResourceValidationResultBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceValidationResultBase)
    }
}

impl DataTypeAware for ClusterComputeResourceDvsConfigurationValidation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceDvsConfigurationValidation)
    }
}

impl DataTypeAware for ClusterComputeResourceHostConfigurationValidation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceHostConfigurationValidation)
    }
}

impl DataTypeAware for VsanClusterConfigPrecheckItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterConfigPrecheckItem)
    }
}

impl DataTypeAware for ClusterComputeResourceVcsSlots {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceVcsSlots)
    }
}

impl DataTypeAware for ComputeResourceConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceConfigInfo)
    }
}

impl DataTypeAware for ClusterConfigInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterConfigInfoEx)
    }
}

impl DataTypeAware for ComputeResourceConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceConfigSpec)
    }
}

impl DataTypeAware for ClusterConfigSpecEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterConfigSpecEx)
    }
}

impl DataTypeAware for ComputeResourceHostSpbmLicenseInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceHostSpbmLicenseInfo)
    }
}

impl DataTypeAware for ComputeResourceHostSeedSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceHostSeedSpec)
    }
}

impl DataTypeAware for ComputeResourceHostSeedSpecSingleHostSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceHostSeedSpecSingleHostSpec)
    }
}

impl DataTypeAware for ComputeResourceSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceSummary)
    }
}

impl DataTypeAware for ClusterComputeResourceSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceSummary)
    }
}

impl DataTypeAware for CustomFieldDef {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomFieldDef)
    }
}

impl DataTypeAware for CustomFieldValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomFieldValue)
    }
}

impl DataTypeAware for CustomFieldStringValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomFieldStringValue)
    }
}

impl DataTypeAware for CustomizationSpecInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationSpecInfo)
    }
}

impl DataTypeAware for CustomizationSpecItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationSpecItem)
    }
}

impl DataTypeAware for DatacenterBasicConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatacenterBasicConnectInfo)
    }
}

impl DataTypeAware for DatacenterConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatacenterConfigInfo)
    }
}

impl DataTypeAware for DatacenterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatacenterConfigSpec)
    }
}

impl DataTypeAware for DatastoreCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreCapability)
    }
}

impl DataTypeAware for DatastoreHostMount {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreHostMount)
    }
}

impl DataTypeAware for DatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreInfo)
    }
}

impl DataTypeAware for LocalDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LocalDatastoreInfo)
    }
}

impl DataTypeAware for NasDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NasDatastoreInfo)
    }
}

impl DataTypeAware for PMemDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PMemDatastoreInfo)
    }
}

impl DataTypeAware for VmfsDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreInfo)
    }
}

impl DataTypeAware for VsanDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreInfo)
    }
}

impl DataTypeAware for VvolDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VvolDatastoreInfo)
    }
}

impl DataTypeAware for DatastoreMountPathDatastorePair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreMountPathDatastorePair)
    }
}

impl DataTypeAware for DatastoreSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreSummary)
    }
}

impl DataTypeAware for DatastoreVVolContainerFailoverPair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreVVolContainerFailoverPair)
    }
}

impl DataTypeAware for DatastoreNamespaceManagerDirectoryInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreNamespaceManagerDirectoryInfo)
    }
}

impl DataTypeAware for Description {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Description)
    }
}

impl DataTypeAware for ElementDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ElementDescription)
    }
}

impl DataTypeAware for EvcMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EvcMode)
    }
}

impl DataTypeAware for ExtendedElementDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtendedElementDescription)
    }
}

impl DataTypeAware for FeatureEvcMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FeatureEvcMode)
    }
}

impl DataTypeAware for OptionDef {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionDef)
    }
}

impl DataTypeAware for ExtendedDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtendedDescription)
    }
}

impl DataTypeAware for MethodDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MethodDescription)
    }
}

impl DataTypeAware for TypeDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TypeDescription)
    }
}

impl DataTypeAware for ScheduledTaskDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskDetail)
    }
}

impl DataTypeAware for DesiredSoftwareSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DesiredSoftwareSpec)
    }
}

impl DataTypeAware for DesiredSoftwareSpecBaseImageSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DesiredSoftwareSpecBaseImageSpec)
    }
}

impl DataTypeAware for DesiredSoftwareSpecComponentSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DesiredSoftwareSpecComponentSpec)
    }
}

impl DataTypeAware for DesiredSoftwareSpecVendorAddOnSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DesiredSoftwareSpecVendorAddOnSpec)
    }
}

impl DataTypeAware for DiagnosticManagerAuditRecordResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiagnosticManagerAuditRecordResult)
    }
}

impl DataTypeAware for DiagnosticManagerBundleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiagnosticManagerBundleInfo)
    }
}

impl DataTypeAware for DiagnosticManagerLogDescriptor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiagnosticManagerLogDescriptor)
    }
}

impl DataTypeAware for DiagnosticManagerLogHeader {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiagnosticManagerLogHeader)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityQuerySpec)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityQueryByDeviceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityQueryByDeviceConfig)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityQueryById {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityQueryById)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityQueryByName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityQueryByName)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityResult)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityInfo)
    }
}

impl DataTypeAware for DirectPathProfileManagerCapacityUnknown {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityUnknown)
    }
}

impl DataTypeAware for DirectPathProfileManagerCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCreateSpec)
    }
}

impl DataTypeAware for DirectPathProfileManagerDirectPathConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerDirectPathConfig)
    }
}

impl DataTypeAware for DirectPathProfileManagerDvxDirectPathConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerDvxDirectPathConfig)
    }
}

impl DataTypeAware for DirectPathProfileManagerDynamicDirectPathConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerDynamicDirectPathConfig)
    }
}

impl DataTypeAware for DirectPathProfileManagerVirtualDeviceGroupDirectPathConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerVirtualDeviceGroupDirectPathConfig)
    }
}

impl DataTypeAware for DirectPathProfileManagerVmiopDirectPathConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerVmiopDirectPathConfig)
    }
}

impl DataTypeAware for DirectPathProfileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileInfo)
    }
}

impl DataTypeAware for DirectPathProfileManagerFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerFilterSpec)
    }
}

impl DataTypeAware for DirectPathProfileManagerTargetEntity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerTargetEntity)
    }
}

impl DataTypeAware for DirectPathProfileManagerTargetCluster {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerTargetCluster)
    }
}

impl DataTypeAware for DirectPathProfileManagerTargetHost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerTargetHost)
    }
}

impl DataTypeAware for DirectPathProfileManagerUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerUpdateSpec)
    }
}

impl DataTypeAware for DvsBackupRestoreCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsBackupRestoreCapability)
    }
}

impl DataTypeAware for DvsCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsCapability)
    }
}

impl DataTypeAware for DvsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsConfigInfo)
    }
}

impl DataTypeAware for VMwareDvsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsConfigInfo)
    }
}

impl DataTypeAware for DvsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsConfigSpec)
    }
}

impl DataTypeAware for VMwareDvsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsConfigSpec)
    }
}

impl DataTypeAware for DvsContactInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsContactInfo)
    }
}

impl DataTypeAware for DvsCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsCreateSpec)
    }
}

impl DataTypeAware for DvsFeatureCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFeatureCapability)
    }
}

impl DataTypeAware for VMwareDvsFeatureCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsFeatureCapability)
    }
}

impl DataTypeAware for DvsHealthCheckConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHealthCheckConfig)
    }
}

impl DataTypeAware for VMwareDvsHealthCheckConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsHealthCheckConfig)
    }
}

impl DataTypeAware for VMwareDvsTeamingHealthCheckConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsTeamingHealthCheckConfig)
    }
}

impl DataTypeAware for VMwareDvsVlanMtuHealthCheckConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsVlanMtuHealthCheckConfig)
    }
}

impl DataTypeAware for DvsHealthCheckCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHealthCheckCapability)
    }
}

impl DataTypeAware for VMwareDvsHealthCheckCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsHealthCheckCapability)
    }
}

impl DataTypeAware for DvsHostInfrastructureTrafficResource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHostInfrastructureTrafficResource)
    }
}

impl DataTypeAware for DvsHostInfrastructureTrafficResourceAllocation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHostInfrastructureTrafficResourceAllocation)
    }
}

impl DataTypeAware for DvsNetworkResourceManagementCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkResourceManagementCapability)
    }
}

impl DataTypeAware for DvsResourceRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsResourceRuntimeInfo)
    }
}

impl DataTypeAware for DvsRollbackCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsRollbackCapability)
    }
}

impl DataTypeAware for DvsRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsRuntimeInfo)
    }
}

impl DataTypeAware for DvsSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsSummary)
    }
}

impl DataTypeAware for DvsPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsPolicy)
    }
}

impl DataTypeAware for DvsUplinkPortPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsUplinkPortPolicy)
    }
}

impl DataTypeAware for DvsNameArrayUplinkPortPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNameArrayUplinkPortPolicy)
    }
}

impl DataTypeAware for EnumDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EnumDescription)
    }
}

impl DataTypeAware for EnvironmentBrowserConfigOptionQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EnvironmentBrowserConfigOptionQuerySpec)
    }
}

impl DataTypeAware for Extension {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Extension)
    }
}

impl DataTypeAware for ExtensionClientInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionClientInfo)
    }
}

impl DataTypeAware for ExtensionEventTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionEventTypeInfo)
    }
}

impl DataTypeAware for ExtensionFaultTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionFaultTypeInfo)
    }
}

impl DataTypeAware for ExtensionHealthInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionHealthInfo)
    }
}

impl DataTypeAware for ExtensionOvfConsumerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionOvfConsumerInfo)
    }
}

impl DataTypeAware for ExtensionPrivilegeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionPrivilegeInfo)
    }
}

impl DataTypeAware for ExtensionResourceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionResourceInfo)
    }
}

impl DataTypeAware for ExtensionServerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionServerInfo)
    }
}

impl DataTypeAware for ExtensionTaskTypeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionTaskTypeInfo)
    }
}

impl DataTypeAware for ExtensionManagerIpAllocationUsage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtensionManagerIpAllocationUsage)
    }
}

impl DataTypeAware for FaultsByHost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultsByHost)
    }
}

impl DataTypeAware for FaultsByVm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultsByVm)
    }
}

impl DataTypeAware for FileLockInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileLockInfo)
    }
}

impl DataTypeAware for FileLockInfoResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileLockInfoResult)
    }
}

impl DataTypeAware for FolderBatchAddHostsToClusterResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderBatchAddHostsToClusterResult)
    }
}

impl DataTypeAware for FolderBatchAddStandaloneHostsResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderBatchAddStandaloneHostsResult)
    }
}

impl DataTypeAware for FolderExternallyManagedFolderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderExternallyManagedFolderInfo)
    }
}

impl DataTypeAware for FolderFailedHostResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderFailedHostResult)
    }
}

impl DataTypeAware for FolderNewHostSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderNewHostSpec)
    }
}

impl DataTypeAware for HbrManagerReplicationVmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrManagerReplicationVmInfo)
    }
}

impl DataTypeAware for ReplicationVmProgressInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationVmProgressInfo)
    }
}

impl DataTypeAware for HbrManagerVmReplicationCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrManagerVmReplicationCapability)
    }
}

impl DataTypeAware for HbrReplicationTargetSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrReplicationTargetSpec)
    }
}

impl DataTypeAware for HbrTargetSpecReplacement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrTargetSpecReplacement)
    }
}

impl DataTypeAware for HbrTargetSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrTargetSpec)
    }
}

impl DataTypeAware for HealthUpdate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HealthUpdate)
    }
}

impl DataTypeAware for HealthUpdateInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HealthUpdateInfo)
    }
}

impl DataTypeAware for PerfInterval {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfInterval)
    }
}

impl DataTypeAware for HostServiceTicket {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostServiceTicket)
    }
}

impl DataTypeAware for HostSystemComplianceCheckState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemComplianceCheckState)
    }
}

impl DataTypeAware for HostSystemReconnectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemReconnectSpec)
    }
}

impl DataTypeAware for HostSystemRemediationState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemRemediationState)
    }
}

impl DataTypeAware for HttpNfcLeaseCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseCapabilities)
    }
}

impl DataTypeAware for HttpNfcLeaseDatastoreLeaseInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseDatastoreLeaseInfo)
    }
}

impl DataTypeAware for HttpNfcLeaseDeviceUrl {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseDeviceUrl)
    }
}

impl DataTypeAware for HttpNfcLeaseHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseHostInfo)
    }
}

impl DataTypeAware for HttpNfcLeaseInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseInfo)
    }
}

impl DataTypeAware for HttpNfcLeaseManifestEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseManifestEntry)
    }
}

impl DataTypeAware for HttpNfcLeaseProbeResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseProbeResult)
    }
}

impl DataTypeAware for HttpNfcLeaseSourceFile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HttpNfcLeaseSourceFile)
    }
}

impl DataTypeAware for ImportSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ImportSpec)
    }
}

impl DataTypeAware for VirtualAppImportSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualAppImportSpec)
    }
}

impl DataTypeAware for VirtualMachineImportSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineImportSpec)
    }
}

impl DataTypeAware for InheritablePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InheritablePolicy)
    }
}

impl DataTypeAware for BoolPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BoolPolicy)
    }
}

impl DataTypeAware for IntPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntPolicy)
    }
}

impl DataTypeAware for LongPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LongPolicy)
    }
}

impl DataTypeAware for StringPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StringPolicy)
    }
}

impl DataTypeAware for DvsFilterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterConfig)
    }
}

impl DataTypeAware for DvsFilterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterConfigSpec)
    }
}

impl DataTypeAware for DvsTrafficFilterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficFilterConfig)
    }
}

impl DataTypeAware for DvsTrafficFilterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficFilterConfigSpec)
    }
}

impl DataTypeAware for DvsFilterPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterPolicy)
    }
}

impl DataTypeAware for DvsTrafficShapingPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficShapingPolicy)
    }
}

impl DataTypeAware for DvsVendorSpecificConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVendorSpecificConfig)
    }
}

impl DataTypeAware for DvsFailureCriteria {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFailureCriteria)
    }
}

impl DataTypeAware for DvsMacLearningPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsMacLearningPolicy)
    }
}

impl DataTypeAware for DvsMacManagementPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsMacManagementPolicy)
    }
}

impl DataTypeAware for DvsSecurityPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsSecurityPolicy)
    }
}

impl DataTypeAware for VMwareUplinkLacpPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareUplinkLacpPolicy)
    }
}

impl DataTypeAware for VMwareUplinkPortOrderPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareUplinkPortOrderPolicy)
    }
}

impl DataTypeAware for VmwareUplinkPortTeamingPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareUplinkPortTeamingPolicy)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchVlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchVlanSpec)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchPvlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchPvlanSpec)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchTrunkVlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchTrunkVlanSpec)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchVlanIdSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchVlanIdSpec)
    }
}

impl DataTypeAware for IoFilterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterInfo)
    }
}

impl DataTypeAware for ClusterIoFilterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterIoFilterInfo)
    }
}

impl DataTypeAware for HostIoFilterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIoFilterInfo)
    }
}

impl DataTypeAware for IoFilterQueryIssueResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterQueryIssueResult)
    }
}

impl DataTypeAware for IoFilterHostIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterHostIssue)
    }
}

impl DataTypeAware for IoFilterManagerSslTrust {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterManagerSslTrust)
    }
}

impl DataTypeAware for PinnedCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PinnedCertificate)
    }
}

impl DataTypeAware for UntrustedCertificate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UntrustedCertificate)
    }
}

impl DataTypeAware for IpPoolManagerIpAllocation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpPoolManagerIpAllocation)
    }
}

impl DataTypeAware for KeyValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KeyValue)
    }
}

impl DataTypeAware for LatencySensitivity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LatencySensitivity)
    }
}

impl DataTypeAware for LicenseAssignmentManagerLicenseAssignment {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseAssignmentManagerLicenseAssignment)
    }
}

impl DataTypeAware for LicenseAvailabilityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseAvailabilityInfo)
    }
}

impl DataTypeAware for LicenseDiagnostics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseDiagnostics)
    }
}

impl DataTypeAware for LicenseManagerEvaluationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseManagerEvaluationInfo)
    }
}

impl DataTypeAware for LicenseFeatureInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseFeatureInfo)
    }
}

impl DataTypeAware for HostLicensableResourceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLicensableResourceInfo)
    }
}

impl DataTypeAware for LicenseManagerLicenseInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseManagerLicenseInfo)
    }
}

impl DataTypeAware for LicenseSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseSource)
    }
}

impl DataTypeAware for EvaluationLicenseSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EvaluationLicenseSource)
    }
}

impl DataTypeAware for LicenseServerSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseServerSource)
    }
}

impl DataTypeAware for LocalLicenseSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LocalLicenseSource)
    }
}

impl DataTypeAware for LicenseUsageInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseUsageInfo)
    }
}

impl DataTypeAware for LicenseReservationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseReservationInfo)
    }
}

impl DataTypeAware for LocalizationManagerMessageCatalog {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LocalizationManagerMessageCatalog)
    }
}

impl DataTypeAware for NegatableExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NegatableExpression)
    }
}

impl DataTypeAware for IntExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntExpression)
    }
}

impl DataTypeAware for IpAddress {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpAddress)
    }
}

impl DataTypeAware for IpRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpRange)
    }
}

impl DataTypeAware for SingleIp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SingleIp)
    }
}

impl DataTypeAware for MacAddress {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MacAddress)
    }
}

impl DataTypeAware for MacRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MacRange)
    }
}

impl DataTypeAware for SingleMac {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SingleMac)
    }
}

impl DataTypeAware for StringExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StringExpression)
    }
}

impl DataTypeAware for DvsIpPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsIpPort)
    }
}

impl DataTypeAware for DvsIpPortRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsIpPortRange)
    }
}

impl DataTypeAware for DvsSingleIpPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsSingleIpPort)
    }
}

impl DataTypeAware for NetworkSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkSummary)
    }
}

impl DataTypeAware for OpaqueNetworkSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OpaqueNetworkSummary)
    }
}

impl DataTypeAware for NumericRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NumericRange)
    }
}

impl DataTypeAware for OpaqueNetworkCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OpaqueNetworkCapability)
    }
}

impl DataTypeAware for OvfConsumerOstNode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfConsumerOstNode)
    }
}

impl DataTypeAware for OvfConsumerOvfSection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfConsumerOvfSection)
    }
}

impl DataTypeAware for OvfManagerCommonParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfManagerCommonParams)
    }
}

impl DataTypeAware for OvfCreateImportSpecParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfCreateImportSpecParams)
    }
}

impl DataTypeAware for OvfImportParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfImportParams)
    }
}

impl DataTypeAware for OvfParseDescriptorParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfParseDescriptorParams)
    }
}

impl DataTypeAware for OvfValidateHostParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfValidateHostParams)
    }
}

impl DataTypeAware for OvfCreateDescriptorParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfCreateDescriptorParams)
    }
}

impl DataTypeAware for OvfCreateDescriptorResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfCreateDescriptorResult)
    }
}

impl DataTypeAware for OvfCreateImportSpecResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfCreateImportSpecResult)
    }
}

impl DataTypeAware for OvfDatastoreMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfDatastoreMapping)
    }
}

impl DataTypeAware for OvfDeploymentOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfDeploymentOption)
    }
}

impl DataTypeAware for OvfFileItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfFileItem)
    }
}

impl DataTypeAware for OvfNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfNetworkInfo)
    }
}

impl DataTypeAware for OvfNetworkMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfNetworkMapping)
    }
}

impl DataTypeAware for OvfFile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfFile)
    }
}

impl DataTypeAware for OvfOptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfOptionInfo)
    }
}

impl DataTypeAware for OvfParseDescriptorResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfParseDescriptorResult)
    }
}

impl DataTypeAware for OvfResourceMap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfResourceMap)
    }
}

impl DataTypeAware for OvfStorageProfileMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfStorageProfileMapping)
    }
}

impl DataTypeAware for OvfValidateHostResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfValidateHostResult)
    }
}

impl DataTypeAware for PasswordField {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PasswordField)
    }
}

impl DataTypeAware for PerformanceDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerformanceDescription)
    }
}

impl DataTypeAware for PerfCompositeMetric {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfCompositeMetric)
    }
}

impl DataTypeAware for PerfCounterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfCounterInfo)
    }
}

impl DataTypeAware for PerformanceManagerCounterLevelMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerformanceManagerCounterLevelMapping)
    }
}

impl DataTypeAware for PerfEntityMetricBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfEntityMetricBase)
    }
}

impl DataTypeAware for PerfEntityMetric {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfEntityMetric)
    }
}

impl DataTypeAware for PerfEntityMetricCsv {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfEntityMetricCsv)
    }
}

impl DataTypeAware for PerfMetricId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfMetricId)
    }
}

impl DataTypeAware for PerfMetricSeries {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfMetricSeries)
    }
}

impl DataTypeAware for PerfMetricIntSeries {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfMetricIntSeries)
    }
}

impl DataTypeAware for PerfMetricSeriesCsv {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfMetricSeriesCsv)
    }
}

impl DataTypeAware for PerfProviderSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfProviderSummary)
    }
}

impl DataTypeAware for PerfQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfQuerySpec)
    }
}

impl DataTypeAware for PerfSampleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfSampleInfo)
    }
}

impl DataTypeAware for PrivilegePolicyDef {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PrivilegePolicyDef)
    }
}

impl DataTypeAware for ResourceAllocationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourceAllocationInfo)
    }
}

impl DataTypeAware for ResourceAllocationOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourceAllocationOption)
    }
}

impl DataTypeAware for ResourceConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourceConfigOption)
    }
}

impl DataTypeAware for ResourceConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourceConfigSpec)
    }
}

impl DataTypeAware for DatabaseSizeEstimate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatabaseSizeEstimate)
    }
}

impl DataTypeAware for DatabaseSizeParam {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatabaseSizeParam)
    }
}

impl DataTypeAware for InventoryDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InventoryDescription)
    }
}

impl DataTypeAware for PerformanceStatisticsDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerformanceStatisticsDescription)
    }
}

impl DataTypeAware for ResourcePoolResourceUsage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolResourceUsage)
    }
}

impl DataTypeAware for ResourcePoolRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolRuntimeInfo)
    }
}

impl DataTypeAware for ResourcePoolSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolSummary)
    }
}

impl DataTypeAware for VirtualAppSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualAppSummary)
    }
}

impl DataTypeAware for ResourcePoolQuickStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolQuickStats)
    }
}

impl DataTypeAware for SddcBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SddcBase)
    }
}

impl DataTypeAware for VimVsanReconfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanReconfigSpec)
    }
}

impl DataTypeAware for SelectionSet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SelectionSet)
    }
}

impl DataTypeAware for DvPortgroupSelection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortgroupSelection)
    }
}

impl DataTypeAware for DvsSelection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsSelection)
    }
}

impl DataTypeAware for HostVMotionCompatibility {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionCompatibility)
    }
}

impl DataTypeAware for ProductComponentInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProductComponentInfo)
    }
}

impl DataTypeAware for ServiceContent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceContent)
    }
}

impl DataTypeAware for ServiceLocator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceLocator)
    }
}

impl DataTypeAware for ServiceLocatorCredential {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceLocatorCredential)
    }
}

impl DataTypeAware for ServiceLocatorNamePassword {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceLocatorNamePassword)
    }
}

impl DataTypeAware for ServiceLocatorSamlCredential {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceLocatorSamlCredential)
    }
}

impl DataTypeAware for ServiceManagerServiceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceManagerServiceInfo)
    }
}

impl DataTypeAware for SessionManagerGenericServiceTicket {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerGenericServiceTicket)
    }
}

impl DataTypeAware for SessionManagerLocalTicket {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerLocalTicket)
    }
}

impl DataTypeAware for SessionManagerServiceRequestSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerServiceRequestSpec)
    }
}

impl DataTypeAware for SessionManagerHttpServiceRequestSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerHttpServiceRequestSpec)
    }
}

impl DataTypeAware for SessionManagerVmomiServiceRequestSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerVmomiServiceRequestSpec)
    }
}

impl DataTypeAware for SharesInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SharesInfo)
    }
}

impl DataTypeAware for SharesOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SharesOption)
    }
}

impl DataTypeAware for SiteInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SiteInfo)
    }
}

impl DataTypeAware for StoragePodSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePodSummary)
    }
}

impl DataTypeAware for StorageIoAllocationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageIoAllocationInfo)
    }
}

impl DataTypeAware for StorageIoAllocationOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageIoAllocationOption)
    }
}

impl DataTypeAware for StorageIormInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageIormInfo)
    }
}

impl DataTypeAware for StorageIormConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageIormConfigOption)
    }
}

impl DataTypeAware for StorageIormConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageIormConfigSpec)
    }
}

impl DataTypeAware for PodStorageDrsEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PodStorageDrsEntry)
    }
}

impl DataTypeAware for StoragePerformanceSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePerformanceSummary)
    }
}

impl DataTypeAware for StorageResourceManagerStorageProfileStatistics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageResourceManagerStorageProfileStatistics)
    }
}

impl DataTypeAware for Tag {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Tag)
    }
}

impl DataTypeAware for TaskDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskDescription)
    }
}

impl DataTypeAware for TaskFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskFilterSpec)
    }
}

impl DataTypeAware for TaskFilterSpecByEntity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskFilterSpecByEntity)
    }
}

impl DataTypeAware for TaskFilterSpecByTime {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskFilterSpecByTime)
    }
}

impl DataTypeAware for TaskFilterSpecByUsername {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskFilterSpecByUsername)
    }
}

impl DataTypeAware for TaskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskInfo)
    }
}

impl DataTypeAware for TaskInfoFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskInfoFilterSpec)
    }
}

impl DataTypeAware for TaskInfoFilterSpecFilterTaskResults {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskInfoFilterSpecFilterTaskResults)
    }
}

impl DataTypeAware for TaskManagerTaskViewSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskManagerTaskViewSpec)
    }
}

impl DataTypeAware for TaskManagerViewByStartId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskManagerViewByStartId)
    }
}

impl DataTypeAware for TaskReason {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReason)
    }
}

impl DataTypeAware for TaskReasonAlarm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReasonAlarm)
    }
}

impl DataTypeAware for TaskReasonSchedule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReasonSchedule)
    }
}

impl DataTypeAware for TaskReasonSystem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReasonSystem)
    }
}

impl DataTypeAware for TaskReasonUser {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReasonUser)
    }
}

impl DataTypeAware for UpdateVirtualMachineFilesResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UpdateVirtualMachineFilesResult)
    }
}

impl DataTypeAware for UpdateVirtualMachineFilesResultFailedVmFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UpdateVirtualMachineFilesResultFailedVmFileInfo)
    }
}

impl DataTypeAware for UserSearchResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserSearchResult)
    }
}

impl DataTypeAware for PosixUserSearchResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PosixUserSearchResult)
    }
}

impl DataTypeAware for UserSession {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserSession)
    }
}

impl DataTypeAware for VVolVmConfigFileUpdateResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VVolVmConfigFileUpdateResult)
    }
}

impl DataTypeAware for VVolVmConfigFileUpdateResultFailedVmConfigFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VVolVmConfigFileUpdateResultFailedVmConfigFileInfo)
    }
}

impl DataTypeAware for VasaStorageArray {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaStorageArray)
    }
}

impl DataTypeAware for VasaStorageArrayDiscoveryFcTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaStorageArrayDiscoveryFcTransport)
    }
}

impl DataTypeAware for VasaStorageArrayDiscoveryIpTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaStorageArrayDiscoveryIpTransport)
    }
}

impl DataTypeAware for VasaStorageArrayDiscoverySvcInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaStorageArrayDiscoverySvcInfo)
    }
}

impl DataTypeAware for VasaProviderContainerSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VasaProviderContainerSpec)
    }
}

impl DataTypeAware for VimVasaProvider {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVasaProvider)
    }
}

impl DataTypeAware for VimVasaProviderStatePerArray {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVasaProviderStatePerArray)
    }
}

impl DataTypeAware for VimVasaProviderVirtualHostConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVasaProviderVirtualHostConfig)
    }
}

impl DataTypeAware for VimVasaProviderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVasaProviderInfo)
    }
}

impl DataTypeAware for VirtualAppLinkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualAppLinkInfo)
    }
}

impl DataTypeAware for VirtualDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSpec)
    }
}

impl DataTypeAware for DeviceBackedVirtualDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DeviceBackedVirtualDiskSpec)
    }
}

impl DataTypeAware for FileBackedVirtualDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileBackedVirtualDiskSpec)
    }
}

impl DataTypeAware for SeSparseVirtualDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SeSparseVirtualDiskSpec)
    }
}

impl DataTypeAware for VirtualMachineConnection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConnection)
    }
}

impl DataTypeAware for VirtualMachineMksConnection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMksConnection)
    }
}

impl DataTypeAware for DiskChangeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiskChangeInfo)
    }
}

impl DataTypeAware for DiskChangeExtent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiskChangeExtent)
    }
}

impl DataTypeAware for VirtualMachineDisplayTopology {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDisplayTopology)
    }
}

impl DataTypeAware for VirtualMachineMksTicket {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMksTicket)
    }
}

impl DataTypeAware for StorageRequirement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageRequirement)
    }
}

impl DataTypeAware for VirtualMachineTicket {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineTicket)
    }
}

impl DataTypeAware for VirtualMachineWipeResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineWipeResult)
    }
}

impl DataTypeAware for VsanComparator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanComparator)
    }
}

impl DataTypeAware for VsanJsonComparator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanJsonComparator)
    }
}

impl DataTypeAware for VsanNestJsonComparator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNestJsonComparator)
    }
}

impl DataTypeAware for VsanDataObfuscationRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataObfuscationRule)
    }
}

impl DataTypeAware for VsanJsonFilterRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanJsonFilterRule)
    }
}

impl DataTypeAware for VsanMassCollectorPropertyParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMassCollectorPropertyParams)
    }
}

impl DataTypeAware for VsanMassCollectorSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMassCollectorSpec)
    }
}

impl DataTypeAware for VsanObjectTypeRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectTypeRule)
    }
}

impl DataTypeAware for VsanRegexBasedRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRegexBasedRule)
    }
}

impl DataTypeAware for VsanResourceConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceConstraint)
    }
}

impl DataTypeAware for VsanCompositeConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCompositeConstraint)
    }
}

impl DataTypeAware for VsanPropertyConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPropertyConstraint)
    }
}

impl DataTypeAware for VsanUpgradeSystemNetworkPartitionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemNetworkPartitionInfo)
    }
}

impl DataTypeAware for VsanUpgradeSystemPreflightCheckIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemPreflightCheckIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemApiBrokenIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemApiBrokenIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemAutoClaimEnabledOnHostsIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemAutoClaimEnabledOnHostsIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemHostsDisconnectedIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemHostsDisconnectedIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemMissingHostsInClusterIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemMissingHostsInClusterIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemNetworkPartitionIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemNetworkPartitionIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemNotEnoughFreeCapacityIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemNotEnoughFreeCapacityIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemRogueHostsInClusterIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemRogueHostsInClusterIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemV2ObjectsPresentDuringDowngradeIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemWrongEsxVersionIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemWrongEsxVersionIssue)
    }
}

impl DataTypeAware for VsanBrokenDiskChainIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanBrokenDiskChainIssue)
    }
}

impl DataTypeAware for VsanDisallowDataMovementIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDisallowDataMovementIssue)
    }
}

impl DataTypeAware for VsanDisallowEvacuateDataIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDisallowEvacuateDataIssue)
    }
}

impl DataTypeAware for VsanDiskUnhealthIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskUnhealthIssue)
    }
}

impl DataTypeAware for VsanHigherObjectsPresentDuringDowngradeIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHigherObjectsPresentDuringDowngradeIssue)
    }
}

impl DataTypeAware for VsanHostPropertyRetrieveIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostPropertyRetrieveIssue)
    }
}

impl DataTypeAware for VsanHostWithHybridDiskgroupIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostWithHybridDiskgroupIssue)
    }
}

impl DataTypeAware for VsanHostsCompressionOnlyNotSupported {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostsCompressionOnlyNotSupported)
    }
}

impl DataTypeAware for VsanMixedEsxVersionInClientIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMixedEsxVersionInClientIssue)
    }
}

impl DataTypeAware for VsanMixedEsxVersionIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMixedEsxVersionIssue)
    }
}

impl DataTypeAware for VsanObjectInaccessibleIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectInaccessibleIssue)
    }
}

impl DataTypeAware for VsanObjectPolicyIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectPolicyIssue)
    }
}

impl DataTypeAware for VsanRemoteClusterNotCompatible {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRemoteClusterNotCompatible)
    }
}

impl DataTypeAware for VsanUnknownScanIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUnknownScanIssue)
    }
}

impl DataTypeAware for VsanUnsupportedHighDiskVersionIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUnsupportedHighDiskVersionIssue)
    }
}

impl DataTypeAware for VsanUpgradeSystemPreflightCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemPreflightCheckResult)
    }
}

impl DataTypeAware for VsanDiskFormatConversionCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskFormatConversionCheckResult)
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeHistoryItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeHistoryItem)
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeHistoryDiskGroupOp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeHistoryDiskGroupOp)
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeHistoryPreflightFail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeHistoryPreflightFail)
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeHistoryStoragePoolOp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeHistoryStoragePoolOp)
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeStatus)
    }
}

impl DataTypeAware for VsanUpgradeStatusEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeStatusEx)
    }
}

impl DataTypeAware for Action {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Action)
    }
}

impl DataTypeAware for CreateTaskAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CreateTaskAction)
    }
}

impl DataTypeAware for MethodAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MethodAction)
    }
}

impl DataTypeAware for RunScriptAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RunScriptAction)
    }
}

impl DataTypeAware for SendEmailAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SendEmailAction)
    }
}

impl DataTypeAware for SendSnmpAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SendSnmpAction)
    }
}

impl DataTypeAware for MethodActionArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MethodActionArgument)
    }
}

impl DataTypeAware for AlarmAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmAction)
    }
}

impl DataTypeAware for AlarmTriggeringAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmTriggeringAction)
    }
}

impl DataTypeAware for GroupAlarmAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupAlarmAction)
    }
}

impl DataTypeAware for AlarmDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmDescription)
    }
}

impl DataTypeAware for AlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmExpression)
    }
}

impl DataTypeAware for AndAlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AndAlarmExpression)
    }
}

impl DataTypeAware for EventAlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventAlarmExpression)
    }
}

impl DataTypeAware for MetricAlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MetricAlarmExpression)
    }
}

impl DataTypeAware for OrAlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OrAlarmExpression)
    }
}

impl DataTypeAware for StateAlarmExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StateAlarmExpression)
    }
}

impl DataTypeAware for AlarmFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmFilterSpec)
    }
}

impl DataTypeAware for AlarmSetting {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmSetting)
    }
}

impl DataTypeAware for AlarmSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmSpec)
    }
}

impl DataTypeAware for AlarmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmInfo)
    }
}

impl DataTypeAware for AlarmState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmState)
    }
}

impl DataTypeAware for AlarmTriggeringActionTransitionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmTriggeringActionTransitionSpec)
    }
}

impl DataTypeAware for EventAlarmExpressionComparison {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventAlarmExpressionComparison)
    }
}

impl DataTypeAware for ClusterAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAction)
    }
}

impl DataTypeAware for ClusterClusterInitialPlacementAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterClusterInitialPlacementAction)
    }
}

impl DataTypeAware for ClusterHostInfraUpdateHaModeAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterHostInfraUpdateHaModeAction)
    }
}

impl DataTypeAware for ClusterHostPowerAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterHostPowerAction)
    }
}

impl DataTypeAware for ClusterInitialPlacementAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterInitialPlacementAction)
    }
}

impl DataTypeAware for ClusterMigrationAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterMigrationAction)
    }
}

impl DataTypeAware for PlacementAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementAction)
    }
}

impl DataTypeAware for HbrDiskMigrationAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrDiskMigrationAction)
    }
}

impl DataTypeAware for StorageMigrationAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageMigrationAction)
    }
}

impl DataTypeAware for StoragePlacementAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePlacementAction)
    }
}

impl DataTypeAware for ClusterActionHistory {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterActionHistory)
    }
}

impl DataTypeAware for ClusterAttemptedVmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAttemptedVmInfo)
    }
}

impl DataTypeAware for ClusterPowerContext {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterPowerContext)
    }
}

impl DataTypeAware for ClusterConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterConfigInfo)
    }
}

impl DataTypeAware for ClusterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterConfigSpec)
    }
}

impl DataTypeAware for ClusterCryptoConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterCryptoConfigInfo)
    }
}

impl DataTypeAware for ClusterDasAamNodeState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAamNodeState)
    }
}

impl DataTypeAware for ClusterDasAdmissionControlInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdmissionControlInfo)
    }
}

impl DataTypeAware for ClusterFailoverHostAdmissionControlInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverHostAdmissionControlInfo)
    }
}

impl DataTypeAware for ClusterFailoverLevelAdmissionControlInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverLevelAdmissionControlInfo)
    }
}

impl DataTypeAware for ClusterFailoverResourcesAdmissionControlInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverResourcesAdmissionControlInfo)
    }
}

impl DataTypeAware for ClusterDasAdmissionControlPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdmissionControlPolicy)
    }
}

impl DataTypeAware for ClusterFailoverHostAdmissionControlPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverHostAdmissionControlPolicy)
    }
}

impl DataTypeAware for ClusterFailoverLevelAdmissionControlPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverLevelAdmissionControlPolicy)
    }
}

impl DataTypeAware for ClusterFailoverResourcesAdmissionControlPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverResourcesAdmissionControlPolicy)
    }
}

impl DataTypeAware for ClusterDasAdvancedRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdvancedRuntimeInfo)
    }
}

impl DataTypeAware for ClusterDasFailoverLevelAdvancedRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo)
    }
}

impl DataTypeAware for DasHeartbeatDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DasHeartbeatDatastoreInfo)
    }
}

impl DataTypeAware for ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo)
    }
}

impl DataTypeAware for ClusterDasConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasConfigInfo)
    }
}

impl DataTypeAware for ClusterDasData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasData)
    }
}

impl DataTypeAware for ClusterDasDataSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasDataSummary)
    }
}

impl DataTypeAware for ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoHostSlots)
    }
}

impl DataTypeAware for ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoSlotInfo)
    }
}

impl DataTypeAware for ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasFailoverLevelAdvancedRuntimeInfoVmSlots)
    }
}

impl DataTypeAware for ClusterDasFdmHostState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasFdmHostState)
    }
}

impl DataTypeAware for ClusterDasHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasHostInfo)
    }
}

impl DataTypeAware for ClusterDasAamHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAamHostInfo)
    }
}

impl DataTypeAware for ClusterDasHostRecommendation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasHostRecommendation)
    }
}

impl DataTypeAware for ClusterDasVmConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasVmConfigInfo)
    }
}

impl DataTypeAware for ClusterDasVmSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasVmSettings)
    }
}

impl DataTypeAware for ClusterDpmConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDpmConfigInfo)
    }
}

impl DataTypeAware for ClusterDpmHostConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDpmHostConfigInfo)
    }
}

impl DataTypeAware for ClusterDrsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsConfigInfo)
    }
}

impl DataTypeAware for ClusterDrsFaults {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsFaults)
    }
}

impl DataTypeAware for ClusterDrsFaultsFaultsByVm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsFaultsFaultsByVm)
    }
}

impl DataTypeAware for ClusterDrsFaultsFaultsByVirtualDisk {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsFaultsFaultsByVirtualDisk)
    }
}

impl DataTypeAware for ClusterDrsMigration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsMigration)
    }
}

impl DataTypeAware for ClusterDrsRecommendation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsRecommendation)
    }
}

impl DataTypeAware for ClusterDrsVmConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsVmConfigInfo)
    }
}

impl DataTypeAware for ClusterEvcManagerCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterEvcManagerCheckResult)
    }
}

impl DataTypeAware for ClusterEvcManagerEvcState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterEvcManagerEvcState)
    }
}

impl DataTypeAware for ClusterEnterMaintenanceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterEnterMaintenanceResult)
    }
}

impl DataTypeAware for ClusterFailoverHostAdmissionControlInfoHostStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFailoverHostAdmissionControlInfoHostStatus)
    }
}

impl DataTypeAware for ClusterGroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterGroupInfo)
    }
}

impl DataTypeAware for ClusterHostGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterHostGroup)
    }
}

impl DataTypeAware for ClusterVmGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmGroup)
    }
}

impl DataTypeAware for ClusterHostRecommendation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterHostRecommendation)
    }
}

impl DataTypeAware for ClusterInfraUpdateHaConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterInfraUpdateHaConfigInfo)
    }
}

impl DataTypeAware for ClusterNotAttemptedVmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterNotAttemptedVmInfo)
    }
}

impl DataTypeAware for ClusterOrchestrationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterOrchestrationInfo)
    }
}

impl DataTypeAware for PerformClusterPowerActionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerformClusterPowerActionSpec)
    }
}

impl DataTypeAware for PlacementResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementResult)
    }
}

impl DataTypeAware for PlacementSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementSpec)
    }
}

impl DataTypeAware for ClusterPowerOnVmResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterPowerOnVmResult)
    }
}

impl DataTypeAware for ClusterPreemptibleVmPairInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterPreemptibleVmPairInfo)
    }
}

impl DataTypeAware for ClusterProactiveDrsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProactiveDrsConfigInfo)
    }
}

impl DataTypeAware for QueryVsanManagedStorageSpaceUsageSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::QueryVsanManagedStorageSpaceUsageSpec)
    }
}

impl DataTypeAware for ClusterRecommendation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterRecommendation)
    }
}

impl DataTypeAware for ClusterResourceUsageSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterResourceUsageSummary)
    }
}

impl DataTypeAware for ClusterRuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterRuleInfo)
    }
}

impl DataTypeAware for ClusterAffinityRuleSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAffinityRuleSpec)
    }
}

impl DataTypeAware for ClusterAntiAffinityRuleSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAntiAffinityRuleSpec)
    }
}

impl DataTypeAware for ClusterDependencyRuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDependencyRuleInfo)
    }
}

impl DataTypeAware for ClusterFtVmHostRuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFtVmHostRuleInfo)
    }
}

impl DataTypeAware for ClusterVmHostRuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmHostRuleInfo)
    }
}

impl DataTypeAware for VirtualDiskAntiAffinityRuleSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskAntiAffinityRuleSpec)
    }
}

impl DataTypeAware for VirtualDiskRuleSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRuleSpec)
    }
}

impl DataTypeAware for VsanSiteFaultDomain {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSiteFaultDomain)
    }
}

impl DataTypeAware for VsanSiteFaultDomainConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSiteFaultDomainConfig)
    }
}

impl DataTypeAware for ClusterSlotPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterSlotPolicy)
    }
}

impl DataTypeAware for ClusterFixedSizeSlotPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterFixedSizeSlotPolicy)
    }
}

impl DataTypeAware for VsanStorageComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStorageComplianceResult)
    }
}

impl DataTypeAware for VsanStorageOperationalStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStorageOperationalStatus)
    }
}

impl DataTypeAware for VsanStoragePolicyStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStoragePolicyStatus)
    }
}

impl DataTypeAware for ClusterSystemVMsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterSystemVMsConfigInfo)
    }
}

impl DataTypeAware for ClusterSystemVMsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterSystemVMsConfigSpec)
    }
}

impl DataTypeAware for ClusterUsageSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterUsageSummary)
    }
}

impl DataTypeAware for VimClusterVsanPreferredFaultDomainInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanPreferredFaultDomainInfo)
    }
}

impl DataTypeAware for VimClusterVsanStretchedClusterCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanStretchedClusterCapability)
    }
}

impl DataTypeAware for VimClusterVsanStretchedClusterFaultDomainConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanStretchedClusterFaultDomainConfig)
    }
}

impl DataTypeAware for VsanStretchedClusterHostVirtualApplianceStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStretchedClusterHostVirtualApplianceStatus)
    }
}

impl DataTypeAware for VimClusterVsanWitnessHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanWitnessHostInfo)
    }
}

impl DataTypeAware for ClusterVmComponentProtectionSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmComponentProtectionSettings)
    }
}

impl DataTypeAware for ClusterVmOrchestrationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmOrchestrationInfo)
    }
}

impl DataTypeAware for ClusterVmReadiness {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmReadiness)
    }
}

impl DataTypeAware for ClusterVmToolsMonitoringSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmToolsMonitoringSettings)
    }
}

impl DataTypeAware for VsanAttachToSrOperation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanAttachToSrOperation)
    }
}

impl DataTypeAware for VsanCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCapability)
    }
}

impl DataTypeAware for VsanClusterAdvCfgSyncHostResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterAdvCfgSyncHostResult)
    }
}

impl DataTypeAware for VsanClusterAdvCfgSyncResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterAdvCfgSyncResult)
    }
}

impl DataTypeAware for VsanClusterBalancePerDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterBalancePerDiskInfo)
    }
}

impl DataTypeAware for VsanClusterBalanceSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterBalanceSummary)
    }
}

impl DataTypeAware for VsanClusterClomdLivenessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterClomdLivenessResult)
    }
}

impl DataTypeAware for VsanClusterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterConfig)
    }
}

impl DataTypeAware for VsanClusterCreateVmHealthTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterCreateVmHealthTestResult)
    }
}

impl DataTypeAware for VsanClusterDitEncryptionHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterDitEncryptionHealthSummary)
    }
}

impl DataTypeAware for VsanClusterEncryptionHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterEncryptionHealthSummary)
    }
}

impl DataTypeAware for VsanClusterFileServiceHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterFileServiceHealthSummary)
    }
}

impl DataTypeAware for VsanClusterGlobalDedupHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterGlobalDedupHealthSummary)
    }
}

impl DataTypeAware for VsanClusterHclInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHclInfo)
    }
}

impl DataTypeAware for VsanClusterHealthAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthAction)
    }
}

impl DataTypeAware for VsanClusterHealthCheckInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthCheckInfo)
    }
}

impl DataTypeAware for VsanClusterHealthConfigs {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthConfigs)
    }
}

impl DataTypeAware for VsanClusterHealthGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthGroup)
    }
}

impl DataTypeAware for VsanClusterHealthLinkBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthLinkBase)
    }
}

impl DataTypeAware for VsanClusterHealthLink {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthLink)
    }
}

impl DataTypeAware for VsanClusterHealthQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthQuerySpec)
    }
}

impl DataTypeAware for VsanClusterHealthResultBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultBase)
    }
}

impl DataTypeAware for VsanClusterHealthResultTable {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultTable)
    }
}

impl DataTypeAware for VsanClusterHealthResultWithRemediation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultWithRemediation)
    }
}

impl DataTypeAware for VsanClusterHealthResultColumnInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultColumnInfo)
    }
}

impl DataTypeAware for VsanClusterHealthResultKeyValuePair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultKeyValuePair)
    }
}

impl DataTypeAware for VsanClusterHealthResultRow {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultRow)
    }
}

impl DataTypeAware for VsanClusterHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthSummary)
    }
}

impl DataTypeAware for VsanClusterHealthSystemObjectsRepairResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthSystemObjectsRepairResult)
    }
}

impl DataTypeAware for VsanClusterHealthSystemStatusResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthSystemStatusResult)
    }
}

impl DataTypeAware for VsanClusterHealthSystemVersionResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthSystemVersionResult)
    }
}

impl DataTypeAware for VsanClusterHealthTest {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthTest)
    }
}

impl DataTypeAware for VsanClusterHostVmknicMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHostVmknicMapping)
    }
}

impl DataTypeAware for VsanClusterLimitHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterLimitHealthResult)
    }
}

impl DataTypeAware for VsanClusterNetworkHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterNetworkHealthResult)
    }
}

impl DataTypeAware for VsanClusterNetworkLoadTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterNetworkLoadTestResult)
    }
}

impl DataTypeAware for VsanClusterNetworkPartitionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterNetworkPartitionInfo)
    }
}

impl DataTypeAware for VsanClusterNetworkPerfTaskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterNetworkPerfTaskSpec)
    }
}

impl DataTypeAware for VsanClusterProactiveTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterProactiveTestResult)
    }
}

impl DataTypeAware for VsanClusterTelemetryProxyConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterTelemetryProxyConfig)
    }
}

impl DataTypeAware for VsanClusterVMsHealthOverallResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterVMsHealthOverallResult)
    }
}

impl DataTypeAware for VsanClusterVMsHealthSummaryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterVMsHealthSummaryResult)
    }
}

impl DataTypeAware for VsanClusterVmdkLoadTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterVmdkLoadTestResult)
    }
}

impl DataTypeAware for VsanClusterWhatifHostFailuresResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterWhatifHostFailuresResult)
    }
}

impl DataTypeAware for VsanConfigGeneration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigGeneration)
    }
}

impl DataTypeAware for VsanDataDrivenApiAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataDrivenApiAction)
    }
}

impl DataTypeAware for VsanDiagnosticsThreshold {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiagnosticsThreshold)
    }
}

impl DataTypeAware for VsanDiskFormatConversionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskFormatConversionSpec)
    }
}

impl DataTypeAware for VimClusterVsanDiskMappingsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanDiskMappingsConfigSpec)
    }
}

impl DataTypeAware for VsanEntitySpaceUsage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEntitySpaceUsage)
    }
}

impl DataTypeAware for VimClusterVsanFaultDomainSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanFaultDomainSpec)
    }
}

impl DataTypeAware for VsanFaultDomainDestroySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFaultDomainDestroySpec)
    }
}

impl DataTypeAware for VsanFaultDomainUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFaultDomainUpdateSpec)
    }
}

impl DataTypeAware for VimClusterVsanFaultDomainsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanFaultDomainsConfigSpec)
    }
}

impl DataTypeAware for VsanHealthActionBase {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthActionBase)
    }
}

impl DataTypeAware for VsanHealthActionSteps {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthActionSteps)
    }
}

impl DataTypeAware for VsanHealthApiBasedAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthApiBasedAction)
    }
}

impl DataTypeAware for VsanHealthCmdBasedAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthCmdBasedAction)
    }
}

impl DataTypeAware for VsanHealthDataDrivenAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthDataDrivenAction)
    }
}

impl DataTypeAware for VsanHealthTxtBasedAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthTxtBasedAction)
    }
}

impl DataTypeAware for VsanHealthConfirmationDialog {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthConfirmationDialog)
    }
}

impl DataTypeAware for VsanHealthCorrelation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthCorrelation)
    }
}

impl DataTypeAware for VsanHealthExtMgmtPreCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthExtMgmtPreCheckResult)
    }
}

impl DataTypeAware for VsanHealthTroubleshooting {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthTroubleshooting)
    }
}

impl DataTypeAware for VsanHistoricalHealthQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHistoricalHealthQuerySpec)
    }
}

impl DataTypeAware for VsanHistoricalHealthTest {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHistoricalHealthTest)
    }
}

impl DataTypeAware for VsanHostClomdLivenessResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostClomdLivenessResult)
    }
}

impl DataTypeAware for VsanHostCreateVmHealthTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostCreateVmHealthTestResult)
    }
}

impl DataTypeAware for VimClusterVsanHostDiskMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanHostDiskMapping)
    }
}

impl DataTypeAware for VsanHostHealthSystemVersionResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostHealthSystemVersionResult)
    }
}

impl DataTypeAware for VsanIoInsightInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoInsightInstance)
    }
}

impl DataTypeAware for VsanIoInsightInstanceQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoInsightInstanceQuerySpec)
    }
}

impl DataTypeAware for VsanIscsiHomeObjectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiHomeObjectSpec)
    }
}

impl DataTypeAware for VsanIscsiInitiatorGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiInitiatorGroup)
    }
}

impl DataTypeAware for VsanIscsiLunCommonInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiLunCommonInfo)
    }
}

impl DataTypeAware for VsanIscsiLun {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiLun)
    }
}

impl DataTypeAware for VsanIscsiLunSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiLunSpec)
    }
}

impl DataTypeAware for VsanIscsiTargetAuthSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetAuthSpec)
    }
}

impl DataTypeAware for VsanIscsiTargetBasicInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetBasicInfo)
    }
}

impl DataTypeAware for VsanIscsiTargetCommonInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetCommonInfo)
    }
}

impl DataTypeAware for VsanIscsiTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTarget)
    }
}

impl DataTypeAware for VsanIscsiTargetSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetSpec)
    }
}

impl DataTypeAware for VsanIscsiTargetServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetServiceConfig)
    }
}

impl DataTypeAware for VsanIscsiTargetServiceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetServiceSpec)
    }
}

impl DataTypeAware for VsanIscsiTargetServiceDefaultConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetServiceDefaultConfigSpec)
    }
}

impl DataTypeAware for VsanNetworkDiagnostics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkDiagnostics)
    }
}

impl DataTypeAware for VsanObjIdentityQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjIdentityQuerySpec)
    }
}

impl DataTypeAware for VsanClusterObjectExtAttrs {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterObjectExtAttrs)
    }
}

impl DataTypeAware for VsanObjectExtraAttributes {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectExtraAttributes)
    }
}

impl DataTypeAware for VsanObjectIdentity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectIdentity)
    }
}

impl DataTypeAware for VsanObjectIdentityAndHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectIdentityAndHealth)
    }
}

impl DataTypeAware for VsanObjectInformation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectInformation)
    }
}

impl DataTypeAware for VsanObjectQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectQuerySpec)
    }
}

impl DataTypeAware for VsanObjectSpaceSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectSpaceSummary)
    }
}

impl DataTypeAware for VsanPerfDiagnoseQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfDiagnoseQuerySpec)
    }
}

impl DataTypeAware for VsanPerfDiagnosticException {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfDiagnosticException)
    }
}

impl DataTypeAware for VsanPerfDiagnosticResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfDiagnosticResult)
    }
}

impl DataTypeAware for VsanPerfEntityMetricCsv {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfEntityMetricCsv)
    }
}

impl DataTypeAware for VsanPerfEntityType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfEntityType)
    }
}

impl DataTypeAware for VsanPerfGraph {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfGraph)
    }
}

impl DataTypeAware for VsanPerfHotspotEntitiesMetrics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfHotspotEntitiesMetrics)
    }
}

impl DataTypeAware for VsanPerfHotspotQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfHotspotQuerySpec)
    }
}

impl DataTypeAware for VsanPerfMasterInformation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfMasterInformation)
    }
}

impl DataTypeAware for VsanPerfMemberInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfMemberInfo)
    }
}

impl DataTypeAware for VsanPerfMetricId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfMetricId)
    }
}

impl DataTypeAware for VsanPerfMetricSeriesCsv {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfMetricSeriesCsv)
    }
}

impl DataTypeAware for VsanPerfNodeInformation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfNodeInformation)
    }
}

impl DataTypeAware for VsanPerfQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfQuerySpec)
    }
}

impl DataTypeAware for VsanPerfThreshold {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfThreshold)
    }
}

impl DataTypeAware for VsanPerfTimeRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfTimeRange)
    }
}

impl DataTypeAware for VsanPerfTimeRangeQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfTimeRangeQuerySpec)
    }
}

impl DataTypeAware for VsanPerfTopEntities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfTopEntities)
    }
}

impl DataTypeAware for VsanPerfTopEntity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfTopEntity)
    }
}

impl DataTypeAware for VsanPerfTopQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfTopQuerySpec)
    }
}

impl DataTypeAware for VsanPerfsvcConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfsvcConfig)
    }
}

impl DataTypeAware for VsanRemoteClusterQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRemoteClusterQuerySpec)
    }
}

impl DataTypeAware for VsanSpaceQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceQuerySpec)
    }
}

impl DataTypeAware for VsanSpaceUsage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceUsage)
    }
}

impl DataTypeAware for VsanSpaceUsageDetailResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceUsageDetailResult)
    }
}

impl DataTypeAware for VsanSpaceUsageWithDatastoreType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceUsageWithDatastoreType)
    }
}

impl DataTypeAware for VsanStorageWorkloadType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStorageWorkloadType)
    }
}

impl DataTypeAware for VsanStretchedClusterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStretchedClusterConfig)
    }
}

impl DataTypeAware for VsanSyncingObjectFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSyncingObjectFilter)
    }
}

impl DataTypeAware for VsanUnicastAddressInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUnicastAddressInfo)
    }
}

impl DataTypeAware for VsanVcKmipServersHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcKmipServersHealth)
    }
}

impl DataTypeAware for VsanVcLifecycleCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcLifecycleCheckResult)
    }
}

impl DataTypeAware for VsanVcLifecycleCheckSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcLifecycleCheckSpec)
    }
}

impl DataTypeAware for VsanVsanClusterPcapGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVsanClusterPcapGroup)
    }
}

impl DataTypeAware for VsanVsanClusterPcapResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVsanClusterPcapResult)
    }
}

impl DataTypeAware for VsanVumSystemConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVumSystemConfig)
    }
}

impl DataTypeAware for VsanWhatifCapacity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanWhatifCapacity)
    }
}

impl DataTypeAware for VimClusterVsanWitnessSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanWitnessSpec)
    }
}

impl DataTypeAware for CnsAccessControlSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsAccessControlSpec)
    }
}

impl DataTypeAware for CnsNfsAccessControlSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsNfsAccessControlSpec)
    }
}

impl DataTypeAware for CnsBackingObjectDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBackingObjectDetails)
    }
}

impl DataTypeAware for CnsBlockBackingDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBlockBackingDetails)
    }
}

impl DataTypeAware for CnsFileBackingDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsFileBackingDetails)
    }
}

impl DataTypeAware for CnsVsanFileShareBackingDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVsanFileShareBackingDetails)
    }
}

impl DataTypeAware for CnsBaseCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBaseCreateSpec)
    }
}

impl DataTypeAware for CnsFileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsFileCreateSpec)
    }
}

impl DataTypeAware for CnsVsanFileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVsanFileCreateSpec)
    }
}

impl DataTypeAware for CnsContainerCluster {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsContainerCluster)
    }
}

impl DataTypeAware for CnsCursor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsCursor)
    }
}

impl DataTypeAware for CnsEntityMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsEntityMetadata)
    }
}

impl DataTypeAware for CnsKubernetesEntityMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsKubernetesEntityMetadata)
    }
}

impl DataTypeAware for CnsKubernetesEntityReference {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsKubernetesEntityReference)
    }
}

impl DataTypeAware for CnsPlacementResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsPlacementResult)
    }
}

impl DataTypeAware for CnsQueryFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsQueryFilter)
    }
}

impl DataTypeAware for CnsKubernetesQueryFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsKubernetesQueryFilter)
    }
}

impl DataTypeAware for CnsQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsQueryResult)
    }
}

impl DataTypeAware for CnsQuerySelection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsQuerySelection)
    }
}

impl DataTypeAware for CnsSnapshotCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsSnapshotCreateSpec)
    }
}

impl DataTypeAware for CnsSnapshotDeleteSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsSnapshotDeleteSpec)
    }
}

impl DataTypeAware for CnsSnapshotId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsSnapshotId)
    }
}

impl DataTypeAware for CnsVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolume)
    }
}

impl DataTypeAware for CnsVolumeAclConfigureSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeAclConfigureSpec)
    }
}

impl DataTypeAware for CnsVolumeAttachDetachSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeAttachDetachSpec)
    }
}

impl DataTypeAware for CnsVolumeCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeCreateSpec)
    }
}

impl DataTypeAware for CnsVolumeExtendSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeExtendSpec)
    }
}

impl DataTypeAware for CnsVolumeId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeId)
    }
}

impl DataTypeAware for CnsVolumeMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeMetadata)
    }
}

impl DataTypeAware for CnsVolumeMetadataUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeMetadataUpdateSpec)
    }
}

impl DataTypeAware for CnsVolumeOperationBatchResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeOperationBatchResult)
    }
}

impl DataTypeAware for CnsVolumeOperationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeOperationResult)
    }
}

impl DataTypeAware for CnsAsyncQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsAsyncQueryResult)
    }
}

impl DataTypeAware for CnsVolumeAttachResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeAttachResult)
    }
}

impl DataTypeAware for CnsVolumeCreateResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeCreateResult)
    }
}

impl DataTypeAware for CnsVolumePolicyReconfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumePolicyReconfigSpec)
    }
}

impl DataTypeAware for CnsVolumeRelocateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeRelocateSpec)
    }
}

impl DataTypeAware for CnsBlockVolumeRelocateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBlockVolumeRelocateSpec)
    }
}

impl DataTypeAware for CnsVolumeSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeSource)
    }
}

impl DataTypeAware for CnsSnapshotVolumeSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsSnapshotVolumeSource)
    }
}

impl DataTypeAware for DistributedVirtualPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualPort)
    }
}

impl DataTypeAware for DvPortConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortConfigInfo)
    }
}

impl DataTypeAware for DvPortConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortConfigSpec)
    }
}

impl DataTypeAware for DvsFilterParameter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterParameter)
    }
}

impl DataTypeAware for DvsHostLocalPortInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHostLocalPortInfo)
    }
}

impl DataTypeAware for DvPortStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortStatus)
    }
}

impl DataTypeAware for DvPortSetting {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortSetting)
    }
}

impl DataTypeAware for VMwareDvsPortSetting {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsPortSetting)
    }
}

impl DataTypeAware for DvPortState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortState)
    }
}

impl DataTypeAware for DvPortgroupConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortgroupConfigInfo)
    }
}

impl DataTypeAware for DvPortgroupConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortgroupConfigSpec)
    }
}

impl DataTypeAware for DistributedVirtualPortgroupNsxPortgroupOperationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualPortgroupNsxPortgroupOperationResult)
    }
}

impl DataTypeAware for DvPortgroupPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortgroupPolicy)
    }
}

impl DataTypeAware for VMwareDvsPortgroupPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsPortgroupPolicy)
    }
}

impl DataTypeAware for DistributedVirtualPortgroupProblem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualPortgroupProblem)
    }
}

impl DataTypeAware for DistributedVirtualPortgroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualPortgroupInfo)
    }
}

impl DataTypeAware for DistributedVirtualSwitchInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchInfo)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerCompatibilityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerCompatibilityResult)
    }
}

impl DataTypeAware for DvsManagerDvsConfigTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsManagerDvsConfigTarget)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerDvsProductSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerDvsProductSpec)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerHostContainer {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostContainer)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerHostDvsFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerHostArrayFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostArrayFilter)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerHostContainerFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostContainerFilter)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerHostDvsMembershipFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostDvsMembershipFilter)
    }
}

impl DataTypeAware for DistributedVirtualSwitchManagerImportResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerImportResult)
    }
}

impl DataTypeAware for DvsManagerPhysicalNicsList {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsManagerPhysicalNicsList)
    }
}

impl DataTypeAware for EntityBackup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityBackup)
    }
}

impl DataTypeAware for EntityBackupConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityBackupConfig)
    }
}

impl DataTypeAware for DvsFilterSpecConnecteeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecConnecteeSpec)
    }
}

impl DataTypeAware for DvsFilterSpecPnicConnecteeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecPnicConnecteeSpec)
    }
}

impl DataTypeAware for DvsFilterSpecVmConnecteeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecVmConnecteeSpec)
    }
}

impl DataTypeAware for DvsFilterSpecVmknicConnecteeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecVmknicConnecteeSpec)
    }
}

impl DataTypeAware for DvsFilterSpecVlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecVlanSpec)
    }
}

impl DataTypeAware for DvsFilterSpecPvlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecPvlanSpec)
    }
}

impl DataTypeAware for DvsFilterSpecTrunkVlanSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecTrunkVlanSpec)
    }
}

impl DataTypeAware for DvsFilterSpecVlanIdSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecVlanIdSpec)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMember {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMember)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberBacking {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberBacking)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberPnicBacking {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberPnicBacking)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberConfigInfo)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberConfigSpec)
    }
}

impl DataTypeAware for HostMemberHealthCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemberHealthCheckResult)
    }
}

impl DataTypeAware for HostMemberUplinkHealthCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemberUplinkHealthCheckResult)
    }
}

impl DataTypeAware for VMwareDvsMtuHealthCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsMtuHealthCheckResult)
    }
}

impl DataTypeAware for VMwareDvsVlanHealthCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsVlanHealthCheckResult)
    }
}

impl DataTypeAware for VMwareDvsTeamingHealthCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsTeamingHealthCheckResult)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberHostUplinkState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberHostUplinkState)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberPnicSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberPnicSpec)
    }
}

impl DataTypeAware for HostMemberRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemberRuntimeInfo)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberRuntimeState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberRuntimeState)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberTransportZoneInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberTransportZoneInfo)
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostProductSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostProductSpec)
    }
}

impl DataTypeAware for DistributedVirtualSwitchKeyedOpaqueBlob {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchKeyedOpaqueBlob)
    }
}

impl DataTypeAware for DistributedVirtualSwitchNetworkOffloadSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchNetworkOffloadSpec)
    }
}

impl DataTypeAware for DvsNetworkResourcePool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkResourcePool)
    }
}

impl DataTypeAware for DvsNetworkResourcePoolAllocationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkResourcePoolAllocationInfo)
    }
}

impl DataTypeAware for DvsNetworkResourcePoolConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkResourcePoolConfigSpec)
    }
}

impl DataTypeAware for DistributedVirtualSwitchPortConnectee {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchPortConnectee)
    }
}

impl DataTypeAware for DistributedVirtualSwitchPortConnection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchPortConnection)
    }
}

impl DataTypeAware for DistributedVirtualSwitchPortCriteria {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchPortCriteria)
    }
}

impl DataTypeAware for DistributedVirtualSwitchPortStatistics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchPortStatistics)
    }
}

impl DataTypeAware for DistributedVirtualSwitchProductSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchProductSpec)
    }
}

impl DataTypeAware for DvsTrafficRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficRule)
    }
}

impl DataTypeAware for DvsNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkRuleAction)
    }
}

impl DataTypeAware for DvsAcceptNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsAcceptNetworkRuleAction)
    }
}

impl DataTypeAware for DvsCopyNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsCopyNetworkRuleAction)
    }
}

impl DataTypeAware for DvsDropNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsDropNetworkRuleAction)
    }
}

impl DataTypeAware for DvsGreEncapNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsGreEncapNetworkRuleAction)
    }
}

impl DataTypeAware for DvsLogNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsLogNetworkRuleAction)
    }
}

impl DataTypeAware for DvsMacRewriteNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsMacRewriteNetworkRuleAction)
    }
}

impl DataTypeAware for DvsPuntNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsPuntNetworkRuleAction)
    }
}

impl DataTypeAware for DvsRateLimitNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsRateLimitNetworkRuleAction)
    }
}

impl DataTypeAware for DvsUpdateTagNetworkRuleAction {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsUpdateTagNetworkRuleAction)
    }
}

impl DataTypeAware for DvsNetworkRuleQualifier {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkRuleQualifier)
    }
}

impl DataTypeAware for DvsIpNetworkRuleQualifier {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsIpNetworkRuleQualifier)
    }
}

impl DataTypeAware for DvsMacNetworkRuleQualifier {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsMacNetworkRuleQualifier)
    }
}

impl DataTypeAware for DvsSystemTrafficNetworkRuleQualifier {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsSystemTrafficNetworkRuleQualifier)
    }
}

impl DataTypeAware for DvsTrafficRuleset {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficRuleset)
    }
}

impl DataTypeAware for DvsVmVnicNetworkResourcePool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVmVnicNetworkResourcePool)
    }
}

impl DataTypeAware for DvsVmVnicResourcePoolConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVmVnicResourcePoolConfigSpec)
    }
}

impl DataTypeAware for DvsVmVnicResourceAllocation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVmVnicResourceAllocation)
    }
}

impl DataTypeAware for DvsVmVnicNetworkResourcePoolRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVmVnicNetworkResourcePoolRuntimeInfo)
    }
}

impl DataTypeAware for DvsVnicAllocatedResource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVnicAllocatedResource)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchDpuFailoverPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchDpuFailoverPolicy)
    }
}

impl DataTypeAware for VMwareDvsDpuCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsDpuCapability)
    }
}

impl DataTypeAware for VMwareIpfixConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareIpfixConfig)
    }
}

impl DataTypeAware for VMwareDvsIpfixCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsIpfixCapability)
    }
}

impl DataTypeAware for VMwareDvsLacpCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsLacpCapability)
    }
}

impl DataTypeAware for VMwareDvsLacpGroupConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsLacpGroupConfig)
    }
}

impl DataTypeAware for VMwareDvsLacpGroupSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsLacpGroupSpec)
    }
}

impl DataTypeAware for VMwareDvsLagIpfixConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsLagIpfixConfig)
    }
}

impl DataTypeAware for VMwareDvsLagVlanConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsLagVlanConfig)
    }
}

impl DataTypeAware for VMwareDvsMtuCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsMtuCapability)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchNetworkOffloadConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchNetworkOffloadConfig)
    }
}

impl DataTypeAware for VMwareDvsPvlanConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsPvlanConfigSpec)
    }
}

impl DataTypeAware for VMwareDvsPvlanMapEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsPvlanMapEntry)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchRealTimeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchRealTimeConfig)
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchRealTimeLanAnnotation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchRealTimeLanAnnotation)
    }
}

impl DataTypeAware for VMwareDvsVspanConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsVspanConfigSpec)
    }
}

impl DataTypeAware for VMwareDvsVspanCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsVspanCapability)
    }
}

impl DataTypeAware for VMwareVspanPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareVspanPort)
    }
}

impl DataTypeAware for VMwareVspanSession {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareVspanSession)
    }
}

impl DataTypeAware for CryptoKeyId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoKeyId)
    }
}

impl DataTypeAware for CryptoKeyPlain {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoKeyPlain)
    }
}

impl DataTypeAware for CryptoKeyResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoKeyResult)
    }
}

impl DataTypeAware for CryptoManagerHostKeyStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerHostKeyStatus)
    }
}

impl DataTypeAware for CryptoManagerKmipCertSignRequest {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCertSignRequest)
    }
}

impl DataTypeAware for CryptoManagerKmipCertificateInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCertificateInfo)
    }
}

impl DataTypeAware for CryptoManagerKmipClusterStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipClusterStatus)
    }
}

impl DataTypeAware for CryptoManagerKmipCryptoKeyStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCryptoKeyStatus)
    }
}

impl DataTypeAware for CryptoManagerKmipCryptoKeyStatusKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCryptoKeyStatusKeyInfo)
    }
}

impl DataTypeAware for CryptoManagerKmipCryptoKeyStatusWrappingKeyIdKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCryptoKeyStatusWrappingKeyIdKeyInfo)
    }
}

impl DataTypeAware for CryptoManagerKmipCryptoKeyStatusWrappingRotationIntervalKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCryptoKeyStatusWrappingRotationIntervalKeyInfo)
    }
}

impl DataTypeAware for CryptoManagerKmipCustomAttributeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCustomAttributeSpec)
    }
}

impl DataTypeAware for CryptoManagerKmipGenerateKeySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipGenerateKeySpec)
    }
}

impl DataTypeAware for CryptoManagerKmipServerCertInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipServerCertInfo)
    }
}

impl DataTypeAware for CryptoManagerKmipServerStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipServerStatus)
    }
}

impl DataTypeAware for CryptoSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpec)
    }
}

impl DataTypeAware for CryptoSpecDecrypt {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecDecrypt)
    }
}

impl DataTypeAware for CryptoSpecDeepRecrypt {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecDeepRecrypt)
    }
}

impl DataTypeAware for CryptoSpecEncrypt {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecEncrypt)
    }
}

impl DataTypeAware for CryptoSpecNoOp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecNoOp)
    }
}

impl DataTypeAware for CryptoSpecRegister {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecRegister)
    }
}

impl DataTypeAware for CryptoSpecShallowRecrypt {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecShallowRecrypt)
    }
}

impl DataTypeAware for KeyProviderId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KeyProviderId)
    }
}

impl DataTypeAware for KmipClusterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipClusterInfo)
    }
}

impl DataTypeAware for KmipClusterInfoKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipClusterInfoKeyInfo)
    }
}

impl DataTypeAware for KmipClusterInfoWrappingKeyIdKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipClusterInfoWrappingKeyIdKeyInfo)
    }
}

impl DataTypeAware for KmipClusterInfoWrappingRotationIntervalKeyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipClusterInfoWrappingRotationIntervalKeyInfo)
    }
}

impl DataTypeAware for KmipServerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerInfo)
    }
}

impl DataTypeAware for KmipServerSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerSpec)
    }
}

impl DataTypeAware for KmipServerSpecKeySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerSpecKeySpec)
    }
}

impl DataTypeAware for KmipServerSpecWrappingKeyIdKeySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerSpecWrappingKeyIdKeySpec)
    }
}

impl DataTypeAware for KmipServerSpecWrappingRotationIntervalKeySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerSpecWrappingRotationIntervalKeySpec)
    }
}

impl DataTypeAware for KmipServerStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerStatus)
    }
}

impl DataTypeAware for ChangesInfoEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ChangesInfoEventArgument)
    }
}

impl DataTypeAware for DvsOutOfSyncHostArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsOutOfSyncHostArgument)
    }
}

impl DataTypeAware for Event {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Event)
    }
}

impl DataTypeAware for EventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventArgument)
    }
}

impl DataTypeAware for EntityEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityEventArgument)
    }
}

impl DataTypeAware for AlarmEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmEventArgument)
    }
}

impl DataTypeAware for ComputeResourceEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceEventArgument)
    }
}

impl DataTypeAware for DatacenterEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatacenterEventArgument)
    }
}

impl DataTypeAware for DatastoreEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreEventArgument)
    }
}

impl DataTypeAware for DvsEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsEventArgument)
    }
}

impl DataTypeAware for FolderEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderEventArgument)
    }
}

impl DataTypeAware for HostEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostEventArgument)
    }
}

impl DataTypeAware for ManagedEntityEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ManagedEntityEventArgument)
    }
}

impl DataTypeAware for NetworkEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkEventArgument)
    }
}

impl DataTypeAware for ResourcePoolEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolEventArgument)
    }
}

impl DataTypeAware for ScheduledTaskEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskEventArgument)
    }
}

impl DataTypeAware for VmEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmEventArgument)
    }
}

impl DataTypeAware for ProfileEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileEventArgument)
    }
}

impl DataTypeAware for RoleEventArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RoleEventArgument)
    }
}

impl DataTypeAware for EventDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventDescription)
    }
}

impl DataTypeAware for EventArgDesc {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventArgDesc)
    }
}

impl DataTypeAware for EventDescriptionEventDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventDescriptionEventDetail)
    }
}

impl DataTypeAware for EventFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventFilterSpec)
    }
}

impl DataTypeAware for EventFilterSpecByEntity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventFilterSpecByEntity)
    }
}

impl DataTypeAware for EventFilterSpecByTime {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventFilterSpecByTime)
    }
}

impl DataTypeAware for EventFilterSpecByUsername {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventFilterSpecByUsername)
    }
}

impl DataTypeAware for EventManagerEventViewSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventManagerEventViewSpec)
    }
}

impl DataTypeAware for EventManagerViewByStartId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventManagerViewByStartId)
    }
}

impl DataTypeAware for ExtendedEventPair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtendedEventPair)
    }
}

impl DataTypeAware for VnicPortArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VnicPortArgument)
    }
}

impl DataTypeAware for ExtExtendedProductInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtExtendedProductInfo)
    }
}

impl DataTypeAware for ManagedByInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ManagedByInfo)
    }
}

impl DataTypeAware for ExtManagedEntityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtManagedEntityInfo)
    }
}

impl DataTypeAware for ExtSolutionManagerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtSolutionManagerInfo)
    }
}

impl DataTypeAware for ExtSolutionManagerInfoTabInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ExtSolutionManagerInfoTabInfo)
    }
}

impl DataTypeAware for AnswerFileUpdateFailure {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileUpdateFailure)
    }
}

impl DataTypeAware for ConflictingConfigurationConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ConflictingConfigurationConfig)
    }
}

impl DataTypeAware for DatacenterMismatchArgument {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatacenterMismatchArgument)
    }
}

impl DataTypeAware for DvsApplyOperationFaultFaultOnObject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsApplyOperationFaultFaultOnObject)
    }
}

impl DataTypeAware for DvsOperationBulkFaultFaultOnHost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsOperationBulkFaultFaultOnHost)
    }
}

impl DataTypeAware for ImportOperationBulkFaultFaultOnImport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ImportOperationBulkFaultFaultOnImport)
    }
}

impl DataTypeAware for MultipleCertificatesVerifyFaultThumbprintData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MultipleCertificatesVerifyFaultThumbprintData)
    }
}

impl DataTypeAware for NoPermissionEntityPrivileges {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoPermissionEntityPrivileges)
    }
}

impl DataTypeAware for ProfileUpdateFailedUpdateFailure {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileUpdateFailedUpdateFailure)
    }
}

impl DataTypeAware for HostActiveDirectory {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostActiveDirectory)
    }
}

impl DataTypeAware for HostActiveDirectorySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostActiveDirectorySpec)
    }
}

impl DataTypeAware for HostAssignableHardwareBinding {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAssignableHardwareBinding)
    }
}

impl DataTypeAware for HostAssignableHardwareConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAssignableHardwareConfig)
    }
}

impl DataTypeAware for HostAssignableHardwareConfigAttributeOverride {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAssignableHardwareConfigAttributeOverride)
    }
}

impl DataTypeAware for HostAuthenticationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAuthenticationInfo)
    }
}

impl DataTypeAware for HostAuthenticationManagerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAuthenticationManagerInfo)
    }
}

impl DataTypeAware for HostAuthenticationStoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAuthenticationStoreInfo)
    }
}

impl DataTypeAware for HostDirectoryStoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDirectoryStoreInfo)
    }
}

impl DataTypeAware for HostActiveDirectoryInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostActiveDirectoryInfo)
    }
}

impl DataTypeAware for HostLocalAuthenticationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLocalAuthenticationInfo)
    }
}

impl DataTypeAware for AutoStartPowerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AutoStartPowerInfo)
    }
}

impl DataTypeAware for HostAutoStartManagerConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAutoStartManagerConfig)
    }
}

impl DataTypeAware for AutoStartDefaults {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AutoStartDefaults)
    }
}

impl DataTypeAware for HostBiosInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostBiosInfo)
    }
}

impl DataTypeAware for HostBootDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostBootDeviceInfo)
    }
}

impl DataTypeAware for HostBootDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostBootDevice)
    }
}

impl DataTypeAware for HostCacheConfigurationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCacheConfigurationInfo)
    }
}

impl DataTypeAware for HostCacheConfigurationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCacheConfigurationSpec)
    }
}

impl DataTypeAware for HostCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCapability)
    }
}

impl DataTypeAware for HostCertificateManagerCertificateInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCertificateManagerCertificateInfo)
    }
}

impl DataTypeAware for HostCertificateManagerCertificateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCertificateManagerCertificateSpec)
    }
}

impl DataTypeAware for HostConfigChange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConfigChange)
    }
}

impl DataTypeAware for HostConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConfigInfo)
    }
}

impl DataTypeAware for HostConfigManager {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConfigManager)
    }
}

impl DataTypeAware for HostConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConfigSpec)
    }
}

impl DataTypeAware for HostConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConnectInfo)
    }
}

impl DataTypeAware for HostDatastoreConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreConnectInfo)
    }
}

impl DataTypeAware for HostDatastoreExistsConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreExistsConnectInfo)
    }
}

impl DataTypeAware for HostDatastoreNameConflictConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreNameConflictConnectInfo)
    }
}

impl DataTypeAware for HostLicenseConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLicenseConnectInfo)
    }
}

impl DataTypeAware for HostConnectInfoNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConnectInfoNetworkInfo)
    }
}

impl DataTypeAware for HostNewNetworkConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNewNetworkConnectInfo)
    }
}

impl DataTypeAware for HostConnectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConnectSpec)
    }
}

impl DataTypeAware for HostCpuIdInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCpuIdInfo)
    }
}

impl DataTypeAware for HostCpuInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCpuInfo)
    }
}

impl DataTypeAware for HostCpuPackage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCpuPackage)
    }
}

impl DataTypeAware for HostCpuPowerManagementInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCpuPowerManagementInfo)
    }
}

impl DataTypeAware for HostCpuSchedulerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostCpuSchedulerInfo)
    }
}

impl DataTypeAware for HostHyperThreadScheduleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHyperThreadScheduleInfo)
    }
}

impl DataTypeAware for HostDataTransportConnectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDataTransportConnectionInfo)
    }
}

impl DataTypeAware for HostNfcConnectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNfcConnectionInfo)
    }
}

impl DataTypeAware for FileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileInfo)
    }
}

impl DataTypeAware for FloppyImageFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FloppyImageFileInfo)
    }
}

impl DataTypeAware for FolderFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderFileInfo)
    }
}

impl DataTypeAware for IsoImageFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IsoImageFileInfo)
    }
}

impl DataTypeAware for VmConfigFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileInfo)
    }
}

impl DataTypeAware for TemplateConfigFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TemplateConfigFileInfo)
    }
}

impl DataTypeAware for VmDiskFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDiskFileInfo)
    }
}

impl DataTypeAware for VmLogFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmLogFileInfo)
    }
}

impl DataTypeAware for VmNvramFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmNvramFileInfo)
    }
}

impl DataTypeAware for VmSnapshotFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmSnapshotFileInfo)
    }
}

impl DataTypeAware for FileQueryFlags {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileQueryFlags)
    }
}

impl DataTypeAware for FileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileQuery)
    }
}

impl DataTypeAware for FloppyImageFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FloppyImageFileQuery)
    }
}

impl DataTypeAware for FolderFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FolderFileQuery)
    }
}

impl DataTypeAware for IsoImageFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IsoImageFileQuery)
    }
}

impl DataTypeAware for VmConfigFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileQuery)
    }
}

impl DataTypeAware for TemplateConfigFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TemplateConfigFileQuery)
    }
}

impl DataTypeAware for VmDiskFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDiskFileQuery)
    }
}

impl DataTypeAware for VmLogFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmLogFileQuery)
    }
}

impl DataTypeAware for VmNvramFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmNvramFileQuery)
    }
}

impl DataTypeAware for VmSnapshotFileQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmSnapshotFileQuery)
    }
}

impl DataTypeAware for HostDatastoreBrowserSearchResults {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreBrowserSearchResults)
    }
}

impl DataTypeAware for HostDatastoreBrowserSearchSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreBrowserSearchSpec)
    }
}

impl DataTypeAware for VmConfigFileEncryptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileEncryptionInfo)
    }
}

impl DataTypeAware for VmConfigFileQueryFlags {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileQueryFlags)
    }
}

impl DataTypeAware for VmConfigFileQueryFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileQueryFilter)
    }
}

impl DataTypeAware for VmDiskFileEncryptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDiskFileEncryptionInfo)
    }
}

impl DataTypeAware for VmDiskFileQueryFlags {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDiskFileQueryFlags)
    }
}

impl DataTypeAware for VmDiskFileQueryFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDiskFileQueryFilter)
    }
}

impl DataTypeAware for HostDatastoreSystemCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreSystemCapabilities)
    }
}

impl DataTypeAware for HostDatastoreSystemDatastoreResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreSystemDatastoreResult)
    }
}

impl DataTypeAware for HostDatastoreSystemVvolDatastoreSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreSystemVvolDatastoreSpec)
    }
}

impl DataTypeAware for HostDateTimeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDateTimeConfig)
    }
}

impl DataTypeAware for HostDateTimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDateTimeInfo)
    }
}

impl DataTypeAware for HostDateTimeSystemServiceTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDateTimeSystemServiceTestResult)
    }
}

impl DataTypeAware for HostDateTimeSystemTimeZone {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDateTimeSystemTimeZone)
    }
}

impl DataTypeAware for HostDeploymentInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDeploymentInfo)
    }
}

impl DataTypeAware for HostDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDevice)
    }
}

impl DataTypeAware for ScsiLun {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScsiLun)
    }
}

impl DataTypeAware for HostScsiDisk {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiDisk)
    }
}

impl DataTypeAware for DevicePciId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DevicePciId)
    }
}

impl DataTypeAware for HostDhcpService {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDhcpService)
    }
}

impl DataTypeAware for HostDhcpServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDhcpServiceConfig)
    }
}

impl DataTypeAware for HostDhcpServiceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDhcpServiceSpec)
    }
}

impl DataTypeAware for HostDiagnosticPartition {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiagnosticPartition)
    }
}

impl DataTypeAware for HostDiagnosticPartitionCreateDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiagnosticPartitionCreateDescription)
    }
}

impl DataTypeAware for HostDiagnosticPartitionCreateOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiagnosticPartitionCreateOption)
    }
}

impl DataTypeAware for HostDiagnosticPartitionCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiagnosticPartitionCreateSpec)
    }
}

impl DataTypeAware for HostDigestInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDigestInfo)
    }
}

impl DataTypeAware for HostTpmDigestInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmDigestInfo)
    }
}

impl DataTypeAware for HostDiskConfigurationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskConfigurationResult)
    }
}

impl DataTypeAware for HostDiskDimensions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskDimensions)
    }
}

impl DataTypeAware for HostDiskDimensionsChs {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskDimensionsChs)
    }
}

impl DataTypeAware for HostDiskDimensionsLba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskDimensionsLba)
    }
}

impl DataTypeAware for HostDiskPartitionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskPartitionInfo)
    }
}

impl DataTypeAware for HostDiskPartitionBlockRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskPartitionBlockRange)
    }
}

impl DataTypeAware for HostDiskPartitionLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskPartitionLayout)
    }
}

impl DataTypeAware for HostDiskPartitionAttributes {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskPartitionAttributes)
    }
}

impl DataTypeAware for HostDiskPartitionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskPartitionSpec)
    }
}

impl DataTypeAware for HostDnsConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDnsConfig)
    }
}

impl DataTypeAware for HostDnsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDnsConfigSpec)
    }
}

impl DataTypeAware for HostDvxClass {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDvxClass)
    }
}

impl DataTypeAware for HostEnterMaintenanceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostEnterMaintenanceResult)
    }
}

impl DataTypeAware for HostEsxAgentHostManagerConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostEsxAgentHostManagerConfigInfo)
    }
}

impl DataTypeAware for HostFaultToleranceManagerComponentHealthInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFaultToleranceManagerComponentHealthInfo)
    }
}

impl DataTypeAware for FcoeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcoeConfig)
    }
}

impl DataTypeAware for FcoeConfigFcoeCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcoeConfigFcoeCapabilities)
    }
}

impl DataTypeAware for FcoeConfigFcoeSpecification {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcoeConfigFcoeSpecification)
    }
}

impl DataTypeAware for FcoeConfigVlanRange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FcoeConfigVlanRange)
    }
}

impl DataTypeAware for HostFeatureCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFeatureCapability)
    }
}

impl DataTypeAware for HostFeatureMask {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFeatureMask)
    }
}

impl DataTypeAware for HostFeatureVersionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFeatureVersionInfo)
    }
}

impl DataTypeAware for HostFibreChannelOverEthernetHbaLinkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelOverEthernetHbaLinkInfo)
    }
}

impl DataTypeAware for HostFileAccess {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFileAccess)
    }
}

impl DataTypeAware for ModeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ModeInfo)
    }
}

impl DataTypeAware for HostFileSystemMountInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFileSystemMountInfo)
    }
}

impl DataTypeAware for HostFileSystemVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFileSystemVolume)
    }
}

impl DataTypeAware for HostLocalFileSystemVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLocalFileSystemVolume)
    }
}

impl DataTypeAware for HostNasVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNasVolume)
    }
}

impl DataTypeAware for HostPMemVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPMemVolume)
    }
}

impl DataTypeAware for HostVfatVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVfatVolume)
    }
}

impl DataTypeAware for HostVffsVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVffsVolume)
    }
}

impl DataTypeAware for HostVmfsVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVmfsVolume)
    }
}

impl DataTypeAware for HostVvolVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVvolVolume)
    }
}

impl DataTypeAware for HostFileSystemVolumeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFileSystemVolumeInfo)
    }
}

impl DataTypeAware for HostFirewallConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallConfig)
    }
}

impl DataTypeAware for HostFirewallConfigRuleSetConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallConfigRuleSetConfig)
    }
}

impl DataTypeAware for HostFirewallInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallInfo)
    }
}

impl DataTypeAware for HostFirewallDefaultPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallDefaultPolicy)
    }
}

impl DataTypeAware for HostFlagInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFlagInfo)
    }
}

impl DataTypeAware for HostForceMountedInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostForceMountedInfo)
    }
}

impl DataTypeAware for HostFru {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFru)
    }
}

impl DataTypeAware for HostGatewaySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostGatewaySpec)
    }
}

impl DataTypeAware for HostGraphicsConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostGraphicsConfig)
    }
}

impl DataTypeAware for HostGraphicsConfigDeviceType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostGraphicsConfigDeviceType)
    }
}

impl DataTypeAware for HostGraphicsInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostGraphicsInfo)
    }
}

impl DataTypeAware for HostHardwareInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHardwareInfo)
    }
}

impl DataTypeAware for HostHardwareStatusInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHardwareStatusInfo)
    }
}

impl DataTypeAware for DpuStatusInfoOperationalInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DpuStatusInfoOperationalInfo)
    }
}

impl DataTypeAware for HostHardwareElementInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHardwareElementInfo)
    }
}

impl DataTypeAware for DpuStatusInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DpuStatusInfo)
    }
}

impl DataTypeAware for HostStorageElementInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageElementInfo)
    }
}

impl DataTypeAware for HostStorageOperationalInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageOperationalInfo)
    }
}

impl DataTypeAware for HostHbaCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHbaCreateSpec)
    }
}

impl DataTypeAware for HostTcpHbaCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTcpHbaCreateSpec)
    }
}

impl DataTypeAware for HealthSystemRuntime {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HealthSystemRuntime)
    }
}

impl DataTypeAware for HostAccessControlEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAccessControlEntry)
    }
}

impl DataTypeAware for HostHostBusAdapter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHostBusAdapter)
    }
}

impl DataTypeAware for HostBlockHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostBlockHba)
    }
}

impl DataTypeAware for HostFibreChannelHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelHba)
    }
}

impl DataTypeAware for HostFibreChannelOverEthernetHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelOverEthernetHba)
    }
}

impl DataTypeAware for HostInternetScsiHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHba)
    }
}

impl DataTypeAware for HostParallelScsiHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostParallelScsiHba)
    }
}

impl DataTypeAware for HostPcieHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPcieHba)
    }
}

impl DataTypeAware for HostRdmaHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaHba)
    }
}

impl DataTypeAware for HostSerialAttachedHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSerialAttachedHba)
    }
}

impl DataTypeAware for HostTcpHba {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTcpHba)
    }
}

impl DataTypeAware for HostProxySwitch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProxySwitch)
    }
}

impl DataTypeAware for HostProxySwitchConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProxySwitchConfig)
    }
}

impl DataTypeAware for HostProxySwitchEnsInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProxySwitchEnsInfo)
    }
}

impl DataTypeAware for HostProxySwitchHostLagConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProxySwitchHostLagConfig)
    }
}

impl DataTypeAware for HostProxySwitchSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProxySwitchSpec)
    }
}

impl DataTypeAware for HostSpbmDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSpbmDatastoreInfo)
    }
}

impl DataTypeAware for HostSpbmHashInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSpbmHashInfo)
    }
}

impl DataTypeAware for HostSpbmPolicyBlobInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSpbmPolicyBlobInfo)
    }
}

impl DataTypeAware for HostSpbmPolicyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSpbmPolicyInfo)
    }
}

impl DataTypeAware for HostImageProfileSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostImageProfileSummary)
    }
}

impl DataTypeAware for HostInternetScsiHbaAuthenticationCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaAuthenticationCapabilities)
    }
}

impl DataTypeAware for HostInternetScsiHbaAuthenticationProperties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaAuthenticationProperties)
    }
}

impl DataTypeAware for HostInternetScsiHbaDigestCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaDigestCapabilities)
    }
}

impl DataTypeAware for HostInternetScsiHbaDigestProperties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaDigestProperties)
    }
}

impl DataTypeAware for HostInternetScsiHbaDiscoveryCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaDiscoveryCapabilities)
    }
}

impl DataTypeAware for HostInternetScsiHbaDiscoveryProperties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaDiscoveryProperties)
    }
}

impl DataTypeAware for HostInternetScsiHbaIpCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaIpCapabilities)
    }
}

impl DataTypeAware for HostInternetScsiHbaIpProperties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaIpProperties)
    }
}

impl DataTypeAware for HostInternetScsiHbaIPv6Properties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaIPv6Properties)
    }
}

impl DataTypeAware for HostInternetScsiHbaIscsiIpv6Address {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaIscsiIpv6Address)
    }
}

impl DataTypeAware for HostInternetScsiHbaSendTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaSendTarget)
    }
}

impl DataTypeAware for HostInternetScsiHbaStaticTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaStaticTarget)
    }
}

impl DataTypeAware for HostInternetScsiHbaTargetSet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaTargetSet)
    }
}

impl DataTypeAware for HostIpConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpConfig)
    }
}

impl DataTypeAware for VsanFileServiceIpConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceIpConfig)
    }
}

impl DataTypeAware for HostIpConfigIpV6Address {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpConfigIpV6Address)
    }
}

impl DataTypeAware for HostIpConfigIpV6AddressConfiguration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpConfigIpV6AddressConfiguration)
    }
}

impl DataTypeAware for HostIpRouteConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteConfig)
    }
}

impl DataTypeAware for HostIpRouteConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteConfigSpec)
    }
}

impl DataTypeAware for HostIpRouteEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteEntry)
    }
}

impl DataTypeAware for HostIpRouteOp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteOp)
    }
}

impl DataTypeAware for HostIpRouteTableConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteTableConfig)
    }
}

impl DataTypeAware for HostIpRouteTableInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteTableInfo)
    }
}

impl DataTypeAware for HostIpmiInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpmiInfo)
    }
}

impl DataTypeAware for IscsiDependencyEntity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IscsiDependencyEntity)
    }
}

impl DataTypeAware for IscsiMigrationDependency {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IscsiMigrationDependency)
    }
}

impl DataTypeAware for IscsiPortInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IscsiPortInfo)
    }
}

impl DataTypeAware for IscsiStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IscsiStatus)
    }
}

impl DataTypeAware for KernelModuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KernelModuleInfo)
    }
}

impl DataTypeAware for KernelModuleSectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KernelModuleSectionInfo)
    }
}

impl DataTypeAware for LacpInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LacpInfo)
    }
}

impl DataTypeAware for LagInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LagInfo)
    }
}

impl DataTypeAware for LagUplinkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LagUplinkInfo)
    }
}

impl DataTypeAware for HostLicenseSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLicenseSpec)
    }
}

impl DataTypeAware for LinkDiscoveryProtocolConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LinkDiscoveryProtocolConfig)
    }
}

impl DataTypeAware for HostAccountSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAccountSpec)
    }
}

impl DataTypeAware for HostPosixAccountSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPosixAccountSpec)
    }
}

impl DataTypeAware for HostLocalFileSystemVolumeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLocalFileSystemVolumeSpec)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerDiskLayoutSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerDiskLayoutSpec)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerFileDeleteResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerFileDeleteResult)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerFileDeleteSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerFileDeleteSpec)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerFileReserveResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerFileReserveResult)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerFileReserveSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerFileReserveSpec)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerSnapshotLayoutSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerSnapshotLayoutSpec)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerVmMigrationStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerVmMigrationStatus)
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerVmRecoveryInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostLowLevelProvisioningManagerVmRecoveryInfo)
    }
}

impl DataTypeAware for HostMaintenanceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMaintenanceSpec)
    }
}

impl DataTypeAware for ServiceConsoleReservationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceConsoleReservationInfo)
    }
}

impl DataTypeAware for VirtualMachineMemoryReservationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMemoryReservationInfo)
    }
}

impl DataTypeAware for VirtualMachineMemoryReservationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMemoryReservationSpec)
    }
}

impl DataTypeAware for HostMemorySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemorySpec)
    }
}

impl DataTypeAware for HostMemoryTierInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemoryTierInfo)
    }
}

impl DataTypeAware for HostMountInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMountInfo)
    }
}

impl DataTypeAware for HostMultipathInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfo)
    }
}

impl DataTypeAware for HostMultipathInfoLogicalUnit {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoLogicalUnit)
    }
}

impl DataTypeAware for HostMultipathInfoLogicalUnitPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoLogicalUnitPolicy)
    }
}

impl DataTypeAware for HostMultipathInfoFixedLogicalUnitPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoFixedLogicalUnitPolicy)
    }
}

impl DataTypeAware for HostMultipathInfoHppLogicalUnitPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoHppLogicalUnitPolicy)
    }
}

impl DataTypeAware for HostMultipathInfoLogicalUnitStorageArrayTypePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoLogicalUnitStorageArrayTypePolicy)
    }
}

impl DataTypeAware for HostMultipathInfoPath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoPath)
    }
}

impl DataTypeAware for HostMultipathStateInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathStateInfo)
    }
}

impl DataTypeAware for HostMultipathStateInfoPath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathStateInfoPath)
    }
}

impl DataTypeAware for HostNasVolumeConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNasVolumeConfig)
    }
}

impl DataTypeAware for HostNasVolumeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNasVolumeSpec)
    }
}

impl DataTypeAware for HostNasVolumeUserInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNasVolumeUserInfo)
    }
}

impl DataTypeAware for HostNatService {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNatService)
    }
}

impl DataTypeAware for HostNatServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNatServiceConfig)
    }
}

impl DataTypeAware for HostNatServiceNameServiceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNatServiceNameServiceSpec)
    }
}

impl DataTypeAware for HostNatServicePortForwardSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNatServicePortForwardSpec)
    }
}

impl DataTypeAware for HostNatServiceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNatServiceSpec)
    }
}

impl DataTypeAware for HostNetCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetCapabilities)
    }
}

impl DataTypeAware for HostNetOffloadCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetOffloadCapabilities)
    }
}

impl DataTypeAware for HostNetStackInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetStackInstance)
    }
}

impl DataTypeAware for HostNetworkConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkConfig)
    }
}

impl DataTypeAware for HostNetworkConfigNetStackSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkConfigNetStackSpec)
    }
}

impl DataTypeAware for HostNetworkConfigResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkConfigResult)
    }
}

impl DataTypeAware for HostNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkInfo)
    }
}

impl DataTypeAware for HostNetworkPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkPolicy)
    }
}

impl DataTypeAware for HostNicFailureCriteria {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNicFailureCriteria)
    }
}

impl DataTypeAware for HostNicOrderPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNicOrderPolicy)
    }
}

impl DataTypeAware for HostNicTeamingPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNicTeamingPolicy)
    }
}

impl DataTypeAware for HostNetworkSecurityPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkSecurityPolicy)
    }
}

impl DataTypeAware for HostNetworkTrafficShapingPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkTrafficShapingPolicy)
    }
}

impl DataTypeAware for HostNtpConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNtpConfig)
    }
}

impl DataTypeAware for HostNumaInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNumaInfo)
    }
}

impl DataTypeAware for HostNumaNode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNumaNode)
    }
}

impl DataTypeAware for HostNumericSensorInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNumericSensorInfo)
    }
}

impl DataTypeAware for NvdimmDimmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmDimmInfo)
    }
}

impl DataTypeAware for NvdimmGuid {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmGuid)
    }
}

impl DataTypeAware for NvdimmHealthInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmHealthInfo)
    }
}

impl DataTypeAware for NvdimmInterleaveSetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmInterleaveSetInfo)
    }
}

impl DataTypeAware for NvdimmNamespaceCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmNamespaceCreateSpec)
    }
}

impl DataTypeAware for NvdimmNamespaceDeleteSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmNamespaceDeleteSpec)
    }
}

impl DataTypeAware for NvdimmNamespaceDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmNamespaceDetails)
    }
}

impl DataTypeAware for NvdimmNamespaceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmNamespaceInfo)
    }
}

impl DataTypeAware for NvdimmSystemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmSystemInfo)
    }
}

impl DataTypeAware for NvdimmPMemNamespaceCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmPMemNamespaceCreateSpec)
    }
}

impl DataTypeAware for NvdimmRegionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmRegionInfo)
    }
}

impl DataTypeAware for NvdimmSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NvdimmSummary)
    }
}

impl DataTypeAware for HostNvmeController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeController)
    }
}

impl DataTypeAware for HostNvmeDisconnectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeDisconnectSpec)
    }
}

impl DataTypeAware for HostNvmeDiscoveryLog {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeDiscoveryLog)
    }
}

impl DataTypeAware for HostNvmeDiscoveryLogEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeDiscoveryLogEntry)
    }
}

impl DataTypeAware for HostNvmeNamespace {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeNamespace)
    }
}

impl DataTypeAware for HostNvmeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeSpec)
    }
}

impl DataTypeAware for HostNvmeConnectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeConnectSpec)
    }
}

impl DataTypeAware for HostNvmeDiscoverSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeDiscoverSpec)
    }
}

impl DataTypeAware for HostNvmeTopology {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeTopology)
    }
}

impl DataTypeAware for HostNvmeTopologyInterface {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeTopologyInterface)
    }
}

impl DataTypeAware for HostNvmeTransportParameters {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeTransportParameters)
    }
}

impl DataTypeAware for HostNvmeOpaqueTransportParameters {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeOpaqueTransportParameters)
    }
}

impl DataTypeAware for HostNvmeOverFibreChannelParameters {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeOverFibreChannelParameters)
    }
}

impl DataTypeAware for HostNvmeOverRdmaParameters {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeOverRdmaParameters)
    }
}

impl DataTypeAware for HostNvmeOverTcpParameters {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeOverTcpParameters)
    }
}

impl DataTypeAware for HostOpaqueNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostOpaqueNetworkInfo)
    }
}

impl DataTypeAware for HostOpaqueSwitch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostOpaqueSwitch)
    }
}

impl DataTypeAware for HostOpaqueSwitchPhysicalNicZone {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostOpaqueSwitchPhysicalNicZone)
    }
}

impl DataTypeAware for HostPartialMaintenanceModeRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPartialMaintenanceModeRuntimeInfo)
    }
}

impl DataTypeAware for HostPatchManagerLocator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPatchManagerLocator)
    }
}

impl DataTypeAware for HostPatchManagerPatchManagerOperationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPatchManagerPatchManagerOperationSpec)
    }
}

impl DataTypeAware for HostPatchManagerResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPatchManagerResult)
    }
}

impl DataTypeAware for HostPatchManagerStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPatchManagerStatus)
    }
}

impl DataTypeAware for HostPatchManagerStatusPrerequisitePatch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPatchManagerStatusPrerequisitePatch)
    }
}

impl DataTypeAware for HostPathSelectionPolicyOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPathSelectionPolicyOption)
    }
}

impl DataTypeAware for HostPciDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPciDevice)
    }
}

impl DataTypeAware for HostPciPassthruConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPciPassthruConfig)
    }
}

impl DataTypeAware for HostSriovConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSriovConfig)
    }
}

impl DataTypeAware for HostPciPassthruInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPciPassthruInfo)
    }
}

impl DataTypeAware for HostSriovInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSriovInfo)
    }
}

impl DataTypeAware for HostPersistentMemoryInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPersistentMemoryInfo)
    }
}

impl DataTypeAware for PhysicalNic {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNic)
    }
}

impl DataTypeAware for PhysicalNicCdpDeviceCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicCdpDeviceCapability)
    }
}

impl DataTypeAware for PhysicalNicCdpInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicCdpInfo)
    }
}

impl DataTypeAware for PhysicalNicConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicConfig)
    }
}

impl DataTypeAware for PhysicalNicLinkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicLinkInfo)
    }
}

impl DataTypeAware for LinkLayerDiscoveryProtocolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LinkLayerDiscoveryProtocolInfo)
    }
}

impl DataTypeAware for PhysicalNicHintInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicHintInfo)
    }
}

impl DataTypeAware for PhysicalNicHint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicHint)
    }
}

impl DataTypeAware for PhysicalNicIpHint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicIpHint)
    }
}

impl DataTypeAware for PhysicalNicNameHint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicNameHint)
    }
}

impl DataTypeAware for PhysicalNicSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicSpec)
    }
}

impl DataTypeAware for HostPlugStoreTopology {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopology)
    }
}

impl DataTypeAware for HostPlugStoreTopologyAdapter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopologyAdapter)
    }
}

impl DataTypeAware for HostPlugStoreTopologyDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopologyDevice)
    }
}

impl DataTypeAware for HostPlugStoreTopologyPath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopologyPath)
    }
}

impl DataTypeAware for HostPlugStoreTopologyPlugin {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopologyPlugin)
    }
}

impl DataTypeAware for HostPlugStoreTopologyTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlugStoreTopologyTarget)
    }
}

impl DataTypeAware for PnicTsoInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PnicTsoInfo)
    }
}

impl DataTypeAware for HostPortGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPortGroup)
    }
}

impl DataTypeAware for HostPortGroupConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPortGroupConfig)
    }
}

impl DataTypeAware for HostPortGroupPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPortGroupPort)
    }
}

impl DataTypeAware for HostPortGroupSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPortGroupSpec)
    }
}

impl DataTypeAware for PowerSystemCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PowerSystemCapability)
    }
}

impl DataTypeAware for PowerSystemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PowerSystemInfo)
    }
}

impl DataTypeAware for HostPowerPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPowerPolicy)
    }
}

impl DataTypeAware for HostProtocolEndpoint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProtocolEndpoint)
    }
}

impl DataTypeAware for HostPtpConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPtpConfig)
    }
}

impl DataTypeAware for HostPtpConfigPtpPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPtpConfigPtpPort)
    }
}

impl DataTypeAware for HostQualifiedName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostQualifiedName)
    }
}

impl DataTypeAware for HostRdmaDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDevice)
    }
}

impl DataTypeAware for HostRdmaDeviceBacking {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDeviceBacking)
    }
}

impl DataTypeAware for HostRdmaDevicePnicBacking {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDevicePnicBacking)
    }
}

impl DataTypeAware for HostRdmaDeviceCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDeviceCapability)
    }
}

impl DataTypeAware for HostRdmaDeviceConnectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDeviceConnectionInfo)
    }
}

impl DataTypeAware for HostReliableMemoryInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostReliableMemoryInfo)
    }
}

impl DataTypeAware for HostResignatureRescanResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostResignatureRescanResult)
    }
}

impl DataTypeAware for HostFirewallRuleset {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallRuleset)
    }
}

impl DataTypeAware for HostFirewallRulesetIpList {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallRulesetIpList)
    }
}

impl DataTypeAware for HostFirewallRulesetIpNetwork {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallRulesetIpNetwork)
    }
}

impl DataTypeAware for HostFirewallRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallRule)
    }
}

impl DataTypeAware for HostFirewallRulesetRulesetSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFirewallRulesetRulesetSpec)
    }
}

impl DataTypeAware for HostRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRuntimeInfo)
    }
}

impl DataTypeAware for HostRuntimeInfoNetStackInstanceRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRuntimeInfoNetStackInstanceRuntimeInfo)
    }
}

impl DataTypeAware for HostNetworkResourceRuntime {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNetworkResourceRuntime)
    }
}

impl DataTypeAware for HostRuntimeInfoNetworkRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRuntimeInfoNetworkRuntimeInfo)
    }
}

impl DataTypeAware for HostPlacedVirtualNicIdentifier {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPlacedVirtualNicIdentifier)
    }
}

impl DataTypeAware for HostPnicNetworkResourceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPnicNetworkResourceInfo)
    }
}

impl DataTypeAware for HostRuntimeInfoStateEncryptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRuntimeInfoStateEncryptionInfo)
    }
}

impl DataTypeAware for HostScsiDiskPartition {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiDiskPartition)
    }
}

impl DataTypeAware for ScsiLunCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScsiLunCapabilities)
    }
}

impl DataTypeAware for ScsiLunDescriptor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScsiLunDescriptor)
    }
}

impl DataTypeAware for ScsiLunDurableName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScsiLunDurableName)
    }
}

impl DataTypeAware for HostScsiTopology {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiTopology)
    }
}

impl DataTypeAware for HostScsiTopologyInterface {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiTopologyInterface)
    }
}

impl DataTypeAware for HostScsiTopologyLun {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiTopologyLun)
    }
}

impl DataTypeAware for HostScsiTopologyTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostScsiTopologyTarget)
    }
}

impl DataTypeAware for HostSecuritySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSecuritySpec)
    }
}

impl DataTypeAware for HostService {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostService)
    }
}

impl DataTypeAware for HostServiceSourcePackage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostServiceSourcePackage)
    }
}

impl DataTypeAware for HostServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostServiceConfig)
    }
}

impl DataTypeAware for HostServiceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostServiceInfo)
    }
}

impl DataTypeAware for HostSevInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSevInfo)
    }
}

impl DataTypeAware for HostSgxInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSgxInfo)
    }
}

impl DataTypeAware for HostSgxRegistrationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSgxRegistrationInfo)
    }
}

impl DataTypeAware for HostSharedGpuCapabilities {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSharedGpuCapabilities)
    }
}

impl DataTypeAware for HostSnmpSystemAgentLimits {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSnmpSystemAgentLimits)
    }
}

impl DataTypeAware for HostSnmpConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSnmpConfigSpec)
    }
}

impl DataTypeAware for HostSnmpDestination {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSnmpDestination)
    }
}

impl DataTypeAware for SoftwarePackage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SoftwarePackage)
    }
}

impl DataTypeAware for SoftwarePackageCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SoftwarePackageCapability)
    }
}

impl DataTypeAware for Relation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Relation)
    }
}

impl DataTypeAware for HostSriovDevicePoolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSriovDevicePoolInfo)
    }
}

impl DataTypeAware for HostSriovNetworkDevicePoolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSriovNetworkDevicePoolInfo)
    }
}

impl DataTypeAware for HostSslThumbprintInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSslThumbprintInfo)
    }
}

impl DataTypeAware for HostStorageArrayTypePolicyOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageArrayTypePolicyOption)
    }
}

impl DataTypeAware for HostStorageDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageDeviceInfo)
    }
}

impl DataTypeAware for HostStorageSystemDiskLocatorLedResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageSystemDiskLocatorLedResult)
    }
}

impl DataTypeAware for HostStorageSystemScsiLunResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageSystemScsiLunResult)
    }
}

impl DataTypeAware for HostStorageSystemVmfsVolumeResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostStorageSystemVmfsVolumeResult)
    }
}

impl DataTypeAware for HostListSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostListSummary)
    }
}

impl DataTypeAware for HostConfigSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConfigSummary)
    }
}

impl DataTypeAware for HostListSummaryGatewaySummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostListSummaryGatewaySummary)
    }
}

impl DataTypeAware for HostHardwareSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHardwareSummary)
    }
}

impl DataTypeAware for HostListSummaryQuickStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostListSummaryQuickStats)
    }
}

impl DataTypeAware for SystemEventInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SystemEventInfo)
    }
}

impl DataTypeAware for HostSystemHealthInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemHealthInfo)
    }
}

impl DataTypeAware for HostSystemIdentificationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemIdentificationInfo)
    }
}

impl DataTypeAware for HostSystemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemInfo)
    }
}

impl DataTypeAware for HostSystemResourceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemResourceInfo)
    }
}

impl DataTypeAware for HostSystemSwapConfiguration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfiguration)
    }
}

impl DataTypeAware for HostSystemSwapConfigurationSystemSwapOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationSystemSwapOption)
    }
}

impl DataTypeAware for HostSystemSwapConfigurationDatastoreOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationDatastoreOption)
    }
}

impl DataTypeAware for HostSystemSwapConfigurationDisabledOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationDisabledOption)
    }
}

impl DataTypeAware for HostSystemSwapConfigurationHostCacheOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationHostCacheOption)
    }
}

impl DataTypeAware for HostSystemSwapConfigurationHostLocalSwapOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationHostLocalSwapOption)
    }
}

impl DataTypeAware for HostTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTargetTransport)
    }
}

impl DataTypeAware for HostBlockAdapterTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostBlockAdapterTargetTransport)
    }
}

impl DataTypeAware for HostFibreChannelTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelTargetTransport)
    }
}

impl DataTypeAware for HostFibreChannelOverEthernetTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelOverEthernetTargetTransport)
    }
}

impl DataTypeAware for HostInternetScsiTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiTargetTransport)
    }
}

impl DataTypeAware for HostParallelScsiTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostParallelScsiTargetTransport)
    }
}

impl DataTypeAware for HostPcieTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPcieTargetTransport)
    }
}

impl DataTypeAware for HostRdmaTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaTargetTransport)
    }
}

impl DataTypeAware for HostSerialAttachedTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSerialAttachedTargetTransport)
    }
}

impl DataTypeAware for HostTcpTargetTransport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTcpTargetTransport)
    }
}

impl DataTypeAware for HostTdxInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTdxInfo)
    }
}

impl DataTypeAware for HostTpmAttestationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmAttestationInfo)
    }
}

impl DataTypeAware for HostTpmAttestationReport {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmAttestationReport)
    }
}

impl DataTypeAware for HostTpmEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmEventDetails)
    }
}

impl DataTypeAware for HostTpmBootCompleteEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmBootCompleteEventDetails)
    }
}

impl DataTypeAware for HostTpmBootSecurityOptionEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmBootSecurityOptionEventDetails)
    }
}

impl DataTypeAware for HostTpmNvTagEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmNvTagEventDetails)
    }
}

impl DataTypeAware for HostTpmSignerEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmSignerEventDetails)
    }
}

impl DataTypeAware for HostTpmCommandEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmCommandEventDetails)
    }
}

impl DataTypeAware for HostTpmOptionEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmOptionEventDetails)
    }
}

impl DataTypeAware for HostTpmSoftwareComponentEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmSoftwareComponentEventDetails)
    }
}

impl DataTypeAware for HostTpmVersionEventDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmVersionEventDetails)
    }
}

impl DataTypeAware for HostTpmEventLogEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmEventLogEntry)
    }
}

impl DataTypeAware for HostTrustAuthorityAttestationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTrustAuthorityAttestationInfo)
    }
}

impl DataTypeAware for HostUnresolvedVmfsExtent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsExtent)
    }
}

impl DataTypeAware for HostUnresolvedVmfsResignatureSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsResignatureSpec)
    }
}

impl DataTypeAware for HostUnresolvedVmfsResolutionResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsResolutionResult)
    }
}

impl DataTypeAware for HostUnresolvedVmfsResolutionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsResolutionSpec)
    }
}

impl DataTypeAware for HostUnresolvedVmfsVolume {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsVolume)
    }
}

impl DataTypeAware for HostUnresolvedVmfsVolumeResolveStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostUnresolvedVmfsVolumeResolveStatus)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashCacheConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashCacheConfigInfo)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashCacheConfigInfoVFlashModuleConfigOption)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashCacheConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashCacheConfigSpec)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashConfigInfo)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashResourceConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashResourceConfigInfo)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashResourceConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashResourceConfigSpec)
    }
}

impl DataTypeAware for HostVFlashManagerVFlashResourceRunTimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashManagerVFlashResourceRunTimeInfo)
    }
}

impl DataTypeAware for HostVFlashResourceConfigurationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVFlashResourceConfigurationResult)
    }
}

impl DataTypeAware for HostVMotionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionConfig)
    }
}

impl DataTypeAware for HostVMotionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionInfo)
    }
}

impl DataTypeAware for HostVMotionManagerDstInstantCloneResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionManagerDstInstantCloneResult)
    }
}

impl DataTypeAware for HostVMotionManagerSrcInstantCloneResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionManagerSrcInstantCloneResult)
    }
}

impl DataTypeAware for HostVMotionNetConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVMotionNetConfig)
    }
}

impl DataTypeAware for VimHostVsanStretchedClusterHostCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimHostVsanStretchedClusterHostCapability)
    }
}

impl DataTypeAware for HostVffsSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVffsSpec)
    }
}

impl DataTypeAware for HostVirtualNic {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNic)
    }
}

impl DataTypeAware for HostVirtualNicConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicConfig)
    }
}

impl DataTypeAware for HostVirtualNicIpRouteSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicIpRouteSpec)
    }
}

impl DataTypeAware for HostVirtualNicOpaqueNetworkSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicOpaqueNetworkSpec)
    }
}

impl DataTypeAware for HostVirtualNicSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicSpec)
    }
}

impl DataTypeAware for HostVirtualNicConnection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicConnection)
    }
}

impl DataTypeAware for VirtualNicManagerNetConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNicManagerNetConfig)
    }
}

impl DataTypeAware for HostVirtualNicManagerNicTypeSelection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicManagerNicTypeSelection)
    }
}

impl DataTypeAware for HostVirtualNicManagerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualNicManagerInfo)
    }
}

impl DataTypeAware for HostVirtualSwitch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitch)
    }
}

impl DataTypeAware for HostVirtualSwitchBeaconConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchBeaconConfig)
    }
}

impl DataTypeAware for HostVirtualSwitchBridge {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchBridge)
    }
}

impl DataTypeAware for HostVirtualSwitchAutoBridge {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchAutoBridge)
    }
}

impl DataTypeAware for HostVirtualSwitchBondBridge {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchBondBridge)
    }
}

impl DataTypeAware for HostVirtualSwitchSimpleBridge {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchSimpleBridge)
    }
}

impl DataTypeAware for HostVirtualSwitchConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchConfig)
    }
}

impl DataTypeAware for HostVirtualSwitchSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchSpec)
    }
}

impl DataTypeAware for HostVmciAccessManagerAccessSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVmciAccessManagerAccessSpec)
    }
}

impl DataTypeAware for VmfsDatastoreOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreOption)
    }
}

impl DataTypeAware for VmfsDatastoreBaseOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreBaseOption)
    }
}

impl DataTypeAware for VmfsDatastoreMultipleExtentOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreMultipleExtentOption)
    }
}

impl DataTypeAware for VmfsDatastoreSingleExtentOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreSingleExtentOption)
    }
}

impl DataTypeAware for VmfsDatastoreAllExtentOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreAllExtentOption)
    }
}

impl DataTypeAware for VmfsDatastoreSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreSpec)
    }
}

impl DataTypeAware for VmfsDatastoreCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreCreateSpec)
    }
}

impl DataTypeAware for VmfsDatastoreExpandSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreExpandSpec)
    }
}

impl DataTypeAware for VmfsDatastoreExtendSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreExtendSpec)
    }
}

impl DataTypeAware for HostVmfsRescanResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVmfsRescanResult)
    }
}

impl DataTypeAware for VmfsConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsConfigOption)
    }
}

impl DataTypeAware for HostVmfsSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVmfsSpec)
    }
}

impl DataTypeAware for VmfsUnmapBandwidthSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsUnmapBandwidthSpec)
    }
}

impl DataTypeAware for VsanBasicDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanBasicDeviceInfo)
    }
}

impl DataTypeAware for VsanClusterMembershipInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterMembershipInfo)
    }
}

impl DataTypeAware for VsanDaemonHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDaemonHealth)
    }
}

impl DataTypeAware for VsanDiskEncryptionHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskEncryptionHealth)
    }
}

impl DataTypeAware for VsanDiskRebalanceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskRebalanceResult)
    }
}

impl DataTypeAware for VsanDitEncryptionHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDitEncryptionHealthSummary)
    }
}

impl DataTypeAware for VsanEncryptionHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEncryptionHealthSummary)
    }
}

impl DataTypeAware for VsanFailedRepairObjectResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFailedRepairObjectResult)
    }
}

impl DataTypeAware for VsanFileServerHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServerHealthSummary)
    }
}

impl DataTypeAware for VsanFileServiceBalanceHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceBalanceHealth)
    }
}

impl DataTypeAware for VsanFileServiceHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceHealthSummary)
    }
}

impl DataTypeAware for VsanFileServiceRootFsHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceRootFsHealth)
    }
}

impl DataTypeAware for VsanFileServiceShareHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceShareHealthSummary)
    }
}

impl DataTypeAware for VsanHclCommonDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclCommonDeviceInfo)
    }
}

impl DataTypeAware for VsanHclNicInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclNicInfo)
    }
}

impl DataTypeAware for VsanHclComputeResource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclComputeResource)
    }
}

impl DataTypeAware for VsanHclControllerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclControllerInfo)
    }
}

impl DataTypeAware for VsanHclDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclDiskInfo)
    }
}

impl DataTypeAware for VsanHclFirmwareFile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclFirmwareFile)
    }
}

impl DataTypeAware for VsanHclFirmwareUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclFirmwareUpdateSpec)
    }
}

impl DataTypeAware for VsanHclMemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclMemInfo)
    }
}

impl DataTypeAware for VsanHealthQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthQuerySpec)
    }
}

impl DataTypeAware for VsanHostCimProviderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostCimProviderInfo)
    }
}

impl DataTypeAware for VsanHostEmmSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostEmmSummary)
    }
}

impl DataTypeAware for VsanHostFwComponent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostFwComponent)
    }
}

impl DataTypeAware for VsanHostGlobalDedupConfigHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostGlobalDedupConfigHealthSummary)
    }
}

impl DataTypeAware for VsanHostHclInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostHclInfo)
    }
}

impl DataTypeAware for VsanHostHealthSystemStatusResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostHealthSystemStatusResult)
    }
}

impl DataTypeAware for VsanHostHwDeviceId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostHwDeviceId)
    }
}

impl DataTypeAware for VsanHostIoInsightInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostIoInsightInfo)
    }
}

impl DataTypeAware for VsanHostQueryCheckLimitsSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostQueryCheckLimitsSpec)
    }
}

impl DataTypeAware for VsanHostReference {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostReference)
    }
}

impl DataTypeAware for VsanHostVirtualApplianceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostVirtualApplianceInfo)
    }
}

impl DataTypeAware for VsanHostVmdkLoadTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostVmdkLoadTestResult)
    }
}

impl DataTypeAware for VsanHwToVcgInfoMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHwToVcgInfoMapping)
    }
}

impl DataTypeAware for HostVsanInternalSystemCmmdsQuery {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVsanInternalSystemCmmdsQuery)
    }
}

impl DataTypeAware for HostVsanInternalSystemDeleteVsanObjectsResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVsanInternalSystemDeleteVsanObjectsResult)
    }
}

impl DataTypeAware for VsanNewPolicyBatch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNewPolicyBatch)
    }
}

impl DataTypeAware for VsanPolicyChangeBatch {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPolicyChangeBatch)
    }
}

impl DataTypeAware for VsanPolicyCost {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPolicyCost)
    }
}

impl DataTypeAware for VsanPolicySatisfiability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPolicySatisfiability)
    }
}

impl DataTypeAware for HostVsanInternalSystemVsanObjectOperationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVsanInternalSystemVsanObjectOperationResult)
    }
}

impl DataTypeAware for HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVsanInternalSystemVsanPhysicalDiskDiagnosticsResult)
    }
}

impl DataTypeAware for VsanIoInsightInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoInsightInfo)
    }
}

impl DataTypeAware for VsanIperfClientSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIperfClientSpec)
    }
}

impl DataTypeAware for VsanKmsHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanKmsHealth)
    }
}

impl DataTypeAware for VsanLimitHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanLimitHealthResult)
    }
}

impl DataTypeAware for VsanNetworkDiagnosticsHealthInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkDiagnosticsHealthInfo)
    }
}

impl DataTypeAware for VsanNetworkHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkHealthResult)
    }
}

impl DataTypeAware for VsanNetworkLoadTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkLoadTestResult)
    }
}

impl DataTypeAware for VsanNetworkPeerHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkPeerHealthResult)
    }
}

impl DataTypeAware for VsanNicRdmaInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNicRdmaInfo)
    }
}

impl DataTypeAware for VsanObjectHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectHealth)
    }
}

impl DataTypeAware for VsanObjectOverallHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectOverallHealth)
    }
}

impl DataTypeAware for VsanPhysicalDiskHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPhysicalDiskHealth)
    }
}

impl DataTypeAware for VsanPhysicalDiskHealthSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPhysicalDiskHealthSummary)
    }
}

impl DataTypeAware for VsanProactiveRebalanceInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanProactiveRebalanceInfoEx)
    }
}

impl DataTypeAware for VsanQueryResultHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanQueryResultHostInfo)
    }
}

impl DataTypeAware for VsanRepairObjectsResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRepairObjectsResult)
    }
}

impl DataTypeAware for VsanResourceHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceHealth)
    }
}

impl DataTypeAware for VsanServerClusterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanServerClusterInfo)
    }
}

impl DataTypeAware for VsanSmartDiskStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSmartDiskStats)
    }
}

impl DataTypeAware for VsanSmartParameter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSmartParameter)
    }
}

impl DataTypeAware for VsanSmartStatsHostSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSmartStatsHostSummary)
    }
}

impl DataTypeAware for VsanVcgDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcgDeviceInfo)
    }
}

impl DataTypeAware for VsanVmdkIoLoadSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVmdkIoLoadSpec)
    }
}

impl DataTypeAware for VsanVmdkLoadTestResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVmdkLoadTestResult)
    }
}

impl DataTypeAware for VsanVmdkLoadTestSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVmdkLoadTestSpec)
    }
}

impl DataTypeAware for VsanVsanPcapResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVsanPcapResult)
    }
}

impl DataTypeAware for HostVvolNqn {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVvolNqn)
    }
}

impl DataTypeAware for VVolHostPe {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VVolHostPe)
    }
}

impl DataTypeAware for HostVvolVolumeHostVvolNqn {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVvolVolumeHostVvolNqn)
    }
}

impl DataTypeAware for HostVvolVolumeSpecification {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVvolVolumeSpecification)
    }
}

impl DataTypeAware for NetDhcpConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDhcpConfigInfo)
    }
}

impl DataTypeAware for NetDhcpConfigInfoDhcpOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDhcpConfigInfoDhcpOptions)
    }
}

impl DataTypeAware for NetDhcpConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDhcpConfigSpec)
    }
}

impl DataTypeAware for NetDhcpConfigSpecDhcpOptionsSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDhcpConfigSpecDhcpOptionsSpec)
    }
}

impl DataTypeAware for NetDnsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDnsConfigInfo)
    }
}

impl DataTypeAware for NetDnsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetDnsConfigSpec)
    }
}

impl DataTypeAware for NetIpConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpConfigInfo)
    }
}

impl DataTypeAware for NetIpConfigInfoIpAddress {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpConfigInfoIpAddress)
    }
}

impl DataTypeAware for NetIpConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpConfigSpec)
    }
}

impl DataTypeAware for NetIpConfigSpecIpAddressSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpConfigSpecIpAddressSpec)
    }
}

impl DataTypeAware for NetIpRouteConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigInfo)
    }
}

impl DataTypeAware for NetIpRouteConfigInfoGateway {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigInfoGateway)
    }
}

impl DataTypeAware for NetIpRouteConfigInfoIpRoute {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigInfoIpRoute)
    }
}

impl DataTypeAware for NetIpRouteConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigSpec)
    }
}

impl DataTypeAware for NetIpRouteConfigSpecGatewaySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigSpecGatewaySpec)
    }
}

impl DataTypeAware for NetIpRouteConfigSpecIpRouteSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpRouteConfigSpecIpRouteSpec)
    }
}

impl DataTypeAware for NetIpStackInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpStackInfo)
    }
}

impl DataTypeAware for NetIpStackInfoDefaultRouter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpStackInfoDefaultRouter)
    }
}

impl DataTypeAware for NetIpStackInfoNetToMedia {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetIpStackInfoNetToMedia)
    }
}

impl DataTypeAware for NetBiosConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetBiosConfigInfo)
    }
}

impl DataTypeAware for WinNetBiosConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::WinNetBiosConfigInfo)
    }
}

impl DataTypeAware for ArrayUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ArrayUpdateSpec)
    }
}

impl DataTypeAware for ClusterDasVmConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasVmConfigSpec)
    }
}

impl DataTypeAware for ClusterDatastoreUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDatastoreUpdateSpec)
    }
}

impl DataTypeAware for ClusterDpmHostConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDpmHostConfigSpec)
    }
}

impl DataTypeAware for ClusterDrsVmConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsVmConfigSpec)
    }
}

impl DataTypeAware for ClusterGroupSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterGroupSpec)
    }
}

impl DataTypeAware for ClusterPreemptibleVmPairSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterPreemptibleVmPairSpec)
    }
}

impl DataTypeAware for ClusterRuleSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterRuleSpec)
    }
}

impl DataTypeAware for ClusterTagCategoryUpdateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterTagCategoryUpdateSpec)
    }
}

impl DataTypeAware for ClusterVmOrchestrationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterVmOrchestrationSpec)
    }
}

impl DataTypeAware for StorageDrsOptionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsOptionSpec)
    }
}

impl DataTypeAware for StorageDrsVmConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsVmConfigSpec)
    }
}

impl DataTypeAware for VAppOvfSectionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppOvfSectionSpec)
    }
}

impl DataTypeAware for VAppProductSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppProductSpec)
    }
}

impl DataTypeAware for VAppPropertySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppPropertySpec)
    }
}

impl DataTypeAware for VirtualMachineCpuIdInfoSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineCpuIdInfoSpec)
    }
}

impl DataTypeAware for OptionType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionType)
    }
}

impl DataTypeAware for BoolOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BoolOption)
    }
}

impl DataTypeAware for ChoiceOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ChoiceOption)
    }
}

impl DataTypeAware for FloatOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FloatOption)
    }
}

impl DataTypeAware for IntOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntOption)
    }
}

impl DataTypeAware for LongOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LongOption)
    }
}

impl DataTypeAware for StringOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StringOption)
    }
}

impl DataTypeAware for OptionValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionValue)
    }
}

impl DataTypeAware for HostInternetScsiHbaParamValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostInternetScsiHbaParamValue)
    }
}

impl DataTypeAware for ApplyProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ApplyProfile)
    }
}

impl DataTypeAware for ProfileApplyProfileElement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileApplyProfileElement)
    }
}

impl DataTypeAware for ActiveDirectoryProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ActiveDirectoryProfile)
    }
}

impl DataTypeAware for AuthenticationProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AuthenticationProfile)
    }
}

impl DataTypeAware for DateTimeProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DateTimeProfile)
    }
}

impl DataTypeAware for DvsProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsProfile)
    }
}

impl DataTypeAware for DvsVNicProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVNicProfile)
    }
}

impl DataTypeAware for DvsHostVNicProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHostVNicProfile)
    }
}

impl DataTypeAware for DvsServiceConsoleVNicProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsServiceConsoleVNicProfile)
    }
}

impl DataTypeAware for FirewallProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FirewallProfile)
    }
}

impl DataTypeAware for FirewallProfileRulesetProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FirewallProfileRulesetProfile)
    }
}

impl DataTypeAware for HostApplyProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostApplyProfile)
    }
}

impl DataTypeAware for HostMemoryProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemoryProfile)
    }
}

impl DataTypeAware for IpAddressProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpAddressProfile)
    }
}

impl DataTypeAware for IpRouteProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpRouteProfile)
    }
}

impl DataTypeAware for NasStorageProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NasStorageProfile)
    }
}

impl DataTypeAware for NetStackInstanceProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetStackInstanceProfile)
    }
}

impl DataTypeAware for NetworkPolicyProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkPolicyProfile)
    }
}

impl DataTypeAware for NetworkProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkProfile)
    }
}

impl DataTypeAware for NetworkProfileDnsConfigProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkProfileDnsConfigProfile)
    }
}

impl DataTypeAware for NsxHostVNicProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NsxHostVNicProfile)
    }
}

impl DataTypeAware for OpaqueSwitchProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OpaqueSwitchProfile)
    }
}

impl DataTypeAware for OptionProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionProfile)
    }
}

impl DataTypeAware for PermissionProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PermissionProfile)
    }
}

impl DataTypeAware for PhysicalNicProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicProfile)
    }
}

impl DataTypeAware for PnicUplinkProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PnicUplinkProfile)
    }
}

impl DataTypeAware for PortGroupProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PortGroupProfile)
    }
}

impl DataTypeAware for HostPortGroupProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPortGroupProfile)
    }
}

impl DataTypeAware for ServiceConsolePortGroupProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceConsolePortGroupProfile)
    }
}

impl DataTypeAware for VmPortGroupProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmPortGroupProfile)
    }
}

impl DataTypeAware for VirtualSwitchSelectionProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSwitchSelectionProfile)
    }
}

impl DataTypeAware for VlanProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VlanProfile)
    }
}

impl DataTypeAware for SecurityProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SecurityProfile)
    }
}

impl DataTypeAware for ServiceProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceProfile)
    }
}

impl DataTypeAware for StaticRouteProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StaticRouteProfile)
    }
}

impl DataTypeAware for StorageProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageProfile)
    }
}

impl DataTypeAware for UserGroupProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserGroupProfile)
    }
}

impl DataTypeAware for UserProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserProfile)
    }
}

impl DataTypeAware for VirtualSwitchProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSwitchProfile)
    }
}

impl DataTypeAware for LinkProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LinkProfile)
    }
}

impl DataTypeAware for NumPortsProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NumPortsProfile)
    }
}

impl DataTypeAware for ProfileApplyProfileProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileApplyProfileProperty)
    }
}

impl DataTypeAware for ComplianceLocator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComplianceLocator)
    }
}

impl DataTypeAware for ComplianceProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComplianceProfile)
    }
}

impl DataTypeAware for ComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComplianceResult)
    }
}

impl DataTypeAware for ComplianceFailure {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComplianceFailure)
    }
}

impl DataTypeAware for ComplianceFailureComplianceFailureValues {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComplianceFailureComplianceFailureValues)
    }
}

impl DataTypeAware for ProfileDeferredPolicyOptionParameter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileDeferredPolicyOptionParameter)
    }
}

impl DataTypeAware for ProfileExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExpression)
    }
}

impl DataTypeAware for ProfileCompositeExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileCompositeExpression)
    }
}

impl DataTypeAware for ProfileSimpleExpression {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileSimpleExpression)
    }
}

impl DataTypeAware for ProfileExpressionMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExpressionMetadata)
    }
}

impl DataTypeAware for ProfileParameterMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileParameterMetadata)
    }
}

impl DataTypeAware for ProfileParameterMetadataParameterRelationMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileParameterMetadataParameterRelationMetadata)
    }
}

impl DataTypeAware for ProfilePolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfilePolicy)
    }
}

impl DataTypeAware for ProfilePolicyMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfilePolicyMetadata)
    }
}

impl DataTypeAware for PolicyOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PolicyOption)
    }
}

impl DataTypeAware for CompositePolicyOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CompositePolicyOption)
    }
}

impl DataTypeAware for ProfilePolicyOptionMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfilePolicyOptionMetadata)
    }
}

impl DataTypeAware for ProfileCompositePolicyOptionMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileCompositePolicyOptionMetadata)
    }
}

impl DataTypeAware for UserInputRequiredParameterMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserInputRequiredParameterMetadata)
    }
}

impl DataTypeAware for ProfileConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileConfigInfo)
    }
}

impl DataTypeAware for ClusterProfileConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileConfigInfo)
    }
}

impl DataTypeAware for HostProfileConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileConfigInfo)
    }
}

impl DataTypeAware for ProfileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileCreateSpec)
    }
}

impl DataTypeAware for ProfileSerializedCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileSerializedCreateSpec)
    }
}

impl DataTypeAware for HostProfileSerializedHostProfileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileSerializedHostProfileSpec)
    }
}

impl DataTypeAware for ClusterProfileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileCreateSpec)
    }
}

impl DataTypeAware for ClusterProfileConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileConfigSpec)
    }
}

impl DataTypeAware for ClusterProfileCompleteConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileCompleteConfigSpec)
    }
}

impl DataTypeAware for ClusterProfileConfigServiceCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileConfigServiceCreateSpec)
    }
}

impl DataTypeAware for HostProfileConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileConfigSpec)
    }
}

impl DataTypeAware for HostProfileCompleteConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileCompleteConfigSpec)
    }
}

impl DataTypeAware for HostProfileHostBasedConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileHostBasedConfigSpec)
    }
}

impl DataTypeAware for ProfileDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileDescription)
    }
}

impl DataTypeAware for ProfileDescriptionSection {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileDescriptionSection)
    }
}

impl DataTypeAware for ProfileMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileMetadata)
    }
}

impl DataTypeAware for ProfileMetadataProfileOperationMessage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileMetadataProfileOperationMessage)
    }
}

impl DataTypeAware for ProfileMetadataProfileSortSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileMetadataProfileSortSpec)
    }
}

impl DataTypeAware for ProfilePropertyPath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfilePropertyPath)
    }
}

impl DataTypeAware for ProfileProfileStructure {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileProfileStructure)
    }
}

impl DataTypeAware for ProfileProfileStructureProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileProfileStructureProperty)
    }
}

impl DataTypeAware for AnswerFile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFile)
    }
}

impl DataTypeAware for AnswerFileStatusResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileStatusResult)
    }
}

impl DataTypeAware for AnswerFileStatusError {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileStatusError)
    }
}

impl DataTypeAware for ProfileExecuteResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExecuteResult)
    }
}

impl DataTypeAware for ApplyHostProfileConfigurationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ApplyHostProfileConfigurationSpec)
    }
}

impl DataTypeAware for ProfileExecuteError {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExecuteError)
    }
}

impl DataTypeAware for HostProfileValidationFailureInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileValidationFailureInfo)
    }
}

impl DataTypeAware for HostSpecification {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSpecification)
    }
}

impl DataTypeAware for HostSubSpecification {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSubSpecification)
    }
}

impl DataTypeAware for AnswerFileCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileCreateSpec)
    }
}

impl DataTypeAware for AnswerFileOptionsCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileOptionsCreateSpec)
    }
}

impl DataTypeAware for AnswerFileSerializedCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileSerializedCreateSpec)
    }
}

impl DataTypeAware for ApplyHostProfileConfigurationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ApplyHostProfileConfigurationResult)
    }
}

impl DataTypeAware for HostProfileManagerCompositionResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerCompositionResult)
    }
}

impl DataTypeAware for HostProfileManagerCompositionResultResultElement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerCompositionResultResultElement)
    }
}

impl DataTypeAware for HostProfileManagerCompositionValidationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerCompositionValidationResult)
    }
}

impl DataTypeAware for HostProfileManagerCompositionValidationResultResultElement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerCompositionValidationResultResultElement)
    }
}

impl DataTypeAware for HostProfileManagerConfigTaskList {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerConfigTaskList)
    }
}

impl DataTypeAware for HostProfilesEntityCustomizations {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfilesEntityCustomizations)
    }
}

impl DataTypeAware for StructuredCustomizations {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StructuredCustomizations)
    }
}

impl DataTypeAware for HostProfileManagerHostToConfigSpecMap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileManagerHostToConfigSpecMap)
    }
}

impl DataTypeAware for ScheduledTaskDescription {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskDescription)
    }
}

impl DataTypeAware for ScheduledTaskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskSpec)
    }
}

impl DataTypeAware for ScheduledTaskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskInfo)
    }
}

impl DataTypeAware for TaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskScheduler)
    }
}

impl DataTypeAware for AfterStartupTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AfterStartupTaskScheduler)
    }
}

impl DataTypeAware for OnceTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OnceTaskScheduler)
    }
}

impl DataTypeAware for RecurrentTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RecurrentTaskScheduler)
    }
}

impl DataTypeAware for HourlyTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HourlyTaskScheduler)
    }
}

impl DataTypeAware for DailyTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DailyTaskScheduler)
    }
}

impl DataTypeAware for MonthlyTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MonthlyTaskScheduler)
    }
}

impl DataTypeAware for MonthlyByDayTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MonthlyByDayTaskScheduler)
    }
}

impl DataTypeAware for MonthlyByWeekdayTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MonthlyByWeekdayTaskScheduler)
    }
}

impl DataTypeAware for WeeklyTaskScheduler {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::WeeklyTaskScheduler)
    }
}

impl DataTypeAware for ApplyStorageRecommendationResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ApplyStorageRecommendationResult)
    }
}

impl DataTypeAware for StorageDrsAutomationConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsAutomationConfig)
    }
}

impl DataTypeAware for StorageDrsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsConfigInfo)
    }
}

impl DataTypeAware for StorageDrsConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsConfigSpec)
    }
}

impl DataTypeAware for StorageDrsIoLoadBalanceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsIoLoadBalanceConfig)
    }
}

impl DataTypeAware for PlacementAffinityRule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementAffinityRule)
    }
}

impl DataTypeAware for PlacementRankResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementRankResult)
    }
}

impl DataTypeAware for PlacementRankSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PlacementRankSpec)
    }
}

impl DataTypeAware for StorageDrsPlacementRankVmSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsPlacementRankVmSpec)
    }
}

impl DataTypeAware for StorageDrsPodConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsPodConfigInfo)
    }
}

impl DataTypeAware for StorageDrsPodConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsPodConfigSpec)
    }
}

impl DataTypeAware for StorageDrsPodSelectionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsPodSelectionSpec)
    }
}

impl DataTypeAware for PodDiskLocator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PodDiskLocator)
    }
}

impl DataTypeAware for VmPodConfigForPlacement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmPodConfigForPlacement)
    }
}

impl DataTypeAware for StorageDrsSpaceLoadBalanceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsSpaceLoadBalanceConfig)
    }
}

impl DataTypeAware for StoragePlacementResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePlacementResult)
    }
}

impl DataTypeAware for StoragePlacementSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePlacementSpec)
    }
}

impl DataTypeAware for StorageDrsVmConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StorageDrsVmConfigInfo)
    }
}

impl DataTypeAware for VAppCloneSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppCloneSpec)
    }
}

impl DataTypeAware for VAppCloneSpecNetworkMappingPair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppCloneSpecNetworkMappingPair)
    }
}

impl DataTypeAware for VAppCloneSpecResourceMap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppCloneSpecResourceMap)
    }
}

impl DataTypeAware for VAppEntityConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppEntityConfigInfo)
    }
}

impl DataTypeAware for VAppIpAssignmentInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppIpAssignmentInfo)
    }
}

impl DataTypeAware for IpPool {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpPool)
    }
}

impl DataTypeAware for IpPoolAssociation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpPoolAssociation)
    }
}

impl DataTypeAware for IpPoolIpPoolConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpPoolIpPoolConfigInfo)
    }
}

impl DataTypeAware for VAppOvfSectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppOvfSectionInfo)
    }
}

impl DataTypeAware for VAppProductInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppProductInfo)
    }
}

impl DataTypeAware for VAppPropertyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppPropertyInfo)
    }
}

impl DataTypeAware for VmConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigInfo)
    }
}

impl DataTypeAware for VAppConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppConfigInfo)
    }
}

impl DataTypeAware for VmConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigSpec)
    }
}

impl DataTypeAware for VAppConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VAppConfigSpec)
    }
}

impl DataTypeAware for ClusterNetworkConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterNetworkConfigSpec)
    }
}

impl DataTypeAware for FailoverNodeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FailoverNodeInfo)
    }
}

impl DataTypeAware for NodeDeploymentSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NodeDeploymentSpec)
    }
}

impl DataTypeAware for PassiveNodeDeploymentSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PassiveNodeDeploymentSpec)
    }
}

impl DataTypeAware for NodeNetworkSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NodeNetworkSpec)
    }
}

impl DataTypeAware for PassiveNodeNetworkSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PassiveNodeNetworkSpec)
    }
}

impl DataTypeAware for SourceNodeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SourceNodeSpec)
    }
}

impl DataTypeAware for VchaClusterConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterConfigInfo)
    }
}

impl DataTypeAware for VchaClusterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterConfigSpec)
    }
}

impl DataTypeAware for VchaClusterDeploymentSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterDeploymentSpec)
    }
}

impl DataTypeAware for VchaClusterNetworkSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterNetworkSpec)
    }
}

impl DataTypeAware for WitnessNodeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::WitnessNodeInfo)
    }
}

impl DataTypeAware for VchaClusterHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterHealth)
    }
}

impl DataTypeAware for VchaClusterRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaClusterRuntimeInfo)
    }
}

impl DataTypeAware for VchaNodeRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VchaNodeRuntimeInfo)
    }
}

impl DataTypeAware for VirtualMachineAffinityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineAffinityInfo)
    }
}

impl DataTypeAware for VirtualMachineBaseIndependentFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBaseIndependentFilterSpec)
    }
}

impl DataTypeAware for VirtualMachineEmptyIndependentFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineEmptyIndependentFilterSpec)
    }
}

impl DataTypeAware for VirtualMachineIndependentFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineIndependentFilterSpec)
    }
}

impl DataTypeAware for VirtualMachineBootOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptions)
    }
}

impl DataTypeAware for VirtualMachineBootOptionsBootableDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableDevice)
    }
}

impl DataTypeAware for VirtualMachineBootOptionsBootableCdromDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableCdromDevice)
    }
}

impl DataTypeAware for VirtualMachineBootOptionsBootableDiskDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableDiskDevice)
    }
}

impl DataTypeAware for VirtualMachineBootOptionsBootableEthernetDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableEthernetDevice)
    }
}

impl DataTypeAware for VirtualMachineBootOptionsBootableFloppyDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableFloppyDevice)
    }
}

impl DataTypeAware for VirtualMachineCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineCapability)
    }
}

impl DataTypeAware for VirtualMachineCertThumbprint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineCertThumbprint)
    }
}

impl DataTypeAware for VirtualMachineCloneSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineCloneSpec)
    }
}

impl DataTypeAware for VirtualMachineConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigInfo)
    }
}

impl DataTypeAware for VirtualMachineConfigInfoDatastoreUrlPair {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigInfoDatastoreUrlPair)
    }
}

impl DataTypeAware for VirtualMachineConfigInfoOverheadInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigInfoOverheadInfo)
    }
}

impl DataTypeAware for VirtualMachineConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigOption)
    }
}

impl DataTypeAware for VirtualMachineConfigOptionDescriptor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigOptionDescriptor)
    }
}

impl DataTypeAware for VirtualMachineConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigSpec)
    }
}

impl DataTypeAware for ConfigTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ConfigTarget)
    }
}

impl DataTypeAware for VirtualMachineConsolePreferences {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConsolePreferences)
    }
}

impl DataTypeAware for VirtualMachineContentLibraryItemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineContentLibraryItemInfo)
    }
}

impl DataTypeAware for DatastoreOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreOption)
    }
}

impl DataTypeAware for VirtualMachineDatastoreVolumeOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDatastoreVolumeOption)
    }
}

impl DataTypeAware for VirtualMachineDefaultPowerOpInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDefaultPowerOpInfo)
    }
}

impl DataTypeAware for VirtualMachineDeviceRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDeviceRuntimeInfo)
    }
}

impl DataTypeAware for VirtualMachineDeviceRuntimeInfoDeviceRuntimeState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState)
    }
}

impl DataTypeAware for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeState)
    }
}

impl DataTypeAware for VirtualMachineDvxClassInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDvxClassInfo)
    }
}

impl DataTypeAware for FaultToleranceConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceConfigInfo)
    }
}

impl DataTypeAware for FaultTolerancePrimaryConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultTolerancePrimaryConfigInfo)
    }
}

impl DataTypeAware for FaultToleranceSecondaryConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceSecondaryConfigInfo)
    }
}

impl DataTypeAware for FaultToleranceConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceConfigSpec)
    }
}

impl DataTypeAware for FaultToleranceMetaSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceMetaSpec)
    }
}

impl DataTypeAware for FaultToleranceSecondaryOpResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceSecondaryOpResult)
    }
}

impl DataTypeAware for FaultToleranceVmConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceVmConfigSpec)
    }
}

impl DataTypeAware for FaultToleranceDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceDiskSpec)
    }
}

impl DataTypeAware for VirtualMachineFeatureRequirement {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFeatureRequirement)
    }
}

impl DataTypeAware for VirtualMachineFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileInfo)
    }
}

impl DataTypeAware for VirtualMachineFileLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayout)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutDiskLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutDiskLayout)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutSnapshotLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutSnapshotLayout)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutEx)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutExDiskLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutExDiskLayout)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutExDiskUnit {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutExDiskUnit)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutExFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutExFileInfo)
    }
}

impl DataTypeAware for VirtualMachineFileLayoutExSnapshotLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFileLayoutExSnapshotLayout)
    }
}

impl DataTypeAware for VirtualMachineFlagInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFlagInfo)
    }
}

impl DataTypeAware for VirtualMachineForkConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineForkConfigInfo)
    }
}

impl DataTypeAware for GuestInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestInfo)
    }
}

impl DataTypeAware for GuestInfoCustomizationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestInfoCustomizationInfo)
    }
}

impl DataTypeAware for GuestDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestDiskInfo)
    }
}

impl DataTypeAware for GuestInfoNamespaceGenerationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestInfoNamespaceGenerationInfo)
    }
}

impl DataTypeAware for GuestNicInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestNicInfo)
    }
}

impl DataTypeAware for GuestScreenInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestScreenInfo)
    }
}

impl DataTypeAware for GuestStackInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestStackInfo)
    }
}

impl DataTypeAware for GuestInfoVirtualDiskMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestInfoVirtualDiskMapping)
    }
}

impl DataTypeAware for VirtualMachineGuestIntegrityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineGuestIntegrityInfo)
    }
}

impl DataTypeAware for VirtualMachineGuestMonitoringModeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineGuestMonitoringModeInfo)
    }
}

impl DataTypeAware for GuestOsDescriptor {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestOsDescriptor)
    }
}

impl DataTypeAware for VirtualMachineGuestQuiesceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineGuestQuiesceSpec)
    }
}

impl DataTypeAware for VirtualMachineWindowsQuiesceSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineWindowsQuiesceSpec)
    }
}

impl DataTypeAware for VirtualMachineIdeDiskDevicePartitionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineIdeDiskDevicePartitionInfo)
    }
}

impl DataTypeAware for VirtualMachineInstantCloneSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineInstantCloneSpec)
    }
}

impl DataTypeAware for VirtualMachineLegacyNetworkSwitchInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineLegacyNetworkSwitchInfo)
    }
}

impl DataTypeAware for VirtualMachineMessage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMessage)
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadata {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMetadataManagerVmMetadata)
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadataInput {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMetadataManagerVmMetadataInput)
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadataOwner {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMetadataManagerVmMetadataOwner)
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadataResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineMetadataManagerVmMetadataResult)
    }
}

impl DataTypeAware for VirtualMachineNetworkShaperInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineNetworkShaperInfo)
    }
}

impl DataTypeAware for VirtualMachineProfileDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineProfileDetails)
    }
}

impl DataTypeAware for VirtualMachineProfileDetailsDiskProfileDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineProfileDetailsDiskProfileDetails)
    }
}

impl DataTypeAware for VirtualMachineProfileRawData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineProfileRawData)
    }
}

impl DataTypeAware for VirtualMachineProfileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineProfileSpec)
    }
}

impl DataTypeAware for VirtualMachineDefaultProfileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDefaultProfileSpec)
    }
}

impl DataTypeAware for VirtualMachineDefinedProfileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDefinedProfileSpec)
    }
}

impl DataTypeAware for VirtualMachineEmptyProfileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineEmptyProfileSpec)
    }
}

impl DataTypeAware for VirtualMachinePropertyRelation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachinePropertyRelation)
    }
}

impl DataTypeAware for VirtualMachineQuestionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineQuestionInfo)
    }
}

impl DataTypeAware for VirtualMachineRelocateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineRelocateSpec)
    }
}

impl DataTypeAware for VirtualMachineRelocateSpecDiskLocator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineRelocateSpecDiskLocator)
    }
}

impl DataTypeAware for VirtualMachineRelocateSpecDiskLocatorBackingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineRelocateSpecDiskLocatorBackingSpec)
    }
}

impl DataTypeAware for ReplicationConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationConfigSpec)
    }
}

impl DataTypeAware for ReplicationInfoDiskSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationInfoDiskSettings)
    }
}

impl DataTypeAware for VirtualMachineRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineRuntimeInfo)
    }
}

impl DataTypeAware for VirtualMachineRuntimeInfoDasProtectionState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineRuntimeInfoDasProtectionState)
    }
}

impl DataTypeAware for ScheduledHardwareUpgradeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledHardwareUpgradeInfo)
    }
}

impl DataTypeAware for VirtualMachineSgxInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSgxInfo)
    }
}

impl DataTypeAware for VirtualMachineSnapshotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSnapshotInfo)
    }
}

impl DataTypeAware for SnapshotSelectionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SnapshotSelectionSpec)
    }
}

impl DataTypeAware for VirtualMachineSnapshotTree {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSnapshotTree)
    }
}

impl DataTypeAware for VirtualMachineSriovDevicePoolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSriovDevicePoolInfo)
    }
}

impl DataTypeAware for VirtualMachineSriovNetworkDevicePoolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSriovNetworkDevicePoolInfo)
    }
}

impl DataTypeAware for VirtualMachineStorageInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineStorageInfo)
    }
}

impl DataTypeAware for VirtualMachineUsageOnDatastore {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineUsageOnDatastore)
    }
}

impl DataTypeAware for SubnetInfoFolderInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SubnetInfoFolderInfo)
    }
}

impl DataTypeAware for VirtualMachineSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSummary)
    }
}

impl DataTypeAware for VirtualMachineConfigSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConfigSummary)
    }
}

impl DataTypeAware for VirtualMachineGuestSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineGuestSummary)
    }
}

impl DataTypeAware for VirtualMachineQuickStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineQuickStats)
    }
}

impl DataTypeAware for VirtualMachineQuickStatsMemoryTierStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineQuickStatsMemoryTierStats)
    }
}

impl DataTypeAware for VirtualMachineStorageSummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineStorageSummary)
    }
}

impl DataTypeAware for VirtualMachineTargetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineTargetInfo)
    }
}

impl DataTypeAware for VirtualMachineCdromInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineCdromInfo)
    }
}

impl DataTypeAware for VirtualMachineDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDatastoreInfo)
    }
}

impl DataTypeAware for VirtualMachineDiskDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDiskDeviceInfo)
    }
}

impl DataTypeAware for VirtualMachineIdeDiskDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineIdeDiskDeviceInfo)
    }
}

impl DataTypeAware for VirtualMachineScsiDiskDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineScsiDiskDeviceInfo)
    }
}

impl DataTypeAware for VirtualMachineDynamicPassthroughInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDynamicPassthroughInfo)
    }
}

impl DataTypeAware for VirtualMachineFloppyInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineFloppyInfo)
    }
}

impl DataTypeAware for VirtualMachineNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineNetworkInfo)
    }
}

impl DataTypeAware for OpaqueNetworkTargetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OpaqueNetworkTargetInfo)
    }
}

impl DataTypeAware for VirtualMachineParallelInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineParallelInfo)
    }
}

impl DataTypeAware for VirtualMachinePciPassthroughInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachinePciPassthroughInfo)
    }
}

impl DataTypeAware for VirtualMachineSriovInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSriovInfo)
    }
}

impl DataTypeAware for VirtualMachinePciSharedGpuPassthroughInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachinePciSharedGpuPassthroughInfo)
    }
}

impl DataTypeAware for VirtualMachinePrecisionClockInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachinePrecisionClockInfo)
    }
}

impl DataTypeAware for VirtualMachineScsiPassthroughInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineScsiPassthroughInfo)
    }
}

impl DataTypeAware for VirtualMachineSerialInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSerialInfo)
    }
}

impl DataTypeAware for VirtualMachineSgxTargetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSgxTargetInfo)
    }
}

impl DataTypeAware for VirtualMachineSoundInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSoundInfo)
    }
}

impl DataTypeAware for SubnetInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SubnetInfo)
    }
}

impl DataTypeAware for VirtualMachineUsbInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineUsbInfo)
    }
}

impl DataTypeAware for VirtualMachineVFlashModuleInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVFlashModuleInfo)
    }
}

impl DataTypeAware for VirtualMachineVMotionStunTimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVMotionStunTimeInfo)
    }
}

impl DataTypeAware for VirtualMachineVendorDeviceGroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVendorDeviceGroupInfo)
    }
}

impl DataTypeAware for VirtualMachineVgpuDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVgpuDeviceInfo)
    }
}

impl DataTypeAware for VirtualMachineVgpuProfileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVgpuProfileInfo)
    }
}

impl DataTypeAware for ToolsConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ToolsConfigInfo)
    }
}

impl DataTypeAware for ToolsConfigInfoToolsLastInstallInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ToolsConfigInfoToolsLastInstallInfo)
    }
}

impl DataTypeAware for UsbScanCodeSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UsbScanCodeSpec)
    }
}

impl DataTypeAware for UsbScanCodeSpecKeyEvent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UsbScanCodeSpecKeyEvent)
    }
}

impl DataTypeAware for UsbScanCodeSpecModifierType {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UsbScanCodeSpecModifierType)
    }
}

impl DataTypeAware for VirtualMachineVcpuConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVcpuConfig)
    }
}

impl DataTypeAware for VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVendorDeviceGroupInfoComponentDeviceInfo)
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceGroups {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceGroups)
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceGroupsDeviceGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup)
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceGroupsVendorDeviceGroup {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceGroupsVendorDeviceGroup)
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceSwap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceSwap)
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceSwapDeviceSwapInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceSwapDeviceSwapInfo)
    }
}

impl DataTypeAware for VirtualHardware {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualHardware)
    }
}

impl DataTypeAware for VirtualHardwareOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualHardwareOption)
    }
}

impl DataTypeAware for VirtualMachineVirtualNuma {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualNuma)
    }
}

impl DataTypeAware for VirtualMachineVirtualNumaInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualNumaInfo)
    }
}

impl DataTypeAware for VirtualMachineVirtualPMem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualPMem)
    }
}

impl DataTypeAware for CheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CheckResult)
    }
}

impl DataTypeAware for CustomizationAdapterMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationAdapterMapping)
    }
}

impl DataTypeAware for CustomizationGlobalIpSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationGlobalIpSettings)
    }
}

impl DataTypeAware for CustomizationGuiRunOnce {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationGuiRunOnce)
    }
}

impl DataTypeAware for CustomizationGuiUnattended {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationGuiUnattended)
    }
}

impl DataTypeAware for CustomizationIpSettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpSettings)
    }
}

impl DataTypeAware for CustomizationIpSettingsIpV6AddressSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpSettingsIpV6AddressSpec)
    }
}

impl DataTypeAware for CustomizationIdentification {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIdentification)
    }
}

impl DataTypeAware for CustomizationIdentitySettings {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIdentitySettings)
    }
}

impl DataTypeAware for CustomizationCloudinitPrep {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationCloudinitPrep)
    }
}

impl DataTypeAware for CustomizationLinuxPrep {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationLinuxPrep)
    }
}

impl DataTypeAware for CustomizationSysprep {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationSysprep)
    }
}

impl DataTypeAware for CustomizationSysprepText {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationSysprepText)
    }
}

impl DataTypeAware for CustomizationIpGenerator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpGenerator)
    }
}

impl DataTypeAware for CustomizationCustomIpGenerator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationCustomIpGenerator)
    }
}

impl DataTypeAware for CustomizationDhcpIpGenerator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationDhcpIpGenerator)
    }
}

impl DataTypeAware for CustomizationFixedIp {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationFixedIp)
    }
}

impl DataTypeAware for CustomizationUnknownIpGenerator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationUnknownIpGenerator)
    }
}

impl DataTypeAware for CustomizationIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpV6Generator)
    }
}

impl DataTypeAware for CustomizationAutoIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationAutoIpV6Generator)
    }
}

impl DataTypeAware for CustomizationCustomIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationCustomIpV6Generator)
    }
}

impl DataTypeAware for CustomizationDhcpIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationDhcpIpV6Generator)
    }
}

impl DataTypeAware for CustomizationFixedIpV6 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationFixedIpV6)
    }
}

impl DataTypeAware for CustomizationStatelessIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationStatelessIpV6Generator)
    }
}

impl DataTypeAware for CustomizationUnknownIpV6Generator {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationUnknownIpV6Generator)
    }
}

impl DataTypeAware for CustomizationLicenseFilePrintData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationLicenseFilePrintData)
    }
}

impl DataTypeAware for CustomizationName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationName)
    }
}

impl DataTypeAware for CustomizationCustomName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationCustomName)
    }
}

impl DataTypeAware for CustomizationFixedName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationFixedName)
    }
}

impl DataTypeAware for CustomizationPrefixName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationPrefixName)
    }
}

impl DataTypeAware for CustomizationUnknownName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationUnknownName)
    }
}

impl DataTypeAware for CustomizationVirtualMachineName {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationVirtualMachineName)
    }
}

impl DataTypeAware for CustomizationOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationOptions)
    }
}

impl DataTypeAware for CustomizationLinuxOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationLinuxOptions)
    }
}

impl DataTypeAware for CustomizationWinOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationWinOptions)
    }
}

impl DataTypeAware for CustomizationPassword {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationPassword)
    }
}

impl DataTypeAware for CustomizationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationSpec)
    }
}

impl DataTypeAware for CustomizationUserData {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationUserData)
    }
}

impl DataTypeAware for HostDiskMappingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskMappingInfo)
    }
}

impl DataTypeAware for HostDiskMappingPartitionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskMappingPartitionInfo)
    }
}

impl DataTypeAware for HostDiskMappingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskMappingOption)
    }
}

impl DataTypeAware for HostDiskMappingPartitionOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDiskMappingPartitionOption)
    }
}

impl DataTypeAware for VirtualDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevice)
    }
}

impl DataTypeAware for VirtualCdrom {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdrom)
    }
}

impl DataTypeAware for VirtualController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualController)
    }
}

impl DataTypeAware for VirtualIdeController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualIdeController)
    }
}

impl DataTypeAware for VirtualNvdimmController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvdimmController)
    }
}

impl DataTypeAware for VirtualNvmeController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvmeController)
    }
}

impl DataTypeAware for VirtualPciController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciController)
    }
}

impl DataTypeAware for VirtualPs2Controller {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPs2Controller)
    }
}

impl DataTypeAware for VirtualSataController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSataController)
    }
}

impl DataTypeAware for VirtualAhciController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualAhciController)
    }
}

impl DataTypeAware for VirtualScsiController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiController)
    }
}

impl DataTypeAware for ParaVirtualScsiController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ParaVirtualScsiController)
    }
}

impl DataTypeAware for VirtualBusLogicController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualBusLogicController)
    }
}

impl DataTypeAware for VirtualLsiLogicController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualLsiLogicController)
    }
}

impl DataTypeAware for VirtualLsiLogicSasController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualLsiLogicSasController)
    }
}

impl DataTypeAware for VirtualSioController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSioController)
    }
}

impl DataTypeAware for VirtualUsbController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbController)
    }
}

impl DataTypeAware for VirtualUsbxhciController {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbxhciController)
    }
}

impl DataTypeAware for VirtualDisk {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDisk)
    }
}

impl DataTypeAware for VirtualEthernetCard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCard)
    }
}

impl DataTypeAware for VirtualE1000 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualE1000)
    }
}

impl DataTypeAware for VirtualE1000E {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualE1000E)
    }
}

impl DataTypeAware for VirtualPcNet32 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPcNet32)
    }
}

impl DataTypeAware for VirtualSriovEthernetCard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSriovEthernetCard)
    }
}

impl DataTypeAware for VirtualVmxnet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet)
    }
}

impl DataTypeAware for VirtualVmxnet2 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet2)
    }
}

impl DataTypeAware for VirtualVmxnet3 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3)
    }
}

impl DataTypeAware for VirtualVmxnet3Vrdma {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3Vrdma)
    }
}

impl DataTypeAware for VirtualFloppy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppy)
    }
}

impl DataTypeAware for VirtualKeyboard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualKeyboard)
    }
}

impl DataTypeAware for VirtualNvdimm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvdimm)
    }
}

impl DataTypeAware for VirtualPciPassthrough {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthrough)
    }
}

impl DataTypeAware for VirtualParallelPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPort)
    }
}

impl DataTypeAware for VirtualPointingDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPointingDevice)
    }
}

impl DataTypeAware for VirtualPrecisionClock {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPrecisionClock)
    }
}

impl DataTypeAware for VirtualScsiPassthrough {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiPassthrough)
    }
}

impl DataTypeAware for VirtualSerialPort {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPort)
    }
}

impl DataTypeAware for VirtualSoundCard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCard)
    }
}

impl DataTypeAware for VirtualEnsoniq1371 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEnsoniq1371)
    }
}

impl DataTypeAware for VirtualHdAudioCard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualHdAudioCard)
    }
}

impl DataTypeAware for VirtualSoundBlaster16 {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundBlaster16)
    }
}

impl DataTypeAware for VirtualTpm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualTpm)
    }
}

impl DataTypeAware for VirtualUsb {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsb)
    }
}

impl DataTypeAware for VirtualMachineVmciDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmciDevice)
    }
}

impl DataTypeAware for VirtualMachineVmirom {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmirom)
    }
}

impl DataTypeAware for VirtualMachineVideoCard {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVideoCard)
    }
}

impl DataTypeAware for VirtualWdt {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualWdt)
    }
}

impl DataTypeAware for VirtualDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualDeviceDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualCdromAtapiBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromAtapiBackingInfo)
    }
}

impl DataTypeAware for VirtualCdromPassthroughBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromPassthroughBackingInfo)
    }
}

impl DataTypeAware for VirtualDiskRawDiskVer2BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskVer2BackingInfo)
    }
}

impl DataTypeAware for VirtualDiskPartitionedRawDiskVer2BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskPartitionedRawDiskVer2BackingInfo)
    }
}

impl DataTypeAware for VirtualEthernetCardLegacyNetworkBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardLegacyNetworkBackingInfo)
    }
}

impl DataTypeAware for VirtualEthernetCardNetworkBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardNetworkBackingInfo)
    }
}

impl DataTypeAware for VirtualFloppyDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualPciPassthroughDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualPciPassthroughDynamicBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDynamicBackingInfo)
    }
}

impl DataTypeAware for VirtualParallelPortDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPortDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualPointingDeviceDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPointingDeviceDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualScsiPassthroughDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiPassthroughDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualSerialPortDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualSoundCardDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCardDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualUsbRemoteHostBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbRemoteHostBackingInfo)
    }
}

impl DataTypeAware for VirtualUsbusbBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbusbBackingInfo)
    }
}

impl DataTypeAware for VirtualDeviceFileBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceFileBackingInfo)
    }
}

impl DataTypeAware for VirtualCdromIsoBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromIsoBackingInfo)
    }
}

impl DataTypeAware for VirtualDiskFlatVer1BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskFlatVer1BackingInfo)
    }
}

impl DataTypeAware for VirtualDiskFlatVer2BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskFlatVer2BackingInfo)
    }
}

impl DataTypeAware for VirtualDiskLocalPMemBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskLocalPMemBackingInfo)
    }
}

impl DataTypeAware for VirtualDiskRawDiskMappingVer1BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskMappingVer1BackingInfo)
    }
}

impl DataTypeAware for VirtualDiskSeSparseBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSeSparseBackingInfo)
    }
}

impl DataTypeAware for VirtualDiskSparseVer1BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSparseVer1BackingInfo)
    }
}

impl DataTypeAware for VirtualDiskSparseVer2BackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSparseVer2BackingInfo)
    }
}

impl DataTypeAware for VirtualFloppyImageBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyImageBackingInfo)
    }
}

impl DataTypeAware for VirtualNvdimmBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvdimmBackingInfo)
    }
}

impl DataTypeAware for VirtualParallelPortFileBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPortFileBackingInfo)
    }
}

impl DataTypeAware for VirtualSerialPortFileBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortFileBackingInfo)
    }
}

impl DataTypeAware for VirtualDevicePipeBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePipeBackingInfo)
    }
}

impl DataTypeAware for VirtualSerialPortPipeBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortPipeBackingInfo)
    }
}

impl DataTypeAware for VirtualDeviceRemoteDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceRemoteDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualCdromRemoteAtapiBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromRemoteAtapiBackingInfo)
    }
}

impl DataTypeAware for VirtualCdromRemotePassthroughBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromRemotePassthroughBackingInfo)
    }
}

impl DataTypeAware for VirtualFloppyRemoteDeviceBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyRemoteDeviceBackingInfo)
    }
}

impl DataTypeAware for VirtualUsbRemoteClientBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbRemoteClientBackingInfo)
    }
}

impl DataTypeAware for VirtualDeviceUriBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceUriBackingInfo)
    }
}

impl DataTypeAware for VirtualSerialPortUriBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortUriBackingInfo)
    }
}

impl DataTypeAware for VirtualEthernetCardDistributedVirtualPortBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardDistributedVirtualPortBackingInfo)
    }
}

impl DataTypeAware for VirtualEthernetCardOpaqueNetworkBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardOpaqueNetworkBackingInfo)
    }
}

impl DataTypeAware for VirtualPciPassthroughDvxBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDvxBackingInfo)
    }
}

impl DataTypeAware for VirtualPciPassthroughPluginBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughPluginBackingInfo)
    }
}

impl DataTypeAware for VirtualPciPassthroughVmiopBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughVmiopBackingInfo)
    }
}

impl DataTypeAware for VirtualPrecisionClockSystemClockBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPrecisionClockSystemClockBackingInfo)
    }
}

impl DataTypeAware for VirtualSerialPortThinPrintBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortThinPrintBackingInfo)
    }
}

impl DataTypeAware for VirtualSriovEthernetCardSriovBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSriovEthernetCardSriovBackingInfo)
    }
}

impl DataTypeAware for VirtualDeviceBusSlotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBusSlotInfo)
    }
}

impl DataTypeAware for VirtualDevicePciBusSlotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePciBusSlotInfo)
    }
}

impl DataTypeAware for VirtualUsbControllerPciBusSlotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbControllerPciBusSlotInfo)
    }
}

impl DataTypeAware for VirtualDeviceConnectInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceConnectInfo)
    }
}

impl DataTypeAware for VirtualDeviceDeviceGroupInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceDeviceGroupInfo)
    }
}

impl DataTypeAware for VirtualDeviceOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceOption)
    }
}

impl DataTypeAware for VirtualCdromOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromOption)
    }
}

impl DataTypeAware for VirtualControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualControllerOption)
    }
}

impl DataTypeAware for VirtualIdeControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualIdeControllerOption)
    }
}

impl DataTypeAware for VirtualNvdimmControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvdimmControllerOption)
    }
}

impl DataTypeAware for VirtualNvmeControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvmeControllerOption)
    }
}

impl DataTypeAware for VirtualPciControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciControllerOption)
    }
}

impl DataTypeAware for VirtualPs2ControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPs2ControllerOption)
    }
}

impl DataTypeAware for VirtualSataControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSataControllerOption)
    }
}

impl DataTypeAware for VirtualAhciControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualAhciControllerOption)
    }
}

impl DataTypeAware for VirtualScsiControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiControllerOption)
    }
}

impl DataTypeAware for ParaVirtualScsiControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ParaVirtualScsiControllerOption)
    }
}

impl DataTypeAware for VirtualBusLogicControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualBusLogicControllerOption)
    }
}

impl DataTypeAware for VirtualLsiLogicControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualLsiLogicControllerOption)
    }
}

impl DataTypeAware for VirtualLsiLogicSasControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualLsiLogicSasControllerOption)
    }
}

impl DataTypeAware for VirtualSioControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSioControllerOption)
    }
}

impl DataTypeAware for VirtualUsbControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbControllerOption)
    }
}

impl DataTypeAware for VirtualUsbxhciControllerOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbxhciControllerOption)
    }
}

impl DataTypeAware for VirtualDiskOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskOption)
    }
}

impl DataTypeAware for VirtualEthernetCardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardOption)
    }
}

impl DataTypeAware for VirtualE1000Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualE1000Option)
    }
}

impl DataTypeAware for VirtualE1000EOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualE1000EOption)
    }
}

impl DataTypeAware for VirtualPcNet32Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPcNet32Option)
    }
}

impl DataTypeAware for VirtualSriovEthernetCardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSriovEthernetCardOption)
    }
}

impl DataTypeAware for VirtualVmxnetOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnetOption)
    }
}

impl DataTypeAware for VirtualVmxnet2Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet2Option)
    }
}

impl DataTypeAware for VirtualVmxnet3Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3Option)
    }
}

impl DataTypeAware for VirtualVmxnet3VrdmaOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3VrdmaOption)
    }
}

impl DataTypeAware for VirtualFloppyOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyOption)
    }
}

impl DataTypeAware for VirtualKeyboardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualKeyboardOption)
    }
}

impl DataTypeAware for VirtualNvdimmOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualNvdimmOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughOption)
    }
}

impl DataTypeAware for VirtualParallelPortOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPortOption)
    }
}

impl DataTypeAware for VirtualPointingDeviceOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPointingDeviceOption)
    }
}

impl DataTypeAware for VirtualPrecisionClockOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPrecisionClockOption)
    }
}

impl DataTypeAware for VirtualScsiPassthroughOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiPassthroughOption)
    }
}

impl DataTypeAware for VirtualSerialPortOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortOption)
    }
}

impl DataTypeAware for VirtualSoundCardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCardOption)
    }
}

impl DataTypeAware for VirtualEnsoniq1371Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEnsoniq1371Option)
    }
}

impl DataTypeAware for VirtualHdAudioCardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualHdAudioCardOption)
    }
}

impl DataTypeAware for VirtualSoundBlaster16Option {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundBlaster16Option)
    }
}

impl DataTypeAware for VirtualTpmOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualTpmOption)
    }
}

impl DataTypeAware for VirtualUsbOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbOption)
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmciDeviceOption)
    }
}

impl DataTypeAware for VirtualVmiromOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmiromOption)
    }
}

impl DataTypeAware for VirtualVideoCardOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVideoCardOption)
    }
}

impl DataTypeAware for VirtualWdtOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualWdtOption)
    }
}

impl DataTypeAware for VirtualDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualDeviceDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualCdromAtapiBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromAtapiBackingOption)
    }
}

impl DataTypeAware for VirtualCdromPassthroughBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromPassthroughBackingOption)
    }
}

impl DataTypeAware for VirtualCdromRemoteAtapiBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromRemoteAtapiBackingOption)
    }
}

impl DataTypeAware for VirtualDiskRawDiskMappingVer1BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskMappingVer1BackingOption)
    }
}

impl DataTypeAware for VirtualDiskRawDiskVer2BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskVer2BackingOption)
    }
}

impl DataTypeAware for VirtualDiskPartitionedRawDiskVer2BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskPartitionedRawDiskVer2BackingOption)
    }
}

impl DataTypeAware for VirtualEthernetCardLegacyNetworkBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardLegacyNetworkBackingOption)
    }
}

impl DataTypeAware for VirtualEthernetCardNetworkBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardNetworkBackingOption)
    }
}

impl DataTypeAware for VirtualFloppyDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughDynamicBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDynamicBackingOption)
    }
}

impl DataTypeAware for VirtualParallelPortDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPortDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualPointingDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPointingDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualScsiPassthroughDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiPassthroughDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualSerialPortDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualSoundCardDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCardDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualUsbRemoteHostBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbRemoteHostBackingOption)
    }
}

impl DataTypeAware for VirtualUsbusbBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbusbBackingOption)
    }
}

impl DataTypeAware for VirtualDeviceFileBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceFileBackingOption)
    }
}

impl DataTypeAware for VirtualCdromIsoBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromIsoBackingOption)
    }
}

impl DataTypeAware for VirtualDiskFlatVer1BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskFlatVer1BackingOption)
    }
}

impl DataTypeAware for VirtualDiskFlatVer2BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskFlatVer2BackingOption)
    }
}

impl DataTypeAware for VirtualDiskLocalPMemBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskLocalPMemBackingOption)
    }
}

impl DataTypeAware for VirtualDiskSeSparseBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSeSparseBackingOption)
    }
}

impl DataTypeAware for VirtualDiskSparseVer1BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSparseVer1BackingOption)
    }
}

impl DataTypeAware for VirtualDiskSparseVer2BackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSparseVer2BackingOption)
    }
}

impl DataTypeAware for VirtualFloppyImageBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyImageBackingOption)
    }
}

impl DataTypeAware for VirtualParallelPortFileBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualParallelPortFileBackingOption)
    }
}

impl DataTypeAware for VirtualSerialPortFileBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortFileBackingOption)
    }
}

impl DataTypeAware for VirtualDevicePipeBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePipeBackingOption)
    }
}

impl DataTypeAware for VirtualSerialPortPipeBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortPipeBackingOption)
    }
}

impl DataTypeAware for VirtualDeviceRemoteDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceRemoteDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualCdromRemotePassthroughBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualCdromRemotePassthroughBackingOption)
    }
}

impl DataTypeAware for VirtualFloppyRemoteDeviceBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualFloppyRemoteDeviceBackingOption)
    }
}

impl DataTypeAware for VirtualUsbRemoteClientBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualUsbRemoteClientBackingOption)
    }
}

impl DataTypeAware for VirtualDeviceUriBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceUriBackingOption)
    }
}

impl DataTypeAware for VirtualSerialPortUriBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortUriBackingOption)
    }
}

impl DataTypeAware for VirtualEthernetCardDvPortBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardDvPortBackingOption)
    }
}

impl DataTypeAware for VirtualEthernetCardOpaqueNetworkBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardOpaqueNetworkBackingOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughDvxBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughDvxBackingOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughPluginBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughPluginBackingOption)
    }
}

impl DataTypeAware for VirtualPciPassthroughVmiopBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughVmiopBackingOption)
    }
}

impl DataTypeAware for VirtualPrecisionClockSystemClockBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPrecisionClockSystemClockBackingOption)
    }
}

impl DataTypeAware for VirtualSerialPortThinPrintBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSerialPortThinPrintBackingOption)
    }
}

impl DataTypeAware for VirtualSriovEthernetCardSriovBackingOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSriovEthernetCardSriovBackingOption)
    }
}

impl DataTypeAware for VirtualDeviceBusSlotOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBusSlotOption)
    }
}

impl DataTypeAware for VirtualDeviceConnectOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceConnectOption)
    }
}

impl DataTypeAware for VirtualDeviceConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceConfigSpec)
    }
}

impl DataTypeAware for VirtualDiskConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskConfigSpec)
    }
}

impl DataTypeAware for VirtualDeviceConfigSpecBackingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceConfigSpecBackingSpec)
    }
}

impl DataTypeAware for VirtualDiskVFlashCacheConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskVFlashCacheConfigInfo)
    }
}

impl DataTypeAware for VirtualDiskId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskId)
    }
}

impl DataTypeAware for VirtualDiskDeltaDiskFormatsSupported {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskDeltaDiskFormatsSupported)
    }
}

impl DataTypeAware for VirtualDiskOptionVFlashCacheConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskOptionVFlashCacheConfigOption)
    }
}

impl DataTypeAware for VirtualEthernetCardResourceAllocation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardResourceAllocation)
    }
}

impl DataTypeAware for VirtualPciPassthroughAllowedDevice {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughAllowedDevice)
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceFilterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmciDeviceFilterInfo)
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmciDeviceFilterSpec)
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceOptionFilterSpecOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVmciDeviceOptionFilterSpecOption)
    }
}

impl DataTypeAware for VirtualVmxnet3StrictLatencyConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3StrictLatencyConfig)
    }
}

impl DataTypeAware for VirtualVmxnet3OptionStrictLatencyConfigOption {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3OptionStrictLatencyConfigOption)
    }
}

impl DataTypeAware for GuestAliases {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAliases)
    }
}

impl DataTypeAware for GuestAuthAliasInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthAliasInfo)
    }
}

impl DataTypeAware for GuestAuthSubject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthSubject)
    }
}

impl DataTypeAware for GuestAuthAnySubject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthAnySubject)
    }
}

impl DataTypeAware for GuestAuthNamedSubject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthNamedSubject)
    }
}

impl DataTypeAware for GuestMappedAliases {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestMappedAliases)
    }
}

impl DataTypeAware for GuestFileAttributes {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestFileAttributes)
    }
}

impl DataTypeAware for GuestPosixFileAttributes {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestPosixFileAttributes)
    }
}

impl DataTypeAware for GuestWindowsFileAttributes {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestWindowsFileAttributes)
    }
}

impl DataTypeAware for GuestFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestFileInfo)
    }
}

impl DataTypeAware for FileTransferInformation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileTransferInformation)
    }
}

impl DataTypeAware for GuestListFileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestListFileInfo)
    }
}

impl DataTypeAware for GuestAuthentication {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthentication)
    }
}

impl DataTypeAware for NamePasswordAuthentication {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NamePasswordAuthentication)
    }
}

impl DataTypeAware for SamlTokenAuthentication {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SamlTokenAuthentication)
    }
}

impl DataTypeAware for SspiAuthentication {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SspiAuthentication)
    }
}

impl DataTypeAware for TicketedSessionAuthentication {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TicketedSessionAuthentication)
    }
}

impl DataTypeAware for GuestProcessInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestProcessInfo)
    }
}

impl DataTypeAware for GuestProgramSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestProgramSpec)
    }
}

impl DataTypeAware for GuestWindowsProgramSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestWindowsProgramSpec)
    }
}

impl DataTypeAware for GuestRegKeySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegKeySpec)
    }
}

impl DataTypeAware for GuestRegKeyNameSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegKeyNameSpec)
    }
}

impl DataTypeAware for GuestRegKeyRecordSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegKeyRecordSpec)
    }
}

impl DataTypeAware for GuestRegValueSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueSpec)
    }
}

impl DataTypeAware for GuestRegValueDataSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueDataSpec)
    }
}

impl DataTypeAware for GuestRegValueBinarySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueBinarySpec)
    }
}

impl DataTypeAware for GuestRegValueDwordSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueDwordSpec)
    }
}

impl DataTypeAware for GuestRegValueExpandStringSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueExpandStringSpec)
    }
}

impl DataTypeAware for GuestRegValueMultiStringSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueMultiStringSpec)
    }
}

impl DataTypeAware for GuestRegValueQwordSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueQwordSpec)
    }
}

impl DataTypeAware for GuestRegValueStringSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueStringSpec)
    }
}

impl DataTypeAware for GuestRegValueNameSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueNameSpec)
    }
}

impl DataTypeAware for DeviceGroupId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DeviceGroupId)
    }
}

impl DataTypeAware for FaultDomainId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultDomainId)
    }
}

impl DataTypeAware for FaultDomainInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultDomainInfo)
    }
}

impl DataTypeAware for ReplicationGroupId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationGroupId)
    }
}

impl DataTypeAware for ReplicationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ReplicationSpec)
    }
}

impl DataTypeAware for VsanCapacityReservationInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCapacityReservationInfo)
    }
}

impl DataTypeAware for ClusterRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterRuntimeInfo)
    }
}

impl DataTypeAware for VsanCompatibilityCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCompatibilityCheckResult)
    }
}

impl DataTypeAware for VimVsanDataEfficiencyCapacityState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanDataEfficiencyCapacityState)
    }
}

impl DataTypeAware for VsanDataEfficiencyConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataEfficiencyConfig)
    }
}

impl DataTypeAware for VsanDataEfficiencyConfigEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataEfficiencyConfigEx)
    }
}

impl DataTypeAware for VsanDataEncryptionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataEncryptionConfig)
    }
}

impl DataTypeAware for VsanDataInTransitEncryptionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataInTransitEncryptionConfig)
    }
}

impl DataTypeAware for VsanDatastoreConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreConfig)
    }
}

impl DataTypeAware for VsanAdvancedDatastoreConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanAdvancedDatastoreConfig)
    }
}

impl DataTypeAware for VsanDatastoreSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreSpec)
    }
}

impl DataTypeAware for VsanClientDatastoreConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClientDatastoreConfig)
    }
}

impl DataTypeAware for VsanXvcClientConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcClientConfig)
    }
}

impl DataTypeAware for DefaultDatastorePolicySelectionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DefaultDatastorePolicySelectionInfo)
    }
}

impl DataTypeAware for VsanDirectoryServerConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDirectoryServerConfig)
    }
}

impl DataTypeAware for ActiveVsanDirectoryServerConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ActiveVsanDirectoryServerConfig)
    }
}

impl DataTypeAware for DiskClaimConfiguration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiskClaimConfiguration)
    }
}

impl DataTypeAware for VsanEntityCompatibilityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEntityCompatibilityResult)
    }
}

impl DataTypeAware for EntityResourceCheckDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityResourceCheckDetails)
    }
}

impl DataTypeAware for VsanDiskGroupResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskGroupResourceCheckResult)
    }
}

impl DataTypeAware for VsanDiskResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskResourceCheckResult)
    }
}

impl DataTypeAware for VsanStoragePoolDiskResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStoragePoolDiskResourceCheckResult)
    }
}

impl DataTypeAware for VsanFaultDomainResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFaultDomainResourceCheckResult)
    }
}

impl DataTypeAware for VsanHostResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostResourceCheckResult)
    }
}

impl DataTypeAware for VsanResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckResult)
    }
}

impl DataTypeAware for VsanResourceCheckComponentResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckComponentResult)
    }
}

impl DataTypeAware for VsanResourceCheckDataPersistenceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckDataPersistenceResult)
    }
}

impl DataTypeAware for VsanResourceCheckVsanResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckVsanResult)
    }
}

impl DataTypeAware for VsanStoragePoolResourceCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStoragePoolResourceCheckResult)
    }
}

impl DataTypeAware for VsanFileServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceConfig)
    }
}

impl DataTypeAware for VsanFileServiceDomain {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceDomain)
    }
}

impl DataTypeAware for VsanFileServiceDomainConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceDomainConfig)
    }
}

impl DataTypeAware for VsanFileServiceDomainQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceDomainQuerySpec)
    }
}

impl DataTypeAware for VsanFileShare {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShare)
    }
}

impl DataTypeAware for VsanFileShareConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareConfig)
    }
}

impl DataTypeAware for VsanFileShareNetPermission {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareNetPermission)
    }
}

impl DataTypeAware for VsanFileShareQueryProperties {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareQueryProperties)
    }
}

impl DataTypeAware for FileShareQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileShareQueryResult)
    }
}

impl DataTypeAware for VsanFileShareQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareQuerySpec)
    }
}

impl DataTypeAware for VsanFileShareRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareRuntimeInfo)
    }
}

impl DataTypeAware for VsanFileShareSmbOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareSmbOptions)
    }
}

impl DataTypeAware for VsanFileShareSnapshot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareSnapshot)
    }
}

impl DataTypeAware for VsanFileShareSnapshotConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareSnapshotConfig)
    }
}

impl DataTypeAware for VsanFileShareSnapshotQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareSnapshotQueryResult)
    }
}

impl DataTypeAware for VsanFileShareSnapshotQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileShareSnapshotQuerySpec)
    }
}

impl DataTypeAware for VsanHciMeshDatastoreSource {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHciMeshDatastoreSource)
    }
}

impl DataTypeAware for VsanIoDiagnosticsFailedCheck {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsFailedCheck)
    }
}

impl DataTypeAware for VsanIoDiagnosticsInstance {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsInstance)
    }
}

impl DataTypeAware for VsanIoDiagnosticsInstanceEvent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsInstanceEvent)
    }
}

impl DataTypeAware for VsanIoDiagnosticsInstanceQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsInstanceQuerySpec)
    }
}

impl DataTypeAware for VsanIoDiagnosticsObjectLayout {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsObjectLayout)
    }
}

impl DataTypeAware for VsanIoDiagnosticsPrecheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsPrecheckResult)
    }
}

impl DataTypeAware for VsanIoDiagnosticsStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsStats)
    }
}

impl DataTypeAware for VsanIoDiagnosticsTarget {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsTarget)
    }
}

impl DataTypeAware for VsanIoDiagnosticsTargetStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoDiagnosticsTargetStats)
    }
}

impl DataTypeAware for VsanIoLatency {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoLatency)
    }
}

impl DataTypeAware for VsanIoLatencyMetrics {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoLatencyMetrics)
    }
}

impl DataTypeAware for LifecycleConfigDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LifecycleConfigDetails)
    }
}

impl DataTypeAware for LifecycleFaultDomainDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LifecycleFaultDomainDetails)
    }
}

impl DataTypeAware for LifecyclePreCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LifecyclePreCheckResult)
    }
}

impl DataTypeAware for LifecycleWitnessDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LifecycleWitnessDetails)
    }
}

impl DataTypeAware for VsanMetricProfile {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMetricProfile)
    }
}

impl DataTypeAware for VsanMetricsConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMetricsConfig)
    }
}

impl DataTypeAware for VsanMountPrecheckItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckItem)
    }
}

impl DataTypeAware for VsanDatastoreSourcePrecheckItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreSourcePrecheckItem)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkConnectivityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkConnectivityResult)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkLatencyResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkLatencyResult)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkConnectivity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkConnectivity)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkConnectivityDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkConnectivityDetail)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkLatency {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkLatency)
    }
}

impl DataTypeAware for VsanMountPrecheckNetworkLatencyDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckNetworkLatencyDetail)
    }
}

impl DataTypeAware for VsanMountPrecheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckResult)
    }
}

impl DataTypeAware for VsanDatastoreSourcePrecheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreSourcePrecheckResult)
    }
}

impl DataTypeAware for VsanObjectHealthTelemetrySummary {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectHealthTelemetrySummary)
    }
}

impl DataTypeAware for VsanObjectIoStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectIoStats)
    }
}

impl DataTypeAware for VsanProactiveRebalanceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanProactiveRebalanceInfo)
    }
}

impl DataTypeAware for VsanRdmaConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRdmaConfig)
    }
}

impl DataTypeAware for VsanRemoteVcInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRemoteVcInfo)
    }
}

impl DataTypeAware for VsanRemoteVcInfoStandalone {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRemoteVcInfoStandalone)
    }
}

impl DataTypeAware for RemoteVsanSite {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RemoteVsanSite)
    }
}

impl DataTypeAware for RemoteVsanSiteAffinity {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RemoteVsanSiteAffinity)
    }
}

impl DataTypeAware for RepairTimerInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RepairTimerInfo)
    }
}

impl DataTypeAware for VsanResourceCheckSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckSpec)
    }
}

impl DataTypeAware for VsanResourceCheckStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckStatus)
    }
}

impl DataTypeAware for VsanResourceCheckTaskDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckTaskDetails)
    }
}

impl DataTypeAware for VsanDiskDataEvacuationResourceCheckTaskDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskDataEvacuationResourceCheckTaskDetails)
    }
}

impl DataTypeAware for ResyncIopsInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResyncIopsInfo)
    }
}

impl DataTypeAware for VsanRuntimeStatsHostMap {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRuntimeStatsHostMap)
    }
}

impl DataTypeAware for SsdEnduranceThresholdSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SsdEnduranceThresholdSpec)
    }
}

impl DataTypeAware for VsanServerHostUnicastInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanServerHostUnicastInfo)
    }
}

impl DataTypeAware for VsanSharedWitnessCompatibilityResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSharedWitnessCompatibilityResult)
    }
}

impl DataTypeAware for VsanSnapServiceConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSnapServiceConfig)
    }
}

impl DataTypeAware for VcRemoteVsanServerClusterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VcRemoteVsanServerClusterConfig)
    }
}

impl DataTypeAware for VcRemoteVsanServerClusterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VcRemoteVsanServerClusterInfo)
    }
}

impl DataTypeAware for VsanIscsiVipConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiVipConfigSpec)
    }
}

impl DataTypeAware for VsanIscsiVipConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiVipConfig)
    }
}

impl DataTypeAware for VsanIscsiVipDVswitchConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiVipDVswitchConfig)
    }
}

impl DataTypeAware for VsanVipNetworkConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVipNetworkConfig)
    }
}

impl DataTypeAware for VsanIscsiVipVswitchConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiVipVswitchConfig)
    }
}

impl DataTypeAware for VsanBurnInTest {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanBurnInTest)
    }
}

impl DataTypeAware for VsanBurnInTestCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanBurnInTestCheckResult)
    }
}

impl DataTypeAware for VsanCloudHealthStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCloudHealthStatus)
    }
}

impl DataTypeAware for VsanClusterBurnInTestResultList {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterBurnInTestResultList)
    }
}

impl DataTypeAware for VsanCompliantDriver {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCompliantDriver)
    }
}

impl DataTypeAware for VsanCompliantFirmware {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanCompliantFirmware)
    }
}

impl DataTypeAware for VsanConfigBaseIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigBaseIssue)
    }
}

impl DataTypeAware for VsanConfigNotAllDisksClaimedIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigNotAllDisksClaimedIssue)
    }
}

impl DataTypeAware for VsanConfigCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigCheckResult)
    }
}

impl DataTypeAware for VsanDatastoreDefaultPolicySelectionConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreDefaultPolicySelectionConfig)
    }
}

impl DataTypeAware for VsanDeconvergedNetConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDeconvergedNetConfig)
    }
}

impl DataTypeAware for VsanDiskModelInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskModelInfo)
    }
}

impl DataTypeAware for VsanDownloadItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDownloadItem)
    }
}

impl DataTypeAware for VsanEsaConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEsaConfig)
    }
}

impl DataTypeAware for VsanEsaConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEsaConfigInfo)
    }
}

impl DataTypeAware for VsanEsaDiskConfiguration {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanEsaDiskConfiguration)
    }
}

impl DataTypeAware for VsanExtendedConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanExtendedConfig)
    }
}

impl DataTypeAware for VsanFileServiceOvfSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServiceOvfSpec)
    }
}

impl DataTypeAware for VsanFileServicePreflightCheckResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanFileServicePreflightCheckResult)
    }
}

impl DataTypeAware for VsanGenericClusterBaseIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanGenericClusterBaseIssue)
    }
}

impl DataTypeAware for VsanGenericClusterBestPracticeHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanGenericClusterBestPracticeHealth)
    }
}

impl DataTypeAware for VsanHclDeviceConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclDeviceConstraint)
    }
}

impl DataTypeAware for VsanHclDiskConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclDiskConstraint)
    }
}

impl DataTypeAware for VsanHclDriverInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclDriverInfo)
    }
}

impl DataTypeAware for VsanHclMinFwConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclMinFwConstraint)
    }
}

impl DataTypeAware for VsanHclQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclQuerySpec)
    }
}

impl DataTypeAware for VsanHclReleaseConstraint {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclReleaseConstraint)
    }
}

impl DataTypeAware for VsanHealthConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthConfigSpec)
    }
}

impl DataTypeAware for VsanHealthCustomizationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthCustomizationSpec)
    }
}

impl DataTypeAware for VsanHealthThreshold {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthThreshold)
    }
}

impl DataTypeAware for VsanHistoricalHealthConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHistoricalHealthConfig)
    }
}

impl DataTypeAware for VsanHostDeviceInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDeviceInfo)
    }
}

impl DataTypeAware for VsanHwToVcgInfoMappingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHwToVcgInfoMappingSpec)
    }
}

impl DataTypeAware for VsanIoTripAnalyzerConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoTripAnalyzerConfig)
    }
}

impl DataTypeAware for VsanIoTripAnalyzerRecurrence {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIoTripAnalyzerRecurrence)
    }
}

impl DataTypeAware for VsanInternalExtendedConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanInternalExtendedConfig)
    }
}

impl DataTypeAware for VsanNetworkConfigBaseIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigBaseIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigPnicSpeedInconsistencyIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigPnicSpeedInconsistencyIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigPortgroupWithNoRedundancyIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigPortgroupWithNoRedundancyIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigVdsScopeIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigVdsScopeIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigVsanNotOnVdsIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigVsanNotOnVdsIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigVswitchWithNoRedundancyIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigVswitchWithNoRedundancyIssue)
    }
}

impl DataTypeAware for VsanNetworkVMotionVmknicNotFountIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkVMotionVmknicNotFountIssue)
    }
}

impl DataTypeAware for VsanNetworkConfigBestPracticeHealth {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigBestPracticeHealth)
    }
}

impl DataTypeAware for VsanObjSnapParams {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjSnapParams)
    }
}

impl DataTypeAware for VsanObjectDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectDetail)
    }
}

impl DataTypeAware for VsanObjectSnapshotId {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectSnapshotId)
    }
}

impl DataTypeAware for VimVsanVsanPMemConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanVsanPMemConfig)
    }
}

impl DataTypeAware for VsanPerfsvcHealthResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPerfsvcHealthResult)
    }
}

impl DataTypeAware for VsanPrepareVsanForVcsaSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPrepareVsanForVcsaSpec)
    }
}

impl DataTypeAware for VsanSnapshotDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSnapshotDetail)
    }
}

impl DataTypeAware for VsanSnapshotQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSnapshotQueryResult)
    }
}

impl DataTypeAware for VsanSnapshotQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSnapshotQuerySpec)
    }
}

impl DataTypeAware for VsanSpaceEfficiencyMetadataSize {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceEfficiencyMetadataSize)
    }
}

impl DataTypeAware for VsanSpaceEfficiencyRatio {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSpaceEfficiencyRatio)
    }
}

impl DataTypeAware for VsanUnmapConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUnmapConfig)
    }
}

impl DataTypeAware for VsanUpdateItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpdateItem)
    }
}

impl DataTypeAware for VsanVcPostDeployConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcPostDeployConfigSpec)
    }
}

impl DataTypeAware for VsanVcStretchedClusterConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcStretchedClusterConfigSpec)
    }
}

impl DataTypeAware for VsanVcsaDeploymentProgress {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVcsaDeploymentProgress)
    }
}

impl DataTypeAware for VsanVdsMigrationPlan {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVdsMigrationPlan)
    }
}

impl DataTypeAware for VsanVdsPgMigrationHostInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVdsPgMigrationHostInfo)
    }
}

impl DataTypeAware for VsanVdsPgMigrationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVdsPgMigrationSpec)
    }
}

impl DataTypeAware for VsanVdsPgMigrationVmInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVdsPgMigrationVmInfo)
    }
}

impl DataTypeAware for VsanVibInstallPreflightStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVibInstallPreflightStatus)
    }
}

impl DataTypeAware for VsanVibScanResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVibScanResult)
    }
}

impl DataTypeAware for VsanVibSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVibSpec)
    }
}

impl DataTypeAware for VsanVmVdsMigrationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVmVdsMigrationSpec)
    }
}

impl DataTypeAware for VsanVnicVdsMigrationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVnicVdsMigrationSpec)
    }
}

impl DataTypeAware for VsanVumConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanVumConfig)
    }
}

impl DataTypeAware for VsanWitnessHostConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanWitnessHostConfig)
    }
}

impl DataTypeAware for VsanXvcClientInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcClientInfo)
    }
}

impl DataTypeAware for VsanXvcDatastoreConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcDatastoreConfig)
    }
}

impl DataTypeAware for VsanXvcDatastoreInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcDatastoreInfo)
    }
}

impl DataTypeAware for VsanXvcClientInfoSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcClientInfoSpec)
    }
}

impl DataTypeAware for VsanXvcQueryCriteria {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcQueryCriteria)
    }
}

impl DataTypeAware for VsanXvcQueryFilter {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcQueryFilter)
    }
}

impl DataTypeAware for VsanXvcQueryPropertyValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcQueryPropertyValue)
    }
}

impl DataTypeAware for VsanXvcQueryResultSet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcQueryResultSet)
    }
}

impl DataTypeAware for VsanXvcQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcQuerySpec)
    }
}

impl DataTypeAware for VsanXvcResultItem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanXvcResultItem)
    }
}

impl DataTypeAware for VsanClusterConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterConfigInfo)
    }
}

impl DataTypeAware for VsanConfigInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigInfoEx)
    }
}

impl DataTypeAware for VsanClusterConfigInfoHostDefaultInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterConfigInfoHostDefaultInfo)
    }
}

impl DataTypeAware for VsanClusterCoreConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterCoreConfig)
    }
}

impl DataTypeAware for VsanClusterCoreConfigSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterCoreConfigSpec)
    }
}

impl DataTypeAware for VsanHostAbortWipeDiskStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostAbortWipeDiskStatus)
    }
}

impl DataTypeAware for VsanHostAboutInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostAboutInfoEx)
    }
}

impl DataTypeAware for VsanAddStoragePoolDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanAddStoragePoolDiskSpec)
    }
}

impl DataTypeAware for VsanHostClusterStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostClusterStatus)
    }
}

impl DataTypeAware for VsanHostClusterStatusState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostClusterStatusState)
    }
}

impl DataTypeAware for VsanHostClusterStatusStateCompletionEstimate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostClusterStatusStateCompletionEstimate)
    }
}

impl DataTypeAware for VsanComplianceDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanComplianceDetail)
    }
}

impl DataTypeAware for VsanComplianceResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanComplianceResult)
    }
}

impl DataTypeAware for VsanHostConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfo)
    }
}

impl DataTypeAware for VsanHostConfigInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoEx)
    }
}

impl DataTypeAware for VsanHostConfigInfoClusterInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoClusterInfo)
    }
}

impl DataTypeAware for VsanHostFaultDomainInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostFaultDomainInfo)
    }
}

impl DataTypeAware for VsanHostConfigInfoNetworkInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoNetworkInfo)
    }
}

impl DataTypeAware for VsanHostConfigInfoNetworkInfoPortConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoNetworkInfoPortConfig)
    }
}

impl DataTypeAware for VsanHostPortConfigEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostPortConfigEx)
    }
}

impl DataTypeAware for VsanHostConfigInfoStorageInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoStorageInfo)
    }
}

impl DataTypeAware for VsanHostCreateNativeKeyProviderSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostCreateNativeKeyProviderSpec)
    }
}

impl DataTypeAware for VsanInTransitEncryptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanInTransitEncryptionInfo)
    }
}

impl DataTypeAware for VsanHostDecommissionMode {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDecommissionMode)
    }
}

impl DataTypeAware for VsanDeleteStoragePoolDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDeleteStoragePoolDiskSpec)
    }
}

impl DataTypeAware for VsanHostDiskMapInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDiskMapInfo)
    }
}

impl DataTypeAware for VimVsanHostDiskMapInfoEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostDiskMapInfoEx)
    }
}

impl DataTypeAware for VsanHostDiskMapResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDiskMapResult)
    }
}

impl DataTypeAware for VsanHostDiskMapping {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDiskMapping)
    }
}

impl DataTypeAware for VimVsanHostDiskMappingCreationSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostDiskMappingCreationSpec)
    }
}

impl DataTypeAware for VsanHostDiskResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDiskResult)
    }
}

impl DataTypeAware for VimVsanHostDiskResultEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostDiskResultEx)
    }
}

impl DataTypeAware for VsanHostDrsStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDrsStats)
    }
}

impl DataTypeAware for VsanHostEncryptionInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostEncryptionInfo)
    }
}

impl DataTypeAware for VsanHostIpConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostIpConfig)
    }
}

impl DataTypeAware for VsanHostIpConfigEx {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostIpConfigEx)
    }
}

impl DataTypeAware for VsanHostMembershipInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostMembershipInfo)
    }
}

impl DataTypeAware for VsanPolicyStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanPolicyStatus)
    }
}

impl DataTypeAware for VimVsanHostQueryVsanDisksSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostQueryVsanDisksSpec)
    }
}

impl DataTypeAware for RemoteVsanServerClusterConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RemoteVsanServerClusterConfig)
    }
}

impl DataTypeAware for VsanHostRuntimeStats {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostRuntimeStats)
    }
}

impl DataTypeAware for VsanHostServerClusterUnicastConfig {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostServerClusterUnicastConfig)
    }
}

impl DataTypeAware for VsanHostServerClusterUnicastInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostServerClusterUnicastInfo)
    }
}

impl DataTypeAware for SiteAffinityInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SiteAffinityInfo)
    }
}

impl DataTypeAware for VsanStoragePoolDisk {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanStoragePoolDisk)
    }
}

impl DataTypeAware for VimVsanHostStoragePoolDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostStoragePoolDiskInfo)
    }
}

impl DataTypeAware for VimVsanHostStoragePoolInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostStoragePoolInfo)
    }
}

impl DataTypeAware for VimVsanHostTrimDiskEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostTrimDiskEntry)
    }
}

impl DataTypeAware for VimVsanHostTrimDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostTrimDiskSpec)
    }
}

impl DataTypeAware for VimVsanHostUpdateStoragePoolDiskSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostUpdateStoragePoolDiskSpec)
    }
}

impl DataTypeAware for VsanHostAssociatedObjects {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostAssociatedObjects)
    }
}

impl DataTypeAware for VsanHostAssociatedObjectsResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostAssociatedObjectsResult)
    }
}

impl DataTypeAware for VsanComplianceQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanComplianceQuerySpec)
    }
}

impl DataTypeAware for VsanHostComponentSyncState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostComponentSyncState)
    }
}

impl DataTypeAware for VimVsanHostVsanDirectStorage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanDirectStorage)
    }
}

impl DataTypeAware for VsanHostVsanDiskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostVsanDiskInfo)
    }
}

impl DataTypeAware for VimVsanHostVsanDiskManagementSystemCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanDiskManagementSystemCapability)
    }
}

impl DataTypeAware for VimVsanHostVsanHostCapability {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanHostCapability)
    }
}

impl DataTypeAware for VimVsanHostVsanManagedDisksInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanManagedDisksInfo)
    }
}

impl DataTypeAware for VimVsanHostVsanManagedPMemInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanManagedPMemInfo)
    }
}

impl DataTypeAware for VsanObjectProfileInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanObjectProfileInfo)
    }
}

impl DataTypeAware for VsanHostVsanObjectSyncState {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostVsanObjectSyncState)
    }
}

impl DataTypeAware for VsanHostRuntimeInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostRuntimeInfo)
    }
}

impl DataTypeAware for VsanHostRuntimeInfoDiskIssue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostRuntimeInfoDiskIssue)
    }
}

impl DataTypeAware for VimVsanHostVsanScsiDisk {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimVsanHostVsanScsiDisk)
    }
}

impl DataTypeAware for VsanHostVsanObjectSyncQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostVsanObjectSyncQueryResult)
    }
}

impl DataTypeAware for VsanSyncingObjectRecoveryDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanSyncingObjectRecoveryDetails)
    }
}

impl DataTypeAware for VsanWhatIfEvacDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanWhatIfEvacDetail)
    }
}

impl DataTypeAware for VsanWhatIfEvacResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanWhatIfEvacResult)
    }
}

impl DataTypeAware for VsanHostWipeDiskStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostWipeDiskStatus)
    }
}

impl DataTypeAware for BaseConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfo)
    }
}

impl DataTypeAware for VStorageObjectConfigInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectConfigInfo)
    }
}

impl DataTypeAware for BaseConfigInfoBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoBackingInfo)
    }
}

impl DataTypeAware for BaseConfigInfoFileBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoFileBackingInfo)
    }
}

impl DataTypeAware for BaseConfigInfoDiskFileBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoDiskFileBackingInfo)
    }
}

impl DataTypeAware for BaseConfigInfoRawDiskMappingBackingInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoRawDiskMappingBackingInfo)
    }
}

impl DataTypeAware for VslmCreateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCreateSpec)
    }
}

impl DataTypeAware for VslmCreateSpecBackingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCreateSpecBackingSpec)
    }
}

impl DataTypeAware for VslmCreateSpecDiskFileBackingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCreateSpecDiskFileBackingSpec)
    }
}

impl DataTypeAware for VslmCreateSpecRawDiskMappingBackingSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCreateSpecRawDiskMappingBackingSpec)
    }
}

impl DataTypeAware for DiskCryptoSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DiskCryptoSpec)
    }
}

impl DataTypeAware for Id {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Id)
    }
}

impl DataTypeAware for VslmInfrastructureObjectPolicy {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmInfrastructureObjectPolicy)
    }
}

impl DataTypeAware for VslmInfrastructureObjectPolicySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmInfrastructureObjectPolicySpec)
    }
}

impl DataTypeAware for VslmMigrateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmMigrateSpec)
    }
}

impl DataTypeAware for VslmCloneSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCloneSpec)
    }
}

impl DataTypeAware for VslmRelocateSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmRelocateSpec)
    }
}

impl DataTypeAware for VStorageObjectReconcileResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectReconcileResult)
    }
}

impl DataTypeAware for VStorageObjectReconcileResultInvalidDiskPath {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectReconcileResultInvalidDiskPath)
    }
}

impl DataTypeAware for VStorageObjectReconcileResultReconcileDetail {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectReconcileResultReconcileDetail)
    }
}

impl DataTypeAware for VStorageObjectReconcileSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectReconcileSpec)
    }
}

impl DataTypeAware for VStorageObjectStateInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectStateInfo)
    }
}

impl DataTypeAware for VslmTagEntry {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTagEntry)
    }
}

impl DataTypeAware for VslmVClockInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVClockInfo)
    }
}

impl DataTypeAware for VStorageObject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObject)
    }
}

impl DataTypeAware for VStorageObjectSnapshot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectSnapshot)
    }
}

impl DataTypeAware for VStorageObjectSnapshotDetails {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectSnapshotDetails)
    }
}

impl DataTypeAware for VStorageObjectSnapshotInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectSnapshotInfo)
    }
}

impl DataTypeAware for VStorageObjectSnapshotInfoVStorageObjectSnapshot {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectSnapshotInfoVStorageObjectSnapshot)
    }
}

impl DataTypeAware for RetrieveVStorageObjSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RetrieveVStorageObjSpec)
    }
}

impl DataTypeAware for VStorageObjectAssociations {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectAssociations)
    }
}

impl DataTypeAware for VStorageObjectAssociationsVmDiskAssociations {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VStorageObjectAssociationsVmDiskAssociations)
    }
}

impl DataTypeAware for DynamicArray {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DynamicArray)
    }
}

impl DataTypeAware for DynamicProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DynamicProperty)
    }
}

impl DataTypeAware for KeyAnyValue {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KeyAnyValue)
    }
}

impl DataTypeAware for LocalizableMessage {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LocalizableMessage)
    }
}

impl DataTypeAware for LocalizedMethodFault {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LocalizedMethodFault)
    }
}

impl DataTypeAware for PropertyChange {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PropertyChange)
    }
}

impl DataTypeAware for PropertyFilterSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PropertyFilterSpec)
    }
}

impl DataTypeAware for PropertyFilterUpdate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PropertyFilterUpdate)
    }
}

impl DataTypeAware for MissingObject {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MissingObject)
    }
}

impl DataTypeAware for MissingProperty {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MissingProperty)
    }
}

impl DataTypeAware for ObjectContent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ObjectContent)
    }
}

impl DataTypeAware for ObjectSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ObjectSpec)
    }
}

impl DataTypeAware for ObjectUpdate {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ObjectUpdate)
    }
}

impl DataTypeAware for PropertySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PropertySpec)
    }
}

impl DataTypeAware for RetrieveOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RetrieveOptions)
    }
}

impl DataTypeAware for RetrieveResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RetrieveResult)
    }
}

impl DataTypeAware for SelectionSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SelectionSpec)
    }
}

impl DataTypeAware for TraversalSpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TraversalSpec)
    }
}

impl DataTypeAware for UpdateSet {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UpdateSet)
    }
}

impl DataTypeAware for WaitOptions {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::WaitOptions)
    }
}

impl DataTypeAware for VslmAboutInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmAboutInfo)
    }
}

impl DataTypeAware for VslmQueryDatastoreInfoResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmQueryDatastoreInfoResult)
    }
}

impl DataTypeAware for VslmServiceInstanceContent {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmServiceInstanceContent)
    }
}

impl DataTypeAware for VslmTaskInfo {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskInfo)
    }
}

impl DataTypeAware for VslmTaskReason {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReason)
    }
}

impl DataTypeAware for VslmTaskReasonAlarm {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReasonAlarm)
    }
}

impl DataTypeAware for VslmTaskReasonSchedule {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReasonSchedule)
    }
}

impl DataTypeAware for VslmTaskReasonSystem {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReasonSystem)
    }
}

impl DataTypeAware for VslmTaskReasonUser {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReasonUser)
    }
}

impl DataTypeAware for VslmDatastoreSyncStatus {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmDatastoreSyncStatus)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectAssociations {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectAssociations)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectAssociationsVmDiskAssociation {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectAssociationsVmDiskAssociation)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectQueryResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectQueryResult)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectQuerySpec {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectQuerySpec)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectResult)
    }
}

impl DataTypeAware for VslmVsoVStorageObjectSnapshotResult {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmVsoVStorageObjectSnapshotResult)
    }
}

impl DataTypeAware for MethodFault {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MethodFault)
    }
}

impl DataTypeAware for MoTypesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpInvalidProtectionReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpMigrationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpProtectionStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpProtectionSupportTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpSnapshotTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpSyncTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpVSphereDataProtectionCapabilitiesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpVssBackupContextEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpVssBackupTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpCapabilitySupportLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpDrSrmWorkflowEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgencyVmPlacementPolicyVmAntiAffinityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgencyVmPlacementPolicyVmDataAffinityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgentConfigInfoAuthenticationSchemeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgentConfigInfoOvfDiskProvisioningEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgentVmHookVmStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EamObjectRuntimeInfoGoalStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EamObjectRuntimeInfoStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EsxAgentManagerMaintenanceModePolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HooksHookTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SolutionsInvalidReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SolutionsNonComplianceReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SolutionsVmDeploymentOptimizationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SolutionsVmDiskProvisioningEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SolutionsVmPlacementPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmLoggingConfigurationComponentEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmLoggingConfigurationLogLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmDebugManagerKeystoreNameEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmObjectTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmVvolTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmCapabilityOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmCapabilitySchemaCapabilityCategoryEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmLineOfServiceInfoLineOfServiceEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmBuiltinGenericTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmBuiltinTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmCapabilityTimeUnitTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmComplianceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmComplianceResultComplianceTaskStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmHealthStatusForEntityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmAssociateAndApplyPolicyStatusPolicyStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmProfileCategoryEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmSystemCreatedProfileTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmIofilterInfoFilterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmPolicyAssociationVolumeAllocationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmProfileResourceTypeEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PbmVmOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EntityReferenceEntityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SmsTaskStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VpCategoryEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VasaProviderCertificateStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ProviderProfileEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VpTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VasaProviderProfileEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VasaProviderStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VasaAuthenticationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SmsAlarmStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AlarmTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for BackingStoragePoolTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SmsEntityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for BlockDeviceInterfaceEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FileSystemInterfaceEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VasaProfileEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StorageContainerVvolContainerTypeEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FileSystemInterfaceVersionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ThinProvisioningStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationReplicationStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for BatchResultResultEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterComputeResourceHciWorkflowStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterComputeResourceVcsHealthStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ComputeResourceHostSpbmLicenseInfoHostSpbmLicenseStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ComputeResourceNetworkBootModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ConfigSpecOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DatastoreAccessibleEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DatastoreSectorFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DatastoreSummaryMaintenanceModeStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DiagnosticManagerLogCreatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DiagnosticManagerLogFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostInfrastructureTrafficClassEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchNetworkResourceControlVersionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchNicTeamingPolicyModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchProductSpecOperationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DrsInjectorWorkloadCorrelationStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FolderDesiredHostStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FolderExternallyManagedFolderTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationVmStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for QuiesceModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HealthUpdateInfoComponentTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSystemConnectionStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCryptoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSystemPowerStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSystemRemediationStateStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostStandbyModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HttpNfcLeaseManifestEntryChecksumTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HttpNfcLeaseModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HttpNfcLeaseStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for IoFilterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for IoFilterOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LatencySensitivitySensitivityLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseFeatureInfoUnitEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseFeatureInfoSourceRestrictionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseFeatureInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostLicensableResourceKeyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseManagerLicenseKeyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseManagerStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseReservationInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ManagedEntityStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for OvfConsumerOstNodeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for OvfCreateImportSpecParamsDiskProvisioningTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PerfSummaryTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PerfStatsTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PerformanceManagerUnitEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PerfFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PlaceVmsXClusterSpecPlacementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ResourceConfigSpecScaleSharesBehaviorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMotionCompatibilityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ValidateMigrationTestTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SessionManagerGenericServiceTicketTicketTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SessionManagerHttpServiceRequestSpecMethodEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SharesLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SimpleCommandEncodingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StorageIormThresholdModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for TaskFilterSpecRecursionOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for TaskFilterSpecTimeOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for TaskInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualAppVAppStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskAdapterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineAppHeartbeatStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConnectionStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineCryptoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFaultToleranceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFaultToleranceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineMovePriorityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineNeedSecondaryReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachinePowerStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineRecordReplayStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineTicketTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanCompositeConstraintConjoinerEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanMassCollectorObjectCollectionEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPropertyConstraintComparatorEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanUpgradeSystemUpgradeHistoryDiskGroupOpTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ActionParameterEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AlarmFilterSpecAlarmTypeByEntityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AlarmFilterSpecAlarmTypeByTriggerEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EventAlarmExpressionComparisonOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for MetricAlarmOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StateAlarmOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ActionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterPowerStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterCryptoConfigInfoCryptoModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasAamNodeStateDasStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasConfigInfoHbDatastoreCandidateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasConfigInfoServiceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasConfigInfoVmMonitoringStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasFdmAvailabilityStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DasVmPriorityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasVmSettingsIsolationResponseEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterDasVmSettingsRestartPriorityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DpmBehaviorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DrsBehaviorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DrsRecommendationReasonCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterHostInfraUpdateHaModeActionOperationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPowerOperationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterInfraUpdateHaConfigInfoBehaviorTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterInfraUpdateHaConfigInfoRemediationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PlacementSpecPlacementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterPowerOnVmOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for RecommendationReasonCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for RecommendationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanStorageComplianceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterSystemVMsConfigInfoDeploymentModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimClusterVsanStretchedClusterConfigIssueEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterVmComponentProtectionSettingsStorageVmReactionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterVmComponentProtectionSettingsVmReactionOnApdClearedEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterVmReadinessReadyConditionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanBaselinePreferenceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanCapabilityStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanCapabilityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanCapabilityType90Enum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanClusterHealthActionIdEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanClusterHealthCategoryEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDatastoreTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimClusterVsanDiskGroupCreationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoInsightInstanceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIscsiLunStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIscsiLunRuntimeStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIscsiTargetAuthTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIscsiTargetServiceProcessStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanObjectTypeEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanObjectTypeEnum90Enum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfDiagnosticQueryTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfStatsUnitTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfStatsTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfSummaryTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfThresholdDirectionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanRelayoutObjectsErrorCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSpaceReportingEntityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHealthLogLevelEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CnsClusterFlavorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CnsClusterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CnsKubernetesEntityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for MetricFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for MetricTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for QuerySelectionNameTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CnsVolumeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsFilterOnFailureEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvPortStatusVmDirectPathGen2InactiveReasonNetworkEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvPortStatusVmDirectPathGen2InactiveReasonOtherEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualPortgroupBackingTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualPortgroupMetaTagNameEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualPortgroupPortgroupTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EntityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EntityImportTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsFilterSpecLinkConfigEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsFilterSpecLinkStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDvsConfigSpecSwitchModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDistributedVirtualSwitchManagerFailoverReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDistributedVirtualSwitchManagerFailoverStageEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberHostComponentStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberHostUplinkStateStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchHostMemberTransportZoneTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DistributedVirtualSwitchPortConnecteeConnecteeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsNetworkRuleDirectionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsLacpApiVersionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsLacpLoadBalanceAlgorithmEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsMacLimitPolicyTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsMulticastFilteringModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmwareDistributedVirtualSwitchPvlanPortTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsTeamingMatchStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareUplinkLacpModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareUplinkLacpTimeoutModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsVspanSessionEncapTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VMwareDvsVspanSessionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CryptoManagerHostKeyManagementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CryptoManagerKmipCryptoKeyStatusKeyUnavailableReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for KmipClusterInfoKeyTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for KmipClusterInfoKmsManagementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CustomizationFailedReasonCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DvsEventPortBlockStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EventEventSeverityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EventCategoryEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for EventFilterSpecRecursionOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDasErrorEventHostDasErrorReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDisconnectedEventReasonCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmDasBeingResetEventReasonCodeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmFailedStartingSecondaryEventFailureReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmShutdownOnIsolationEventOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AffinityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AgentInstallFailedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CannotEnableVmcpForClusterReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CannotMoveFaultToleranceVmMoveTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CannotPowerOffVmInClusterOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CannotUseNetworkReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DasConfigFaultDasConfigFaultReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DeviceNotSupportedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DisallowedChangeByServiceDisallowedChangeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FtIssuesOnHostHostSelectionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostHasComponentFailureHostComponentTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostIncompatibleForFaultToleranceReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostIncompatibleForRecordReplayReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for IncompatibleHostForVmReplicationIncompatibleReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for InvalidDasConfigArgumentEntryForInvalidArgumentEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for InvalidProfileReferenceHostReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LicenseAssignmentFailedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NotSupportedDeviceForFtDeviceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NumVirtualCpusIncompatibleReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for QuarantineModeFaultFaultTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationDiskConfigFaultReasonForFaultEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationVmConfigFaultReasonForFaultEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationVmFaultReasonForFaultEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ReplicationVmInProgressFaultActivityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ThirdPartyLicenseAssignmentFailedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VFlashModuleNotSupportedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmFaultToleranceConfigIssueReasonForIssueEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VmFaultToleranceInvalidFileBackingDeviceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for WillLoseHaProtectionResolutionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostActiveDirectoryAuthenticationCertificateDigestEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostActiveDirectoryInfoDomainMembershipStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AutoStartActionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AutoStartWaitHeartbeatSettingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostBiosInfoFirmwareTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCapabilityFtUnsupportedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostReplayUnsupportedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCapabilityUnmapMethodSupportedEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCapabilityVmDirectPathGen2UnsupportedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCertificateManagerCertificateInfoCertificateStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCertificateManagerCertificateKindEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostConfigChangeModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostConfigChangeOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostConfigChangeOwnerEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCpuPackageVendorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCpuPowerManagementInfoPolicyTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostCpuSchedulerInfoCpuSchedulerPolicyInfoEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDateTimeInfoProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DiagnosticPartitionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DiagnosticPartitionStorageTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDigestInfoDigestMethodTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDigestVerificationSettingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDiskPartitionInfoPartitionFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostDiskPartitionInfoTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFeatureVersionKeyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FibreChannelPortTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for FileSystemMountInfoVStorageSupportStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFileSystemVolumeFileSystemTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFirewallSystemRuleSetIdEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFirewallSystemServiceNameEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFruFruTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostGraphicsConfigGraphicsTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostGraphicsConfigSharedPassthruAssignmentPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostGraphicsConfigVgpuModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostGraphicsInfoGraphicsTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostGraphicsInfoVgpuModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostHardwareElementStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostAccessModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostLockdownModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostImageAcceptanceLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaChapAuthenticationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaDigestTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for InternetScsiSnsDiscoveryMethodEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SlpDiscoveryMethodEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaIscsiIpv6AddressAddressConfigurationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaIscsiIpv6AddressIPv6AddressOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaNetworkBindingSupportTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostInternetScsiHbaStaticTargetTargetDiscoveryMethodEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostIpConfigIpV6AddressConfigTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostIpConfigIpV6AddressStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for IscsiPortInfoPathStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LinkDiscoveryProtocolConfigOperationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for LinkDiscoveryProtocolConfigProtocolTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerFileTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostLowLevelProvisioningManagerReloadTargetEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMaintenanceSpecPurposeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineMemoryAllocationPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMemoryTierFlagsEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMemoryTierTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMemoryTieringTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMountModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMountInfoInaccessibleReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostMountInfoMountFailedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for MultipathStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNasVolumeSecurityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNetStackInstanceCongestionControlAlgorithmTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNetStackInstanceSystemStackKeyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNumericSensorHealthStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNumericSensorTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNvdimmHealthInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmInterleaveSetStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNamespaceDetailsHealthStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNamespaceDetailsStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNamespaceHealthStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNamespaceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmNamespaceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NvdimmRangeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNvmeDiscoveryLogSubsystemTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNvmeDiscoveryLogTransportRequirementsEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNvmeTransportParametersNvmeAddressFamilyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostNvmeTransportTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostOpaqueSwitchOpaqueSwitchStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPartialMaintenanceModeIdEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPartialMaintenanceModeStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPatchManagerInstallStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPatchManagerIntegrityStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPatchManagerReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PhysicalNicResourcePoolSchedulerDisallowedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PhysicalNicVmDirectPathGen2SupportedModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PortGroupConnecteeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProtocolEndpointPeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProtocolEndpointProtocolEndpointTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostPtpConfigDeviceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostQualifiedNameTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostRdmaDeviceConnectionStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for RdmaProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFirewallRuleDirectionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFirewallRulePortTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostFirewallRuleProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostRuntimeInfoNetStackInstanceRuntimeInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostRuntimeInfoStateEncryptionInfoProtectionModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostRuntimeInfoStatelessNvdsMigrationStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiDiskTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiLunDescriptorQualityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DeviceProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiLunLunReservationStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiLunTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiLunStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScsiLunVStorageSupportStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostServicePolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSevInfoSevStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSgxInfoFlcModesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSgxInfoSgxStatesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSgxRegistrationInfoRegistrationStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSgxRegistrationInfoRegistrationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSnmpAgentCapabilityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SoftwarePackageConstraintEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for SoftwarePackageVibTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostStorageProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostSystemIdentificationInfoIdentifierEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostTdxInfoTdxStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostTpmAttestationInfoAcceptanceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostTrustAuthorityAttestationInfoAttestationStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostUnresolvedVmfsExtentUnresolvedReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostUnresolvedVmfsResolutionSpecVmfsUuidResolutionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostVirtualNicManagerNicTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostVmciAccessManagerModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostVmfsVolumeUnmapBandwidthPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostVmfsVolumeUnmapPriorityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanControllerTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskBalanceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanEncryptionIssueEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostQueryCheckLimitsOptionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoInsightStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanObjectHealthStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPeerHostConnectivityHealthStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSmartParameterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NetIpConfigInfoIpAddressOriginEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NetIpConfigInfoIpAddressStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NetIpStackInfoEntryTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NetIpStackInfoPreferenceEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for NetBiosConfigInfoModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ArrayUpdateOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ComplianceResultStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ProfileNumericComparatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ProfileParameterMetadataRelationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterProfileServiceTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ProfileExecuteResultStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileValidationFailureInfoUpdateTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileValidationStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileManagerAnswerFileStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ApplyHostProfileConfigurationResultStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileManagerCompositionResultResultElementStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileManagerCompositionValidationResultResultElementStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HostProfileManagerTaskListRequirementEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for AnswerFileValidationInfoStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for DayOfWeekEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for WeekOfMonthEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PlacementAffinityRuleRuleScopeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PlacementAffinityRuleRuleTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StorageDrsPodConfigInfoBehaviorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StorageDrsSpaceLoadBalanceConfigSpaceThresholdModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StoragePlacementSpecPlacementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskRuleSpecRuleTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VAppCloneSpecProvisioningTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VAppAutoStartActionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VAppIpAssignmentInfoAllocationSchemesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VAppIpAssignmentInfoIpAllocationPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VAppIpAssignmentInfoProtocolsEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VchaStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VchaClusterModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VchaClusterStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VchaNodeRoleEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VchaNodeStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineBootOptionsNetworkBootProtocolTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineCertThumbprintHashAlgorithmEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineCloneSpecTpmProvisionPolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConfigInfoNpivWwnTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConfigInfoSwapPlacementTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConfigSpecEncryptedFtModesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConfigSpecEncryptedVMotionModesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineConfigSpecNpivWwnOpEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachinePowerOpTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineStandbyActionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonOtherEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineDeviceRuntimeInfoVirtualEthernetCardRuntimeStateVmDirectPathGen2InactiveReasonVmEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFileLayoutExFileTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineHtSharingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFlagInfoMonitorTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachinePowerOffBehaviorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFlagInfoVirtualExecUsageEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineFlagInfoVirtualMmuUsageEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineForkConfigInfoChildTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestInfoAppStateTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestInfoCustomizationStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineGuestStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineToolsInstallTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineToolsRunningStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineToolsStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineToolsVersionStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestOsDescriptorFirmwareTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineGuestOsFamilyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineGuestOsIdentifierEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestOsDescriptorSupportLevelEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestQuiesceEndGuestQuiesceErrorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadataOpEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineMetadataManagerVmMetadataOwnerOwnerEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineRelocateDiskMoveOptionsEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineRelocateTransformationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScheduledHardwareUpgradeInfoHardwareUpgradePolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ScheduledHardwareUpgradeInfoHardwareUpgradeStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineScsiPassthroughTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineSgxInfoFlcModesEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineTargetInfoConfigurationTagEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for UpgradePolicyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineUsbInfoFamilyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineUsbInfoSpeedEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVendorDeviceGroupInfoComponentDeviceInfoComponentTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVgpuProfileInfoProfileClassEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVgpuProfileInfoProfileSharingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVirtualDeviceSwapDeviceSwapStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualHardwareMotherboardLayoutEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVirtualPMemSnapshotModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineWindowsQuiesceSpecVssBackupContextEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CheckTestTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CustomizationNetBiosModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CustomizationLicenseDataModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for CustomizationSysprepRebootOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceConnectInfoMigrateConnectOpEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceConnectInfoStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceFileExtensionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceUriBackingOptionDirectionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceConfigSpecChangeModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceConfigSpecFileOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDeviceConfigSpecOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskDeltaDiskFormatEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskDeltaDiskFormatVariantEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskSharingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskVFlashCacheConfigInfoCacheConsistencyTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskVFlashCacheConfigInfoCacheModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskCompatibilityModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualDiskModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualEthernetCardLegacyNetworkDeviceNameEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualEthernetCardMacTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualNvmeControllerSharingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualPointingDeviceHostChoiceEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualScsiSharingEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualSerialPortEndPointEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceActionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceDirectionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVmciDeviceProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualMachineVideoCardUse3DRendererEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualVmxnet3StrictLatencyConfigDisableOffloadEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VirtualVmxnet3VrdmaOptionDeviceProtocolsEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestFileTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for GuestRegKeyWowSpecEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanCapacityReservationStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileServiceConfigOpTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileServicePreflightCheckScopeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileServiceVmStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileShareAccessTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileShareManagingEntityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileShareNfsSecTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileProtocolEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanFileShareSmbEncryptionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for HciMeshClientOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoDiagnosticsFailedCheckTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoDiagnosticsInstanceEventTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoDiagnosticsInstanceStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoDiagnosticsTargetTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoLatencyTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanLifecycleCheckOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanLifecycleClusterTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanLifecyclePreCheckTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanModeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanMountPrecheckTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPerfsvcRemediateActionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PrecheckDatastoreSourceOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanRemoteVcLinkTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for RemoteVsanNetworkTopologyEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanResourceCheckComponentTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ResourceCheckDedupStoreHealthStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanResourceCheckStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSnapshotCreatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSnapshotTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanAnalyticsEventLocationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanAnalyticsEventSnapshotTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanAnalyticsEventTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanConfigTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskCompatibilityTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHealthPerspectiveEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHealthPerspective90Enum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHealthStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHealthThresholdTargetEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanIoTripAnalyzerRecurrenceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanPolicyRegulationCheckOpEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanVsanScanObjectsIssueTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanServiceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSiteLocationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSnapHealthTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSnapStatsExpirationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSnapVmMembershipChangeStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSyncReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanSyncStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanUpdateItemImpactTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanUpdateItemTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanVsanVcsaDeploymentPhaseEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanVibTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanXvcQueryCriteriaOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanXvcQueryFilterOperatorEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanClusterComplianceResourceCheckStatusTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanClusterVsanManagedStorageTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ClusterPowerStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanComplianceStatusEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostDecommissionModeObjectActionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanHostDiskMappingCreationTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostDiskResultStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanEncryptionOperationEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanEncryptionTransitionStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostHealthStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostNodeStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanHostTrafficTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VimVsanHostTrafficType90Enum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ServerNodeTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostStatsTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for StoragePoolDiskTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for TrimDiskTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskEvacReasonEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskTrimOptionEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskgroupCapabilityEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanDiskIssueTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostWipeDiskEligibleEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VsanHostWipeDiskStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for BaseConfigInfoDiskFileBackingInfoProvisioningTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmDiskInfoFlagEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VStorageObjectConsumptionTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmVStorageObjectControlFlagEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for PropertyChangeOpEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for ObjectUpdateKindEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmTaskInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmEventTypeEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmEventVslmEventInfoStateEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmVsoVStorageObjectQuerySpecQueryFieldEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for VslmVsoVStorageObjectQuerySpecQueryOperatorEnumEnum {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Str
    }
}

impl DataTypeAware for Box<dyn super::traits::DataObjectTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DataObject)
    }
}

impl DataTypeAware for Box<dyn super::traits::AgencyScopeTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyScope)
    }
}

impl DataTypeAware for Box<dyn super::traits::AgentSslTrustTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentSslTrust)
    }
}

impl DataTypeAware for Box<dyn super::traits::AgentStoragePolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentStoragePolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::EamObjectRuntimeInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EamObjectRuntimeInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::IssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Issue)
    }
}

impl DataTypeAware for Box<dyn super::traits::AgencyIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgencyIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::AgentIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AgentIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VibIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VibNotInstalledTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibNotInstalled)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmDeployedTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmDeployed)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmPoweredOffTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmPoweredOff)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmNotDeployedTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmNotDeployed)
    }
}

impl DataTypeAware for Box<dyn super::traits::NoAgentVmDatastoreTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoAgentVmDatastore)
    }
}

impl DataTypeAware for Box<dyn super::traits::NoAgentVmNetworkTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NoAgentVmNetwork)
    }
}

impl DataTypeAware for Box<dyn super::traits::PersonalityAgentPmIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgentPmIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterAgentAgentIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentAgentIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterAgentVmIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterAgentVmPoweredOffTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmPoweredOff)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterAgentVmNotDeployedTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAgentVmNotDeployed)
    }
}

impl DataTypeAware for Box<dyn super::traits::IntegrityAgencyVumIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IntegrityAgencyVumIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::PersonalityAgencyPmIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyPmIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::PersonalityAgencyDepotIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PersonalityAgencyDepotIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::SolutionsHookAcknowledgeConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsHookAcknowledgeConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::SolutionsStoragePolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsStoragePolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::SolutionsTypeSpecificSolutionConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsTypeSpecificSolutionConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::SolutionsVmSourceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SolutionsVmSource)
    }
}

impl DataTypeAware for Box<dyn super::traits::VibVibServicesSslTrustTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VibVibServicesSslTrust)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmCapabilityTypeInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityTypeInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmLineOfServiceInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmLineOfServiceInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmPlacementMatchingResourcesTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementMatchingResources)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmPlacementRequirementTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmPlacementRequirement)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmCapabilityConstraintsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityConstraints)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmProfileTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmProfile)
    }
}

impl DataTypeAware for Box<dyn super::traits::PbmCapabilityProfileTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PbmCapabilityProfile)
    }
}

impl DataTypeAware for Box<dyn super::traits::SmsProviderInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsProviderInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::SmsProviderSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SmsProviderSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::StoragePortTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::StoragePort)
    }
}

impl DataTypeAware for Box<dyn super::traits::DeviceIdTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DeviceId)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineIdTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineId)
    }
}

impl DataTypeAware for Box<dyn super::traits::FailoverParamTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FailoverParam)
    }
}

impl DataTypeAware for Box<dyn super::traits::GroupInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::GroupOperationResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GroupOperationResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::TargetGroupMemberInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TargetGroupMemberInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterComputeResourceValidationResultBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterComputeResourceValidationResultBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::ComputeResourceConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ComputeResourceConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ComputeResourceSummaryTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ComputeResourceSummary)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomFieldValueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomFieldValue)
    }
}

impl DataTypeAware for Box<dyn super::traits::DatastoreInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DatastoreInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::DescriptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Description)
    }
}

impl DataTypeAware for Box<dyn super::traits::ElementDescriptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ElementDescription)
    }
}

impl DataTypeAware for Box<dyn super::traits::TypeDescriptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TypeDescription)
    }
}

impl DataTypeAware for Box<dyn super::traits::DirectPathProfileManagerCapacityQuerySpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityQuerySpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::DirectPathProfileManagerCapacityResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerCapacityResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::DirectPathProfileManagerDirectPathConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerDirectPathConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::DirectPathProfileManagerTargetEntityTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DirectPathProfileManagerTargetEntity)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsFeatureCapabilityTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFeatureCapability)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsHealthCheckConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHealthCheckConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::VMwareDvsHealthCheckConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VMwareDvsHealthCheckConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsHealthCheckCapabilityTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsHealthCheckCapability)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsUplinkPortPolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsUplinkPortPolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::HbrReplicationTargetSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HbrReplicationTargetSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ImportSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ImportSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::InheritablePolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::InheritablePolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsFilterConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsTrafficFilterConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsTrafficFilterConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmwareDistributedVirtualSwitchVlanSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmwareDistributedVirtualSwitchVlanSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::IoFilterInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::IoFilterManagerSslTrustTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IoFilterManagerSslTrust)
    }
}

impl DataTypeAware for Box<dyn super::traits::LicenseSourceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::LicenseSource)
    }
}

impl DataTypeAware for Box<dyn super::traits::NegatableExpressionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NegatableExpression)
    }
}

impl DataTypeAware for Box<dyn super::traits::IpAddressTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::IpAddress)
    }
}

impl DataTypeAware for Box<dyn super::traits::MacAddressTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MacAddress)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsIpPortTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsIpPort)
    }
}

impl DataTypeAware for Box<dyn super::traits::NetworkSummaryTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetworkSummary)
    }
}

impl DataTypeAware for Box<dyn super::traits::OvfManagerCommonParamsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfManagerCommonParams)
    }
}

impl DataTypeAware for Box<dyn super::traits::OvfCreateImportSpecParamsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OvfCreateImportSpecParams)
    }
}

impl DataTypeAware for Box<dyn super::traits::PerfEntityMetricBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfEntityMetricBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::PerfMetricSeriesTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PerfMetricSeries)
    }
}

impl DataTypeAware for Box<dyn super::traits::ResourcePoolSummaryTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ResourcePoolSummary)
    }
}

impl DataTypeAware for Box<dyn super::traits::SddcBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SddcBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::SelectionSetTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SelectionSet)
    }
}

impl DataTypeAware for Box<dyn super::traits::ServiceLocatorCredentialTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ServiceLocatorCredential)
    }
}

impl DataTypeAware for Box<dyn super::traits::SessionManagerServiceRequestSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SessionManagerServiceRequestSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::TaskManagerTaskViewSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskManagerTaskViewSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::TaskReasonTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskReason)
    }
}

impl DataTypeAware for Box<dyn super::traits::UserSearchResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::UserSearchResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDiskSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::FileBackedVirtualDiskSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileBackedVirtualDiskSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineConnectionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineConnection)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanComparatorTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanComparator)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanResourceConstraintTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceConstraint)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanUpgradeSystemPreflightCheckIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemPreflightCheckIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanUpgradeSystemPreflightCheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemPreflightCheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanUpgradeSystemUpgradeHistoryItemTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeHistoryItem)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanUpgradeSystemUpgradeStatusTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanUpgradeSystemUpgradeStatus)
    }
}

impl DataTypeAware for Box<dyn super::traits::ActionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::Action)
    }
}

impl DataTypeAware for Box<dyn super::traits::AlarmActionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmAction)
    }
}

impl DataTypeAware for Box<dyn super::traits::AlarmExpressionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmExpression)
    }
}

impl DataTypeAware for Box<dyn super::traits::AlarmSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AlarmSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterActionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterAction)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDasAdmissionControlInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdmissionControlInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDasAdmissionControlPolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdmissionControlPolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDasAdvancedRuntimeInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasAdvancedRuntimeInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDasDataTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasData)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDasHostInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDasHostInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterDrsFaultsFaultsByVmTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterDrsFaultsFaultsByVm)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterGroupInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterGroupInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterRuleInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterRuleInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterSlotPolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterSlotPolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanClusterHealthLinkBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthLinkBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanClusterHealthResultBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterHealthResultBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::VimClusterVsanFaultDomainSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VimClusterVsanFaultDomainSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHealthActionBaseTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHealthActionBase)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanIscsiLunCommonInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiLunCommonInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanIscsiTargetBasicInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetBasicInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanIscsiTargetCommonInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetCommonInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanIscsiTargetServiceConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiTargetServiceConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsAccessControlSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsAccessControlSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsBackingObjectDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBackingObjectDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsFileBackingDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsFileBackingDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsBaseCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsBaseCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsFileCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsFileCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsEntityMetadataTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsEntityMetadata)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsQueryFilterTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsQueryFilter)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsVolumeOperationResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeOperationResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsVolumeRelocateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeRelocateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::CnsVolumeSourceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CnsVolumeSource)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvPortSettingTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortSetting)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvPortgroupPolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvPortgroupPolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::DistributedVirtualSwitchManagerHostDvsFilterSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchManagerHostDvsFilterSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsFilterSpecConnecteeSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecConnecteeSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsFilterSpecVlanSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsFilterSpecVlanSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::DistributedVirtualSwitchHostMemberBackingTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DistributedVirtualSwitchHostMemberBacking)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostMemberHealthCheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemberHealthCheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostMemberUplinkHealthCheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMemberUplinkHealthCheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsNetworkRuleActionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkRuleAction)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsNetworkRuleQualifierTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsNetworkRuleQualifier)
    }
}

impl DataTypeAware for Box<dyn super::traits::CryptoManagerKmipCryptoKeyStatusKeyInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoManagerKmipCryptoKeyStatusKeyInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::CryptoSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::CryptoSpecNoOpTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CryptoSpecNoOp)
    }
}

impl DataTypeAware for Box<dyn super::traits::KmipClusterInfoKeyInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipClusterInfoKeyInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::KmipServerSpecKeySpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::KmipServerSpecKeySpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::EventArgumentTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventArgument)
    }
}

impl DataTypeAware for Box<dyn super::traits::EntityEventArgumentTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityEventArgument)
    }
}

impl DataTypeAware for Box<dyn super::traits::EventManagerEventViewSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EventManagerEventViewSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostAuthenticationStoreInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAuthenticationStoreInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDirectoryStoreInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDirectoryStoreInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDatastoreConnectInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDatastoreConnectInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostConnectInfoNetworkInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostConnectInfoNetworkInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDataTransportConnectionInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDataTransportConnectionInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::FileInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmConfigFileInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::FileQueryTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FileQuery)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmConfigFileQueryTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigFileQuery)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDeviceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDevice)
    }
}

impl DataTypeAware for Box<dyn super::traits::ScsiLunTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScsiLun)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDigestInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDigestInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostDnsConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostDnsConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostFileSystemVolumeTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFileSystemVolume)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostHardwareElementInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHardwareElementInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostHbaCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHbaCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostHostBusAdapterTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostHostBusAdapter)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostFibreChannelHbaTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelHba)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostIpConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostIpRouteConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostIpRouteConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostAccountSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostAccountSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostMultipathInfoLogicalUnitPolicyTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostMultipathInfoLogicalUnitPolicy)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostNvmeSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostNvmeTransportParametersTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostNvmeTransportParameters)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostPciPassthruConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPciPassthruConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostPciPassthruInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostPciPassthruInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::PhysicalNicHintTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PhysicalNicHint)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostRdmaDeviceBackingTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostRdmaDeviceBacking)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostSriovDevicePoolInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSriovDevicePoolInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostSystemSwapConfigurationSystemSwapOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostSystemSwapConfigurationSystemSwapOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostTargetTransportTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTargetTransport)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostFibreChannelTargetTransportTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostFibreChannelTargetTransport)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostTpmEventDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmEventDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostTpmBootSecurityOptionEventDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostTpmBootSecurityOptionEventDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostVirtualSwitchBridgeTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostVirtualSwitchBridge)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmfsDatastoreBaseOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreBaseOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmfsDatastoreSingleExtentOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreSingleExtentOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmfsDatastoreSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmfsDatastoreSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHclCommonDeviceInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHclCommonDeviceInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::NetBiosConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NetBiosConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ArrayUpdateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ArrayUpdateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::OptionTypeTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionType)
    }
}

impl DataTypeAware for Box<dyn super::traits::OptionValueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::OptionValue)
    }
}

impl DataTypeAware for Box<dyn super::traits::ApplyProfileTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ApplyProfile)
    }
}

impl DataTypeAware for Box<dyn super::traits::DvsVNicProfileTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DvsVNicProfile)
    }
}

impl DataTypeAware for Box<dyn super::traits::PortGroupProfileTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PortGroupProfile)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfileExpressionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExpression)
    }
}

impl DataTypeAware for Box<dyn super::traits::PolicyOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::PolicyOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfilePolicyOptionMetadataTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfilePolicyOptionMetadata)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfileConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfileCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfileSerializedCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileSerializedCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterProfileCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ClusterProfileConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ClusterProfileConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostProfileConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfileConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::ProfileExecuteResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ProfileExecuteResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::AnswerFileCreateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::AnswerFileCreateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::HostProfilesEntityCustomizationsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HostProfilesEntityCustomizations)
    }
}

impl DataTypeAware for Box<dyn super::traits::ScheduledTaskSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::ScheduledTaskSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::TaskSchedulerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::TaskScheduler)
    }
}

impl DataTypeAware for Box<dyn super::traits::RecurrentTaskSchedulerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::RecurrentTaskScheduler)
    }
}

impl DataTypeAware for Box<dyn super::traits::HourlyTaskSchedulerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::HourlyTaskScheduler)
    }
}

impl DataTypeAware for Box<dyn super::traits::DailyTaskSchedulerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::DailyTaskScheduler)
    }
}

impl DataTypeAware for Box<dyn super::traits::MonthlyTaskSchedulerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::MonthlyTaskScheduler)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VmConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VmConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::NodeDeploymentSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NodeDeploymentSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::NodeNetworkSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::NodeNetworkSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineBaseIndependentFilterSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBaseIndependentFilterSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineBootOptionsBootableDeviceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineBootOptionsBootableDevice)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineDeviceRuntimeInfoDeviceRuntimeStateTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDeviceRuntimeInfoDeviceRuntimeState)
    }
}

impl DataTypeAware for Box<dyn super::traits::FaultToleranceConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultToleranceConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineGuestQuiesceSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineGuestQuiesceSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineProfileSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineProfileSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineSriovDevicePoolInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineSriovDevicePoolInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineTargetInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineTargetInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineDiskDeviceInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineDiskDeviceInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachinePciPassthroughInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachinePciPassthroughInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualMachineVirtualDeviceGroupsDeviceGroupTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualMachineVirtualDeviceGroupsDeviceGroup)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomizationIdentitySettingsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIdentitySettings)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomizationIpGeneratorTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpGenerator)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomizationIpV6GeneratorTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationIpV6Generator)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomizationNameTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationName)
    }
}

impl DataTypeAware for Box<dyn super::traits::CustomizationOptionsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::CustomizationOptions)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevice)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualControllerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualController)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualSataControllerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSataController)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualScsiControllerTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiController)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualEthernetCardTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCard)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualVmxnetTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualVmxnet3Trait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualSoundCardTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCard)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceDeviceBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceDeviceBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDiskRawDiskVer2BackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskVer2BackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceFileBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceFileBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDevicePipeBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePipeBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceRemoteDeviceBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceRemoteDeviceBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceUriBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceUriBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualPciPassthroughPluginBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughPluginBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceBusSlotInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBusSlotInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDevicePciBusSlotInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePciBusSlotInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualControllerOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualControllerOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualSataControllerOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSataControllerOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualScsiControllerOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualScsiControllerOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualEthernetCardOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualEthernetCardOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualVmxnetOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnetOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualVmxnet3OptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualVmxnet3Option)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualSoundCardOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualSoundCardOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceDeviceBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceDeviceBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDiskRawDiskVer2BackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDiskRawDiskVer2BackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceFileBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceFileBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDevicePipeBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDevicePipeBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceRemoteDeviceBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceRemoteDeviceBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceUriBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceUriBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualPciPassthroughPluginBackingOptionTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualPciPassthroughPluginBackingOption)
    }
}

impl DataTypeAware for Box<dyn super::traits::VirtualDeviceConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VirtualDeviceConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::GuestAuthSubjectTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthSubject)
    }
}

impl DataTypeAware for Box<dyn super::traits::GuestFileAttributesTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestFileAttributes)
    }
}

impl DataTypeAware for Box<dyn super::traits::GuestAuthenticationTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestAuthentication)
    }
}

impl DataTypeAware for Box<dyn super::traits::GuestProgramSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestProgramSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::GuestRegValueDataSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::GuestRegValueDataSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::FaultDomainIdTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::FaultDomainId)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanDataEfficiencyConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDataEfficiencyConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanDatastoreConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanDatastoreSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDatastoreSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanDirectoryServerConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDirectoryServerConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::EntityResourceCheckDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::EntityResourceCheckDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanDiskResourceCheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanDiskResourceCheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanResourceCheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanResourceCheckComponentResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckComponentResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanMountPrecheckItemTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckItem)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanMountPrecheckResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanMountPrecheckResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanRemoteVcInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanRemoteVcInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanResourceCheckTaskDetailsTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanResourceCheckTaskDetails)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanIscsiVipConfigSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanIscsiVipConfigSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanConfigBaseIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanConfigBaseIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanNetworkConfigBaseIssueTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanNetworkConfigBaseIssue)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanClusterConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanClusterConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHostConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHostConfigInfoNetworkInfoPortConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostConfigInfoNetworkInfoPortConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHostDiskResultTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostDiskResult)
    }
}

impl DataTypeAware for Box<dyn super::traits::VsanHostIpConfigTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VsanHostIpConfig)
    }
}

impl DataTypeAware for Box<dyn super::traits::BaseConfigInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::BaseConfigInfoBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::BaseConfigInfoFileBackingInfoTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::BaseConfigInfoFileBackingInfo)
    }
}

impl DataTypeAware for Box<dyn super::traits::VslmCreateSpecBackingSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmCreateSpecBackingSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VslmMigrateSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmMigrateSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::SelectionSpecTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::SelectionSpec)
    }
}

impl DataTypeAware for Box<dyn super::traits::VslmTaskReasonTrait> {
    fn data_type() -> ApiFieldType {
        ApiFieldType::Object(StructType::VslmTaskReason)
    }
}

