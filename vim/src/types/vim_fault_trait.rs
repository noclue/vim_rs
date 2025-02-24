use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The common base type for all virtual infrastructure management
/// exceptions.
pub trait VimFaultTrait : super::method_fault_trait::MethodFaultTrait {
}
impl<'s> serde::Serialize for dyn VimFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VimFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VimFaultVisitor)
            }
        }

struct VimFaultVisitor;

impl<'de> de::Visitor<'de> for VimFaultVisitor {
    type Value = Box<dyn VimFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VimFaultTrait JSON object with a _typeName field")
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

impl VimFaultTrait for VimFault {
}
impl VimFaultTrait for ActiveDirectoryFault {
}
impl VimFaultTrait for DomainNotFound {
}
impl VimFaultTrait for InvalidCamServer {
}
impl VimFaultTrait for CamServerRefusedConnection {
}
impl VimFaultTrait for InvalidCamCertificate {
}
impl VimFaultTrait for NoPermissionOnAd {
}
impl VimFaultTrait for NonAdUserRequired {
}
impl VimFaultTrait for AlreadyExists {
}
impl VimFaultTrait for AlreadyUpgraded {
}
impl VimFaultTrait for AnswerFileUpdateFailed {
}
impl VimFaultTrait for AuthMinimumAdminPermission {
}
impl VimFaultTrait for CannotAccessLocalSource {
}
impl VimFaultTrait for CannotDisconnectHostWithFaultToleranceVm {
}
impl VimFaultTrait for CannotEnableVmcpForCluster {
}
impl VimFaultTrait for CannotMoveFaultToleranceVm {
}
impl VimFaultTrait for CannotMoveHostWithFaultToleranceVm {
}
impl VimFaultTrait for CannotPlaceWithoutPrerequisiteMoves {
}
impl VimFaultTrait for ConcurrentAccess {
}
impl VimFaultTrait for CustomizationFault {
}
impl VimFaultTrait for CannotDecryptPasswords {
}
impl VimFaultTrait for CustomizationPending {
}
impl VimFaultTrait for IpHostnameGeneratorError {
}
impl VimFaultTrait for LinuxVolumeNotClean {
}
impl VimFaultTrait for MissingLinuxCustResources {
}
impl VimFaultTrait for MissingWindowsCustResources {
}
impl VimFaultTrait for MountError {
}
impl VimFaultTrait for NicSettingMismatch {
}
impl VimFaultTrait for NoDisksToCustomize {
}
impl VimFaultTrait for UncustomizableGuest {
}
impl VimFaultTrait for UnexpectedCustomizationFault {
}
impl VimFaultTrait for VolumeEditorError {
}
impl VimFaultTrait for DasConfigFault {
}
impl VimFaultTrait for DrsDisabledOnVm {
}
impl VimFaultTrait for DuplicateName {
}
impl VimFaultTrait for DvsFault {
}
impl VimFaultTrait for BackupBlobReadFailure {
}
impl VimFaultTrait for BackupBlobWriteFailure {
}
impl VimFaultTrait for CollectorAddressUnset {
}
impl VimFaultTrait for ConflictingConfiguration {
}
impl VimFaultTrait for DvsApplyOperationFault {
}
impl VimFaultTrait for DvsNotAuthorized {
}
impl VimFaultTrait for DvsOperationBulkFault {
}
impl VimFaultTrait for DvsScopeViolated {
}
impl VimFaultTrait for ImportHostAddFailure {
}
impl VimFaultTrait for ImportOperationBulkFault {
}
impl VimFaultTrait for InvalidIpfixConfig {
}
impl VimFaultTrait for RollbackFailure {
}
impl VimFaultTrait for SwitchIpUnset {
}
impl VimFaultTrait for SwitchNotInUpgradeMode {
}
impl VimFaultTrait for VspanDestPortConflict {
}
impl VimFaultTrait for VspanPortConflict {
}
impl VimFaultTrait for VspanPortMoveFault {
}
impl VimFaultTrait for VspanPortPromiscChangeFault {
}
impl VimFaultTrait for VspanPortgroupPromiscChangeFault {
}
impl VimFaultTrait for VspanPortgroupTypeChangeFault {
}
impl VimFaultTrait for VspanPromiscuousPortNotSupported {
}
impl VimFaultTrait for VspanSameSessionPortConflict {
}
impl VimFaultTrait for EvcConfigFault {
}
impl VimFaultTrait for ActiveVMsBlockingEvc {
}
impl VimFaultTrait for DisconnectedHostsBlockingEvc {
}
impl VimFaultTrait for EvcModeIllegalByVendor {
}
impl VimFaultTrait for EvcModeUnsupportedByHosts {
}
impl VimFaultTrait for EvcUnsupportedByHostHardware {
}
impl VimFaultTrait for EvcUnsupportedByHostSoftware {
}
impl VimFaultTrait for HeterogenousHostsBlockingEvc {
}
impl VimFaultTrait for ExtendedFault {
}
impl VimFaultTrait for FaultToleranceVmNotDasProtected {
}
impl VimFaultTrait for FcoeFault {
}
impl VimFaultTrait for FcoeFaultPnicHasNoPortSet {
}
impl VimFaultTrait for FileFault {
}
impl VimFaultTrait for CannotAccessFile {
}
impl VimFaultTrait for CannotCreateFile {
}
impl VimFaultTrait for CannotDeleteFile {
}
impl VimFaultTrait for DirectoryNotEmpty {
}
impl VimFaultTrait for FileAlreadyExists {
}
impl VimFaultTrait for FileLocked {
}
impl VimFaultTrait for FileNameTooLong {
}
impl VimFaultTrait for FileNotFound {
}
impl VimFaultTrait for FileNotWritable {
}
impl VimFaultTrait for FileTooLarge {
}
impl VimFaultTrait for IncorrectFileType {
}
impl VimFaultTrait for NetworkCopyFault {
}
impl VimFaultTrait for NoDiskSpace {
}
impl VimFaultTrait for NotADirectory {
}
impl VimFaultTrait for NotAFile {
}
impl VimFaultTrait for TooManyConcurrentNativeClones {
}
impl VimFaultTrait for TooManyNativeCloneLevels {
}
impl VimFaultTrait for TooManyNativeClonesOnFile {
}
impl VimFaultTrait for GenericDrsFault {
}
impl VimFaultTrait for GuestOperationsFault {
}
impl VimFaultTrait for GuestAuthenticationChallenge {
}
impl VimFaultTrait for GuestComponentsOutOfDate {
}
impl VimFaultTrait for GuestMultipleMappings {
}
impl VimFaultTrait for GuestOperationsUnavailable {
}
impl VimFaultTrait for GuestPermissionDenied {
}
impl VimFaultTrait for GuestProcessNotFound {
}
impl VimFaultTrait for GuestRegistryFault {
}
impl VimFaultTrait for GuestRegistryKeyFault {
}
impl VimFaultTrait for GuestRegistryKeyAlreadyExists {
}
impl VimFaultTrait for GuestRegistryKeyHasSubkeys {
}
impl VimFaultTrait for GuestRegistryKeyInvalid {
}
impl VimFaultTrait for GuestRegistryKeyParentVolatile {
}
impl VimFaultTrait for GuestRegistryValueFault {
}
impl VimFaultTrait for GuestRegistryValueNotFound {
}
impl VimFaultTrait for InvalidGuestLogin {
}
impl VimFaultTrait for OperationDisabledByGuest {
}
impl VimFaultTrait for OperationNotSupportedByGuest {
}
impl VimFaultTrait for TooManyGuestLogons {
}
impl VimFaultTrait for HostConfigFault {
}
impl VimFaultTrait for AdminDisabled {
}
impl VimFaultTrait for AdminNotDisabled {
}
impl VimFaultTrait for BlockedByFirewall {
}
impl VimFaultTrait for ClockSkew {
}
impl VimFaultTrait for DisableAdminNotSupported {
}
impl VimFaultTrait for HostConfigFailed {
}
impl VimFaultTrait for HostInDomain {
}
impl VimFaultTrait for InvalidHostName {
}
impl VimFaultTrait for NasConfigFault {
}
impl VimFaultTrait for InvalidNasCredentials {
}
impl VimFaultTrait for InvalidNetworkResource {
}
impl VimFaultTrait for NasConnectionLimitReached {
}
impl VimFaultTrait for NasSessionCredentialConflict {
}
impl VimFaultTrait for NasVolumeNotMounted {
}
impl VimFaultTrait for NetworkInaccessible {
}
impl VimFaultTrait for NoPermissionOnNasVolume {
}
impl VimFaultTrait for NoGateway {
}
impl VimFaultTrait for NoVirtualNic {
}
impl VimFaultTrait for PlatformConfigFault {
}
impl VimFaultTrait for InvalidBundle {
}
impl VimFaultTrait for PatchInstallFailed {
}
impl VimFaultTrait for PatchIntegrityError {
}
impl VimFaultTrait for VmfsMountFault {
}
impl VimFaultTrait for VmfsAlreadyMounted {
}
impl VimFaultTrait for VmfsAmbiguousMount {
}
impl VimFaultTrait for HostConnectFault {
}
impl VimFaultTrait for AgentInstallFailed {
}
impl VimFaultTrait for AlreadyBeingManaged {
}
impl VimFaultTrait for AlreadyConnected {
}
impl VimFaultTrait for CannotAddHostWithFtVmAsStandalone {
}
impl VimFaultTrait for CannotAddHostWithFtVmToDifferentCluster {
}
impl VimFaultTrait for CannotAddHostWithFtVmToNonHaCluster {
}
impl VimFaultTrait for GatewayConnectFault {
}
impl VimFaultTrait for GatewayNotFound {
}
impl VimFaultTrait for GatewayNotReachable {
}
impl VimFaultTrait for GatewayOperationRefused {
}
impl VimFaultTrait for GatewayToHostConnectFault {
}
impl VimFaultTrait for GatewayHostNotReachable {
}
impl VimFaultTrait for GatewayToHostAuthFault {
}
impl VimFaultTrait for GatewayToHostTrustVerifyFault {
}
impl VimFaultTrait for MultipleCertificatesVerifyFault {
}
impl VimFaultTrait for NoHost {
}
impl VimFaultTrait for NoPermissionOnHost {
}
impl VimFaultTrait for NotSupportedHost {
}
impl VimFaultTrait for NonVmwareOuiMacNotSupportedHost {
}
impl VimFaultTrait for NotSupportedHostForVFlash {
}
impl VimFaultTrait for NotSupportedHostForVmcp {
}
impl VimFaultTrait for NotSupportedHostForVmemFile {
}
impl VimFaultTrait for NotSupportedHostForVsan {
}
impl VimFaultTrait for NotSupportedHostInCluster {
}
impl VimFaultTrait for EvcAdmissionFailed {
}
impl VimFaultTrait for EvcAdmissionFailedCpuFeaturesForMode {
}
impl VimFaultTrait for EvcAdmissionFailedCpuModel {
}
impl VimFaultTrait for EvcAdmissionFailedCpuModelForMode {
}
impl VimFaultTrait for EvcAdmissionFailedCpuVendor {
}
impl VimFaultTrait for EvcAdmissionFailedCpuVendorUnknown {
}
impl VimFaultTrait for EvcAdmissionFailedHostDisconnected {
}
impl VimFaultTrait for EvcAdmissionFailedHostSoftware {
}
impl VimFaultTrait for EvcAdmissionFailedHostSoftwareForMode {
}
impl VimFaultTrait for EvcAdmissionFailedVmActive {
}
impl VimFaultTrait for NotSupportedHostInDvs {
}
impl VimFaultTrait for NotSupportedHostInHaCluster {
}
impl VimFaultTrait for ReadHostResourcePoolTreeFailed {
}
impl VimFaultTrait for SslDisabledFault {
}
impl VimFaultTrait for SslVerifyFault {
}
impl VimFaultTrait for TooManyHosts {
}
impl VimFaultTrait for HostHasComponentFailure {
}
impl VimFaultTrait for HostIncompatibleForRecordReplay {
}
impl VimFaultTrait for HostPowerOpFailed {
}
impl VimFaultTrait for NoPeerHostFound {
}
impl VimFaultTrait for VmotionInterfaceNotEnabled {
}
impl VimFaultTrait for WakeOnLanNotSupportedByVmotionNic {
}
impl VimFaultTrait for HostSpecificationOperationFailed {
}
impl VimFaultTrait for HttpFault {
}
impl VimFaultTrait for IormNotSupportedHostOnDatastore {
}
impl VimFaultTrait for InaccessibleVFlashSource {
}
impl VimFaultTrait for InsufficientResourcesFault {
}
impl VimFaultTrait for InsufficientAgentVmsDeployed {
}
impl VimFaultTrait for InsufficientCpuResourcesFault {
}
impl VimFaultTrait for InsufficientFailoverResourcesFault {
}
impl VimFaultTrait for InsufficientGraphicsResourcesFault {
}
impl VimFaultTrait for InsufficientHostCapacityFault {
}
impl VimFaultTrait for InsufficientHostCpuCapacityFault {
}
impl VimFaultTrait for InsufficientHostMemoryCapacityFault {
}
impl VimFaultTrait for InsufficientPerCpuCapacity {
}
impl VimFaultTrait for InsufficientMemoryResourcesFault {
}
impl VimFaultTrait for InsufficientNetworkCapacity {
}
impl VimFaultTrait for InsufficientNetworkResourcePoolCapacity {
}
impl VimFaultTrait for InsufficientStandbyResource {
}
impl VimFaultTrait for InsufficientStandbyCpuResource {
}
impl VimFaultTrait for InsufficientStandbyMemoryResource {
}
impl VimFaultTrait for InsufficientStorageSpace {
}
impl VimFaultTrait for InsufficientVFlashResourcesFault {
}
impl VimFaultTrait for InvalidResourcePoolStructureFault {
}
impl VimFaultTrait for NumVirtualCpusExceedsLimit {
}
impl VimFaultTrait for VmFaultToleranceTooManyFtVcpusOnHost {
}
impl VimFaultTrait for VmFaultToleranceTooManyVMsOnHost {
}
impl VimFaultTrait for VmSmpFaultToleranceTooManyVMsOnHost {
}
impl VimFaultTrait for InsufficientStorageIops {
}
impl VimFaultTrait for InvalidAffinitySettingFault {
}
impl VimFaultTrait for InvalidBmcRole {
}
impl VimFaultTrait for InvalidDatastore {
}
impl VimFaultTrait for DatastoreNotWritableOnHost {
}
impl VimFaultTrait for SwapDatastoreNotWritableOnHost {
}
impl VimFaultTrait for InaccessibleDatastore {
}
impl VimFaultTrait for InaccessibleFtMetadataDatastore {
}
impl VimFaultTrait for InvalidDatastorePath {
}
impl VimFaultTrait for InvalidEvent {
}
impl VimFaultTrait for InvalidFolder {
}
impl VimFaultTrait for VmAlreadyExistsInDatacenter {
}
impl VimFaultTrait for InvalidIpmiLoginInfo {
}
impl VimFaultTrait for InvalidIpmiMacAddress {
}
impl VimFaultTrait for InvalidLicense {
}
impl VimFaultTrait for InvalidLocale {
}
impl VimFaultTrait for InvalidLogin {
}
impl VimFaultTrait for InvalidClientCertificate {
}
impl VimFaultTrait for PasswordExpired {
}
impl VimFaultTrait for InvalidName {
}
impl VimFaultTrait for InvalidPrivilege {
}
impl VimFaultTrait for InvalidState {
}
impl VimFaultTrait for CannotPowerOffVmInCluster {
}
impl VimFaultTrait for EncryptionKeyRequired {
}
impl VimFaultTrait for InvalidDatastoreState {
}
impl VimFaultTrait for InvalidHostState {
}
impl VimFaultTrait for InvalidHostConnectionState {
}
impl VimFaultTrait for InvalidPowerState {
}
impl VimFaultTrait for InvalidVmState {
}
impl VimFaultTrait for MksConnectionLimitReached {
}
impl VimFaultTrait for NoActiveHostInCluster {
}
impl VimFaultTrait for OvfConsumerPowerOnFault {
}
impl VimFaultTrait for QuestionPending {
}
impl VimFaultTrait for VmPowerOnDisabled {
}
impl VimFaultTrait for IscsiFault {
}
impl VimFaultTrait for IscsiFaultInvalidVnic {
}
impl VimFaultTrait for IscsiFaultPnicInUse {
}
impl VimFaultTrait for IscsiFaultVnicAlreadyBound {
}
impl VimFaultTrait for IscsiFaultVnicHasActivePaths {
}
impl VimFaultTrait for IscsiFaultVnicHasMultipleUplinks {
}
impl VimFaultTrait for IscsiFaultVnicHasNoUplinks {
}
impl VimFaultTrait for IscsiFaultVnicHasWrongUplink {
}
impl VimFaultTrait for IscsiFaultVnicInUse {
}
impl VimFaultTrait for IscsiFaultVnicIsLastPath {
}
impl VimFaultTrait for IscsiFaultVnicNotBound {
}
impl VimFaultTrait for IscsiFaultVnicNotFound {
}
impl VimFaultTrait for KeyNotFound {
}
impl VimFaultTrait for LicenseEntityNotFound {
}
impl VimFaultTrait for LicenseServerUnavailable {
}
impl VimFaultTrait for LimitExceeded {
}
impl VimFaultTrait for LogBundlingFailed {
}
impl VimFaultTrait for MigrationFault {
}
impl VimFaultTrait for AffinityConfigured {
}
impl VimFaultTrait for CannotModifyConfigCpuRequirements {
}
impl VimFaultTrait for CannotMoveVmWithDeltaDisk {
}
impl VimFaultTrait for CannotMoveVmWithNativeDeltaDisk {
}
impl VimFaultTrait for CloneFromSnapshotNotSupported {
}
impl VimFaultTrait for DatacenterMismatch {
}
impl VimFaultTrait for DisallowedMigrationDeviceAttached {
}
impl VimFaultTrait for DiskMoveTypeNotSupported {
}
impl VimFaultTrait for FaultToleranceAntiAffinityViolated {
}
impl VimFaultTrait for FaultToleranceNeedsThickDisk {
}
impl VimFaultTrait for FaultToleranceNotSameBuild {
}
impl VimFaultTrait for HaErrorsAtDest {
}
impl VimFaultTrait for IncompatibleDefaultDevice {
}
impl VimFaultTrait for LargeRdmConversionNotSupported {
}
impl VimFaultTrait for MaintenanceModeFileMove {
}
impl VimFaultTrait for MigrationDisabled {
}
impl VimFaultTrait for MigrationFeatureNotSupported {
}
impl VimFaultTrait for FullStorageVMotionNotSupported {
}
impl VimFaultTrait for IndependentDiskVMotionNotSupported {
}
impl VimFaultTrait for NonHomeRdmvMotionNotSupported {
}
impl VimFaultTrait for StorageVMotionNotSupported {
}
impl VimFaultTrait for UnsharedSwapVMotionNotSupported {
}
impl VimFaultTrait for VMotionAcrossNetworkNotSupported {
}
impl VimFaultTrait for MigrationNotReady {
}
impl VimFaultTrait for MismatchedNetworkPolicies {
}
impl VimFaultTrait for MismatchedVMotionNetworkNames {
}
impl VimFaultTrait for NetworksMayNotBeTheSame {
}
impl VimFaultTrait for NoGuestHeartbeat {
}
impl VimFaultTrait for RdmConversionNotSupported {
}
impl VimFaultTrait for RdmNotPreserved {
}
impl VimFaultTrait for ReadOnlyDisksWithLegacyDestination {
}
impl VimFaultTrait for SnapshotCopyNotSupported {
}
impl VimFaultTrait for HotSnapshotMoveNotSupported {
}
impl VimFaultTrait for SnapshotCloneNotSupported {
}
impl VimFaultTrait for SnapshotMoveFromNonHomeNotSupported {
}
impl VimFaultTrait for SnapshotMoveNotSupported {
}
impl VimFaultTrait for SnapshotMoveToNonHomeNotSupported {
}
impl VimFaultTrait for SnapshotRevertIssue {
}
impl VimFaultTrait for SuspendedRelocateNotSupported {
}
impl VimFaultTrait for TooManyDisksOnLegacyHost {
}
impl VimFaultTrait for ToolsInstallationInProgress {
}
impl VimFaultTrait for UncommittedUndoableDisk {
}
impl VimFaultTrait for VMotionInterfaceIssue {
}
impl VimFaultTrait for VMotionLinkCapacityLow {
}
impl VimFaultTrait for VMotionLinkDown {
}
impl VimFaultTrait for VMotionNotConfigured {
}
impl VimFaultTrait for VMotionNotLicensed {
}
impl VimFaultTrait for VMotionNotSupported {
}
impl VimFaultTrait for VMotionProtocolIncompatible {
}
impl VimFaultTrait for WillLoseHaProtection {
}
impl VimFaultTrait for WillModifyConfigCpuRequirements {
}
impl VimFaultTrait for WillResetSnapshotDirectory {
}
impl VimFaultTrait for MismatchedBundle {
}
impl VimFaultTrait for MissingBmcSupport {
}
impl VimFaultTrait for NamespaceFull {
}
impl VimFaultTrait for NamespaceLimitReached {
}
impl VimFaultTrait for NamespaceWriteProtected {
}
impl VimFaultTrait for NetworkDisruptedAndConfigRolledBack {
}
impl VimFaultTrait for NoClientCertificate {
}
impl VimFaultTrait for NoCompatibleDatastore {
}
impl VimFaultTrait for NoCompatibleHost {
}
impl VimFaultTrait for NoCompatibleHostWithAccessToDevice {
}
impl VimFaultTrait for NoConnectedDatastore {
}
impl VimFaultTrait for NoDiskFound {
}
impl VimFaultTrait for NoSubjectName {
}
impl VimFaultTrait for NotFound {
}
impl VimFaultTrait for NotSupportedHostForChecksum {
}
impl VimFaultTrait for OutOfBounds {
}
impl VimFaultTrait for OvfFault {
}
impl VimFaultTrait for OvfConsumerCallbackFault {
}
impl VimFaultTrait for OvfConsumerCommunicationError {
}
impl VimFaultTrait for OvfConsumerFault {
}
impl VimFaultTrait for OvfConsumerInvalidSection {
}
impl VimFaultTrait for OvfConsumerUndeclaredSection {
}
impl VimFaultTrait for OvfConsumerUndefinedPrefix {
}
impl VimFaultTrait for OvfExport {
}
impl VimFaultTrait for ConnectedIso {
}
impl VimFaultTrait for OvfDuplicatedPropertyIdExport {
}
impl VimFaultTrait for OvfDuplicatedPropertyIdImport {
}
impl VimFaultTrait for OvfExportFailed {
}
impl VimFaultTrait for OvfHardwareExport {
}
impl VimFaultTrait for OvfConnectedDevice {
}
impl VimFaultTrait for OvfConnectedDeviceFloppy {
}
impl VimFaultTrait for OvfConnectedDeviceIso {
}
impl VimFaultTrait for OvfUnableToExportDisk {
}
impl VimFaultTrait for OvfUnknownDeviceBacking {
}
impl VimFaultTrait for OvfUnsupportedDeviceExport {
}
impl VimFaultTrait for OvfPropertyExport {
}
impl VimFaultTrait for OvfPropertyNetworkExport {
}
impl VimFaultTrait for OvfImport {
}
impl VimFaultTrait for OvfCpuCompatibility {
}
impl VimFaultTrait for OvfCpuCompatibilityCheckNotSupported {
}
impl VimFaultTrait for OvfHardwareCheck {
}
impl VimFaultTrait for OvfImportFailed {
}
impl VimFaultTrait for OvfMappedOsId {
}
impl VimFaultTrait for OvfMissingHardware {
}
impl VimFaultTrait for OvfNetworkMappingNotSupported {
}
impl VimFaultTrait for OvfUnsupportedDiskProvisioning {
}
impl VimFaultTrait for OvfInvalidPackage {
}
impl VimFaultTrait for OvfAttribute {
}
impl VimFaultTrait for OvfInvalidValue {
}
impl VimFaultTrait for OvfInvalidValueConfiguration {
}
impl VimFaultTrait for OvfInvalidValueEmpty {
}
impl VimFaultTrait for OvfInvalidValueFormatMalformed {
}
impl VimFaultTrait for OvfInvalidValueReference {
}
impl VimFaultTrait for OvfMissingAttribute {
}
impl VimFaultTrait for OvfConstraint {
}
impl VimFaultTrait for OvfDiskOrderConstraint {
}
impl VimFaultTrait for OvfHostResourceConstraint {
}
impl VimFaultTrait for OvfElement {
}
impl VimFaultTrait for OvfDuplicateElement {
}
impl VimFaultTrait for OvfDuplicatedElementBoundary {
}
impl VimFaultTrait for OvfElementInvalidValue {
}
impl VimFaultTrait for OvfMissingElement {
}
impl VimFaultTrait for OvfMissingElementNormalBoundary {
}
impl VimFaultTrait for OvfUnexpectedElement {
}
impl VimFaultTrait for OvfWrongElement {
}
impl VimFaultTrait for OvfProperty {
}
impl VimFaultTrait for OvfPropertyNetwork {
}
impl VimFaultTrait for OvfPropertyQualifier {
}
impl VimFaultTrait for OvfPropertyQualifierDuplicate {
}
impl VimFaultTrait for OvfPropertyQualifierIgnored {
}
impl VimFaultTrait for OvfPropertyType {
}
impl VimFaultTrait for OvfPropertyValue {
}
impl VimFaultTrait for OvfWrongNamespace {
}
impl VimFaultTrait for OvfXmlFormat {
}
impl VimFaultTrait for OvfSystemFault {
}
impl VimFaultTrait for OvfDiskMappingNotFound {
}
impl VimFaultTrait for OvfHostValueNotParsed {
}
impl VimFaultTrait for OvfInternalError {
}
impl VimFaultTrait for OvfToXmlUnsupportedElement {
}
impl VimFaultTrait for OvfUnknownDevice {
}
impl VimFaultTrait for OvfUnknownEntity {
}
impl VimFaultTrait for OvfUnsupportedDeviceBackingInfo {
}
impl VimFaultTrait for OvfUnsupportedDeviceBackingOption {
}
impl VimFaultTrait for OvfUnsupportedPackage {
}
impl VimFaultTrait for OvfInvalidVmName {
}
impl VimFaultTrait for OvfNoHostNic {
}
impl VimFaultTrait for OvfNoSupportedHardwareFamily {
}
impl VimFaultTrait for OvfUnsupportedAttribute {
}
impl VimFaultTrait for OvfUnsupportedAttributeValue {
}
impl VimFaultTrait for OvfUnsupportedElement {
}
impl VimFaultTrait for OvfNoSpaceOnController {
}
impl VimFaultTrait for OvfUnsupportedElementValue {
}
impl VimFaultTrait for OvfUnsupportedSection {
}
impl VimFaultTrait for OvfUnsupportedSubType {
}
impl VimFaultTrait for OvfUnsupportedType {
}
impl VimFaultTrait for PatchBinariesNotFound {
}
impl VimFaultTrait for PatchMetadataInvalid {
}
impl VimFaultTrait for PatchMetadataCorrupted {
}
impl VimFaultTrait for PatchMetadataNotFound {
}
impl VimFaultTrait for PatchNotApplicable {
}
impl VimFaultTrait for PatchAlreadyInstalled {
}
impl VimFaultTrait for PatchMissingDependencies {
}
impl VimFaultTrait for PatchSuperseded {
}
impl VimFaultTrait for ProfileUpdateFailed {
}
impl VimFaultTrait for RebootRequired {
}
impl VimFaultTrait for RecordReplayDisabled {
}
impl VimFaultTrait for RemoveFailed {
}
impl VimFaultTrait for ReplicationFault {
}
impl VimFaultTrait for IncompatibleHostForVmReplication {
}
impl VimFaultTrait for ReplicationConfigFault {
}
impl VimFaultTrait for ReplicationDiskConfigFault {
}
impl VimFaultTrait for ReplicationVmConfigFault {
}
impl VimFaultTrait for ReplicationIncompatibleWithFt {
}
impl VimFaultTrait for ReplicationInvalidOptions {
}
impl VimFaultTrait for ReplicationNotSupportedOnHost {
}
impl VimFaultTrait for ReplicationVmFault {
}
impl VimFaultTrait for ReplicationVmInProgressFault {
}
impl VimFaultTrait for ResourceInUse {
}
impl VimFaultTrait for FilterInUse {
}
impl VimFaultTrait for QuiesceDatastoreIoForHaFailed {
}
impl VimFaultTrait for ResourceNotAvailable {
}
impl VimFaultTrait for SspiChallenge {
}
impl VimFaultTrait for ShrinkDiskFault {
}
impl VimFaultTrait for SnapshotFault {
}
impl VimFaultTrait for ApplicationQuiesceFault {
}
impl VimFaultTrait for FilesystemQuiesceFault {
}
impl VimFaultTrait for MemorySnapshotOnIndependentDisk {
}
impl VimFaultTrait for MultipleSnapshotsNotSupported {
}
impl VimFaultTrait for SnapshotDisabled {
}
impl VimFaultTrait for SnapshotIncompatibleDeviceInVm {
}
impl VimFaultTrait for SnapshotLocked {
}
impl VimFaultTrait for SnapshotNoChange {
}
impl VimFaultTrait for TooManySnapshotLevels {
}
impl VimFaultTrait for SsdDiskNotAvailable {
}
impl VimFaultTrait for StorageDrsCannotMoveDiskInMultiWriterMode {
}
impl VimFaultTrait for StorageDrsCannotMoveFtVm {
}
impl VimFaultTrait for StorageDrsCannotMoveIndependentDisk {
}
impl VimFaultTrait for StorageDrsCannotMoveManuallyPlacedSwapFile {
}
impl VimFaultTrait for StorageDrsCannotMoveManuallyPlacedVm {
}
impl VimFaultTrait for StorageDrsCannotMoveSharedDisk {
}
impl VimFaultTrait for StorageDrsCannotMoveTemplate {
}
impl VimFaultTrait for StorageDrsCannotMoveVmInUserFolder {
}
impl VimFaultTrait for StorageDrsCannotMoveVmWithMountedCdrom {
}
impl VimFaultTrait for StorageDrsCannotMoveVmWithNoFilesInLayout {
}
impl VimFaultTrait for StorageDrsDatacentersCannotShareDatastore {
}
impl VimFaultTrait for StorageDrsDisabledOnVm {
}
impl VimFaultTrait for StorageDrsHbrDiskNotMovable {
}
impl VimFaultTrait for StorageDrsHmsMoveInProgress {
}
impl VimFaultTrait for StorageDrsHmsUnreachable {
}
impl VimFaultTrait for StorageDrsIolbDisabledInternally {
}
impl VimFaultTrait for StorageDrsRelocateDisabled {
}
impl VimFaultTrait for StorageDrsStaleHmsCollection {
}
impl VimFaultTrait for StorageDrsUnableToMoveFiles {
}
impl VimFaultTrait for SwapDatastoreUnset {
}
impl VimFaultTrait for TaskInProgress {
}
impl VimFaultTrait for VAppTaskInProgress {
}
impl VimFaultTrait for Timedout {
}
impl VimFaultTrait for PowerOnFtSecondaryTimedout {
}
impl VimFaultTrait for TooManyConsecutiveOverrides {
}
impl VimFaultTrait for ToolsUnavailable {
}
impl VimFaultTrait for UnrecognizedHost {
}
impl VimFaultTrait for UnsupportedVimApiVersion {
}
impl VimFaultTrait for UserNotFound {
}
impl VimFaultTrait for VAppConfigFault {
}
impl VimFaultTrait for MissingPowerOffConfiguration {
}
impl VimFaultTrait for MissingPowerOnConfiguration {
}
impl VimFaultTrait for NoVmInVApp {
}
impl VimFaultTrait for VFlashModuleVersionIncompatible {
}
impl VimFaultTrait for VmConfigFault {
}
impl VimFaultTrait for CannotAccessVmComponent {
}
impl VimFaultTrait for CannotAccessVmConfig {
}
impl VimFaultTrait for CannotAccessVmDevice {
}
impl VimFaultTrait for CannotAccessNetwork {
}
impl VimFaultTrait for DestinationSwitchFull {
}
impl VimFaultTrait for LegacyNetworkInterfaceInUse {
}
impl VimFaultTrait for VmOnConflictDvPort {
}
impl VimFaultTrait for VmOnVirtualIntranet {
}
impl VimFaultTrait for CannotAccessVmDisk {
}
impl VimFaultTrait for RdmPointsToInaccessibleDisk {
}
impl VimFaultTrait for CannotDisableSnapshot {
}
impl VimFaultTrait for CannotUseNetwork {
}
impl VimFaultTrait for CpuHotPlugNotSupported {
}
impl VimFaultTrait for DeltaDiskFormatNotSupported {
}
impl VimFaultTrait for EightHostLimitViolated {
}
impl VimFaultTrait for FaultToleranceCannotEditMem {
}
impl VimFaultTrait for GenericVmConfigFault {
}
impl VimFaultTrait for InvalidFormat {
}
impl VimFaultTrait for InvalidDiskFormat {
}
impl VimFaultTrait for InvalidSnapshotFormat {
}
impl VimFaultTrait for InvalidVmConfig {
}
impl VimFaultTrait for InvalidDeviceSpec {
}
impl VimFaultTrait for DeviceHotPlugNotSupported {
}
impl VimFaultTrait for DeviceNotFound {
}
impl VimFaultTrait for DeviceUnsupportedForVmPlatform {
}
impl VimFaultTrait for DeviceUnsupportedForVmVersion {
}
impl VimFaultTrait for DisallowedDiskModeChange {
}
impl VimFaultTrait for InvalidController {
}
impl VimFaultTrait for InvalidDeviceBacking {
}
impl VimFaultTrait for InvalidDeviceOperation {
}
impl VimFaultTrait for MissingController {
}
impl VimFaultTrait for SwapPlacementOverrideNotSupported {
}
impl VimFaultTrait for TooManyDevices {
}
impl VimFaultTrait for UnsupportedGuest {
}
impl VimFaultTrait for VmWwnConflict {
}
impl VimFaultTrait for LargeRdmNotSupportedOnDatastore {
}
impl VimFaultTrait for MemoryHotPlugNotSupported {
}
impl VimFaultTrait for NoCompatibleHardAffinityHost {
}
impl VimFaultTrait for NoCompatibleSoftAffinityHost {
}
impl VimFaultTrait for NumVirtualCpusIncompatible {
}
impl VimFaultTrait for OvfConsumerValidationFault {
}
impl VimFaultTrait for QuarantineModeFault {
}
impl VimFaultTrait for RdmNotSupportedOnDatastore {
}
impl VimFaultTrait for RuleViolation {
}
impl VimFaultTrait for SoftRuleVioCorrectionDisallowed {
}
impl VimFaultTrait for SoftRuleVioCorrectionImpact {
}
impl VimFaultTrait for UnsupportedDatastore {
}
impl VimFaultTrait for MemoryFileFormatNotSupportedByDatastore {
}
impl VimFaultTrait for UnSupportedDatastoreForVFlash {
}
impl VimFaultTrait for UnsupportedVmxLocation {
}
impl VimFaultTrait for VAppNotRunning {
}
impl VimFaultTrait for VAppPropertyFault {
}
impl VimFaultTrait for InvalidNetworkInType {
}
impl VimFaultTrait for InvalidPropertyType {
}
impl VimFaultTrait for InvalidPropertyValue {
}
impl VimFaultTrait for UnconfiguredPropertyValue {
}
impl VimFaultTrait for MissingIpPool {
}
impl VimFaultTrait for MissingNetworkIpConfig {
}
impl VimFaultTrait for NoAvailableIp {
}
impl VimFaultTrait for NoVcManagedIpConfigured {
}
impl VimFaultTrait for NotUserConfigurableProperty {
}
impl VimFaultTrait for VFlashCacheHotConfigNotSupported {
}
impl VimFaultTrait for VFlashModuleNotSupported {
}
impl VimFaultTrait for VirtualHardwareCompatibilityIssue {
}
impl VimFaultTrait for CpuIncompatible {
}
impl VimFaultTrait for CpuCompatibilityUnknown {
}
impl VimFaultTrait for CpuIncompatible1Ecx {
}
impl VimFaultTrait for CpuIncompatible81Edx {
}
impl VimFaultTrait for FaultToleranceCpuIncompatible {
}
impl VimFaultTrait for DeviceNotSupported {
}
impl VimFaultTrait for DeviceBackingNotSupported {
}
impl VimFaultTrait for DvPortNotSupported {
}
impl VimFaultTrait for UnusedVirtualDiskBlocksNotScrubbed {
}
impl VimFaultTrait for VirtualDiskBlocksNotFullyProvisioned {
}
impl VimFaultTrait for DeviceControllerNotSupported {
}
impl VimFaultTrait for DigestNotSupported {
}
impl VimFaultTrait for FileBackedPortNotSupported {
}
impl VimFaultTrait for MultiWriterNotSupported {
}
impl VimFaultTrait for NonPersistentDisksNotSupported {
}
impl VimFaultTrait for RdmNotSupported {
}
impl VimFaultTrait for PhysCompatRdmNotSupported {
}
impl VimFaultTrait for RawDiskNotSupported {
}
impl VimFaultTrait for RemoteDeviceNotSupported {
}
impl VimFaultTrait for SharedBusControllerNotSupported {
}
impl VimFaultTrait for VmiNotSupported {
}
impl VimFaultTrait for VirtualDiskModeNotSupported {
}
impl VimFaultTrait for VirtualEthernetCardNotSupported {
}
impl VimFaultTrait for DiskNotSupported {
}
impl VimFaultTrait for IdeDiskNotSupported {
}
impl VimFaultTrait for DrsVmotionIncompatibleFault {
}
impl VimFaultTrait for FeatureRequirementsNotMet {
}
impl VimFaultTrait for MemorySizeNotRecommended {
}
impl VimFaultTrait for MemorySizeNotSupported {
}
impl VimFaultTrait for MemorySizeNotSupportedByDatastore {
}
impl VimFaultTrait for NotEnoughCpus {
}
impl VimFaultTrait for NotEnoughLogicalCpus {
}
impl VimFaultTrait for NumVirtualCoresPerSocketNotSupported {
}
impl VimFaultTrait for NumVirtualCpusNotSupported {
}
impl VimFaultTrait for StorageVmotionIncompatible {
}
impl VimFaultTrait for VirtualHardwareVersionNotSupported {
}
impl VimFaultTrait for WakeOnLanNotSupported {
}
impl VimFaultTrait for VmConfigIncompatibleForFaultTolerance {
}
impl VimFaultTrait for VmConfigIncompatibleForRecordReplay {
}
impl VimFaultTrait for VmHostAffinityRuleViolation {
}
impl VimFaultTrait for VmFaultToleranceIssue {
}
impl VimFaultTrait for CannotChangeDrsBehaviorForFtSecondary {
}
impl VimFaultTrait for CannotChangeHaSettingsForFtSecondary {
}
impl VimFaultTrait for CannotComputeFtCompatibleHosts {
}
impl VimFaultTrait for FaultToleranceNotLicensed {
}
impl VimFaultTrait for FaultTolerancePrimaryPowerOnNotAttempted {
}
impl VimFaultTrait for FtIssuesOnHost {
}
impl VimFaultTrait for HostIncompatibleForFaultTolerance {
}
impl VimFaultTrait for IncompatibleHostForFtSecondary {
}
impl VimFaultTrait for InvalidOperationOnSecondaryVm {
}
impl VimFaultTrait for NoHostSuitableForFtSecondary {
}
impl VimFaultTrait for NotSupportedDeviceForFt {
}
impl VimFaultTrait for PowerOnFtSecondaryFailed {
}
impl VimFaultTrait for SecondaryVmAlreadyDisabled {
}
impl VimFaultTrait for SecondaryVmAlreadyEnabled {
}
impl VimFaultTrait for SecondaryVmAlreadyRegistered {
}
impl VimFaultTrait for SecondaryVmNotRegistered {
}
impl VimFaultTrait for VmFaultToleranceConfigIssue {
}
impl VimFaultTrait for VmFaultToleranceConfigIssueWrapper {
}
impl VimFaultTrait for VmFaultToleranceInvalidFileBacking {
}
impl VimFaultTrait for VmFaultToleranceOpIssuesList {
}
impl VimFaultTrait for VmMetadataManagerFault {
}
impl VimFaultTrait for VmMonitorIncompatibleForFaultTolerance {
}
impl VimFaultTrait for VmToolsUpgradeFault {
}
impl VimFaultTrait for ToolsAlreadyUpgraded {
}
impl VimFaultTrait for ToolsAutoUpgradeNotSupported {
}
impl VimFaultTrait for ToolsImageCopyFailed {
}
impl VimFaultTrait for ToolsImageNotAvailable {
}
impl VimFaultTrait for ToolsImageSignatureCheckFailed {
}
impl VimFaultTrait for ToolsUpgradeCancelled {
}
impl VimFaultTrait for VmValidateMaxDevice {
}
impl VimFaultTrait for VsanFault {
}
impl VimFaultTrait for CannotChangeVsanClusterUuid {
}
impl VimFaultTrait for CannotChangeVsanNodeUuid {
}
impl VimFaultTrait for CannotMoveVsanEnabledHost {
}
impl VimFaultTrait for DestinationVsanDisabled {
}
impl VimFaultTrait for VsanClusterUuidMismatch {
}
impl VimFaultTrait for CannotReconfigureVsanWhenHaEnabled {
}
impl VimFaultTrait for DuplicateVsanNetworkInterface {
}
impl VimFaultTrait for VsanDiskFault {
}
impl VimFaultTrait for DiskHasPartitions {
}
impl VimFaultTrait for DiskIsLastRemainingNonSsd {
}
impl VimFaultTrait for DiskIsNonLocal {
}
impl VimFaultTrait for DiskIsUsb {
}
impl VimFaultTrait for DiskTooSmall {
}
impl VimFaultTrait for DuplicateDisks {
}
impl VimFaultTrait for InsufficientDisks {
}
impl VimFaultTrait for VsanIncompatibleDiskMapping {
}
impl VimFaultTrait for WipeDiskFault {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VimFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VimFault => Some(from.as_any_ref().downcast_ref::<VimFault>()?),
            StructType::ActiveDirectoryFault => Some(from.as_any_ref().downcast_ref::<ActiveDirectoryFault>()?),
            StructType::DomainNotFound => Some(from.as_any_ref().downcast_ref::<DomainNotFound>()?),
            StructType::InvalidCamServer => Some(from.as_any_ref().downcast_ref::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Some(from.as_any_ref().downcast_ref::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Some(from.as_any_ref().downcast_ref::<InvalidCamCertificate>()?),
            StructType::NoPermissionOnAd => Some(from.as_any_ref().downcast_ref::<NoPermissionOnAd>()?),
            StructType::NonAdUserRequired => Some(from.as_any_ref().downcast_ref::<NonAdUserRequired>()?),
            StructType::AlreadyExists => Some(from.as_any_ref().downcast_ref::<AlreadyExists>()?),
            StructType::AlreadyUpgraded => Some(from.as_any_ref().downcast_ref::<AlreadyUpgraded>()?),
            StructType::AnswerFileUpdateFailed => Some(from.as_any_ref().downcast_ref::<AnswerFileUpdateFailed>()?),
            StructType::AuthMinimumAdminPermission => Some(from.as_any_ref().downcast_ref::<AuthMinimumAdminPermission>()?),
            StructType::CannotAccessLocalSource => Some(from.as_any_ref().downcast_ref::<CannotAccessLocalSource>()?),
            StructType::CannotDisconnectHostWithFaultToleranceVm => Some(from.as_any_ref().downcast_ref::<CannotDisconnectHostWithFaultToleranceVm>()?),
            StructType::CannotEnableVmcpForCluster => Some(from.as_any_ref().downcast_ref::<CannotEnableVmcpForCluster>()?),
            StructType::CannotMoveFaultToleranceVm => Some(from.as_any_ref().downcast_ref::<CannotMoveFaultToleranceVm>()?),
            StructType::CannotMoveHostWithFaultToleranceVm => Some(from.as_any_ref().downcast_ref::<CannotMoveHostWithFaultToleranceVm>()?),
            StructType::CannotPlaceWithoutPrerequisiteMoves => Some(from.as_any_ref().downcast_ref::<CannotPlaceWithoutPrerequisiteMoves>()?),
            StructType::ConcurrentAccess => Some(from.as_any_ref().downcast_ref::<ConcurrentAccess>()?),
            StructType::CustomizationFault => Some(from.as_any_ref().downcast_ref::<CustomizationFault>()?),
            StructType::CannotDecryptPasswords => Some(from.as_any_ref().downcast_ref::<CannotDecryptPasswords>()?),
            StructType::CustomizationPending => Some(from.as_any_ref().downcast_ref::<CustomizationPending>()?),
            StructType::IpHostnameGeneratorError => Some(from.as_any_ref().downcast_ref::<IpHostnameGeneratorError>()?),
            StructType::LinuxVolumeNotClean => Some(from.as_any_ref().downcast_ref::<LinuxVolumeNotClean>()?),
            StructType::MissingLinuxCustResources => Some(from.as_any_ref().downcast_ref::<MissingLinuxCustResources>()?),
            StructType::MissingWindowsCustResources => Some(from.as_any_ref().downcast_ref::<MissingWindowsCustResources>()?),
            StructType::MountError => Some(from.as_any_ref().downcast_ref::<MountError>()?),
            StructType::NicSettingMismatch => Some(from.as_any_ref().downcast_ref::<NicSettingMismatch>()?),
            StructType::NoDisksToCustomize => Some(from.as_any_ref().downcast_ref::<NoDisksToCustomize>()?),
            StructType::UncustomizableGuest => Some(from.as_any_ref().downcast_ref::<UncustomizableGuest>()?),
            StructType::UnexpectedCustomizationFault => Some(from.as_any_ref().downcast_ref::<UnexpectedCustomizationFault>()?),
            StructType::VolumeEditorError => Some(from.as_any_ref().downcast_ref::<VolumeEditorError>()?),
            StructType::DasConfigFault => Some(from.as_any_ref().downcast_ref::<DasConfigFault>()?),
            StructType::DrsDisabledOnVm => Some(from.as_any_ref().downcast_ref::<DrsDisabledOnVm>()?),
            StructType::DuplicateName => Some(from.as_any_ref().downcast_ref::<DuplicateName>()?),
            StructType::DvsFault => Some(from.as_any_ref().downcast_ref::<DvsFault>()?),
            StructType::BackupBlobReadFailure => Some(from.as_any_ref().downcast_ref::<BackupBlobReadFailure>()?),
            StructType::BackupBlobWriteFailure => Some(from.as_any_ref().downcast_ref::<BackupBlobWriteFailure>()?),
            StructType::CollectorAddressUnset => Some(from.as_any_ref().downcast_ref::<CollectorAddressUnset>()?),
            StructType::ConflictingConfiguration => Some(from.as_any_ref().downcast_ref::<ConflictingConfiguration>()?),
            StructType::DvsApplyOperationFault => Some(from.as_any_ref().downcast_ref::<DvsApplyOperationFault>()?),
            StructType::DvsNotAuthorized => Some(from.as_any_ref().downcast_ref::<DvsNotAuthorized>()?),
            StructType::DvsOperationBulkFault => Some(from.as_any_ref().downcast_ref::<DvsOperationBulkFault>()?),
            StructType::DvsScopeViolated => Some(from.as_any_ref().downcast_ref::<DvsScopeViolated>()?),
            StructType::ImportHostAddFailure => Some(from.as_any_ref().downcast_ref::<ImportHostAddFailure>()?),
            StructType::ImportOperationBulkFault => Some(from.as_any_ref().downcast_ref::<ImportOperationBulkFault>()?),
            StructType::InvalidIpfixConfig => Some(from.as_any_ref().downcast_ref::<InvalidIpfixConfig>()?),
            StructType::RollbackFailure => Some(from.as_any_ref().downcast_ref::<RollbackFailure>()?),
            StructType::SwitchIpUnset => Some(from.as_any_ref().downcast_ref::<SwitchIpUnset>()?),
            StructType::SwitchNotInUpgradeMode => Some(from.as_any_ref().downcast_ref::<SwitchNotInUpgradeMode>()?),
            StructType::VspanDestPortConflict => Some(from.as_any_ref().downcast_ref::<VspanDestPortConflict>()?),
            StructType::VspanPortConflict => Some(from.as_any_ref().downcast_ref::<VspanPortConflict>()?),
            StructType::VspanPortMoveFault => Some(from.as_any_ref().downcast_ref::<VspanPortMoveFault>()?),
            StructType::VspanPortPromiscChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortPromiscChangeFault>()?),
            StructType::VspanPortgroupPromiscChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortgroupPromiscChangeFault>()?),
            StructType::VspanPortgroupTypeChangeFault => Some(from.as_any_ref().downcast_ref::<VspanPortgroupTypeChangeFault>()?),
            StructType::VspanPromiscuousPortNotSupported => Some(from.as_any_ref().downcast_ref::<VspanPromiscuousPortNotSupported>()?),
            StructType::VspanSameSessionPortConflict => Some(from.as_any_ref().downcast_ref::<VspanSameSessionPortConflict>()?),
            StructType::EvcConfigFault => Some(from.as_any_ref().downcast_ref::<EvcConfigFault>()?),
            StructType::ActiveVMsBlockingEvc => Some(from.as_any_ref().downcast_ref::<ActiveVMsBlockingEvc>()?),
            StructType::DisconnectedHostsBlockingEvc => Some(from.as_any_ref().downcast_ref::<DisconnectedHostsBlockingEvc>()?),
            StructType::EvcModeIllegalByVendor => Some(from.as_any_ref().downcast_ref::<EvcModeIllegalByVendor>()?),
            StructType::EvcModeUnsupportedByHosts => Some(from.as_any_ref().downcast_ref::<EvcModeUnsupportedByHosts>()?),
            StructType::EvcUnsupportedByHostHardware => Some(from.as_any_ref().downcast_ref::<EvcUnsupportedByHostHardware>()?),
            StructType::EvcUnsupportedByHostSoftware => Some(from.as_any_ref().downcast_ref::<EvcUnsupportedByHostSoftware>()?),
            StructType::HeterogenousHostsBlockingEvc => Some(from.as_any_ref().downcast_ref::<HeterogenousHostsBlockingEvc>()?),
            StructType::ExtendedFault => Some(from.as_any_ref().downcast_ref::<ExtendedFault>()?),
            StructType::FaultToleranceVmNotDasProtected => Some(from.as_any_ref().downcast_ref::<FaultToleranceVmNotDasProtected>()?),
            StructType::FcoeFault => Some(from.as_any_ref().downcast_ref::<FcoeFault>()?),
            StructType::FcoeFaultPnicHasNoPortSet => Some(from.as_any_ref().downcast_ref::<FcoeFaultPnicHasNoPortSet>()?),
            StructType::FileFault => Some(from.as_any_ref().downcast_ref::<FileFault>()?),
            StructType::CannotAccessFile => Some(from.as_any_ref().downcast_ref::<CannotAccessFile>()?),
            StructType::CannotCreateFile => Some(from.as_any_ref().downcast_ref::<CannotCreateFile>()?),
            StructType::CannotDeleteFile => Some(from.as_any_ref().downcast_ref::<CannotDeleteFile>()?),
            StructType::DirectoryNotEmpty => Some(from.as_any_ref().downcast_ref::<DirectoryNotEmpty>()?),
            StructType::FileAlreadyExists => Some(from.as_any_ref().downcast_ref::<FileAlreadyExists>()?),
            StructType::FileLocked => Some(from.as_any_ref().downcast_ref::<FileLocked>()?),
            StructType::FileNameTooLong => Some(from.as_any_ref().downcast_ref::<FileNameTooLong>()?),
            StructType::FileNotFound => Some(from.as_any_ref().downcast_ref::<FileNotFound>()?),
            StructType::FileNotWritable => Some(from.as_any_ref().downcast_ref::<FileNotWritable>()?),
            StructType::FileTooLarge => Some(from.as_any_ref().downcast_ref::<FileTooLarge>()?),
            StructType::IncorrectFileType => Some(from.as_any_ref().downcast_ref::<IncorrectFileType>()?),
            StructType::NetworkCopyFault => Some(from.as_any_ref().downcast_ref::<NetworkCopyFault>()?),
            StructType::NoDiskSpace => Some(from.as_any_ref().downcast_ref::<NoDiskSpace>()?),
            StructType::NotADirectory => Some(from.as_any_ref().downcast_ref::<NotADirectory>()?),
            StructType::NotAFile => Some(from.as_any_ref().downcast_ref::<NotAFile>()?),
            StructType::TooManyConcurrentNativeClones => Some(from.as_any_ref().downcast_ref::<TooManyConcurrentNativeClones>()?),
            StructType::TooManyNativeCloneLevels => Some(from.as_any_ref().downcast_ref::<TooManyNativeCloneLevels>()?),
            StructType::TooManyNativeClonesOnFile => Some(from.as_any_ref().downcast_ref::<TooManyNativeClonesOnFile>()?),
            StructType::GenericDrsFault => Some(from.as_any_ref().downcast_ref::<GenericDrsFault>()?),
            StructType::GuestOperationsFault => Some(from.as_any_ref().downcast_ref::<GuestOperationsFault>()?),
            StructType::GuestAuthenticationChallenge => Some(from.as_any_ref().downcast_ref::<GuestAuthenticationChallenge>()?),
            StructType::GuestComponentsOutOfDate => Some(from.as_any_ref().downcast_ref::<GuestComponentsOutOfDate>()?),
            StructType::GuestMultipleMappings => Some(from.as_any_ref().downcast_ref::<GuestMultipleMappings>()?),
            StructType::GuestOperationsUnavailable => Some(from.as_any_ref().downcast_ref::<GuestOperationsUnavailable>()?),
            StructType::GuestPermissionDenied => Some(from.as_any_ref().downcast_ref::<GuestPermissionDenied>()?),
            StructType::GuestProcessNotFound => Some(from.as_any_ref().downcast_ref::<GuestProcessNotFound>()?),
            StructType::GuestRegistryFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Some(from.as_any_ref().downcast_ref::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Some(from.as_any_ref().downcast_ref::<GuestRegistryValueNotFound>()?),
            StructType::InvalidGuestLogin => Some(from.as_any_ref().downcast_ref::<InvalidGuestLogin>()?),
            StructType::OperationDisabledByGuest => Some(from.as_any_ref().downcast_ref::<OperationDisabledByGuest>()?),
            StructType::OperationNotSupportedByGuest => Some(from.as_any_ref().downcast_ref::<OperationNotSupportedByGuest>()?),
            StructType::TooManyGuestLogons => Some(from.as_any_ref().downcast_ref::<TooManyGuestLogons>()?),
            StructType::HostConfigFault => Some(from.as_any_ref().downcast_ref::<HostConfigFault>()?),
            StructType::AdminDisabled => Some(from.as_any_ref().downcast_ref::<AdminDisabled>()?),
            StructType::AdminNotDisabled => Some(from.as_any_ref().downcast_ref::<AdminNotDisabled>()?),
            StructType::BlockedByFirewall => Some(from.as_any_ref().downcast_ref::<BlockedByFirewall>()?),
            StructType::ClockSkew => Some(from.as_any_ref().downcast_ref::<ClockSkew>()?),
            StructType::DisableAdminNotSupported => Some(from.as_any_ref().downcast_ref::<DisableAdminNotSupported>()?),
            StructType::HostConfigFailed => Some(from.as_any_ref().downcast_ref::<HostConfigFailed>()?),
            StructType::HostInDomain => Some(from.as_any_ref().downcast_ref::<HostInDomain>()?),
            StructType::InvalidHostName => Some(from.as_any_ref().downcast_ref::<InvalidHostName>()?),
            StructType::NasConfigFault => Some(from.as_any_ref().downcast_ref::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Some(from.as_any_ref().downcast_ref::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Some(from.as_any_ref().downcast_ref::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Some(from.as_any_ref().downcast_ref::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Some(from.as_any_ref().downcast_ref::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Some(from.as_any_ref().downcast_ref::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Some(from.as_any_ref().downcast_ref::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Some(from.as_any_ref().downcast_ref::<NoPermissionOnNasVolume>()?),
            StructType::NoGateway => Some(from.as_any_ref().downcast_ref::<NoGateway>()?),
            StructType::NoVirtualNic => Some(from.as_any_ref().downcast_ref::<NoVirtualNic>()?),
            StructType::PlatformConfigFault => Some(from.as_any_ref().downcast_ref::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Some(from.as_any_ref().downcast_ref::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Some(from.as_any_ref().downcast_ref::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Some(from.as_any_ref().downcast_ref::<PatchIntegrityError>()?),
            StructType::VmfsMountFault => Some(from.as_any_ref().downcast_ref::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Some(from.as_any_ref().downcast_ref::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Some(from.as_any_ref().downcast_ref::<VmfsAmbiguousMount>()?),
            StructType::HostConnectFault => Some(from.as_any_ref().downcast_ref::<HostConnectFault>()?),
            StructType::AgentInstallFailed => Some(from.as_any_ref().downcast_ref::<AgentInstallFailed>()?),
            StructType::AlreadyBeingManaged => Some(from.as_any_ref().downcast_ref::<AlreadyBeingManaged>()?),
            StructType::AlreadyConnected => Some(from.as_any_ref().downcast_ref::<AlreadyConnected>()?),
            StructType::CannotAddHostWithFtVmAsStandalone => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmAsStandalone>()?),
            StructType::CannotAddHostWithFtVmToDifferentCluster => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmToDifferentCluster>()?),
            StructType::CannotAddHostWithFtVmToNonHaCluster => Some(from.as_any_ref().downcast_ref::<CannotAddHostWithFtVmToNonHaCluster>()?),
            StructType::GatewayConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Some(from.as_any_ref().downcast_ref::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Some(from.as_any_ref().downcast_ref::<GatewayOperationRefused>()?),
            StructType::GatewayToHostConnectFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Some(from.as_any_ref().downcast_ref::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Some(from.as_any_ref().downcast_ref::<GatewayToHostTrustVerifyFault>()?),
            StructType::MultipleCertificatesVerifyFault => Some(from.as_any_ref().downcast_ref::<MultipleCertificatesVerifyFault>()?),
            StructType::NoHost => Some(from.as_any_ref().downcast_ref::<NoHost>()?),
            StructType::NoPermissionOnHost => Some(from.as_any_ref().downcast_ref::<NoPermissionOnHost>()?),
            StructType::NotSupportedHost => Some(from.as_any_ref().downcast_ref::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Some(from.as_any_ref().downcast_ref::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVsan>()?),
            StructType::NotSupportedHostInCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInCluster>()?),
            StructType::EvcAdmissionFailed => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedVmActive>()?),
            StructType::NotSupportedHostInDvs => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInHaCluster>()?),
            StructType::ReadHostResourcePoolTreeFailed => Some(from.as_any_ref().downcast_ref::<ReadHostResourcePoolTreeFailed>()?),
            StructType::SslDisabledFault => Some(from.as_any_ref().downcast_ref::<SslDisabledFault>()?),
            StructType::SslVerifyFault => Some(from.as_any_ref().downcast_ref::<SslVerifyFault>()?),
            StructType::TooManyHosts => Some(from.as_any_ref().downcast_ref::<TooManyHosts>()?),
            StructType::HostHasComponentFailure => Some(from.as_any_ref().downcast_ref::<HostHasComponentFailure>()?),
            StructType::HostIncompatibleForRecordReplay => Some(from.as_any_ref().downcast_ref::<HostIncompatibleForRecordReplay>()?),
            StructType::HostPowerOpFailed => Some(from.as_any_ref().downcast_ref::<HostPowerOpFailed>()?),
            StructType::NoPeerHostFound => Some(from.as_any_ref().downcast_ref::<NoPeerHostFound>()?),
            StructType::VmotionInterfaceNotEnabled => Some(from.as_any_ref().downcast_ref::<VmotionInterfaceNotEnabled>()?),
            StructType::WakeOnLanNotSupportedByVmotionNic => Some(from.as_any_ref().downcast_ref::<WakeOnLanNotSupportedByVmotionNic>()?),
            StructType::HostSpecificationOperationFailed => Some(from.as_any_ref().downcast_ref::<HostSpecificationOperationFailed>()?),
            StructType::HttpFault => Some(from.as_any_ref().downcast_ref::<HttpFault>()?),
            StructType::IormNotSupportedHostOnDatastore => Some(from.as_any_ref().downcast_ref::<IormNotSupportedHostOnDatastore>()?),
            StructType::InaccessibleVFlashSource => Some(from.as_any_ref().downcast_ref::<InaccessibleVFlashSource>()?),
            StructType::InsufficientResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientResourcesFault>()?),
            StructType::InsufficientAgentVmsDeployed => Some(from.as_any_ref().downcast_ref::<InsufficientAgentVmsDeployed>()?),
            StructType::InsufficientCpuResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientCpuResourcesFault>()?),
            StructType::InsufficientFailoverResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientFailoverResourcesFault>()?),
            StructType::InsufficientGraphicsResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientGraphicsResourcesFault>()?),
            StructType::InsufficientHostCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Some(from.as_any_ref().downcast_ref::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientPerCpuCapacity>()?),
            StructType::InsufficientMemoryResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientMemoryResourcesFault>()?),
            StructType::InsufficientNetworkCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientNetworkCapacity>()?),
            StructType::InsufficientNetworkResourcePoolCapacity => Some(from.as_any_ref().downcast_ref::<InsufficientNetworkResourcePoolCapacity>()?),
            StructType::InsufficientStandbyResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Some(from.as_any_ref().downcast_ref::<InsufficientStandbyMemoryResource>()?),
            StructType::InsufficientStorageSpace => Some(from.as_any_ref().downcast_ref::<InsufficientStorageSpace>()?),
            StructType::InsufficientVFlashResourcesFault => Some(from.as_any_ref().downcast_ref::<InsufficientVFlashResourcesFault>()?),
            StructType::InvalidResourcePoolStructureFault => Some(from.as_any_ref().downcast_ref::<InvalidResourcePoolStructureFault>()?),
            StructType::NumVirtualCpusExceedsLimit => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusExceedsLimit>()?),
            StructType::VmFaultToleranceTooManyFtVcpusOnHost => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTooManyFtVcpusOnHost>()?),
            StructType::VmFaultToleranceTooManyVMsOnHost => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceTooManyVMsOnHost>()?),
            StructType::VmSmpFaultToleranceTooManyVMsOnHost => Some(from.as_any_ref().downcast_ref::<VmSmpFaultToleranceTooManyVMsOnHost>()?),
            StructType::InsufficientStorageIops => Some(from.as_any_ref().downcast_ref::<InsufficientStorageIops>()?),
            StructType::InvalidAffinitySettingFault => Some(from.as_any_ref().downcast_ref::<InvalidAffinitySettingFault>()?),
            StructType::InvalidBmcRole => Some(from.as_any_ref().downcast_ref::<InvalidBmcRole>()?),
            StructType::InvalidDatastore => Some(from.as_any_ref().downcast_ref::<InvalidDatastore>()?),
            StructType::DatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Some(from.as_any_ref().downcast_ref::<SwapDatastoreNotWritableOnHost>()?),
            StructType::InaccessibleDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Some(from.as_any_ref().downcast_ref::<InaccessibleFtMetadataDatastore>()?),
            StructType::InvalidDatastorePath => Some(from.as_any_ref().downcast_ref::<InvalidDatastorePath>()?),
            StructType::InvalidEvent => Some(from.as_any_ref().downcast_ref::<InvalidEvent>()?),
            StructType::InvalidFolder => Some(from.as_any_ref().downcast_ref::<InvalidFolder>()?),
            StructType::VmAlreadyExistsInDatacenter => Some(from.as_any_ref().downcast_ref::<VmAlreadyExistsInDatacenter>()?),
            StructType::InvalidIpmiLoginInfo => Some(from.as_any_ref().downcast_ref::<InvalidIpmiLoginInfo>()?),
            StructType::InvalidIpmiMacAddress => Some(from.as_any_ref().downcast_ref::<InvalidIpmiMacAddress>()?),
            StructType::InvalidLicense => Some(from.as_any_ref().downcast_ref::<InvalidLicense>()?),
            StructType::InvalidLocale => Some(from.as_any_ref().downcast_ref::<InvalidLocale>()?),
            StructType::InvalidLogin => Some(from.as_any_ref().downcast_ref::<InvalidLogin>()?),
            StructType::InvalidClientCertificate => Some(from.as_any_ref().downcast_ref::<InvalidClientCertificate>()?),
            StructType::PasswordExpired => Some(from.as_any_ref().downcast_ref::<PasswordExpired>()?),
            StructType::InvalidName => Some(from.as_any_ref().downcast_ref::<InvalidName>()?),
            StructType::InvalidPrivilege => Some(from.as_any_ref().downcast_ref::<InvalidPrivilege>()?),
            StructType::InvalidState => Some(from.as_any_ref().downcast_ref::<InvalidState>()?),
            StructType::CannotPowerOffVmInCluster => Some(from.as_any_ref().downcast_ref::<CannotPowerOffVmInCluster>()?),
            StructType::EncryptionKeyRequired => Some(from.as_any_ref().downcast_ref::<EncryptionKeyRequired>()?),
            StructType::InvalidDatastoreState => Some(from.as_any_ref().downcast_ref::<InvalidDatastoreState>()?),
            StructType::InvalidHostState => Some(from.as_any_ref().downcast_ref::<InvalidHostState>()?),
            StructType::InvalidHostConnectionState => Some(from.as_any_ref().downcast_ref::<InvalidHostConnectionState>()?),
            StructType::InvalidPowerState => Some(from.as_any_ref().downcast_ref::<InvalidPowerState>()?),
            StructType::InvalidVmState => Some(from.as_any_ref().downcast_ref::<InvalidVmState>()?),
            StructType::MksConnectionLimitReached => Some(from.as_any_ref().downcast_ref::<MksConnectionLimitReached>()?),
            StructType::NoActiveHostInCluster => Some(from.as_any_ref().downcast_ref::<NoActiveHostInCluster>()?),
            StructType::OvfConsumerPowerOnFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerPowerOnFault>()?),
            StructType::QuestionPending => Some(from.as_any_ref().downcast_ref::<QuestionPending>()?),
            StructType::VmPowerOnDisabled => Some(from.as_any_ref().downcast_ref::<VmPowerOnDisabled>()?),
            StructType::IscsiFault => Some(from.as_any_ref().downcast_ref::<IscsiFault>()?),
            StructType::IscsiFaultInvalidVnic => Some(from.as_any_ref().downcast_ref::<IscsiFaultInvalidVnic>()?),
            StructType::IscsiFaultPnicInUse => Some(from.as_any_ref().downcast_ref::<IscsiFaultPnicInUse>()?),
            StructType::IscsiFaultVnicAlreadyBound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicAlreadyBound>()?),
            StructType::IscsiFaultVnicHasActivePaths => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasActivePaths>()?),
            StructType::IscsiFaultVnicHasMultipleUplinks => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasMultipleUplinks>()?),
            StructType::IscsiFaultVnicHasNoUplinks => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasNoUplinks>()?),
            StructType::IscsiFaultVnicHasWrongUplink => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicHasWrongUplink>()?),
            StructType::IscsiFaultVnicInUse => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicInUse>()?),
            StructType::IscsiFaultVnicIsLastPath => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicIsLastPath>()?),
            StructType::IscsiFaultVnicNotBound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicNotBound>()?),
            StructType::IscsiFaultVnicNotFound => Some(from.as_any_ref().downcast_ref::<IscsiFaultVnicNotFound>()?),
            StructType::KeyNotFound => Some(from.as_any_ref().downcast_ref::<KeyNotFound>()?),
            StructType::LicenseEntityNotFound => Some(from.as_any_ref().downcast_ref::<LicenseEntityNotFound>()?),
            StructType::LicenseServerUnavailable => Some(from.as_any_ref().downcast_ref::<LicenseServerUnavailable>()?),
            StructType::LimitExceeded => Some(from.as_any_ref().downcast_ref::<LimitExceeded>()?),
            StructType::LogBundlingFailed => Some(from.as_any_ref().downcast_ref::<LogBundlingFailed>()?),
            StructType::MigrationFault => Some(from.as_any_ref().downcast_ref::<MigrationFault>()?),
            StructType::AffinityConfigured => Some(from.as_any_ref().downcast_ref::<AffinityConfigured>()?),
            StructType::CannotModifyConfigCpuRequirements => Some(from.as_any_ref().downcast_ref::<CannotModifyConfigCpuRequirements>()?),
            StructType::CannotMoveVmWithDeltaDisk => Some(from.as_any_ref().downcast_ref::<CannotMoveVmWithDeltaDisk>()?),
            StructType::CannotMoveVmWithNativeDeltaDisk => Some(from.as_any_ref().downcast_ref::<CannotMoveVmWithNativeDeltaDisk>()?),
            StructType::CloneFromSnapshotNotSupported => Some(from.as_any_ref().downcast_ref::<CloneFromSnapshotNotSupported>()?),
            StructType::DatacenterMismatch => Some(from.as_any_ref().downcast_ref::<DatacenterMismatch>()?),
            StructType::DisallowedMigrationDeviceAttached => Some(from.as_any_ref().downcast_ref::<DisallowedMigrationDeviceAttached>()?),
            StructType::DiskMoveTypeNotSupported => Some(from.as_any_ref().downcast_ref::<DiskMoveTypeNotSupported>()?),
            StructType::FaultToleranceAntiAffinityViolated => Some(from.as_any_ref().downcast_ref::<FaultToleranceAntiAffinityViolated>()?),
            StructType::FaultToleranceNeedsThickDisk => Some(from.as_any_ref().downcast_ref::<FaultToleranceNeedsThickDisk>()?),
            StructType::FaultToleranceNotSameBuild => Some(from.as_any_ref().downcast_ref::<FaultToleranceNotSameBuild>()?),
            StructType::HaErrorsAtDest => Some(from.as_any_ref().downcast_ref::<HaErrorsAtDest>()?),
            StructType::IncompatibleDefaultDevice => Some(from.as_any_ref().downcast_ref::<IncompatibleDefaultDevice>()?),
            StructType::LargeRdmConversionNotSupported => Some(from.as_any_ref().downcast_ref::<LargeRdmConversionNotSupported>()?),
            StructType::MaintenanceModeFileMove => Some(from.as_any_ref().downcast_ref::<MaintenanceModeFileMove>()?),
            StructType::MigrationDisabled => Some(from.as_any_ref().downcast_ref::<MigrationDisabled>()?),
            StructType::MigrationFeatureNotSupported => Some(from.as_any_ref().downcast_ref::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Some(from.as_any_ref().downcast_ref::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Some(from.as_any_ref().downcast_ref::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionAcrossNetworkNotSupported>()?),
            StructType::MigrationNotReady => Some(from.as_any_ref().downcast_ref::<MigrationNotReady>()?),
            StructType::MismatchedNetworkPolicies => Some(from.as_any_ref().downcast_ref::<MismatchedNetworkPolicies>()?),
            StructType::MismatchedVMotionNetworkNames => Some(from.as_any_ref().downcast_ref::<MismatchedVMotionNetworkNames>()?),
            StructType::NetworksMayNotBeTheSame => Some(from.as_any_ref().downcast_ref::<NetworksMayNotBeTheSame>()?),
            StructType::NoGuestHeartbeat => Some(from.as_any_ref().downcast_ref::<NoGuestHeartbeat>()?),
            StructType::RdmConversionNotSupported => Some(from.as_any_ref().downcast_ref::<RdmConversionNotSupported>()?),
            StructType::RdmNotPreserved => Some(from.as_any_ref().downcast_ref::<RdmNotPreserved>()?),
            StructType::ReadOnlyDisksWithLegacyDestination => Some(from.as_any_ref().downcast_ref::<ReadOnlyDisksWithLegacyDestination>()?),
            StructType::SnapshotCopyNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Some(from.as_any_ref().downcast_ref::<SnapshotMoveToNonHomeNotSupported>()?),
            StructType::SnapshotRevertIssue => Some(from.as_any_ref().downcast_ref::<SnapshotRevertIssue>()?),
            StructType::SuspendedRelocateNotSupported => Some(from.as_any_ref().downcast_ref::<SuspendedRelocateNotSupported>()?),
            StructType::TooManyDisksOnLegacyHost => Some(from.as_any_ref().downcast_ref::<TooManyDisksOnLegacyHost>()?),
            StructType::ToolsInstallationInProgress => Some(from.as_any_ref().downcast_ref::<ToolsInstallationInProgress>()?),
            StructType::UncommittedUndoableDisk => Some(from.as_any_ref().downcast_ref::<UncommittedUndoableDisk>()?),
            StructType::VMotionInterfaceIssue => Some(from.as_any_ref().downcast_ref::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Some(from.as_any_ref().downcast_ref::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Some(from.as_any_ref().downcast_ref::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Some(from.as_any_ref().downcast_ref::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Some(from.as_any_ref().downcast_ref::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Some(from.as_any_ref().downcast_ref::<VMotionNotSupported>()?),
            StructType::VMotionProtocolIncompatible => Some(from.as_any_ref().downcast_ref::<VMotionProtocolIncompatible>()?),
            StructType::WillLoseHaProtection => Some(from.as_any_ref().downcast_ref::<WillLoseHaProtection>()?),
            StructType::WillModifyConfigCpuRequirements => Some(from.as_any_ref().downcast_ref::<WillModifyConfigCpuRequirements>()?),
            StructType::WillResetSnapshotDirectory => Some(from.as_any_ref().downcast_ref::<WillResetSnapshotDirectory>()?),
            StructType::MismatchedBundle => Some(from.as_any_ref().downcast_ref::<MismatchedBundle>()?),
            StructType::MissingBmcSupport => Some(from.as_any_ref().downcast_ref::<MissingBmcSupport>()?),
            StructType::NamespaceFull => Some(from.as_any_ref().downcast_ref::<NamespaceFull>()?),
            StructType::NamespaceLimitReached => Some(from.as_any_ref().downcast_ref::<NamespaceLimitReached>()?),
            StructType::NamespaceWriteProtected => Some(from.as_any_ref().downcast_ref::<NamespaceWriteProtected>()?),
            StructType::NetworkDisruptedAndConfigRolledBack => Some(from.as_any_ref().downcast_ref::<NetworkDisruptedAndConfigRolledBack>()?),
            StructType::NoClientCertificate => Some(from.as_any_ref().downcast_ref::<NoClientCertificate>()?),
            StructType::NoCompatibleDatastore => Some(from.as_any_ref().downcast_ref::<NoCompatibleDatastore>()?),
            StructType::NoCompatibleHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleHost>()?),
            StructType::NoCompatibleHostWithAccessToDevice => Some(from.as_any_ref().downcast_ref::<NoCompatibleHostWithAccessToDevice>()?),
            StructType::NoConnectedDatastore => Some(from.as_any_ref().downcast_ref::<NoConnectedDatastore>()?),
            StructType::NoDiskFound => Some(from.as_any_ref().downcast_ref::<NoDiskFound>()?),
            StructType::NoSubjectName => Some(from.as_any_ref().downcast_ref::<NoSubjectName>()?),
            StructType::NotFound => Some(from.as_any_ref().downcast_ref::<NotFound>()?),
            StructType::NotSupportedHostForChecksum => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForChecksum>()?),
            StructType::OutOfBounds => Some(from.as_any_ref().downcast_ref::<OutOfBounds>()?),
            StructType::OvfFault => Some(from.as_any_ref().downcast_ref::<OvfFault>()?),
            StructType::OvfConsumerCallbackFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Some(from.as_any_ref().downcast_ref::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Some(from.as_any_ref().downcast_ref::<OvfConsumerUndefinedPrefix>()?),
            StructType::OvfExport => Some(from.as_any_ref().downcast_ref::<OvfExport>()?),
            StructType::ConnectedIso => Some(from.as_any_ref().downcast_ref::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Some(from.as_any_ref().downcast_ref::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Some(from.as_any_ref().downcast_ref::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Some(from.as_any_ref().downcast_ref::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Some(from.as_any_ref().downcast_ref::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Some(from.as_any_ref().downcast_ref::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Some(from.as_any_ref().downcast_ref::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetworkExport>()?),
            StructType::OvfImport => Some(from.as_any_ref().downcast_ref::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Some(from.as_any_ref().downcast_ref::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Some(from.as_any_ref().downcast_ref::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Some(from.as_any_ref().downcast_ref::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Some(from.as_any_ref().downcast_ref::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Some(from.as_any_ref().downcast_ref::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Some(from.as_any_ref().downcast_ref::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDiskProvisioning>()?),
            StructType::OvfInvalidPackage => Some(from.as_any_ref().downcast_ref::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Some(from.as_any_ref().downcast_ref::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Some(from.as_any_ref().downcast_ref::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Some(from.as_any_ref().downcast_ref::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Some(from.as_any_ref().downcast_ref::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Some(from.as_any_ref().downcast_ref::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Some(from.as_any_ref().downcast_ref::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Some(from.as_any_ref().downcast_ref::<OvfElement>()?),
            StructType::OvfDuplicateElement => Some(from.as_any_ref().downcast_ref::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Some(from.as_any_ref().downcast_ref::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Some(from.as_any_ref().downcast_ref::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Some(from.as_any_ref().downcast_ref::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Some(from.as_any_ref().downcast_ref::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Some(from.as_any_ref().downcast_ref::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Some(from.as_any_ref().downcast_ref::<OvfWrongElement>()?),
            StructType::OvfProperty => Some(from.as_any_ref().downcast_ref::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Some(from.as_any_ref().downcast_ref::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Some(from.as_any_ref().downcast_ref::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Some(from.as_any_ref().downcast_ref::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Some(from.as_any_ref().downcast_ref::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Some(from.as_any_ref().downcast_ref::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Some(from.as_any_ref().downcast_ref::<OvfXmlFormat>()?),
            StructType::OvfSystemFault => Some(from.as_any_ref().downcast_ref::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Some(from.as_any_ref().downcast_ref::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Some(from.as_any_ref().downcast_ref::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Some(from.as_any_ref().downcast_ref::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Some(from.as_any_ref().downcast_ref::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Some(from.as_any_ref().downcast_ref::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedDeviceBackingOption>()?),
            StructType::OvfUnsupportedPackage => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Some(from.as_any_ref().downcast_ref::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Some(from.as_any_ref().downcast_ref::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Some(from.as_any_ref().downcast_ref::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Some(from.as_any_ref().downcast_ref::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Some(from.as_any_ref().downcast_ref::<OvfUnsupportedType>()?),
            StructType::PatchBinariesNotFound => Some(from.as_any_ref().downcast_ref::<PatchBinariesNotFound>()?),
            StructType::PatchMetadataInvalid => Some(from.as_any_ref().downcast_ref::<PatchMetadataInvalid>()?),
            StructType::PatchMetadataCorrupted => Some(from.as_any_ref().downcast_ref::<PatchMetadataCorrupted>()?),
            StructType::PatchMetadataNotFound => Some(from.as_any_ref().downcast_ref::<PatchMetadataNotFound>()?),
            StructType::PatchNotApplicable => Some(from.as_any_ref().downcast_ref::<PatchNotApplicable>()?),
            StructType::PatchAlreadyInstalled => Some(from.as_any_ref().downcast_ref::<PatchAlreadyInstalled>()?),
            StructType::PatchMissingDependencies => Some(from.as_any_ref().downcast_ref::<PatchMissingDependencies>()?),
            StructType::PatchSuperseded => Some(from.as_any_ref().downcast_ref::<PatchSuperseded>()?),
            StructType::ProfileUpdateFailed => Some(from.as_any_ref().downcast_ref::<ProfileUpdateFailed>()?),
            StructType::RebootRequired => Some(from.as_any_ref().downcast_ref::<RebootRequired>()?),
            StructType::RecordReplayDisabled => Some(from.as_any_ref().downcast_ref::<RecordReplayDisabled>()?),
            StructType::RemoveFailed => Some(from.as_any_ref().downcast_ref::<RemoveFailed>()?),
            StructType::ReplicationFault => Some(from.as_any_ref().downcast_ref::<ReplicationFault>()?),
            StructType::IncompatibleHostForVmReplication => Some(from.as_any_ref().downcast_ref::<IncompatibleHostForVmReplication>()?),
            StructType::ReplicationConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmConfigFault>()?),
            StructType::ReplicationIncompatibleWithFt => Some(from.as_any_ref().downcast_ref::<ReplicationIncompatibleWithFt>()?),
            StructType::ReplicationInvalidOptions => Some(from.as_any_ref().downcast_ref::<ReplicationInvalidOptions>()?),
            StructType::ReplicationNotSupportedOnHost => Some(from.as_any_ref().downcast_ref::<ReplicationNotSupportedOnHost>()?),
            StructType::ReplicationVmFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Some(from.as_any_ref().downcast_ref::<ReplicationVmInProgressFault>()?),
            StructType::ResourceInUse => Some(from.as_any_ref().downcast_ref::<ResourceInUse>()?),
            StructType::FilterInUse => Some(from.as_any_ref().downcast_ref::<FilterInUse>()?),
            StructType::QuiesceDatastoreIoForHaFailed => Some(from.as_any_ref().downcast_ref::<QuiesceDatastoreIoForHaFailed>()?),
            StructType::ResourceNotAvailable => Some(from.as_any_ref().downcast_ref::<ResourceNotAvailable>()?),
            StructType::SspiChallenge => Some(from.as_any_ref().downcast_ref::<SspiChallenge>()?),
            StructType::ShrinkDiskFault => Some(from.as_any_ref().downcast_ref::<ShrinkDiskFault>()?),
            StructType::SnapshotFault => Some(from.as_any_ref().downcast_ref::<SnapshotFault>()?),
            StructType::ApplicationQuiesceFault => Some(from.as_any_ref().downcast_ref::<ApplicationQuiesceFault>()?),
            StructType::FilesystemQuiesceFault => Some(from.as_any_ref().downcast_ref::<FilesystemQuiesceFault>()?),
            StructType::MemorySnapshotOnIndependentDisk => Some(from.as_any_ref().downcast_ref::<MemorySnapshotOnIndependentDisk>()?),
            StructType::MultipleSnapshotsNotSupported => Some(from.as_any_ref().downcast_ref::<MultipleSnapshotsNotSupported>()?),
            StructType::SnapshotDisabled => Some(from.as_any_ref().downcast_ref::<SnapshotDisabled>()?),
            StructType::SnapshotIncompatibleDeviceInVm => Some(from.as_any_ref().downcast_ref::<SnapshotIncompatibleDeviceInVm>()?),
            StructType::SnapshotLocked => Some(from.as_any_ref().downcast_ref::<SnapshotLocked>()?),
            StructType::SnapshotNoChange => Some(from.as_any_ref().downcast_ref::<SnapshotNoChange>()?),
            StructType::TooManySnapshotLevels => Some(from.as_any_ref().downcast_ref::<TooManySnapshotLevels>()?),
            StructType::SsdDiskNotAvailable => Some(from.as_any_ref().downcast_ref::<SsdDiskNotAvailable>()?),
            StructType::StorageDrsCannotMoveDiskInMultiWriterMode => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveDiskInMultiWriterMode>()?),
            StructType::StorageDrsCannotMoveFtVm => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveFtVm>()?),
            StructType::StorageDrsCannotMoveIndependentDisk => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveIndependentDisk>()?),
            StructType::StorageDrsCannotMoveManuallyPlacedSwapFile => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveManuallyPlacedSwapFile>()?),
            StructType::StorageDrsCannotMoveManuallyPlacedVm => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveManuallyPlacedVm>()?),
            StructType::StorageDrsCannotMoveSharedDisk => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveSharedDisk>()?),
            StructType::StorageDrsCannotMoveTemplate => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveTemplate>()?),
            StructType::StorageDrsCannotMoveVmInUserFolder => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveVmInUserFolder>()?),
            StructType::StorageDrsCannotMoveVmWithMountedCdrom => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveVmWithMountedCdrom>()?),
            StructType::StorageDrsCannotMoveVmWithNoFilesInLayout => Some(from.as_any_ref().downcast_ref::<StorageDrsCannotMoveVmWithNoFilesInLayout>()?),
            StructType::StorageDrsDatacentersCannotShareDatastore => Some(from.as_any_ref().downcast_ref::<StorageDrsDatacentersCannotShareDatastore>()?),
            StructType::StorageDrsDisabledOnVm => Some(from.as_any_ref().downcast_ref::<StorageDrsDisabledOnVm>()?),
            StructType::StorageDrsHbrDiskNotMovable => Some(from.as_any_ref().downcast_ref::<StorageDrsHbrDiskNotMovable>()?),
            StructType::StorageDrsHmsMoveInProgress => Some(from.as_any_ref().downcast_ref::<StorageDrsHmsMoveInProgress>()?),
            StructType::StorageDrsHmsUnreachable => Some(from.as_any_ref().downcast_ref::<StorageDrsHmsUnreachable>()?),
            StructType::StorageDrsIolbDisabledInternally => Some(from.as_any_ref().downcast_ref::<StorageDrsIolbDisabledInternally>()?),
            StructType::StorageDrsRelocateDisabled => Some(from.as_any_ref().downcast_ref::<StorageDrsRelocateDisabled>()?),
            StructType::StorageDrsStaleHmsCollection => Some(from.as_any_ref().downcast_ref::<StorageDrsStaleHmsCollection>()?),
            StructType::StorageDrsUnableToMoveFiles => Some(from.as_any_ref().downcast_ref::<StorageDrsUnableToMoveFiles>()?),
            StructType::SwapDatastoreUnset => Some(from.as_any_ref().downcast_ref::<SwapDatastoreUnset>()?),
            StructType::TaskInProgress => Some(from.as_any_ref().downcast_ref::<TaskInProgress>()?),
            StructType::VAppTaskInProgress => Some(from.as_any_ref().downcast_ref::<VAppTaskInProgress>()?),
            StructType::Timedout => Some(from.as_any_ref().downcast_ref::<Timedout>()?),
            StructType::PowerOnFtSecondaryTimedout => Some(from.as_any_ref().downcast_ref::<PowerOnFtSecondaryTimedout>()?),
            StructType::TooManyConsecutiveOverrides => Some(from.as_any_ref().downcast_ref::<TooManyConsecutiveOverrides>()?),
            StructType::ToolsUnavailable => Some(from.as_any_ref().downcast_ref::<ToolsUnavailable>()?),
            StructType::UnrecognizedHost => Some(from.as_any_ref().downcast_ref::<UnrecognizedHost>()?),
            StructType::UnsupportedVimApiVersion => Some(from.as_any_ref().downcast_ref::<UnsupportedVimApiVersion>()?),
            StructType::UserNotFound => Some(from.as_any_ref().downcast_ref::<UserNotFound>()?),
            StructType::VAppConfigFault => Some(from.as_any_ref().downcast_ref::<VAppConfigFault>()?),
            StructType::MissingPowerOffConfiguration => Some(from.as_any_ref().downcast_ref::<MissingPowerOffConfiguration>()?),
            StructType::MissingPowerOnConfiguration => Some(from.as_any_ref().downcast_ref::<MissingPowerOnConfiguration>()?),
            StructType::NoVmInVApp => Some(from.as_any_ref().downcast_ref::<NoVmInVApp>()?),
            StructType::VFlashModuleVersionIncompatible => Some(from.as_any_ref().downcast_ref::<VFlashModuleVersionIncompatible>()?),
            StructType::VmConfigFault => Some(from.as_any_ref().downcast_ref::<VmConfigFault>()?),
            StructType::CannotAccessVmComponent => Some(from.as_any_ref().downcast_ref::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Some(from.as_any_ref().downcast_ref::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Some(from.as_any_ref().downcast_ref::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Some(from.as_any_ref().downcast_ref::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Some(from.as_any_ref().downcast_ref::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Some(from.as_any_ref().downcast_ref::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Some(from.as_any_ref().downcast_ref::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Some(from.as_any_ref().downcast_ref::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Some(from.as_any_ref().downcast_ref::<RdmPointsToInaccessibleDisk>()?),
            StructType::CannotDisableSnapshot => Some(from.as_any_ref().downcast_ref::<CannotDisableSnapshot>()?),
            StructType::CannotUseNetwork => Some(from.as_any_ref().downcast_ref::<CannotUseNetwork>()?),
            StructType::CpuHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<CpuHotPlugNotSupported>()?),
            StructType::DeltaDiskFormatNotSupported => Some(from.as_any_ref().downcast_ref::<DeltaDiskFormatNotSupported>()?),
            StructType::EightHostLimitViolated => Some(from.as_any_ref().downcast_ref::<EightHostLimitViolated>()?),
            StructType::FaultToleranceCannotEditMem => Some(from.as_any_ref().downcast_ref::<FaultToleranceCannotEditMem>()?),
            StructType::GenericVmConfigFault => Some(from.as_any_ref().downcast_ref::<GenericVmConfigFault>()?),
            StructType::InvalidFormat => Some(from.as_any_ref().downcast_ref::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Some(from.as_any_ref().downcast_ref::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Some(from.as_any_ref().downcast_ref::<InvalidSnapshotFormat>()?),
            StructType::InvalidVmConfig => Some(from.as_any_ref().downcast_ref::<InvalidVmConfig>()?),
            StructType::InvalidDeviceSpec => Some(from.as_any_ref().downcast_ref::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Some(from.as_any_ref().downcast_ref::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Some(from.as_any_ref().downcast_ref::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Some(from.as_any_ref().downcast_ref::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Some(from.as_any_ref().downcast_ref::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Some(from.as_any_ref().downcast_ref::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Some(from.as_any_ref().downcast_ref::<InvalidDeviceOperation>()?),
            StructType::MissingController => Some(from.as_any_ref().downcast_ref::<MissingController>()?),
            StructType::SwapPlacementOverrideNotSupported => Some(from.as_any_ref().downcast_ref::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Some(from.as_any_ref().downcast_ref::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Some(from.as_any_ref().downcast_ref::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Some(from.as_any_ref().downcast_ref::<VmWwnConflict>()?),
            StructType::LargeRdmNotSupportedOnDatastore => Some(from.as_any_ref().downcast_ref::<LargeRdmNotSupportedOnDatastore>()?),
            StructType::MemoryHotPlugNotSupported => Some(from.as_any_ref().downcast_ref::<MemoryHotPlugNotSupported>()?),
            StructType::NoCompatibleHardAffinityHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleHardAffinityHost>()?),
            StructType::NoCompatibleSoftAffinityHost => Some(from.as_any_ref().downcast_ref::<NoCompatibleSoftAffinityHost>()?),
            StructType::NumVirtualCpusIncompatible => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusIncompatible>()?),
            StructType::OvfConsumerValidationFault => Some(from.as_any_ref().downcast_ref::<OvfConsumerValidationFault>()?),
            StructType::QuarantineModeFault => Some(from.as_any_ref().downcast_ref::<QuarantineModeFault>()?),
            StructType::RdmNotSupportedOnDatastore => Some(from.as_any_ref().downcast_ref::<RdmNotSupportedOnDatastore>()?),
            StructType::RuleViolation => Some(from.as_any_ref().downcast_ref::<RuleViolation>()?),
            StructType::SoftRuleVioCorrectionDisallowed => Some(from.as_any_ref().downcast_ref::<SoftRuleVioCorrectionDisallowed>()?),
            StructType::SoftRuleVioCorrectionImpact => Some(from.as_any_ref().downcast_ref::<SoftRuleVioCorrectionImpact>()?),
            StructType::UnsupportedDatastore => Some(from.as_any_ref().downcast_ref::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Some(from.as_any_ref().downcast_ref::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Some(from.as_any_ref().downcast_ref::<UnSupportedDatastoreForVFlash>()?),
            StructType::UnsupportedVmxLocation => Some(from.as_any_ref().downcast_ref::<UnsupportedVmxLocation>()?),
            StructType::VAppNotRunning => Some(from.as_any_ref().downcast_ref::<VAppNotRunning>()?),
            StructType::VAppPropertyFault => Some(from.as_any_ref().downcast_ref::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Some(from.as_any_ref().downcast_ref::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Some(from.as_any_ref().downcast_ref::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Some(from.as_any_ref().downcast_ref::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Some(from.as_any_ref().downcast_ref::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Some(from.as_any_ref().downcast_ref::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Some(from.as_any_ref().downcast_ref::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Some(from.as_any_ref().downcast_ref::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Some(from.as_any_ref().downcast_ref::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Some(from.as_any_ref().downcast_ref::<NotUserConfigurableProperty>()?),
            StructType::VFlashCacheHotConfigNotSupported => Some(from.as_any_ref().downcast_ref::<VFlashCacheHotConfigNotSupported>()?),
            StructType::VFlashModuleNotSupported => Some(from.as_any_ref().downcast_ref::<VFlashModuleNotSupported>()?),
            StructType::VirtualHardwareCompatibilityIssue => Some(from.as_any_ref().downcast_ref::<VirtualHardwareCompatibilityIssue>()?),
            StructType::CpuIncompatible => Some(from.as_any_ref().downcast_ref::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Some(from.as_any_ref().downcast_ref::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Some(from.as_any_ref().downcast_ref::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Some(from.as_any_ref().downcast_ref::<FaultToleranceCpuIncompatible>()?),
            StructType::DeviceNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Some(from.as_any_ref().downcast_ref::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Some(from.as_any_ref().downcast_ref::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Some(from.as_any_ref().downcast_ref::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Some(from.as_any_ref().downcast_ref::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Some(from.as_any_ref().downcast_ref::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Some(from.as_any_ref().downcast_ref::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Some(from.as_any_ref().downcast_ref::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Some(from.as_any_ref().downcast_ref::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Some(from.as_any_ref().downcast_ref::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Some(from.as_any_ref().downcast_ref::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Some(from.as_any_ref().downcast_ref::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Some(from.as_any_ref().downcast_ref::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Some(from.as_any_ref().downcast_ref::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Some(from.as_any_ref().downcast_ref::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualEthernetCardNotSupported>()?),
            StructType::DiskNotSupported => Some(from.as_any_ref().downcast_ref::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Some(from.as_any_ref().downcast_ref::<IdeDiskNotSupported>()?),
            StructType::DrsVmotionIncompatibleFault => Some(from.as_any_ref().downcast_ref::<DrsVmotionIncompatibleFault>()?),
            StructType::FeatureRequirementsNotMet => Some(from.as_any_ref().downcast_ref::<FeatureRequirementsNotMet>()?),
            StructType::MemorySizeNotRecommended => Some(from.as_any_ref().downcast_ref::<MemorySizeNotRecommended>()?),
            StructType::MemorySizeNotSupported => Some(from.as_any_ref().downcast_ref::<MemorySizeNotSupported>()?),
            StructType::MemorySizeNotSupportedByDatastore => Some(from.as_any_ref().downcast_ref::<MemorySizeNotSupportedByDatastore>()?),
            StructType::NotEnoughCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Some(from.as_any_ref().downcast_ref::<NotEnoughLogicalCpus>()?),
            StructType::NumVirtualCoresPerSocketNotSupported => Some(from.as_any_ref().downcast_ref::<NumVirtualCoresPerSocketNotSupported>()?),
            StructType::NumVirtualCpusNotSupported => Some(from.as_any_ref().downcast_ref::<NumVirtualCpusNotSupported>()?),
            StructType::StorageVmotionIncompatible => Some(from.as_any_ref().downcast_ref::<StorageVmotionIncompatible>()?),
            StructType::VirtualHardwareVersionNotSupported => Some(from.as_any_ref().downcast_ref::<VirtualHardwareVersionNotSupported>()?),
            StructType::WakeOnLanNotSupported => Some(from.as_any_ref().downcast_ref::<WakeOnLanNotSupported>()?),
            StructType::VmConfigIncompatibleForFaultTolerance => Some(from.as_any_ref().downcast_ref::<VmConfigIncompatibleForFaultTolerance>()?),
            StructType::VmConfigIncompatibleForRecordReplay => Some(from.as_any_ref().downcast_ref::<VmConfigIncompatibleForRecordReplay>()?),
            StructType::VmHostAffinityRuleViolation => Some(from.as_any_ref().downcast_ref::<VmHostAffinityRuleViolation>()?),
            StructType::VmFaultToleranceIssue => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceIssue>()?),
            StructType::CannotChangeDrsBehaviorForFtSecondary => Some(from.as_any_ref().downcast_ref::<CannotChangeDrsBehaviorForFtSecondary>()?),
            StructType::CannotChangeHaSettingsForFtSecondary => Some(from.as_any_ref().downcast_ref::<CannotChangeHaSettingsForFtSecondary>()?),
            StructType::CannotComputeFtCompatibleHosts => Some(from.as_any_ref().downcast_ref::<CannotComputeFtCompatibleHosts>()?),
            StructType::FaultToleranceNotLicensed => Some(from.as_any_ref().downcast_ref::<FaultToleranceNotLicensed>()?),
            StructType::FaultTolerancePrimaryPowerOnNotAttempted => Some(from.as_any_ref().downcast_ref::<FaultTolerancePrimaryPowerOnNotAttempted>()?),
            StructType::FtIssuesOnHost => Some(from.as_any_ref().downcast_ref::<FtIssuesOnHost>()?),
            StructType::HostIncompatibleForFaultTolerance => Some(from.as_any_ref().downcast_ref::<HostIncompatibleForFaultTolerance>()?),
            StructType::IncompatibleHostForFtSecondary => Some(from.as_any_ref().downcast_ref::<IncompatibleHostForFtSecondary>()?),
            StructType::InvalidOperationOnSecondaryVm => Some(from.as_any_ref().downcast_ref::<InvalidOperationOnSecondaryVm>()?),
            StructType::NoHostSuitableForFtSecondary => Some(from.as_any_ref().downcast_ref::<NoHostSuitableForFtSecondary>()?),
            StructType::NotSupportedDeviceForFt => Some(from.as_any_ref().downcast_ref::<NotSupportedDeviceForFt>()?),
            StructType::PowerOnFtSecondaryFailed => Some(from.as_any_ref().downcast_ref::<PowerOnFtSecondaryFailed>()?),
            StructType::SecondaryVmAlreadyDisabled => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyDisabled>()?),
            StructType::SecondaryVmAlreadyEnabled => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyEnabled>()?),
            StructType::SecondaryVmAlreadyRegistered => Some(from.as_any_ref().downcast_ref::<SecondaryVmAlreadyRegistered>()?),
            StructType::SecondaryVmNotRegistered => Some(from.as_any_ref().downcast_ref::<SecondaryVmNotRegistered>()?),
            StructType::VmFaultToleranceConfigIssue => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceConfigIssue>()?),
            StructType::VmFaultToleranceConfigIssueWrapper => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceConfigIssueWrapper>()?),
            StructType::VmFaultToleranceInvalidFileBacking => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceInvalidFileBacking>()?),
            StructType::VmFaultToleranceOpIssuesList => Some(from.as_any_ref().downcast_ref::<VmFaultToleranceOpIssuesList>()?),
            StructType::VmMetadataManagerFault => Some(from.as_any_ref().downcast_ref::<VmMetadataManagerFault>()?),
            StructType::VmMonitorIncompatibleForFaultTolerance => Some(from.as_any_ref().downcast_ref::<VmMonitorIncompatibleForFaultTolerance>()?),
            StructType::VmToolsUpgradeFault => Some(from.as_any_ref().downcast_ref::<VmToolsUpgradeFault>()?),
            StructType::ToolsAlreadyUpgraded => Some(from.as_any_ref().downcast_ref::<ToolsAlreadyUpgraded>()?),
            StructType::ToolsAutoUpgradeNotSupported => Some(from.as_any_ref().downcast_ref::<ToolsAutoUpgradeNotSupported>()?),
            StructType::ToolsImageCopyFailed => Some(from.as_any_ref().downcast_ref::<ToolsImageCopyFailed>()?),
            StructType::ToolsImageNotAvailable => Some(from.as_any_ref().downcast_ref::<ToolsImageNotAvailable>()?),
            StructType::ToolsImageSignatureCheckFailed => Some(from.as_any_ref().downcast_ref::<ToolsImageSignatureCheckFailed>()?),
            StructType::ToolsUpgradeCancelled => Some(from.as_any_ref().downcast_ref::<ToolsUpgradeCancelled>()?),
            StructType::VmValidateMaxDevice => Some(from.as_any_ref().downcast_ref::<VmValidateMaxDevice>()?),
            StructType::VsanFault => Some(from.as_any_ref().downcast_ref::<VsanFault>()?),
            StructType::CannotChangeVsanClusterUuid => Some(from.as_any_ref().downcast_ref::<CannotChangeVsanClusterUuid>()?),
            StructType::CannotChangeVsanNodeUuid => Some(from.as_any_ref().downcast_ref::<CannotChangeVsanNodeUuid>()?),
            StructType::CannotMoveVsanEnabledHost => Some(from.as_any_ref().downcast_ref::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Some(from.as_any_ref().downcast_ref::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Some(from.as_any_ref().downcast_ref::<VsanClusterUuidMismatch>()?),
            StructType::CannotReconfigureVsanWhenHaEnabled => Some(from.as_any_ref().downcast_ref::<CannotReconfigureVsanWhenHaEnabled>()?),
            StructType::DuplicateVsanNetworkInterface => Some(from.as_any_ref().downcast_ref::<DuplicateVsanNetworkInterface>()?),
            StructType::VsanDiskFault => Some(from.as_any_ref().downcast_ref::<VsanDiskFault>()?),
            StructType::DiskHasPartitions => Some(from.as_any_ref().downcast_ref::<DiskHasPartitions>()?),
            StructType::DiskIsLastRemainingNonSsd => Some(from.as_any_ref().downcast_ref::<DiskIsLastRemainingNonSsd>()?),
            StructType::DiskIsNonLocal => Some(from.as_any_ref().downcast_ref::<DiskIsNonLocal>()?),
            StructType::DiskIsUsb => Some(from.as_any_ref().downcast_ref::<DiskIsUsb>()?),
            StructType::DiskTooSmall => Some(from.as_any_ref().downcast_ref::<DiskTooSmall>()?),
            StructType::DuplicateDisks => Some(from.as_any_ref().downcast_ref::<DuplicateDisks>()?),
            StructType::InsufficientDisks => Some(from.as_any_ref().downcast_ref::<InsufficientDisks>()?),
            StructType::VsanIncompatibleDiskMapping => Some(from.as_any_ref().downcast_ref::<VsanIncompatibleDiskMapping>()?),
            StructType::WipeDiskFault => Some(from.as_any_ref().downcast_ref::<WipeDiskFault>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VimFault => Ok(from.as_any_box().downcast::<VimFault>()?),
            StructType::ActiveDirectoryFault => Ok(from.as_any_box().downcast::<ActiveDirectoryFault>()?),
            StructType::DomainNotFound => Ok(from.as_any_box().downcast::<DomainNotFound>()?),
            StructType::InvalidCamServer => Ok(from.as_any_box().downcast::<InvalidCamServer>()?),
            StructType::CamServerRefusedConnection => Ok(from.as_any_box().downcast::<CamServerRefusedConnection>()?),
            StructType::InvalidCamCertificate => Ok(from.as_any_box().downcast::<InvalidCamCertificate>()?),
            StructType::NoPermissionOnAd => Ok(from.as_any_box().downcast::<NoPermissionOnAd>()?),
            StructType::NonAdUserRequired => Ok(from.as_any_box().downcast::<NonAdUserRequired>()?),
            StructType::AlreadyExists => Ok(from.as_any_box().downcast::<AlreadyExists>()?),
            StructType::AlreadyUpgraded => Ok(from.as_any_box().downcast::<AlreadyUpgraded>()?),
            StructType::AnswerFileUpdateFailed => Ok(from.as_any_box().downcast::<AnswerFileUpdateFailed>()?),
            StructType::AuthMinimumAdminPermission => Ok(from.as_any_box().downcast::<AuthMinimumAdminPermission>()?),
            StructType::CannotAccessLocalSource => Ok(from.as_any_box().downcast::<CannotAccessLocalSource>()?),
            StructType::CannotDisconnectHostWithFaultToleranceVm => Ok(from.as_any_box().downcast::<CannotDisconnectHostWithFaultToleranceVm>()?),
            StructType::CannotEnableVmcpForCluster => Ok(from.as_any_box().downcast::<CannotEnableVmcpForCluster>()?),
            StructType::CannotMoveFaultToleranceVm => Ok(from.as_any_box().downcast::<CannotMoveFaultToleranceVm>()?),
            StructType::CannotMoveHostWithFaultToleranceVm => Ok(from.as_any_box().downcast::<CannotMoveHostWithFaultToleranceVm>()?),
            StructType::CannotPlaceWithoutPrerequisiteMoves => Ok(from.as_any_box().downcast::<CannotPlaceWithoutPrerequisiteMoves>()?),
            StructType::ConcurrentAccess => Ok(from.as_any_box().downcast::<ConcurrentAccess>()?),
            StructType::CustomizationFault => Ok(from.as_any_box().downcast::<CustomizationFault>()?),
            StructType::CannotDecryptPasswords => Ok(from.as_any_box().downcast::<CannotDecryptPasswords>()?),
            StructType::CustomizationPending => Ok(from.as_any_box().downcast::<CustomizationPending>()?),
            StructType::IpHostnameGeneratorError => Ok(from.as_any_box().downcast::<IpHostnameGeneratorError>()?),
            StructType::LinuxVolumeNotClean => Ok(from.as_any_box().downcast::<LinuxVolumeNotClean>()?),
            StructType::MissingLinuxCustResources => Ok(from.as_any_box().downcast::<MissingLinuxCustResources>()?),
            StructType::MissingWindowsCustResources => Ok(from.as_any_box().downcast::<MissingWindowsCustResources>()?),
            StructType::MountError => Ok(from.as_any_box().downcast::<MountError>()?),
            StructType::NicSettingMismatch => Ok(from.as_any_box().downcast::<NicSettingMismatch>()?),
            StructType::NoDisksToCustomize => Ok(from.as_any_box().downcast::<NoDisksToCustomize>()?),
            StructType::UncustomizableGuest => Ok(from.as_any_box().downcast::<UncustomizableGuest>()?),
            StructType::UnexpectedCustomizationFault => Ok(from.as_any_box().downcast::<UnexpectedCustomizationFault>()?),
            StructType::VolumeEditorError => Ok(from.as_any_box().downcast::<VolumeEditorError>()?),
            StructType::DasConfigFault => Ok(from.as_any_box().downcast::<DasConfigFault>()?),
            StructType::DrsDisabledOnVm => Ok(from.as_any_box().downcast::<DrsDisabledOnVm>()?),
            StructType::DuplicateName => Ok(from.as_any_box().downcast::<DuplicateName>()?),
            StructType::DvsFault => Ok(from.as_any_box().downcast::<DvsFault>()?),
            StructType::BackupBlobReadFailure => Ok(from.as_any_box().downcast::<BackupBlobReadFailure>()?),
            StructType::BackupBlobWriteFailure => Ok(from.as_any_box().downcast::<BackupBlobWriteFailure>()?),
            StructType::CollectorAddressUnset => Ok(from.as_any_box().downcast::<CollectorAddressUnset>()?),
            StructType::ConflictingConfiguration => Ok(from.as_any_box().downcast::<ConflictingConfiguration>()?),
            StructType::DvsApplyOperationFault => Ok(from.as_any_box().downcast::<DvsApplyOperationFault>()?),
            StructType::DvsNotAuthorized => Ok(from.as_any_box().downcast::<DvsNotAuthorized>()?),
            StructType::DvsOperationBulkFault => Ok(from.as_any_box().downcast::<DvsOperationBulkFault>()?),
            StructType::DvsScopeViolated => Ok(from.as_any_box().downcast::<DvsScopeViolated>()?),
            StructType::ImportHostAddFailure => Ok(from.as_any_box().downcast::<ImportHostAddFailure>()?),
            StructType::ImportOperationBulkFault => Ok(from.as_any_box().downcast::<ImportOperationBulkFault>()?),
            StructType::InvalidIpfixConfig => Ok(from.as_any_box().downcast::<InvalidIpfixConfig>()?),
            StructType::RollbackFailure => Ok(from.as_any_box().downcast::<RollbackFailure>()?),
            StructType::SwitchIpUnset => Ok(from.as_any_box().downcast::<SwitchIpUnset>()?),
            StructType::SwitchNotInUpgradeMode => Ok(from.as_any_box().downcast::<SwitchNotInUpgradeMode>()?),
            StructType::VspanDestPortConflict => Ok(from.as_any_box().downcast::<VspanDestPortConflict>()?),
            StructType::VspanPortConflict => Ok(from.as_any_box().downcast::<VspanPortConflict>()?),
            StructType::VspanPortMoveFault => Ok(from.as_any_box().downcast::<VspanPortMoveFault>()?),
            StructType::VspanPortPromiscChangeFault => Ok(from.as_any_box().downcast::<VspanPortPromiscChangeFault>()?),
            StructType::VspanPortgroupPromiscChangeFault => Ok(from.as_any_box().downcast::<VspanPortgroupPromiscChangeFault>()?),
            StructType::VspanPortgroupTypeChangeFault => Ok(from.as_any_box().downcast::<VspanPortgroupTypeChangeFault>()?),
            StructType::VspanPromiscuousPortNotSupported => Ok(from.as_any_box().downcast::<VspanPromiscuousPortNotSupported>()?),
            StructType::VspanSameSessionPortConflict => Ok(from.as_any_box().downcast::<VspanSameSessionPortConflict>()?),
            StructType::EvcConfigFault => Ok(from.as_any_box().downcast::<EvcConfigFault>()?),
            StructType::ActiveVMsBlockingEvc => Ok(from.as_any_box().downcast::<ActiveVMsBlockingEvc>()?),
            StructType::DisconnectedHostsBlockingEvc => Ok(from.as_any_box().downcast::<DisconnectedHostsBlockingEvc>()?),
            StructType::EvcModeIllegalByVendor => Ok(from.as_any_box().downcast::<EvcModeIllegalByVendor>()?),
            StructType::EvcModeUnsupportedByHosts => Ok(from.as_any_box().downcast::<EvcModeUnsupportedByHosts>()?),
            StructType::EvcUnsupportedByHostHardware => Ok(from.as_any_box().downcast::<EvcUnsupportedByHostHardware>()?),
            StructType::EvcUnsupportedByHostSoftware => Ok(from.as_any_box().downcast::<EvcUnsupportedByHostSoftware>()?),
            StructType::HeterogenousHostsBlockingEvc => Ok(from.as_any_box().downcast::<HeterogenousHostsBlockingEvc>()?),
            StructType::ExtendedFault => Ok(from.as_any_box().downcast::<ExtendedFault>()?),
            StructType::FaultToleranceVmNotDasProtected => Ok(from.as_any_box().downcast::<FaultToleranceVmNotDasProtected>()?),
            StructType::FcoeFault => Ok(from.as_any_box().downcast::<FcoeFault>()?),
            StructType::FcoeFaultPnicHasNoPortSet => Ok(from.as_any_box().downcast::<FcoeFaultPnicHasNoPortSet>()?),
            StructType::FileFault => Ok(from.as_any_box().downcast::<FileFault>()?),
            StructType::CannotAccessFile => Ok(from.as_any_box().downcast::<CannotAccessFile>()?),
            StructType::CannotCreateFile => Ok(from.as_any_box().downcast::<CannotCreateFile>()?),
            StructType::CannotDeleteFile => Ok(from.as_any_box().downcast::<CannotDeleteFile>()?),
            StructType::DirectoryNotEmpty => Ok(from.as_any_box().downcast::<DirectoryNotEmpty>()?),
            StructType::FileAlreadyExists => Ok(from.as_any_box().downcast::<FileAlreadyExists>()?),
            StructType::FileLocked => Ok(from.as_any_box().downcast::<FileLocked>()?),
            StructType::FileNameTooLong => Ok(from.as_any_box().downcast::<FileNameTooLong>()?),
            StructType::FileNotFound => Ok(from.as_any_box().downcast::<FileNotFound>()?),
            StructType::FileNotWritable => Ok(from.as_any_box().downcast::<FileNotWritable>()?),
            StructType::FileTooLarge => Ok(from.as_any_box().downcast::<FileTooLarge>()?),
            StructType::IncorrectFileType => Ok(from.as_any_box().downcast::<IncorrectFileType>()?),
            StructType::NetworkCopyFault => Ok(from.as_any_box().downcast::<NetworkCopyFault>()?),
            StructType::NoDiskSpace => Ok(from.as_any_box().downcast::<NoDiskSpace>()?),
            StructType::NotADirectory => Ok(from.as_any_box().downcast::<NotADirectory>()?),
            StructType::NotAFile => Ok(from.as_any_box().downcast::<NotAFile>()?),
            StructType::TooManyConcurrentNativeClones => Ok(from.as_any_box().downcast::<TooManyConcurrentNativeClones>()?),
            StructType::TooManyNativeCloneLevels => Ok(from.as_any_box().downcast::<TooManyNativeCloneLevels>()?),
            StructType::TooManyNativeClonesOnFile => Ok(from.as_any_box().downcast::<TooManyNativeClonesOnFile>()?),
            StructType::GenericDrsFault => Ok(from.as_any_box().downcast::<GenericDrsFault>()?),
            StructType::GuestOperationsFault => Ok(from.as_any_box().downcast::<GuestOperationsFault>()?),
            StructType::GuestAuthenticationChallenge => Ok(from.as_any_box().downcast::<GuestAuthenticationChallenge>()?),
            StructType::GuestComponentsOutOfDate => Ok(from.as_any_box().downcast::<GuestComponentsOutOfDate>()?),
            StructType::GuestMultipleMappings => Ok(from.as_any_box().downcast::<GuestMultipleMappings>()?),
            StructType::GuestOperationsUnavailable => Ok(from.as_any_box().downcast::<GuestOperationsUnavailable>()?),
            StructType::GuestPermissionDenied => Ok(from.as_any_box().downcast::<GuestPermissionDenied>()?),
            StructType::GuestProcessNotFound => Ok(from.as_any_box().downcast::<GuestProcessNotFound>()?),
            StructType::GuestRegistryFault => Ok(from.as_any_box().downcast::<GuestRegistryFault>()?),
            StructType::GuestRegistryKeyFault => Ok(from.as_any_box().downcast::<GuestRegistryKeyFault>()?),
            StructType::GuestRegistryKeyAlreadyExists => Ok(from.as_any_box().downcast::<GuestRegistryKeyAlreadyExists>()?),
            StructType::GuestRegistryKeyHasSubkeys => Ok(from.as_any_box().downcast::<GuestRegistryKeyHasSubkeys>()?),
            StructType::GuestRegistryKeyInvalid => Ok(from.as_any_box().downcast::<GuestRegistryKeyInvalid>()?),
            StructType::GuestRegistryKeyParentVolatile => Ok(from.as_any_box().downcast::<GuestRegistryKeyParentVolatile>()?),
            StructType::GuestRegistryValueFault => Ok(from.as_any_box().downcast::<GuestRegistryValueFault>()?),
            StructType::GuestRegistryValueNotFound => Ok(from.as_any_box().downcast::<GuestRegistryValueNotFound>()?),
            StructType::InvalidGuestLogin => Ok(from.as_any_box().downcast::<InvalidGuestLogin>()?),
            StructType::OperationDisabledByGuest => Ok(from.as_any_box().downcast::<OperationDisabledByGuest>()?),
            StructType::OperationNotSupportedByGuest => Ok(from.as_any_box().downcast::<OperationNotSupportedByGuest>()?),
            StructType::TooManyGuestLogons => Ok(from.as_any_box().downcast::<TooManyGuestLogons>()?),
            StructType::HostConfigFault => Ok(from.as_any_box().downcast::<HostConfigFault>()?),
            StructType::AdminDisabled => Ok(from.as_any_box().downcast::<AdminDisabled>()?),
            StructType::AdminNotDisabled => Ok(from.as_any_box().downcast::<AdminNotDisabled>()?),
            StructType::BlockedByFirewall => Ok(from.as_any_box().downcast::<BlockedByFirewall>()?),
            StructType::ClockSkew => Ok(from.as_any_box().downcast::<ClockSkew>()?),
            StructType::DisableAdminNotSupported => Ok(from.as_any_box().downcast::<DisableAdminNotSupported>()?),
            StructType::HostConfigFailed => Ok(from.as_any_box().downcast::<HostConfigFailed>()?),
            StructType::HostInDomain => Ok(from.as_any_box().downcast::<HostInDomain>()?),
            StructType::InvalidHostName => Ok(from.as_any_box().downcast::<InvalidHostName>()?),
            StructType::NasConfigFault => Ok(from.as_any_box().downcast::<NasConfigFault>()?),
            StructType::InvalidNasCredentials => Ok(from.as_any_box().downcast::<InvalidNasCredentials>()?),
            StructType::InvalidNetworkResource => Ok(from.as_any_box().downcast::<InvalidNetworkResource>()?),
            StructType::NasConnectionLimitReached => Ok(from.as_any_box().downcast::<NasConnectionLimitReached>()?),
            StructType::NasSessionCredentialConflict => Ok(from.as_any_box().downcast::<NasSessionCredentialConflict>()?),
            StructType::NasVolumeNotMounted => Ok(from.as_any_box().downcast::<NasVolumeNotMounted>()?),
            StructType::NetworkInaccessible => Ok(from.as_any_box().downcast::<NetworkInaccessible>()?),
            StructType::NoPermissionOnNasVolume => Ok(from.as_any_box().downcast::<NoPermissionOnNasVolume>()?),
            StructType::NoGateway => Ok(from.as_any_box().downcast::<NoGateway>()?),
            StructType::NoVirtualNic => Ok(from.as_any_box().downcast::<NoVirtualNic>()?),
            StructType::PlatformConfigFault => Ok(from.as_any_box().downcast::<PlatformConfigFault>()?),
            StructType::InvalidBundle => Ok(from.as_any_box().downcast::<InvalidBundle>()?),
            StructType::PatchInstallFailed => Ok(from.as_any_box().downcast::<PatchInstallFailed>()?),
            StructType::PatchIntegrityError => Ok(from.as_any_box().downcast::<PatchIntegrityError>()?),
            StructType::VmfsMountFault => Ok(from.as_any_box().downcast::<VmfsMountFault>()?),
            StructType::VmfsAlreadyMounted => Ok(from.as_any_box().downcast::<VmfsAlreadyMounted>()?),
            StructType::VmfsAmbiguousMount => Ok(from.as_any_box().downcast::<VmfsAmbiguousMount>()?),
            StructType::HostConnectFault => Ok(from.as_any_box().downcast::<HostConnectFault>()?),
            StructType::AgentInstallFailed => Ok(from.as_any_box().downcast::<AgentInstallFailed>()?),
            StructType::AlreadyBeingManaged => Ok(from.as_any_box().downcast::<AlreadyBeingManaged>()?),
            StructType::AlreadyConnected => Ok(from.as_any_box().downcast::<AlreadyConnected>()?),
            StructType::CannotAddHostWithFtVmAsStandalone => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmAsStandalone>()?),
            StructType::CannotAddHostWithFtVmToDifferentCluster => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmToDifferentCluster>()?),
            StructType::CannotAddHostWithFtVmToNonHaCluster => Ok(from.as_any_box().downcast::<CannotAddHostWithFtVmToNonHaCluster>()?),
            StructType::GatewayConnectFault => Ok(from.as_any_box().downcast::<GatewayConnectFault>()?),
            StructType::GatewayNotFound => Ok(from.as_any_box().downcast::<GatewayNotFound>()?),
            StructType::GatewayNotReachable => Ok(from.as_any_box().downcast::<GatewayNotReachable>()?),
            StructType::GatewayOperationRefused => Ok(from.as_any_box().downcast::<GatewayOperationRefused>()?),
            StructType::GatewayToHostConnectFault => Ok(from.as_any_box().downcast::<GatewayToHostConnectFault>()?),
            StructType::GatewayHostNotReachable => Ok(from.as_any_box().downcast::<GatewayHostNotReachable>()?),
            StructType::GatewayToHostAuthFault => Ok(from.as_any_box().downcast::<GatewayToHostAuthFault>()?),
            StructType::GatewayToHostTrustVerifyFault => Ok(from.as_any_box().downcast::<GatewayToHostTrustVerifyFault>()?),
            StructType::MultipleCertificatesVerifyFault => Ok(from.as_any_box().downcast::<MultipleCertificatesVerifyFault>()?),
            StructType::NoHost => Ok(from.as_any_box().downcast::<NoHost>()?),
            StructType::NoPermissionOnHost => Ok(from.as_any_box().downcast::<NoPermissionOnHost>()?),
            StructType::NotSupportedHost => Ok(from.as_any_box().downcast::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Ok(from.as_any_box().downcast::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Ok(from.as_any_box().downcast::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Ok(from.as_any_box().downcast::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Ok(from.as_any_box().downcast::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Ok(from.as_any_box().downcast::<NotSupportedHostForVsan>()?),
            StructType::NotSupportedHostInCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInCluster>()?),
            StructType::EvcAdmissionFailed => Ok(from.as_any_box().downcast::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Ok(from.as_any_box().downcast::<EvcAdmissionFailedVmActive>()?),
            StructType::NotSupportedHostInDvs => Ok(from.as_any_box().downcast::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInHaCluster>()?),
            StructType::ReadHostResourcePoolTreeFailed => Ok(from.as_any_box().downcast::<ReadHostResourcePoolTreeFailed>()?),
            StructType::SslDisabledFault => Ok(from.as_any_box().downcast::<SslDisabledFault>()?),
            StructType::SslVerifyFault => Ok(from.as_any_box().downcast::<SslVerifyFault>()?),
            StructType::TooManyHosts => Ok(from.as_any_box().downcast::<TooManyHosts>()?),
            StructType::HostHasComponentFailure => Ok(from.as_any_box().downcast::<HostHasComponentFailure>()?),
            StructType::HostIncompatibleForRecordReplay => Ok(from.as_any_box().downcast::<HostIncompatibleForRecordReplay>()?),
            StructType::HostPowerOpFailed => Ok(from.as_any_box().downcast::<HostPowerOpFailed>()?),
            StructType::NoPeerHostFound => Ok(from.as_any_box().downcast::<NoPeerHostFound>()?),
            StructType::VmotionInterfaceNotEnabled => Ok(from.as_any_box().downcast::<VmotionInterfaceNotEnabled>()?),
            StructType::WakeOnLanNotSupportedByVmotionNic => Ok(from.as_any_box().downcast::<WakeOnLanNotSupportedByVmotionNic>()?),
            StructType::HostSpecificationOperationFailed => Ok(from.as_any_box().downcast::<HostSpecificationOperationFailed>()?),
            StructType::HttpFault => Ok(from.as_any_box().downcast::<HttpFault>()?),
            StructType::IormNotSupportedHostOnDatastore => Ok(from.as_any_box().downcast::<IormNotSupportedHostOnDatastore>()?),
            StructType::InaccessibleVFlashSource => Ok(from.as_any_box().downcast::<InaccessibleVFlashSource>()?),
            StructType::InsufficientResourcesFault => Ok(from.as_any_box().downcast::<InsufficientResourcesFault>()?),
            StructType::InsufficientAgentVmsDeployed => Ok(from.as_any_box().downcast::<InsufficientAgentVmsDeployed>()?),
            StructType::InsufficientCpuResourcesFault => Ok(from.as_any_box().downcast::<InsufficientCpuResourcesFault>()?),
            StructType::InsufficientFailoverResourcesFault => Ok(from.as_any_box().downcast::<InsufficientFailoverResourcesFault>()?),
            StructType::InsufficientGraphicsResourcesFault => Ok(from.as_any_box().downcast::<InsufficientGraphicsResourcesFault>()?),
            StructType::InsufficientHostCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCapacityFault>()?),
            StructType::InsufficientHostCpuCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostCpuCapacityFault>()?),
            StructType::InsufficientHostMemoryCapacityFault => Ok(from.as_any_box().downcast::<InsufficientHostMemoryCapacityFault>()?),
            StructType::InsufficientPerCpuCapacity => Ok(from.as_any_box().downcast::<InsufficientPerCpuCapacity>()?),
            StructType::InsufficientMemoryResourcesFault => Ok(from.as_any_box().downcast::<InsufficientMemoryResourcesFault>()?),
            StructType::InsufficientNetworkCapacity => Ok(from.as_any_box().downcast::<InsufficientNetworkCapacity>()?),
            StructType::InsufficientNetworkResourcePoolCapacity => Ok(from.as_any_box().downcast::<InsufficientNetworkResourcePoolCapacity>()?),
            StructType::InsufficientStandbyResource => Ok(from.as_any_box().downcast::<InsufficientStandbyResource>()?),
            StructType::InsufficientStandbyCpuResource => Ok(from.as_any_box().downcast::<InsufficientStandbyCpuResource>()?),
            StructType::InsufficientStandbyMemoryResource => Ok(from.as_any_box().downcast::<InsufficientStandbyMemoryResource>()?),
            StructType::InsufficientStorageSpace => Ok(from.as_any_box().downcast::<InsufficientStorageSpace>()?),
            StructType::InsufficientVFlashResourcesFault => Ok(from.as_any_box().downcast::<InsufficientVFlashResourcesFault>()?),
            StructType::InvalidResourcePoolStructureFault => Ok(from.as_any_box().downcast::<InvalidResourcePoolStructureFault>()?),
            StructType::NumVirtualCpusExceedsLimit => Ok(from.as_any_box().downcast::<NumVirtualCpusExceedsLimit>()?),
            StructType::VmFaultToleranceTooManyFtVcpusOnHost => Ok(from.as_any_box().downcast::<VmFaultToleranceTooManyFtVcpusOnHost>()?),
            StructType::VmFaultToleranceTooManyVMsOnHost => Ok(from.as_any_box().downcast::<VmFaultToleranceTooManyVMsOnHost>()?),
            StructType::VmSmpFaultToleranceTooManyVMsOnHost => Ok(from.as_any_box().downcast::<VmSmpFaultToleranceTooManyVMsOnHost>()?),
            StructType::InsufficientStorageIops => Ok(from.as_any_box().downcast::<InsufficientStorageIops>()?),
            StructType::InvalidAffinitySettingFault => Ok(from.as_any_box().downcast::<InvalidAffinitySettingFault>()?),
            StructType::InvalidBmcRole => Ok(from.as_any_box().downcast::<InvalidBmcRole>()?),
            StructType::InvalidDatastore => Ok(from.as_any_box().downcast::<InvalidDatastore>()?),
            StructType::DatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<DatastoreNotWritableOnHost>()?),
            StructType::SwapDatastoreNotWritableOnHost => Ok(from.as_any_box().downcast::<SwapDatastoreNotWritableOnHost>()?),
            StructType::InaccessibleDatastore => Ok(from.as_any_box().downcast::<InaccessibleDatastore>()?),
            StructType::InaccessibleFtMetadataDatastore => Ok(from.as_any_box().downcast::<InaccessibleFtMetadataDatastore>()?),
            StructType::InvalidDatastorePath => Ok(from.as_any_box().downcast::<InvalidDatastorePath>()?),
            StructType::InvalidEvent => Ok(from.as_any_box().downcast::<InvalidEvent>()?),
            StructType::InvalidFolder => Ok(from.as_any_box().downcast::<InvalidFolder>()?),
            StructType::VmAlreadyExistsInDatacenter => Ok(from.as_any_box().downcast::<VmAlreadyExistsInDatacenter>()?),
            StructType::InvalidIpmiLoginInfo => Ok(from.as_any_box().downcast::<InvalidIpmiLoginInfo>()?),
            StructType::InvalidIpmiMacAddress => Ok(from.as_any_box().downcast::<InvalidIpmiMacAddress>()?),
            StructType::InvalidLicense => Ok(from.as_any_box().downcast::<InvalidLicense>()?),
            StructType::InvalidLocale => Ok(from.as_any_box().downcast::<InvalidLocale>()?),
            StructType::InvalidLogin => Ok(from.as_any_box().downcast::<InvalidLogin>()?),
            StructType::InvalidClientCertificate => Ok(from.as_any_box().downcast::<InvalidClientCertificate>()?),
            StructType::PasswordExpired => Ok(from.as_any_box().downcast::<PasswordExpired>()?),
            StructType::InvalidName => Ok(from.as_any_box().downcast::<InvalidName>()?),
            StructType::InvalidPrivilege => Ok(from.as_any_box().downcast::<InvalidPrivilege>()?),
            StructType::InvalidState => Ok(from.as_any_box().downcast::<InvalidState>()?),
            StructType::CannotPowerOffVmInCluster => Ok(from.as_any_box().downcast::<CannotPowerOffVmInCluster>()?),
            StructType::EncryptionKeyRequired => Ok(from.as_any_box().downcast::<EncryptionKeyRequired>()?),
            StructType::InvalidDatastoreState => Ok(from.as_any_box().downcast::<InvalidDatastoreState>()?),
            StructType::InvalidHostState => Ok(from.as_any_box().downcast::<InvalidHostState>()?),
            StructType::InvalidHostConnectionState => Ok(from.as_any_box().downcast::<InvalidHostConnectionState>()?),
            StructType::InvalidPowerState => Ok(from.as_any_box().downcast::<InvalidPowerState>()?),
            StructType::InvalidVmState => Ok(from.as_any_box().downcast::<InvalidVmState>()?),
            StructType::MksConnectionLimitReached => Ok(from.as_any_box().downcast::<MksConnectionLimitReached>()?),
            StructType::NoActiveHostInCluster => Ok(from.as_any_box().downcast::<NoActiveHostInCluster>()?),
            StructType::OvfConsumerPowerOnFault => Ok(from.as_any_box().downcast::<OvfConsumerPowerOnFault>()?),
            StructType::QuestionPending => Ok(from.as_any_box().downcast::<QuestionPending>()?),
            StructType::VmPowerOnDisabled => Ok(from.as_any_box().downcast::<VmPowerOnDisabled>()?),
            StructType::IscsiFault => Ok(from.as_any_box().downcast::<IscsiFault>()?),
            StructType::IscsiFaultInvalidVnic => Ok(from.as_any_box().downcast::<IscsiFaultInvalidVnic>()?),
            StructType::IscsiFaultPnicInUse => Ok(from.as_any_box().downcast::<IscsiFaultPnicInUse>()?),
            StructType::IscsiFaultVnicAlreadyBound => Ok(from.as_any_box().downcast::<IscsiFaultVnicAlreadyBound>()?),
            StructType::IscsiFaultVnicHasActivePaths => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasActivePaths>()?),
            StructType::IscsiFaultVnicHasMultipleUplinks => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasMultipleUplinks>()?),
            StructType::IscsiFaultVnicHasNoUplinks => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasNoUplinks>()?),
            StructType::IscsiFaultVnicHasWrongUplink => Ok(from.as_any_box().downcast::<IscsiFaultVnicHasWrongUplink>()?),
            StructType::IscsiFaultVnicInUse => Ok(from.as_any_box().downcast::<IscsiFaultVnicInUse>()?),
            StructType::IscsiFaultVnicIsLastPath => Ok(from.as_any_box().downcast::<IscsiFaultVnicIsLastPath>()?),
            StructType::IscsiFaultVnicNotBound => Ok(from.as_any_box().downcast::<IscsiFaultVnicNotBound>()?),
            StructType::IscsiFaultVnicNotFound => Ok(from.as_any_box().downcast::<IscsiFaultVnicNotFound>()?),
            StructType::KeyNotFound => Ok(from.as_any_box().downcast::<KeyNotFound>()?),
            StructType::LicenseEntityNotFound => Ok(from.as_any_box().downcast::<LicenseEntityNotFound>()?),
            StructType::LicenseServerUnavailable => Ok(from.as_any_box().downcast::<LicenseServerUnavailable>()?),
            StructType::LimitExceeded => Ok(from.as_any_box().downcast::<LimitExceeded>()?),
            StructType::LogBundlingFailed => Ok(from.as_any_box().downcast::<LogBundlingFailed>()?),
            StructType::MigrationFault => Ok(from.as_any_box().downcast::<MigrationFault>()?),
            StructType::AffinityConfigured => Ok(from.as_any_box().downcast::<AffinityConfigured>()?),
            StructType::CannotModifyConfigCpuRequirements => Ok(from.as_any_box().downcast::<CannotModifyConfigCpuRequirements>()?),
            StructType::CannotMoveVmWithDeltaDisk => Ok(from.as_any_box().downcast::<CannotMoveVmWithDeltaDisk>()?),
            StructType::CannotMoveVmWithNativeDeltaDisk => Ok(from.as_any_box().downcast::<CannotMoveVmWithNativeDeltaDisk>()?),
            StructType::CloneFromSnapshotNotSupported => Ok(from.as_any_box().downcast::<CloneFromSnapshotNotSupported>()?),
            StructType::DatacenterMismatch => Ok(from.as_any_box().downcast::<DatacenterMismatch>()?),
            StructType::DisallowedMigrationDeviceAttached => Ok(from.as_any_box().downcast::<DisallowedMigrationDeviceAttached>()?),
            StructType::DiskMoveTypeNotSupported => Ok(from.as_any_box().downcast::<DiskMoveTypeNotSupported>()?),
            StructType::FaultToleranceAntiAffinityViolated => Ok(from.as_any_box().downcast::<FaultToleranceAntiAffinityViolated>()?),
            StructType::FaultToleranceNeedsThickDisk => Ok(from.as_any_box().downcast::<FaultToleranceNeedsThickDisk>()?),
            StructType::FaultToleranceNotSameBuild => Ok(from.as_any_box().downcast::<FaultToleranceNotSameBuild>()?),
            StructType::HaErrorsAtDest => Ok(from.as_any_box().downcast::<HaErrorsAtDest>()?),
            StructType::IncompatibleDefaultDevice => Ok(from.as_any_box().downcast::<IncompatibleDefaultDevice>()?),
            StructType::LargeRdmConversionNotSupported => Ok(from.as_any_box().downcast::<LargeRdmConversionNotSupported>()?),
            StructType::MaintenanceModeFileMove => Ok(from.as_any_box().downcast::<MaintenanceModeFileMove>()?),
            StructType::MigrationDisabled => Ok(from.as_any_box().downcast::<MigrationDisabled>()?),
            StructType::MigrationFeatureNotSupported => Ok(from.as_any_box().downcast::<MigrationFeatureNotSupported>()?),
            StructType::FullStorageVMotionNotSupported => Ok(from.as_any_box().downcast::<FullStorageVMotionNotSupported>()?),
            StructType::IndependentDiskVMotionNotSupported => Ok(from.as_any_box().downcast::<IndependentDiskVMotionNotSupported>()?),
            StructType::NonHomeRdmvMotionNotSupported => Ok(from.as_any_box().downcast::<NonHomeRdmvMotionNotSupported>()?),
            StructType::StorageVMotionNotSupported => Ok(from.as_any_box().downcast::<StorageVMotionNotSupported>()?),
            StructType::UnsharedSwapVMotionNotSupported => Ok(from.as_any_box().downcast::<UnsharedSwapVMotionNotSupported>()?),
            StructType::VMotionAcrossNetworkNotSupported => Ok(from.as_any_box().downcast::<VMotionAcrossNetworkNotSupported>()?),
            StructType::MigrationNotReady => Ok(from.as_any_box().downcast::<MigrationNotReady>()?),
            StructType::MismatchedNetworkPolicies => Ok(from.as_any_box().downcast::<MismatchedNetworkPolicies>()?),
            StructType::MismatchedVMotionNetworkNames => Ok(from.as_any_box().downcast::<MismatchedVMotionNetworkNames>()?),
            StructType::NetworksMayNotBeTheSame => Ok(from.as_any_box().downcast::<NetworksMayNotBeTheSame>()?),
            StructType::NoGuestHeartbeat => Ok(from.as_any_box().downcast::<NoGuestHeartbeat>()?),
            StructType::RdmConversionNotSupported => Ok(from.as_any_box().downcast::<RdmConversionNotSupported>()?),
            StructType::RdmNotPreserved => Ok(from.as_any_box().downcast::<RdmNotPreserved>()?),
            StructType::ReadOnlyDisksWithLegacyDestination => Ok(from.as_any_box().downcast::<ReadOnlyDisksWithLegacyDestination>()?),
            StructType::SnapshotCopyNotSupported => Ok(from.as_any_box().downcast::<SnapshotCopyNotSupported>()?),
            StructType::HotSnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<HotSnapshotMoveNotSupported>()?),
            StructType::SnapshotCloneNotSupported => Ok(from.as_any_box().downcast::<SnapshotCloneNotSupported>()?),
            StructType::SnapshotMoveFromNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveFromNonHomeNotSupported>()?),
            StructType::SnapshotMoveNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveNotSupported>()?),
            StructType::SnapshotMoveToNonHomeNotSupported => Ok(from.as_any_box().downcast::<SnapshotMoveToNonHomeNotSupported>()?),
            StructType::SnapshotRevertIssue => Ok(from.as_any_box().downcast::<SnapshotRevertIssue>()?),
            StructType::SuspendedRelocateNotSupported => Ok(from.as_any_box().downcast::<SuspendedRelocateNotSupported>()?),
            StructType::TooManyDisksOnLegacyHost => Ok(from.as_any_box().downcast::<TooManyDisksOnLegacyHost>()?),
            StructType::ToolsInstallationInProgress => Ok(from.as_any_box().downcast::<ToolsInstallationInProgress>()?),
            StructType::UncommittedUndoableDisk => Ok(from.as_any_box().downcast::<UncommittedUndoableDisk>()?),
            StructType::VMotionInterfaceIssue => Ok(from.as_any_box().downcast::<VMotionInterfaceIssue>()?),
            StructType::VMotionLinkCapacityLow => Ok(from.as_any_box().downcast::<VMotionLinkCapacityLow>()?),
            StructType::VMotionLinkDown => Ok(from.as_any_box().downcast::<VMotionLinkDown>()?),
            StructType::VMotionNotConfigured => Ok(from.as_any_box().downcast::<VMotionNotConfigured>()?),
            StructType::VMotionNotLicensed => Ok(from.as_any_box().downcast::<VMotionNotLicensed>()?),
            StructType::VMotionNotSupported => Ok(from.as_any_box().downcast::<VMotionNotSupported>()?),
            StructType::VMotionProtocolIncompatible => Ok(from.as_any_box().downcast::<VMotionProtocolIncompatible>()?),
            StructType::WillLoseHaProtection => Ok(from.as_any_box().downcast::<WillLoseHaProtection>()?),
            StructType::WillModifyConfigCpuRequirements => Ok(from.as_any_box().downcast::<WillModifyConfigCpuRequirements>()?),
            StructType::WillResetSnapshotDirectory => Ok(from.as_any_box().downcast::<WillResetSnapshotDirectory>()?),
            StructType::MismatchedBundle => Ok(from.as_any_box().downcast::<MismatchedBundle>()?),
            StructType::MissingBmcSupport => Ok(from.as_any_box().downcast::<MissingBmcSupport>()?),
            StructType::NamespaceFull => Ok(from.as_any_box().downcast::<NamespaceFull>()?),
            StructType::NamespaceLimitReached => Ok(from.as_any_box().downcast::<NamespaceLimitReached>()?),
            StructType::NamespaceWriteProtected => Ok(from.as_any_box().downcast::<NamespaceWriteProtected>()?),
            StructType::NetworkDisruptedAndConfigRolledBack => Ok(from.as_any_box().downcast::<NetworkDisruptedAndConfigRolledBack>()?),
            StructType::NoClientCertificate => Ok(from.as_any_box().downcast::<NoClientCertificate>()?),
            StructType::NoCompatibleDatastore => Ok(from.as_any_box().downcast::<NoCompatibleDatastore>()?),
            StructType::NoCompatibleHost => Ok(from.as_any_box().downcast::<NoCompatibleHost>()?),
            StructType::NoCompatibleHostWithAccessToDevice => Ok(from.as_any_box().downcast::<NoCompatibleHostWithAccessToDevice>()?),
            StructType::NoConnectedDatastore => Ok(from.as_any_box().downcast::<NoConnectedDatastore>()?),
            StructType::NoDiskFound => Ok(from.as_any_box().downcast::<NoDiskFound>()?),
            StructType::NoSubjectName => Ok(from.as_any_box().downcast::<NoSubjectName>()?),
            StructType::NotFound => Ok(from.as_any_box().downcast::<NotFound>()?),
            StructType::NotSupportedHostForChecksum => Ok(from.as_any_box().downcast::<NotSupportedHostForChecksum>()?),
            StructType::OutOfBounds => Ok(from.as_any_box().downcast::<OutOfBounds>()?),
            StructType::OvfFault => Ok(from.as_any_box().downcast::<OvfFault>()?),
            StructType::OvfConsumerCallbackFault => Ok(from.as_any_box().downcast::<OvfConsumerCallbackFault>()?),
            StructType::OvfConsumerCommunicationError => Ok(from.as_any_box().downcast::<OvfConsumerCommunicationError>()?),
            StructType::OvfConsumerFault => Ok(from.as_any_box().downcast::<OvfConsumerFault>()?),
            StructType::OvfConsumerInvalidSection => Ok(from.as_any_box().downcast::<OvfConsumerInvalidSection>()?),
            StructType::OvfConsumerUndeclaredSection => Ok(from.as_any_box().downcast::<OvfConsumerUndeclaredSection>()?),
            StructType::OvfConsumerUndefinedPrefix => Ok(from.as_any_box().downcast::<OvfConsumerUndefinedPrefix>()?),
            StructType::OvfExport => Ok(from.as_any_box().downcast::<OvfExport>()?),
            StructType::ConnectedIso => Ok(from.as_any_box().downcast::<ConnectedIso>()?),
            StructType::OvfDuplicatedPropertyIdExport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdExport>()?),
            StructType::OvfDuplicatedPropertyIdImport => Ok(from.as_any_box().downcast::<OvfDuplicatedPropertyIdImport>()?),
            StructType::OvfExportFailed => Ok(from.as_any_box().downcast::<OvfExportFailed>()?),
            StructType::OvfHardwareExport => Ok(from.as_any_box().downcast::<OvfHardwareExport>()?),
            StructType::OvfConnectedDevice => Ok(from.as_any_box().downcast::<OvfConnectedDevice>()?),
            StructType::OvfConnectedDeviceFloppy => Ok(from.as_any_box().downcast::<OvfConnectedDeviceFloppy>()?),
            StructType::OvfConnectedDeviceIso => Ok(from.as_any_box().downcast::<OvfConnectedDeviceIso>()?),
            StructType::OvfUnableToExportDisk => Ok(from.as_any_box().downcast::<OvfUnableToExportDisk>()?),
            StructType::OvfUnknownDeviceBacking => Ok(from.as_any_box().downcast::<OvfUnknownDeviceBacking>()?),
            StructType::OvfUnsupportedDeviceExport => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceExport>()?),
            StructType::OvfPropertyExport => Ok(from.as_any_box().downcast::<OvfPropertyExport>()?),
            StructType::OvfPropertyNetworkExport => Ok(from.as_any_box().downcast::<OvfPropertyNetworkExport>()?),
            StructType::OvfImport => Ok(from.as_any_box().downcast::<OvfImport>()?),
            StructType::OvfCpuCompatibility => Ok(from.as_any_box().downcast::<OvfCpuCompatibility>()?),
            StructType::OvfCpuCompatibilityCheckNotSupported => Ok(from.as_any_box().downcast::<OvfCpuCompatibilityCheckNotSupported>()?),
            StructType::OvfHardwareCheck => Ok(from.as_any_box().downcast::<OvfHardwareCheck>()?),
            StructType::OvfImportFailed => Ok(from.as_any_box().downcast::<OvfImportFailed>()?),
            StructType::OvfMappedOsId => Ok(from.as_any_box().downcast::<OvfMappedOsId>()?),
            StructType::OvfMissingHardware => Ok(from.as_any_box().downcast::<OvfMissingHardware>()?),
            StructType::OvfNetworkMappingNotSupported => Ok(from.as_any_box().downcast::<OvfNetworkMappingNotSupported>()?),
            StructType::OvfUnsupportedDiskProvisioning => Ok(from.as_any_box().downcast::<OvfUnsupportedDiskProvisioning>()?),
            StructType::OvfInvalidPackage => Ok(from.as_any_box().downcast::<OvfInvalidPackage>()?),
            StructType::OvfAttribute => Ok(from.as_any_box().downcast::<OvfAttribute>()?),
            StructType::OvfInvalidValue => Ok(from.as_any_box().downcast::<OvfInvalidValue>()?),
            StructType::OvfInvalidValueConfiguration => Ok(from.as_any_box().downcast::<OvfInvalidValueConfiguration>()?),
            StructType::OvfInvalidValueEmpty => Ok(from.as_any_box().downcast::<OvfInvalidValueEmpty>()?),
            StructType::OvfInvalidValueFormatMalformed => Ok(from.as_any_box().downcast::<OvfInvalidValueFormatMalformed>()?),
            StructType::OvfInvalidValueReference => Ok(from.as_any_box().downcast::<OvfInvalidValueReference>()?),
            StructType::OvfMissingAttribute => Ok(from.as_any_box().downcast::<OvfMissingAttribute>()?),
            StructType::OvfConstraint => Ok(from.as_any_box().downcast::<OvfConstraint>()?),
            StructType::OvfDiskOrderConstraint => Ok(from.as_any_box().downcast::<OvfDiskOrderConstraint>()?),
            StructType::OvfHostResourceConstraint => Ok(from.as_any_box().downcast::<OvfHostResourceConstraint>()?),
            StructType::OvfElement => Ok(from.as_any_box().downcast::<OvfElement>()?),
            StructType::OvfDuplicateElement => Ok(from.as_any_box().downcast::<OvfDuplicateElement>()?),
            StructType::OvfDuplicatedElementBoundary => Ok(from.as_any_box().downcast::<OvfDuplicatedElementBoundary>()?),
            StructType::OvfElementInvalidValue => Ok(from.as_any_box().downcast::<OvfElementInvalidValue>()?),
            StructType::OvfMissingElement => Ok(from.as_any_box().downcast::<OvfMissingElement>()?),
            StructType::OvfMissingElementNormalBoundary => Ok(from.as_any_box().downcast::<OvfMissingElementNormalBoundary>()?),
            StructType::OvfUnexpectedElement => Ok(from.as_any_box().downcast::<OvfUnexpectedElement>()?),
            StructType::OvfWrongElement => Ok(from.as_any_box().downcast::<OvfWrongElement>()?),
            StructType::OvfProperty => Ok(from.as_any_box().downcast::<OvfProperty>()?),
            StructType::OvfPropertyNetwork => Ok(from.as_any_box().downcast::<OvfPropertyNetwork>()?),
            StructType::OvfPropertyQualifier => Ok(from.as_any_box().downcast::<OvfPropertyQualifier>()?),
            StructType::OvfPropertyQualifierDuplicate => Ok(from.as_any_box().downcast::<OvfPropertyQualifierDuplicate>()?),
            StructType::OvfPropertyQualifierIgnored => Ok(from.as_any_box().downcast::<OvfPropertyQualifierIgnored>()?),
            StructType::OvfPropertyType => Ok(from.as_any_box().downcast::<OvfPropertyType>()?),
            StructType::OvfPropertyValue => Ok(from.as_any_box().downcast::<OvfPropertyValue>()?),
            StructType::OvfWrongNamespace => Ok(from.as_any_box().downcast::<OvfWrongNamespace>()?),
            StructType::OvfXmlFormat => Ok(from.as_any_box().downcast::<OvfXmlFormat>()?),
            StructType::OvfSystemFault => Ok(from.as_any_box().downcast::<OvfSystemFault>()?),
            StructType::OvfDiskMappingNotFound => Ok(from.as_any_box().downcast::<OvfDiskMappingNotFound>()?),
            StructType::OvfHostValueNotParsed => Ok(from.as_any_box().downcast::<OvfHostValueNotParsed>()?),
            StructType::OvfInternalError => Ok(from.as_any_box().downcast::<OvfInternalError>()?),
            StructType::OvfToXmlUnsupportedElement => Ok(from.as_any_box().downcast::<OvfToXmlUnsupportedElement>()?),
            StructType::OvfUnknownDevice => Ok(from.as_any_box().downcast::<OvfUnknownDevice>()?),
            StructType::OvfUnknownEntity => Ok(from.as_any_box().downcast::<OvfUnknownEntity>()?),
            StructType::OvfUnsupportedDeviceBackingInfo => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingInfo>()?),
            StructType::OvfUnsupportedDeviceBackingOption => Ok(from.as_any_box().downcast::<OvfUnsupportedDeviceBackingOption>()?),
            StructType::OvfUnsupportedPackage => Ok(from.as_any_box().downcast::<OvfUnsupportedPackage>()?),
            StructType::OvfInvalidVmName => Ok(from.as_any_box().downcast::<OvfInvalidVmName>()?),
            StructType::OvfNoHostNic => Ok(from.as_any_box().downcast::<OvfNoHostNic>()?),
            StructType::OvfNoSupportedHardwareFamily => Ok(from.as_any_box().downcast::<OvfNoSupportedHardwareFamily>()?),
            StructType::OvfUnsupportedAttribute => Ok(from.as_any_box().downcast::<OvfUnsupportedAttribute>()?),
            StructType::OvfUnsupportedAttributeValue => Ok(from.as_any_box().downcast::<OvfUnsupportedAttributeValue>()?),
            StructType::OvfUnsupportedElement => Ok(from.as_any_box().downcast::<OvfUnsupportedElement>()?),
            StructType::OvfNoSpaceOnController => Ok(from.as_any_box().downcast::<OvfNoSpaceOnController>()?),
            StructType::OvfUnsupportedElementValue => Ok(from.as_any_box().downcast::<OvfUnsupportedElementValue>()?),
            StructType::OvfUnsupportedSection => Ok(from.as_any_box().downcast::<OvfUnsupportedSection>()?),
            StructType::OvfUnsupportedSubType => Ok(from.as_any_box().downcast::<OvfUnsupportedSubType>()?),
            StructType::OvfUnsupportedType => Ok(from.as_any_box().downcast::<OvfUnsupportedType>()?),
            StructType::PatchBinariesNotFound => Ok(from.as_any_box().downcast::<PatchBinariesNotFound>()?),
            StructType::PatchMetadataInvalid => Ok(from.as_any_box().downcast::<PatchMetadataInvalid>()?),
            StructType::PatchMetadataCorrupted => Ok(from.as_any_box().downcast::<PatchMetadataCorrupted>()?),
            StructType::PatchMetadataNotFound => Ok(from.as_any_box().downcast::<PatchMetadataNotFound>()?),
            StructType::PatchNotApplicable => Ok(from.as_any_box().downcast::<PatchNotApplicable>()?),
            StructType::PatchAlreadyInstalled => Ok(from.as_any_box().downcast::<PatchAlreadyInstalled>()?),
            StructType::PatchMissingDependencies => Ok(from.as_any_box().downcast::<PatchMissingDependencies>()?),
            StructType::PatchSuperseded => Ok(from.as_any_box().downcast::<PatchSuperseded>()?),
            StructType::ProfileUpdateFailed => Ok(from.as_any_box().downcast::<ProfileUpdateFailed>()?),
            StructType::RebootRequired => Ok(from.as_any_box().downcast::<RebootRequired>()?),
            StructType::RecordReplayDisabled => Ok(from.as_any_box().downcast::<RecordReplayDisabled>()?),
            StructType::RemoveFailed => Ok(from.as_any_box().downcast::<RemoveFailed>()?),
            StructType::ReplicationFault => Ok(from.as_any_box().downcast::<ReplicationFault>()?),
            StructType::IncompatibleHostForVmReplication => Ok(from.as_any_box().downcast::<IncompatibleHostForVmReplication>()?),
            StructType::ReplicationConfigFault => Ok(from.as_any_box().downcast::<ReplicationConfigFault>()?),
            StructType::ReplicationDiskConfigFault => Ok(from.as_any_box().downcast::<ReplicationDiskConfigFault>()?),
            StructType::ReplicationVmConfigFault => Ok(from.as_any_box().downcast::<ReplicationVmConfigFault>()?),
            StructType::ReplicationIncompatibleWithFt => Ok(from.as_any_box().downcast::<ReplicationIncompatibleWithFt>()?),
            StructType::ReplicationInvalidOptions => Ok(from.as_any_box().downcast::<ReplicationInvalidOptions>()?),
            StructType::ReplicationNotSupportedOnHost => Ok(from.as_any_box().downcast::<ReplicationNotSupportedOnHost>()?),
            StructType::ReplicationVmFault => Ok(from.as_any_box().downcast::<ReplicationVmFault>()?),
            StructType::ReplicationVmInProgressFault => Ok(from.as_any_box().downcast::<ReplicationVmInProgressFault>()?),
            StructType::ResourceInUse => Ok(from.as_any_box().downcast::<ResourceInUse>()?),
            StructType::FilterInUse => Ok(from.as_any_box().downcast::<FilterInUse>()?),
            StructType::QuiesceDatastoreIoForHaFailed => Ok(from.as_any_box().downcast::<QuiesceDatastoreIoForHaFailed>()?),
            StructType::ResourceNotAvailable => Ok(from.as_any_box().downcast::<ResourceNotAvailable>()?),
            StructType::SspiChallenge => Ok(from.as_any_box().downcast::<SspiChallenge>()?),
            StructType::ShrinkDiskFault => Ok(from.as_any_box().downcast::<ShrinkDiskFault>()?),
            StructType::SnapshotFault => Ok(from.as_any_box().downcast::<SnapshotFault>()?),
            StructType::ApplicationQuiesceFault => Ok(from.as_any_box().downcast::<ApplicationQuiesceFault>()?),
            StructType::FilesystemQuiesceFault => Ok(from.as_any_box().downcast::<FilesystemQuiesceFault>()?),
            StructType::MemorySnapshotOnIndependentDisk => Ok(from.as_any_box().downcast::<MemorySnapshotOnIndependentDisk>()?),
            StructType::MultipleSnapshotsNotSupported => Ok(from.as_any_box().downcast::<MultipleSnapshotsNotSupported>()?),
            StructType::SnapshotDisabled => Ok(from.as_any_box().downcast::<SnapshotDisabled>()?),
            StructType::SnapshotIncompatibleDeviceInVm => Ok(from.as_any_box().downcast::<SnapshotIncompatibleDeviceInVm>()?),
            StructType::SnapshotLocked => Ok(from.as_any_box().downcast::<SnapshotLocked>()?),
            StructType::SnapshotNoChange => Ok(from.as_any_box().downcast::<SnapshotNoChange>()?),
            StructType::TooManySnapshotLevels => Ok(from.as_any_box().downcast::<TooManySnapshotLevels>()?),
            StructType::SsdDiskNotAvailable => Ok(from.as_any_box().downcast::<SsdDiskNotAvailable>()?),
            StructType::StorageDrsCannotMoveDiskInMultiWriterMode => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveDiskInMultiWriterMode>()?),
            StructType::StorageDrsCannotMoveFtVm => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveFtVm>()?),
            StructType::StorageDrsCannotMoveIndependentDisk => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveIndependentDisk>()?),
            StructType::StorageDrsCannotMoveManuallyPlacedSwapFile => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveManuallyPlacedSwapFile>()?),
            StructType::StorageDrsCannotMoveManuallyPlacedVm => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveManuallyPlacedVm>()?),
            StructType::StorageDrsCannotMoveSharedDisk => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveSharedDisk>()?),
            StructType::StorageDrsCannotMoveTemplate => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveTemplate>()?),
            StructType::StorageDrsCannotMoveVmInUserFolder => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveVmInUserFolder>()?),
            StructType::StorageDrsCannotMoveVmWithMountedCdrom => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveVmWithMountedCdrom>()?),
            StructType::StorageDrsCannotMoveVmWithNoFilesInLayout => Ok(from.as_any_box().downcast::<StorageDrsCannotMoveVmWithNoFilesInLayout>()?),
            StructType::StorageDrsDatacentersCannotShareDatastore => Ok(from.as_any_box().downcast::<StorageDrsDatacentersCannotShareDatastore>()?),
            StructType::StorageDrsDisabledOnVm => Ok(from.as_any_box().downcast::<StorageDrsDisabledOnVm>()?),
            StructType::StorageDrsHbrDiskNotMovable => Ok(from.as_any_box().downcast::<StorageDrsHbrDiskNotMovable>()?),
            StructType::StorageDrsHmsMoveInProgress => Ok(from.as_any_box().downcast::<StorageDrsHmsMoveInProgress>()?),
            StructType::StorageDrsHmsUnreachable => Ok(from.as_any_box().downcast::<StorageDrsHmsUnreachable>()?),
            StructType::StorageDrsIolbDisabledInternally => Ok(from.as_any_box().downcast::<StorageDrsIolbDisabledInternally>()?),
            StructType::StorageDrsRelocateDisabled => Ok(from.as_any_box().downcast::<StorageDrsRelocateDisabled>()?),
            StructType::StorageDrsStaleHmsCollection => Ok(from.as_any_box().downcast::<StorageDrsStaleHmsCollection>()?),
            StructType::StorageDrsUnableToMoveFiles => Ok(from.as_any_box().downcast::<StorageDrsUnableToMoveFiles>()?),
            StructType::SwapDatastoreUnset => Ok(from.as_any_box().downcast::<SwapDatastoreUnset>()?),
            StructType::TaskInProgress => Ok(from.as_any_box().downcast::<TaskInProgress>()?),
            StructType::VAppTaskInProgress => Ok(from.as_any_box().downcast::<VAppTaskInProgress>()?),
            StructType::Timedout => Ok(from.as_any_box().downcast::<Timedout>()?),
            StructType::PowerOnFtSecondaryTimedout => Ok(from.as_any_box().downcast::<PowerOnFtSecondaryTimedout>()?),
            StructType::TooManyConsecutiveOverrides => Ok(from.as_any_box().downcast::<TooManyConsecutiveOverrides>()?),
            StructType::ToolsUnavailable => Ok(from.as_any_box().downcast::<ToolsUnavailable>()?),
            StructType::UnrecognizedHost => Ok(from.as_any_box().downcast::<UnrecognizedHost>()?),
            StructType::UnsupportedVimApiVersion => Ok(from.as_any_box().downcast::<UnsupportedVimApiVersion>()?),
            StructType::UserNotFound => Ok(from.as_any_box().downcast::<UserNotFound>()?),
            StructType::VAppConfigFault => Ok(from.as_any_box().downcast::<VAppConfigFault>()?),
            StructType::MissingPowerOffConfiguration => Ok(from.as_any_box().downcast::<MissingPowerOffConfiguration>()?),
            StructType::MissingPowerOnConfiguration => Ok(from.as_any_box().downcast::<MissingPowerOnConfiguration>()?),
            StructType::NoVmInVApp => Ok(from.as_any_box().downcast::<NoVmInVApp>()?),
            StructType::VFlashModuleVersionIncompatible => Ok(from.as_any_box().downcast::<VFlashModuleVersionIncompatible>()?),
            StructType::VmConfigFault => Ok(from.as_any_box().downcast::<VmConfigFault>()?),
            StructType::CannotAccessVmComponent => Ok(from.as_any_box().downcast::<CannotAccessVmComponent>()?),
            StructType::CannotAccessVmConfig => Ok(from.as_any_box().downcast::<CannotAccessVmConfig>()?),
            StructType::CannotAccessVmDevice => Ok(from.as_any_box().downcast::<CannotAccessVmDevice>()?),
            StructType::CannotAccessNetwork => Ok(from.as_any_box().downcast::<CannotAccessNetwork>()?),
            StructType::DestinationSwitchFull => Ok(from.as_any_box().downcast::<DestinationSwitchFull>()?),
            StructType::LegacyNetworkInterfaceInUse => Ok(from.as_any_box().downcast::<LegacyNetworkInterfaceInUse>()?),
            StructType::VmOnConflictDvPort => Ok(from.as_any_box().downcast::<VmOnConflictDvPort>()?),
            StructType::VmOnVirtualIntranet => Ok(from.as_any_box().downcast::<VmOnVirtualIntranet>()?),
            StructType::CannotAccessVmDisk => Ok(from.as_any_box().downcast::<CannotAccessVmDisk>()?),
            StructType::RdmPointsToInaccessibleDisk => Ok(from.as_any_box().downcast::<RdmPointsToInaccessibleDisk>()?),
            StructType::CannotDisableSnapshot => Ok(from.as_any_box().downcast::<CannotDisableSnapshot>()?),
            StructType::CannotUseNetwork => Ok(from.as_any_box().downcast::<CannotUseNetwork>()?),
            StructType::CpuHotPlugNotSupported => Ok(from.as_any_box().downcast::<CpuHotPlugNotSupported>()?),
            StructType::DeltaDiskFormatNotSupported => Ok(from.as_any_box().downcast::<DeltaDiskFormatNotSupported>()?),
            StructType::EightHostLimitViolated => Ok(from.as_any_box().downcast::<EightHostLimitViolated>()?),
            StructType::FaultToleranceCannotEditMem => Ok(from.as_any_box().downcast::<FaultToleranceCannotEditMem>()?),
            StructType::GenericVmConfigFault => Ok(from.as_any_box().downcast::<GenericVmConfigFault>()?),
            StructType::InvalidFormat => Ok(from.as_any_box().downcast::<InvalidFormat>()?),
            StructType::InvalidDiskFormat => Ok(from.as_any_box().downcast::<InvalidDiskFormat>()?),
            StructType::InvalidSnapshotFormat => Ok(from.as_any_box().downcast::<InvalidSnapshotFormat>()?),
            StructType::InvalidVmConfig => Ok(from.as_any_box().downcast::<InvalidVmConfig>()?),
            StructType::InvalidDeviceSpec => Ok(from.as_any_box().downcast::<InvalidDeviceSpec>()?),
            StructType::DeviceHotPlugNotSupported => Ok(from.as_any_box().downcast::<DeviceHotPlugNotSupported>()?),
            StructType::DeviceNotFound => Ok(from.as_any_box().downcast::<DeviceNotFound>()?),
            StructType::DeviceUnsupportedForVmPlatform => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmPlatform>()?),
            StructType::DeviceUnsupportedForVmVersion => Ok(from.as_any_box().downcast::<DeviceUnsupportedForVmVersion>()?),
            StructType::DisallowedDiskModeChange => Ok(from.as_any_box().downcast::<DisallowedDiskModeChange>()?),
            StructType::InvalidController => Ok(from.as_any_box().downcast::<InvalidController>()?),
            StructType::InvalidDeviceBacking => Ok(from.as_any_box().downcast::<InvalidDeviceBacking>()?),
            StructType::InvalidDeviceOperation => Ok(from.as_any_box().downcast::<InvalidDeviceOperation>()?),
            StructType::MissingController => Ok(from.as_any_box().downcast::<MissingController>()?),
            StructType::SwapPlacementOverrideNotSupported => Ok(from.as_any_box().downcast::<SwapPlacementOverrideNotSupported>()?),
            StructType::TooManyDevices => Ok(from.as_any_box().downcast::<TooManyDevices>()?),
            StructType::UnsupportedGuest => Ok(from.as_any_box().downcast::<UnsupportedGuest>()?),
            StructType::VmWwnConflict => Ok(from.as_any_box().downcast::<VmWwnConflict>()?),
            StructType::LargeRdmNotSupportedOnDatastore => Ok(from.as_any_box().downcast::<LargeRdmNotSupportedOnDatastore>()?),
            StructType::MemoryHotPlugNotSupported => Ok(from.as_any_box().downcast::<MemoryHotPlugNotSupported>()?),
            StructType::NoCompatibleHardAffinityHost => Ok(from.as_any_box().downcast::<NoCompatibleHardAffinityHost>()?),
            StructType::NoCompatibleSoftAffinityHost => Ok(from.as_any_box().downcast::<NoCompatibleSoftAffinityHost>()?),
            StructType::NumVirtualCpusIncompatible => Ok(from.as_any_box().downcast::<NumVirtualCpusIncompatible>()?),
            StructType::OvfConsumerValidationFault => Ok(from.as_any_box().downcast::<OvfConsumerValidationFault>()?),
            StructType::QuarantineModeFault => Ok(from.as_any_box().downcast::<QuarantineModeFault>()?),
            StructType::RdmNotSupportedOnDatastore => Ok(from.as_any_box().downcast::<RdmNotSupportedOnDatastore>()?),
            StructType::RuleViolation => Ok(from.as_any_box().downcast::<RuleViolation>()?),
            StructType::SoftRuleVioCorrectionDisallowed => Ok(from.as_any_box().downcast::<SoftRuleVioCorrectionDisallowed>()?),
            StructType::SoftRuleVioCorrectionImpact => Ok(from.as_any_box().downcast::<SoftRuleVioCorrectionImpact>()?),
            StructType::UnsupportedDatastore => Ok(from.as_any_box().downcast::<UnsupportedDatastore>()?),
            StructType::MemoryFileFormatNotSupportedByDatastore => Ok(from.as_any_box().downcast::<MemoryFileFormatNotSupportedByDatastore>()?),
            StructType::UnSupportedDatastoreForVFlash => Ok(from.as_any_box().downcast::<UnSupportedDatastoreForVFlash>()?),
            StructType::UnsupportedVmxLocation => Ok(from.as_any_box().downcast::<UnsupportedVmxLocation>()?),
            StructType::VAppNotRunning => Ok(from.as_any_box().downcast::<VAppNotRunning>()?),
            StructType::VAppPropertyFault => Ok(from.as_any_box().downcast::<VAppPropertyFault>()?),
            StructType::InvalidNetworkInType => Ok(from.as_any_box().downcast::<InvalidNetworkInType>()?),
            StructType::InvalidPropertyType => Ok(from.as_any_box().downcast::<InvalidPropertyType>()?),
            StructType::InvalidPropertyValue => Ok(from.as_any_box().downcast::<InvalidPropertyValue>()?),
            StructType::UnconfiguredPropertyValue => Ok(from.as_any_box().downcast::<UnconfiguredPropertyValue>()?),
            StructType::MissingIpPool => Ok(from.as_any_box().downcast::<MissingIpPool>()?),
            StructType::MissingNetworkIpConfig => Ok(from.as_any_box().downcast::<MissingNetworkIpConfig>()?),
            StructType::NoAvailableIp => Ok(from.as_any_box().downcast::<NoAvailableIp>()?),
            StructType::NoVcManagedIpConfigured => Ok(from.as_any_box().downcast::<NoVcManagedIpConfigured>()?),
            StructType::NotUserConfigurableProperty => Ok(from.as_any_box().downcast::<NotUserConfigurableProperty>()?),
            StructType::VFlashCacheHotConfigNotSupported => Ok(from.as_any_box().downcast::<VFlashCacheHotConfigNotSupported>()?),
            StructType::VFlashModuleNotSupported => Ok(from.as_any_box().downcast::<VFlashModuleNotSupported>()?),
            StructType::VirtualHardwareCompatibilityIssue => Ok(from.as_any_box().downcast::<VirtualHardwareCompatibilityIssue>()?),
            StructType::CpuIncompatible => Ok(from.as_any_box().downcast::<CpuIncompatible>()?),
            StructType::CpuCompatibilityUnknown => Ok(from.as_any_box().downcast::<CpuCompatibilityUnknown>()?),
            StructType::CpuIncompatible1Ecx => Ok(from.as_any_box().downcast::<CpuIncompatible1Ecx>()?),
            StructType::CpuIncompatible81Edx => Ok(from.as_any_box().downcast::<CpuIncompatible81Edx>()?),
            StructType::FaultToleranceCpuIncompatible => Ok(from.as_any_box().downcast::<FaultToleranceCpuIncompatible>()?),
            StructType::DeviceNotSupported => Ok(from.as_any_box().downcast::<DeviceNotSupported>()?),
            StructType::DeviceBackingNotSupported => Ok(from.as_any_box().downcast::<DeviceBackingNotSupported>()?),
            StructType::DvPortNotSupported => Ok(from.as_any_box().downcast::<DvPortNotSupported>()?),
            StructType::UnusedVirtualDiskBlocksNotScrubbed => Ok(from.as_any_box().downcast::<UnusedVirtualDiskBlocksNotScrubbed>()?),
            StructType::VirtualDiskBlocksNotFullyProvisioned => Ok(from.as_any_box().downcast::<VirtualDiskBlocksNotFullyProvisioned>()?),
            StructType::DeviceControllerNotSupported => Ok(from.as_any_box().downcast::<DeviceControllerNotSupported>()?),
            StructType::DigestNotSupported => Ok(from.as_any_box().downcast::<DigestNotSupported>()?),
            StructType::FileBackedPortNotSupported => Ok(from.as_any_box().downcast::<FileBackedPortNotSupported>()?),
            StructType::MultiWriterNotSupported => Ok(from.as_any_box().downcast::<MultiWriterNotSupported>()?),
            StructType::NonPersistentDisksNotSupported => Ok(from.as_any_box().downcast::<NonPersistentDisksNotSupported>()?),
            StructType::RdmNotSupported => Ok(from.as_any_box().downcast::<RdmNotSupported>()?),
            StructType::PhysCompatRdmNotSupported => Ok(from.as_any_box().downcast::<PhysCompatRdmNotSupported>()?),
            StructType::RawDiskNotSupported => Ok(from.as_any_box().downcast::<RawDiskNotSupported>()?),
            StructType::RemoteDeviceNotSupported => Ok(from.as_any_box().downcast::<RemoteDeviceNotSupported>()?),
            StructType::SharedBusControllerNotSupported => Ok(from.as_any_box().downcast::<SharedBusControllerNotSupported>()?),
            StructType::VmiNotSupported => Ok(from.as_any_box().downcast::<VmiNotSupported>()?),
            StructType::VirtualDiskModeNotSupported => Ok(from.as_any_box().downcast::<VirtualDiskModeNotSupported>()?),
            StructType::VirtualEthernetCardNotSupported => Ok(from.as_any_box().downcast::<VirtualEthernetCardNotSupported>()?),
            StructType::DiskNotSupported => Ok(from.as_any_box().downcast::<DiskNotSupported>()?),
            StructType::IdeDiskNotSupported => Ok(from.as_any_box().downcast::<IdeDiskNotSupported>()?),
            StructType::DrsVmotionIncompatibleFault => Ok(from.as_any_box().downcast::<DrsVmotionIncompatibleFault>()?),
            StructType::FeatureRequirementsNotMet => Ok(from.as_any_box().downcast::<FeatureRequirementsNotMet>()?),
            StructType::MemorySizeNotRecommended => Ok(from.as_any_box().downcast::<MemorySizeNotRecommended>()?),
            StructType::MemorySizeNotSupported => Ok(from.as_any_box().downcast::<MemorySizeNotSupported>()?),
            StructType::MemorySizeNotSupportedByDatastore => Ok(from.as_any_box().downcast::<MemorySizeNotSupportedByDatastore>()?),
            StructType::NotEnoughCpus => Ok(from.as_any_box().downcast::<NotEnoughCpus>()?),
            StructType::NotEnoughLogicalCpus => Ok(from.as_any_box().downcast::<NotEnoughLogicalCpus>()?),
            StructType::NumVirtualCoresPerSocketNotSupported => Ok(from.as_any_box().downcast::<NumVirtualCoresPerSocketNotSupported>()?),
            StructType::NumVirtualCpusNotSupported => Ok(from.as_any_box().downcast::<NumVirtualCpusNotSupported>()?),
            StructType::StorageVmotionIncompatible => Ok(from.as_any_box().downcast::<StorageVmotionIncompatible>()?),
            StructType::VirtualHardwareVersionNotSupported => Ok(from.as_any_box().downcast::<VirtualHardwareVersionNotSupported>()?),
            StructType::WakeOnLanNotSupported => Ok(from.as_any_box().downcast::<WakeOnLanNotSupported>()?),
            StructType::VmConfigIncompatibleForFaultTolerance => Ok(from.as_any_box().downcast::<VmConfigIncompatibleForFaultTolerance>()?),
            StructType::VmConfigIncompatibleForRecordReplay => Ok(from.as_any_box().downcast::<VmConfigIncompatibleForRecordReplay>()?),
            StructType::VmHostAffinityRuleViolation => Ok(from.as_any_box().downcast::<VmHostAffinityRuleViolation>()?),
            StructType::VmFaultToleranceIssue => Ok(from.as_any_box().downcast::<VmFaultToleranceIssue>()?),
            StructType::CannotChangeDrsBehaviorForFtSecondary => Ok(from.as_any_box().downcast::<CannotChangeDrsBehaviorForFtSecondary>()?),
            StructType::CannotChangeHaSettingsForFtSecondary => Ok(from.as_any_box().downcast::<CannotChangeHaSettingsForFtSecondary>()?),
            StructType::CannotComputeFtCompatibleHosts => Ok(from.as_any_box().downcast::<CannotComputeFtCompatibleHosts>()?),
            StructType::FaultToleranceNotLicensed => Ok(from.as_any_box().downcast::<FaultToleranceNotLicensed>()?),
            StructType::FaultTolerancePrimaryPowerOnNotAttempted => Ok(from.as_any_box().downcast::<FaultTolerancePrimaryPowerOnNotAttempted>()?),
            StructType::FtIssuesOnHost => Ok(from.as_any_box().downcast::<FtIssuesOnHost>()?),
            StructType::HostIncompatibleForFaultTolerance => Ok(from.as_any_box().downcast::<HostIncompatibleForFaultTolerance>()?),
            StructType::IncompatibleHostForFtSecondary => Ok(from.as_any_box().downcast::<IncompatibleHostForFtSecondary>()?),
            StructType::InvalidOperationOnSecondaryVm => Ok(from.as_any_box().downcast::<InvalidOperationOnSecondaryVm>()?),
            StructType::NoHostSuitableForFtSecondary => Ok(from.as_any_box().downcast::<NoHostSuitableForFtSecondary>()?),
            StructType::NotSupportedDeviceForFt => Ok(from.as_any_box().downcast::<NotSupportedDeviceForFt>()?),
            StructType::PowerOnFtSecondaryFailed => Ok(from.as_any_box().downcast::<PowerOnFtSecondaryFailed>()?),
            StructType::SecondaryVmAlreadyDisabled => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyDisabled>()?),
            StructType::SecondaryVmAlreadyEnabled => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyEnabled>()?),
            StructType::SecondaryVmAlreadyRegistered => Ok(from.as_any_box().downcast::<SecondaryVmAlreadyRegistered>()?),
            StructType::SecondaryVmNotRegistered => Ok(from.as_any_box().downcast::<SecondaryVmNotRegistered>()?),
            StructType::VmFaultToleranceConfigIssue => Ok(from.as_any_box().downcast::<VmFaultToleranceConfigIssue>()?),
            StructType::VmFaultToleranceConfigIssueWrapper => Ok(from.as_any_box().downcast::<VmFaultToleranceConfigIssueWrapper>()?),
            StructType::VmFaultToleranceInvalidFileBacking => Ok(from.as_any_box().downcast::<VmFaultToleranceInvalidFileBacking>()?),
            StructType::VmFaultToleranceOpIssuesList => Ok(from.as_any_box().downcast::<VmFaultToleranceOpIssuesList>()?),
            StructType::VmMetadataManagerFault => Ok(from.as_any_box().downcast::<VmMetadataManagerFault>()?),
            StructType::VmMonitorIncompatibleForFaultTolerance => Ok(from.as_any_box().downcast::<VmMonitorIncompatibleForFaultTolerance>()?),
            StructType::VmToolsUpgradeFault => Ok(from.as_any_box().downcast::<VmToolsUpgradeFault>()?),
            StructType::ToolsAlreadyUpgraded => Ok(from.as_any_box().downcast::<ToolsAlreadyUpgraded>()?),
            StructType::ToolsAutoUpgradeNotSupported => Ok(from.as_any_box().downcast::<ToolsAutoUpgradeNotSupported>()?),
            StructType::ToolsImageCopyFailed => Ok(from.as_any_box().downcast::<ToolsImageCopyFailed>()?),
            StructType::ToolsImageNotAvailable => Ok(from.as_any_box().downcast::<ToolsImageNotAvailable>()?),
            StructType::ToolsImageSignatureCheckFailed => Ok(from.as_any_box().downcast::<ToolsImageSignatureCheckFailed>()?),
            StructType::ToolsUpgradeCancelled => Ok(from.as_any_box().downcast::<ToolsUpgradeCancelled>()?),
            StructType::VmValidateMaxDevice => Ok(from.as_any_box().downcast::<VmValidateMaxDevice>()?),
            StructType::VsanFault => Ok(from.as_any_box().downcast::<VsanFault>()?),
            StructType::CannotChangeVsanClusterUuid => Ok(from.as_any_box().downcast::<CannotChangeVsanClusterUuid>()?),
            StructType::CannotChangeVsanNodeUuid => Ok(from.as_any_box().downcast::<CannotChangeVsanNodeUuid>()?),
            StructType::CannotMoveVsanEnabledHost => Ok(from.as_any_box().downcast::<CannotMoveVsanEnabledHost>()?),
            StructType::DestinationVsanDisabled => Ok(from.as_any_box().downcast::<DestinationVsanDisabled>()?),
            StructType::VsanClusterUuidMismatch => Ok(from.as_any_box().downcast::<VsanClusterUuidMismatch>()?),
            StructType::CannotReconfigureVsanWhenHaEnabled => Ok(from.as_any_box().downcast::<CannotReconfigureVsanWhenHaEnabled>()?),
            StructType::DuplicateVsanNetworkInterface => Ok(from.as_any_box().downcast::<DuplicateVsanNetworkInterface>()?),
            StructType::VsanDiskFault => Ok(from.as_any_box().downcast::<VsanDiskFault>()?),
            StructType::DiskHasPartitions => Ok(from.as_any_box().downcast::<DiskHasPartitions>()?),
            StructType::DiskIsLastRemainingNonSsd => Ok(from.as_any_box().downcast::<DiskIsLastRemainingNonSsd>()?),
            StructType::DiskIsNonLocal => Ok(from.as_any_box().downcast::<DiskIsNonLocal>()?),
            StructType::DiskIsUsb => Ok(from.as_any_box().downcast::<DiskIsUsb>()?),
            StructType::DiskTooSmall => Ok(from.as_any_box().downcast::<DiskTooSmall>()?),
            StructType::DuplicateDisks => Ok(from.as_any_box().downcast::<DuplicateDisks>()?),
            StructType::InsufficientDisks => Ok(from.as_any_box().downcast::<InsufficientDisks>()?),
            StructType::VsanIncompatibleDiskMapping => Ok(from.as_any_box().downcast::<VsanIncompatibleDiskMapping>()?),
            StructType::WipeDiskFault => Ok(from.as_any_box().downcast::<WipeDiskFault>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
