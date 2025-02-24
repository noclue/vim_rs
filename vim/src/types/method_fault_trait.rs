use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base data object type for all the object model faults
/// that an application might handle.
pub trait MethodFaultTrait : super::vim_object_trait::VimObjectTrait {
    /// Fault which is the cause of this fault.
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>>;
    /// Message which has details about the error
    /// Message can also contain a key to message catalog which
    /// can be used to generate better localized messages.
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>>;
}
impl<'s> serde::Serialize for dyn MethodFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn MethodFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(MethodFaultVisitor)
            }
        }

struct MethodFaultVisitor;

impl<'de> de::Visitor<'de> for MethodFaultVisitor {
    type Value = Box<dyn MethodFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid MethodFaultTrait JSON object with a _typeName field")
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

impl MethodFaultTrait for MethodFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VimFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ActiveDirectoryFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DomainNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidCamServer {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CamServerRefusedConnection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidCamCertificate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoPermissionOnAd {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NonAdUserRequired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AlreadyExists {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AlreadyUpgraded {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AnswerFileUpdateFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AuthMinimumAdminPermission {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessLocalSource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotDisconnectHostWithFaultToleranceVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotEnableVmcpForCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotMoveFaultToleranceVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotMoveHostWithFaultToleranceVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotPlaceWithoutPrerequisiteMoves {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ConcurrentAccess {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CustomizationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotDecryptPasswords {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CustomizationPending {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IpHostnameGeneratorError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LinuxVolumeNotClean {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingLinuxCustResources {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingWindowsCustResources {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MountError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NicSettingMismatch {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoDisksToCustomize {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UncustomizableGuest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnexpectedCustomizationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VolumeEditorError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DasConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DrsDisabledOnVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DuplicateName {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvsFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for BackupBlobReadFailure {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for BackupBlobWriteFailure {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CollectorAddressUnset {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ConflictingConfiguration {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvsApplyOperationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvsNotAuthorized {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvsOperationBulkFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvsScopeViolated {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ImportHostAddFailure {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ImportOperationBulkFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidIpfixConfig {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RollbackFailure {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SwitchIpUnset {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SwitchNotInUpgradeMode {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanDestPortConflict {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPortConflict {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPortMoveFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPortPromiscChangeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPortgroupPromiscChangeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPortgroupTypeChangeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanPromiscuousPortNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VspanSameSessionPortConflict {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ActiveVMsBlockingEvc {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisconnectedHostsBlockingEvc {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcModeIllegalByVendor {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcModeUnsupportedByHosts {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcUnsupportedByHostHardware {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcUnsupportedByHostSoftware {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HeterogenousHostsBlockingEvc {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ExtendedFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceVmNotDasProtected {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FcoeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FcoeFaultPnicHasNoPortSet {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotCreateFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotDeleteFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DirectoryNotEmpty {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileAlreadyExists {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileLocked {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileNameTooLong {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileNotWritable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileTooLarge {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncorrectFileType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NetworkCopyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoDiskSpace {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotADirectory {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotAFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyConcurrentNativeClones {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyNativeCloneLevels {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyNativeClonesOnFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GenericDrsFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestOperationsFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestAuthenticationChallenge {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestComponentsOutOfDate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestMultipleMappings {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestOperationsUnavailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestPermissionDenied {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestProcessNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryKeyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryKeyAlreadyExists {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryKeyHasSubkeys {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryKeyInvalid {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryKeyParentVolatile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryValueFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GuestRegistryValueNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidGuestLogin {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OperationDisabledByGuest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OperationNotSupportedByGuest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyGuestLogons {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AdminDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AdminNotDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for BlockedByFirewall {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ClockSkew {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisableAdminNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostConfigFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostInDomain {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidHostName {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NasConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidNasCredentials {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidNetworkResource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NasConnectionLimitReached {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NasSessionCredentialConflict {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NasVolumeNotMounted {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NetworkInaccessible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoPermissionOnNasVolume {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoGateway {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoVirtualNic {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PlatformConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidBundle {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchInstallFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchIntegrityError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmfsMountFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmfsAlreadyMounted {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmfsAmbiguousMount {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostConnectFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AgentInstallFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AlreadyBeingManaged {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AlreadyConnected {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAddHostWithFtVmAsStandalone {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAddHostWithFtVmToDifferentCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAddHostWithFtVmToNonHaCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayConnectFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayNotReachable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayOperationRefused {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayToHostConnectFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayHostNotReachable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayToHostAuthFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GatewayToHostTrustVerifyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MultipleCertificatesVerifyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoPermissionOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NonVmwareOuiMacNotSupportedHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostForVFlash {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostForVmcp {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostForVmemFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostForVsan {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostInCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedCpuFeaturesForMode {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedCpuModel {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedCpuModelForMode {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedCpuVendor {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedCpuVendorUnknown {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedHostDisconnected {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedHostSoftware {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedHostSoftwareForMode {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EvcAdmissionFailedVmActive {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostInDvs {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostInHaCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReadHostResourcePoolTreeFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SslDisabledFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SslVerifyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyHosts {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostHasComponentFailure {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostIncompatibleForRecordReplay {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostPowerOpFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoPeerHostFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmotionInterfaceNotEnabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WakeOnLanNotSupportedByVmotionNic {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostSpecificationOperationFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HttpFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IormNotSupportedHostOnDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InaccessibleVFlashSource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientAgentVmsDeployed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientCpuResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientFailoverResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientGraphicsResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientHostCapacityFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientHostCpuCapacityFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientHostMemoryCapacityFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientPerCpuCapacity {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientMemoryResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientNetworkCapacity {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientNetworkResourcePoolCapacity {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientStandbyResource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientStandbyCpuResource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientStandbyMemoryResource {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientStorageSpace {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientVFlashResourcesFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidResourcePoolStructureFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NumVirtualCpusExceedsLimit {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceTooManyFtVcpusOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceTooManyVMsOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmSmpFaultToleranceTooManyVMsOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientStorageIops {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidAffinitySettingFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidBmcRole {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DatastoreNotWritableOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SwapDatastoreNotWritableOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InaccessibleDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InaccessibleFtMetadataDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDatastorePath {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidEvent {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidFolder {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmAlreadyExistsInDatacenter {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidIpmiLoginInfo {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidIpmiMacAddress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidLocale {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidLogin {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidClientCertificate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PasswordExpired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidName {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidPrivilege {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotPowerOffVmInCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EncryptionKeyRequired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDatastoreState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidHostState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidHostConnectionState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidPowerState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidVmState {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MksConnectionLimitReached {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoActiveHostInCluster {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerPowerOnFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for QuestionPending {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmPowerOnDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultInvalidVnic {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultPnicInUse {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicAlreadyBound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicHasActivePaths {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicHasMultipleUplinks {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicHasNoUplinks {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicHasWrongUplink {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicInUse {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicIsLastPath {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicNotBound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IscsiFaultVnicNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for KeyNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseEntityNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseServerUnavailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LimitExceeded {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LogBundlingFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MigrationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for AffinityConfigured {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotModifyConfigCpuRequirements {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotMoveVmWithDeltaDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotMoveVmWithNativeDeltaDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CloneFromSnapshotNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DatacenterMismatch {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisallowedMigrationDeviceAttached {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskMoveTypeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceAntiAffinityViolated {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceNeedsThickDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceNotSameBuild {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HaErrorsAtDest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncompatibleDefaultDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LargeRdmConversionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MaintenanceModeFileMove {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MigrationDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MigrationFeatureNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FullStorageVMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IndependentDiskVMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NonHomeRdmvMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageVMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnsharedSwapVMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionAcrossNetworkNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MigrationNotReady {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MismatchedNetworkPolicies {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MismatchedVMotionNetworkNames {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NetworksMayNotBeTheSame {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoGuestHeartbeat {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RdmConversionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RdmNotPreserved {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReadOnlyDisksWithLegacyDestination {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotCopyNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HotSnapshotMoveNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotCloneNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotMoveFromNonHomeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotMoveNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotMoveToNonHomeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotRevertIssue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SuspendedRelocateNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyDisksOnLegacyHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsInstallationInProgress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UncommittedUndoableDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionInterfaceIssue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionLinkCapacityLow {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionLinkDown {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionNotConfigured {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionNotLicensed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VMotionProtocolIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WillLoseHaProtection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WillModifyConfigCpuRequirements {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WillResetSnapshotDirectory {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MismatchedBundle {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingBmcSupport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NamespaceFull {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NamespaceLimitReached {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NamespaceWriteProtected {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NetworkDisruptedAndConfigRolledBack {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoClientCertificate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoCompatibleDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoCompatibleHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoCompatibleHostWithAccessToDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoConnectedDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoDiskFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoSubjectName {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedHostForChecksum {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OutOfBounds {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerCallbackFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerCommunicationError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerInvalidSection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerUndeclaredSection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerUndefinedPrefix {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ConnectedIso {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDuplicatedPropertyIdExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDuplicatedPropertyIdImport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfExportFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfHardwareExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConnectedDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConnectedDeviceFloppy {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConnectedDeviceIso {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnableToExportDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnknownDeviceBacking {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedDeviceExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyNetworkExport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfImport {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfCpuCompatibility {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfCpuCompatibilityCheckNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfHardwareCheck {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfImportFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfMappedOsId {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfMissingHardware {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfNetworkMappingNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedDiskProvisioning {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidPackage {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfAttribute {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidValueConfiguration {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidValueEmpty {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidValueFormatMalformed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidValueReference {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfMissingAttribute {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConstraint {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDiskOrderConstraint {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfHostResourceConstraint {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDuplicateElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDuplicatedElementBoundary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfElementInvalidValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfMissingElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfMissingElementNormalBoundary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnexpectedElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfWrongElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfProperty {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyNetwork {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyQualifier {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyQualifierDuplicate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyQualifierIgnored {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfPropertyValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfWrongNamespace {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfXmlFormat {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfSystemFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfDiskMappingNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfHostValueNotParsed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInternalError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfToXmlUnsupportedElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnknownDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnknownEntity {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedDeviceBackingInfo {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedDeviceBackingOption {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedPackage {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfInvalidVmName {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfNoHostNic {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfNoSupportedHardwareFamily {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedAttribute {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedAttributeValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedElement {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfNoSpaceOnController {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedElementValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedSection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedSubType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfUnsupportedType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchBinariesNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchMetadataInvalid {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchMetadataCorrupted {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchMetadataNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchNotApplicable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchAlreadyInstalled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchMissingDependencies {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PatchSuperseded {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ProfileUpdateFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RebootRequired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RecordReplayDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RemoveFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncompatibleHostForVmReplication {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationDiskConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationVmConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationIncompatibleWithFt {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationInvalidOptions {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationNotSupportedOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationVmFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ReplicationVmInProgressFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ResourceInUse {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FilterInUse {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for QuiesceDatastoreIoForHaFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ResourceNotAvailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SspiChallenge {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ShrinkDiskFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ApplicationQuiesceFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FilesystemQuiesceFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemorySnapshotOnIndependentDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MultipleSnapshotsNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotIncompatibleDeviceInVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotLocked {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SnapshotNoChange {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManySnapshotLevels {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SsdDiskNotAvailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveDiskInMultiWriterMode {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveFtVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveIndependentDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveManuallyPlacedSwapFile {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveManuallyPlacedVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveSharedDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveTemplate {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveVmInUserFolder {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveVmWithMountedCdrom {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsCannotMoveVmWithNoFilesInLayout {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsDatacentersCannotShareDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsDisabledOnVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsHbrDiskNotMovable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsHmsMoveInProgress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsHmsUnreachable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsIolbDisabledInternally {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsRelocateDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsStaleHmsCollection {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageDrsUnableToMoveFiles {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SwapDatastoreUnset {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TaskInProgress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VAppTaskInProgress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for Timedout {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PowerOnFtSecondaryTimedout {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyConsecutiveOverrides {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsUnavailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnrecognizedHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnsupportedVimApiVersion {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UserNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VAppConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingPowerOffConfiguration {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingPowerOnConfiguration {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoVmInVApp {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VFlashModuleVersionIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessVmComponent {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessVmConfig {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessVmDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessNetwork {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DestinationSwitchFull {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LegacyNetworkInterfaceInUse {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmOnConflictDvPort {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmOnVirtualIntranet {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotAccessVmDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RdmPointsToInaccessibleDisk {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotDisableSnapshot {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotUseNetwork {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CpuHotPlugNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeltaDiskFormatNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for EightHostLimitViolated {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceCannotEditMem {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for GenericVmConfigFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidFormat {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDiskFormat {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidSnapshotFormat {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidVmConfig {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDeviceSpec {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceHotPlugNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceUnsupportedForVmPlatform {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceUnsupportedForVmVersion {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisallowedDiskModeChange {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidController {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDeviceBacking {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDeviceOperation {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingController {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SwapPlacementOverrideNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for TooManyDevices {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnsupportedGuest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmWwnConflict {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LargeRdmNotSupportedOnDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemoryHotPlugNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoCompatibleHardAffinityHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoCompatibleSoftAffinityHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NumVirtualCpusIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OvfConsumerValidationFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for QuarantineModeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RdmNotSupportedOnDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RuleViolation {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SoftRuleVioCorrectionDisallowed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SoftRuleVioCorrectionImpact {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnsupportedDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemoryFileFormatNotSupportedByDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnSupportedDatastoreForVFlash {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnsupportedVmxLocation {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VAppNotRunning {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VAppPropertyFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidNetworkInType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidPropertyType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidPropertyValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnconfiguredPropertyValue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingIpPool {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MissingNetworkIpConfig {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoAvailableIp {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoVcManagedIpConfigured {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotUserConfigurableProperty {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VFlashCacheHotConfigNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VFlashModuleNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VirtualHardwareCompatibilityIssue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CpuIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CpuCompatibilityUnknown {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CpuIncompatible1Ecx {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CpuIncompatible81Edx {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceCpuIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceBackingNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DvPortNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnusedVirtualDiskBlocksNotScrubbed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VirtualDiskBlocksNotFullyProvisioned {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DeviceControllerNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DigestNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FileBackedPortNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MultiWriterNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NonPersistentDisksNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RdmNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PhysCompatRdmNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RawDiskNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RemoteDeviceNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SharedBusControllerNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmiNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VirtualDiskModeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VirtualEthernetCardNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IdeDiskNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DrsVmotionIncompatibleFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FeatureRequirementsNotMet {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemorySizeNotRecommended {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemorySizeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MemorySizeNotSupportedByDatastore {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotEnoughCpus {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotEnoughLogicalCpus {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NumVirtualCoresPerSocketNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NumVirtualCpusNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for StorageVmotionIncompatible {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VirtualHardwareVersionNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WakeOnLanNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmConfigIncompatibleForFaultTolerance {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmConfigIncompatibleForRecordReplay {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmHostAffinityRuleViolation {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceIssue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotChangeDrsBehaviorForFtSecondary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotChangeHaSettingsForFtSecondary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotComputeFtCompatibleHosts {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultToleranceNotLicensed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FaultTolerancePrimaryPowerOnNotAttempted {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FtIssuesOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostIncompatibleForFaultTolerance {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncompatibleHostForFtSecondary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidOperationOnSecondaryVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoHostSuitableForFtSecondary {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupportedDeviceForFt {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for PowerOnFtSecondaryFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SecondaryVmAlreadyDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SecondaryVmAlreadyEnabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SecondaryVmAlreadyRegistered {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SecondaryVmNotRegistered {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceConfigIssue {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceConfigIssueWrapper {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceInvalidFileBacking {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmFaultToleranceOpIssuesList {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmMetadataManagerFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmMonitorIncompatibleForFaultTolerance {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmToolsUpgradeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsAlreadyUpgraded {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsAutoUpgradeNotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsImageCopyFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsImageNotAvailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsImageSignatureCheckFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ToolsUpgradeCancelled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmValidateMaxDevice {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VsanFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotChangeVsanClusterUuid {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotChangeVsanNodeUuid {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotMoveVsanEnabledHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DestinationVsanDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VsanClusterUuidMismatch {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotReconfigureVsanWhenHaEnabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DuplicateVsanNetworkInterface {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VsanDiskFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskHasPartitions {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskIsLastRemainingNonSsd {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskIsNonLocal {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskIsUsb {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DiskTooSmall {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DuplicateDisks {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InsufficientDisks {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VsanIncompatibleDiskMapping {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for WipeDiskFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RuntimeFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for CannotDisableDrsOnClustersWithVApps {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ConflictingDatastoreFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DatabaseError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisallowedChangeByService {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for DisallowedOperationOnFailoverHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FailToLockFaultToleranceVMs {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidProfileReferenceHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidScheduledTask {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseAssignmentFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MethodAlreadyDisabledFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MethodDisabled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for OperationDisallowedOnHost {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RestrictedByAdministrator {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ThirdPartyLicenseAssignmentFailed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VAppOperationInProgress {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostCommunication {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostNotConnected {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostNotReachable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidArgument {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncompatibleSetting {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDasConfigArgument {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDasRestartPriorityForFtVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidDrsBehaviorForFtVm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidIndexArgument {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidRequest {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidType {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for MethodNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ManagedObjectNotFound {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotEnoughLicenses {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ExpiredFeatureLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ExpiredAddonLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for ExpiredEditionLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for FailToEnableSpbm {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostInventoryFull {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InUseFeatureManipulationDisallowed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for IncorrectHostInformation {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidEditionLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InventoryHasStandardAloneHosts {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseDowngradeDisallowed {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseExpired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseKeyEntityMismatch {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseRestricted {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for LicenseSourceUnavailable {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoLicenseServerConfigured {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VmLimitLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for VramLimitLicense {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotImplemented {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotSupported {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for HostAccessRestrictedToManagementServer {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RequestCanceled {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SecurityError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NoPermission {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for NotAuthenticated {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for RestrictedVersion {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SolutionUserRequired {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for SystemError {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for UnexpectedFault {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidCollectorVersion {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl MethodFaultTrait for InvalidProperty {
    fn get_fault_cause(&self) -> &Option<Box<dyn super::method_fault_trait::MethodFaultTrait>> { &self.fault_cause }
    fn get_fault_message(&self) -> &Option<Vec<LocalizableMessage>> { &self.fault_message }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn MethodFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::MethodFault => Some(from.as_any_ref().downcast_ref::<MethodFault>()?),
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
            StructType::RuntimeFault => Some(from.as_any_ref().downcast_ref::<RuntimeFault>()?),
            StructType::CannotDisableDrsOnClustersWithVApps => Some(from.as_any_ref().downcast_ref::<CannotDisableDrsOnClustersWithVApps>()?),
            StructType::ConflictingDatastoreFound => Some(from.as_any_ref().downcast_ref::<ConflictingDatastoreFound>()?),
            StructType::DatabaseError => Some(from.as_any_ref().downcast_ref::<DatabaseError>()?),
            StructType::DisallowedChangeByService => Some(from.as_any_ref().downcast_ref::<DisallowedChangeByService>()?),
            StructType::DisallowedOperationOnFailoverHost => Some(from.as_any_ref().downcast_ref::<DisallowedOperationOnFailoverHost>()?),
            StructType::FailToLockFaultToleranceVMs => Some(from.as_any_ref().downcast_ref::<FailToLockFaultToleranceVMs>()?),
            StructType::InvalidProfileReferenceHost => Some(from.as_any_ref().downcast_ref::<InvalidProfileReferenceHost>()?),
            StructType::InvalidScheduledTask => Some(from.as_any_ref().downcast_ref::<InvalidScheduledTask>()?),
            StructType::LicenseAssignmentFailed => Some(from.as_any_ref().downcast_ref::<LicenseAssignmentFailed>()?),
            StructType::MethodAlreadyDisabledFault => Some(from.as_any_ref().downcast_ref::<MethodAlreadyDisabledFault>()?),
            StructType::MethodDisabled => Some(from.as_any_ref().downcast_ref::<MethodDisabled>()?),
            StructType::OperationDisallowedOnHost => Some(from.as_any_ref().downcast_ref::<OperationDisallowedOnHost>()?),
            StructType::RestrictedByAdministrator => Some(from.as_any_ref().downcast_ref::<RestrictedByAdministrator>()?),
            StructType::ThirdPartyLicenseAssignmentFailed => Some(from.as_any_ref().downcast_ref::<ThirdPartyLicenseAssignmentFailed>()?),
            StructType::VAppOperationInProgress => Some(from.as_any_ref().downcast_ref::<VAppOperationInProgress>()?),
            StructType::HostCommunication => Some(from.as_any_ref().downcast_ref::<HostCommunication>()?),
            StructType::HostNotConnected => Some(from.as_any_ref().downcast_ref::<HostNotConnected>()?),
            StructType::HostNotReachable => Some(from.as_any_ref().downcast_ref::<HostNotReachable>()?),
            StructType::InvalidArgument => Some(from.as_any_ref().downcast_ref::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Some(from.as_any_ref().downcast_ref::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Some(from.as_any_ref().downcast_ref::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Some(from.as_any_ref().downcast_ref::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Some(from.as_any_ref().downcast_ref::<InvalidIndexArgument>()?),
            StructType::InvalidRequest => Some(from.as_any_ref().downcast_ref::<InvalidRequest>()?),
            StructType::InvalidType => Some(from.as_any_ref().downcast_ref::<InvalidType>()?),
            StructType::MethodNotFound => Some(from.as_any_ref().downcast_ref::<MethodNotFound>()?),
            StructType::ManagedObjectNotFound => Some(from.as_any_ref().downcast_ref::<ManagedObjectNotFound>()?),
            StructType::NotEnoughLicenses => Some(from.as_any_ref().downcast_ref::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Some(from.as_any_ref().downcast_ref::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Some(from.as_any_ref().downcast_ref::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Some(from.as_any_ref().downcast_ref::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Some(from.as_any_ref().downcast_ref::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Some(from.as_any_ref().downcast_ref::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Some(from.as_any_ref().downcast_ref::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Some(from.as_any_ref().downcast_ref::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Some(from.as_any_ref().downcast_ref::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Some(from.as_any_ref().downcast_ref::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Some(from.as_any_ref().downcast_ref::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Some(from.as_any_ref().downcast_ref::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Some(from.as_any_ref().downcast_ref::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Some(from.as_any_ref().downcast_ref::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Some(from.as_any_ref().downcast_ref::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Some(from.as_any_ref().downcast_ref::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Some(from.as_any_ref().downcast_ref::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Some(from.as_any_ref().downcast_ref::<VramLimitLicense>()?),
            StructType::NotImplemented => Some(from.as_any_ref().downcast_ref::<NotImplemented>()?),
            StructType::NotSupported => Some(from.as_any_ref().downcast_ref::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Some(from.as_any_ref().downcast_ref::<HostAccessRestrictedToManagementServer>()?),
            StructType::RequestCanceled => Some(from.as_any_ref().downcast_ref::<RequestCanceled>()?),
            StructType::SecurityError => Some(from.as_any_ref().downcast_ref::<SecurityError>()?),
            StructType::NoPermission => Some(from.as_any_ref().downcast_ref::<NoPermission>()?),
            StructType::NotAuthenticated => Some(from.as_any_ref().downcast_ref::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Some(from.as_any_ref().downcast_ref::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Some(from.as_any_ref().downcast_ref::<SolutionUserRequired>()?),
            StructType::SystemError => Some(from.as_any_ref().downcast_ref::<SystemError>()?),
            StructType::UnexpectedFault => Some(from.as_any_ref().downcast_ref::<UnexpectedFault>()?),
            StructType::InvalidCollectorVersion => Some(from.as_any_ref().downcast_ref::<InvalidCollectorVersion>()?),
            StructType::InvalidProperty => Some(from.as_any_ref().downcast_ref::<InvalidProperty>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::MethodFault => Ok(from.as_any_box().downcast::<MethodFault>()?),
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
            StructType::RuntimeFault => Ok(from.as_any_box().downcast::<RuntimeFault>()?),
            StructType::CannotDisableDrsOnClustersWithVApps => Ok(from.as_any_box().downcast::<CannotDisableDrsOnClustersWithVApps>()?),
            StructType::ConflictingDatastoreFound => Ok(from.as_any_box().downcast::<ConflictingDatastoreFound>()?),
            StructType::DatabaseError => Ok(from.as_any_box().downcast::<DatabaseError>()?),
            StructType::DisallowedChangeByService => Ok(from.as_any_box().downcast::<DisallowedChangeByService>()?),
            StructType::DisallowedOperationOnFailoverHost => Ok(from.as_any_box().downcast::<DisallowedOperationOnFailoverHost>()?),
            StructType::FailToLockFaultToleranceVMs => Ok(from.as_any_box().downcast::<FailToLockFaultToleranceVMs>()?),
            StructType::InvalidProfileReferenceHost => Ok(from.as_any_box().downcast::<InvalidProfileReferenceHost>()?),
            StructType::InvalidScheduledTask => Ok(from.as_any_box().downcast::<InvalidScheduledTask>()?),
            StructType::LicenseAssignmentFailed => Ok(from.as_any_box().downcast::<LicenseAssignmentFailed>()?),
            StructType::MethodAlreadyDisabledFault => Ok(from.as_any_box().downcast::<MethodAlreadyDisabledFault>()?),
            StructType::MethodDisabled => Ok(from.as_any_box().downcast::<MethodDisabled>()?),
            StructType::OperationDisallowedOnHost => Ok(from.as_any_box().downcast::<OperationDisallowedOnHost>()?),
            StructType::RestrictedByAdministrator => Ok(from.as_any_box().downcast::<RestrictedByAdministrator>()?),
            StructType::ThirdPartyLicenseAssignmentFailed => Ok(from.as_any_box().downcast::<ThirdPartyLicenseAssignmentFailed>()?),
            StructType::VAppOperationInProgress => Ok(from.as_any_box().downcast::<VAppOperationInProgress>()?),
            StructType::HostCommunication => Ok(from.as_any_box().downcast::<HostCommunication>()?),
            StructType::HostNotConnected => Ok(from.as_any_box().downcast::<HostNotConnected>()?),
            StructType::HostNotReachable => Ok(from.as_any_box().downcast::<HostNotReachable>()?),
            StructType::InvalidArgument => Ok(from.as_any_box().downcast::<InvalidArgument>()?),
            StructType::IncompatibleSetting => Ok(from.as_any_box().downcast::<IncompatibleSetting>()?),
            StructType::InvalidDasConfigArgument => Ok(from.as_any_box().downcast::<InvalidDasConfigArgument>()?),
            StructType::InvalidDasRestartPriorityForFtVm => Ok(from.as_any_box().downcast::<InvalidDasRestartPriorityForFtVm>()?),
            StructType::InvalidDrsBehaviorForFtVm => Ok(from.as_any_box().downcast::<InvalidDrsBehaviorForFtVm>()?),
            StructType::InvalidIndexArgument => Ok(from.as_any_box().downcast::<InvalidIndexArgument>()?),
            StructType::InvalidRequest => Ok(from.as_any_box().downcast::<InvalidRequest>()?),
            StructType::InvalidType => Ok(from.as_any_box().downcast::<InvalidType>()?),
            StructType::MethodNotFound => Ok(from.as_any_box().downcast::<MethodNotFound>()?),
            StructType::ManagedObjectNotFound => Ok(from.as_any_box().downcast::<ManagedObjectNotFound>()?),
            StructType::NotEnoughLicenses => Ok(from.as_any_box().downcast::<NotEnoughLicenses>()?),
            StructType::ExpiredFeatureLicense => Ok(from.as_any_box().downcast::<ExpiredFeatureLicense>()?),
            StructType::ExpiredAddonLicense => Ok(from.as_any_box().downcast::<ExpiredAddonLicense>()?),
            StructType::ExpiredEditionLicense => Ok(from.as_any_box().downcast::<ExpiredEditionLicense>()?),
            StructType::FailToEnableSpbm => Ok(from.as_any_box().downcast::<FailToEnableSpbm>()?),
            StructType::HostInventoryFull => Ok(from.as_any_box().downcast::<HostInventoryFull>()?),
            StructType::InUseFeatureManipulationDisallowed => Ok(from.as_any_box().downcast::<InUseFeatureManipulationDisallowed>()?),
            StructType::IncorrectHostInformation => Ok(from.as_any_box().downcast::<IncorrectHostInformation>()?),
            StructType::InvalidEditionLicense => Ok(from.as_any_box().downcast::<InvalidEditionLicense>()?),
            StructType::InventoryHasStandardAloneHosts => Ok(from.as_any_box().downcast::<InventoryHasStandardAloneHosts>()?),
            StructType::LicenseDowngradeDisallowed => Ok(from.as_any_box().downcast::<LicenseDowngradeDisallowed>()?),
            StructType::LicenseExpired => Ok(from.as_any_box().downcast::<LicenseExpired>()?),
            StructType::LicenseKeyEntityMismatch => Ok(from.as_any_box().downcast::<LicenseKeyEntityMismatch>()?),
            StructType::LicenseRestricted => Ok(from.as_any_box().downcast::<LicenseRestricted>()?),
            StructType::LicenseSourceUnavailable => Ok(from.as_any_box().downcast::<LicenseSourceUnavailable>()?),
            StructType::NoLicenseServerConfigured => Ok(from.as_any_box().downcast::<NoLicenseServerConfigured>()?),
            StructType::VmLimitLicense => Ok(from.as_any_box().downcast::<VmLimitLicense>()?),
            StructType::VramLimitLicense => Ok(from.as_any_box().downcast::<VramLimitLicense>()?),
            StructType::NotImplemented => Ok(from.as_any_box().downcast::<NotImplemented>()?),
            StructType::NotSupported => Ok(from.as_any_box().downcast::<NotSupported>()?),
            StructType::HostAccessRestrictedToManagementServer => Ok(from.as_any_box().downcast::<HostAccessRestrictedToManagementServer>()?),
            StructType::RequestCanceled => Ok(from.as_any_box().downcast::<RequestCanceled>()?),
            StructType::SecurityError => Ok(from.as_any_box().downcast::<SecurityError>()?),
            StructType::NoPermission => Ok(from.as_any_box().downcast::<NoPermission>()?),
            StructType::NotAuthenticated => Ok(from.as_any_box().downcast::<NotAuthenticated>()?),
            StructType::RestrictedVersion => Ok(from.as_any_box().downcast::<RestrictedVersion>()?),
            StructType::SolutionUserRequired => Ok(from.as_any_box().downcast::<SolutionUserRequired>()?),
            StructType::SystemError => Ok(from.as_any_box().downcast::<SystemError>()?),
            StructType::UnexpectedFault => Ok(from.as_any_box().downcast::<UnexpectedFault>()?),
            StructType::InvalidCollectorVersion => Ok(from.as_any_box().downcast::<InvalidCollectorVersion>()?),
            StructType::InvalidProperty => Ok(from.as_any_box().downcast::<InvalidProperty>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
