use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are virtual machine events.
pub trait VmEventTrait : super::event_trait::EventTrait {
    /// Indicates whether or not the virtual machine is marked as a template.
    fn get_template(&self) -> bool;
}
impl<'s> serde::Serialize for dyn VmEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VmEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VmEventVisitor)
            }
        }

struct VmEventVisitor;

impl<'de> de::Visitor<'de> for VmEventVisitor {
    type Value = Box<dyn VmEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VmEventTrait JSON object with a _typeName field")
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

impl VmEventTrait for VmEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationFailed {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationLinuxIdentityFailed {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationNetworkSetupFailed {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationSysprepFailed {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationUnknownFailure {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationStartedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for CustomizationSucceeded {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for DrsRuleComplianceEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for DrsRuleViolationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for DrsSoftRuleViolationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationErrorEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationHostErrorEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationHostWarningEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationResourceErrorEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationResourceWarningEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for MigrationWarningEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for NoMaintenanceModeDrsRecommendationForVm {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for NotEnoughResourcesToStartVmEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmAcquiredMksTicketEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmAcquiredTicketEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmAutoRenameEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingCreatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingDeployedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingHotMigratedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingMigratedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmCloneEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingClonedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingClonedNoFolderEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmCloneFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmClonedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmConfigMissingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmConnectedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmCreatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDasBeingResetEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDasBeingResetWithScreenshotEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDasResetFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDasUpdateErrorEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDasUpdateOkEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDateRolledBackEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDeployFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDeployedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDisconnectedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDiscoveredEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmDiskFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmEmigratingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmEndRecordingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmEndReplayingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedMigrateEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedRelayoutEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedRelayoutOnVmfs2DatastoreEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedStartingSecondaryEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToPowerOffEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToPowerOnEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToRebootGuestEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToResetEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToShutdownGuestEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToStandbyGuestEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedToSuspendEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailedUpdatingSecondaryConfig {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFailoverFailed {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFaultToleranceStateChangedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFaultToleranceTurnedOffEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmFaultToleranceVmTerminatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmGuestOsCrashedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmGuestRebootEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmGuestShutdownEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmGuestStandbyEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmInstanceUuidAssignedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmInstanceUuidChangedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmInstanceUuidConflictEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMacAssignedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMacChangedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMacConflictEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMaxFtRestartCountReached {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMaxRestartCountReached {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMessageErrorEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMessageEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMessageWarningEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmMigratedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for DrsVmMigratedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmNoCompatibleHostForSecondaryEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmNoNetworkAccessEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmOrphanedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmPoweredOffEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmPowerOffOnIsolationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmShutdownOnIsolationEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmPoweredOnEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for DrsVmPoweredOnEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRestartedOnAlternateHostEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmPoweringOnWithCustomizedDvPortEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmPrimaryFailoverEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmReconfiguredEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRegisteredEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRelayoutSuccessfulEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRelayoutUpToDateEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmReloadFromPathEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmReloadFromPathFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRelocateSpecEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmBeingRelocatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRelocateFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRelocatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRemoteConsoleConnectedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRemoteConsoleDisconnectedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRemovedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRenamedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmRequirementsExceedCurrentEvcModeEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmResettingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmResourcePoolMovedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmResourceReallocatedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmResumingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSecondaryAddedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSecondaryDisabledBySystemEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSecondaryDisabledEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSecondaryEnabledEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSecondaryStartedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStartRecordingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStartReplayingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStartingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUnsupportedStartingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStartingSecondaryEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStaticMacConflictEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmStoppingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSuspendedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmSuspendingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmTimedoutStartingSecondaryEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUpgradeCompleteEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUpgradeFailedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUpgradingEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUuidAssignedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUuidChangedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmUuidConflictEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmWwnAssignedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmWwnChangedEvent {
    fn get_template(&self) -> bool { self.template }
}
impl VmEventTrait for VmWwnConflictEvent {
    fn get_template(&self) -> bool { self.template }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VmEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
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
            _ => Err(from.as_any_box()),
        }
    }
}
