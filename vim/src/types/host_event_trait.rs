use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are host-related events.
pub trait HostEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn HostEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostEventVisitor)
            }
        }

struct HostEventVisitor;

impl<'de> de::Visitor<'de> for HostEventVisitor {
    type Value = Box<dyn HostEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostEventTrait JSON object with a _typeName field")
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

impl HostEventTrait for HostEvent {
}
impl HostEventTrait for AccountCreatedEvent {
}
impl HostEventTrait for AccountRemovedEvent {
}
impl HostEventTrait for AccountUpdatedEvent {
}
impl HostEventTrait for AdminPasswordNotChangedEvent {
}
impl HostEventTrait for CanceledHostOperationEvent {
}
impl HostEventTrait for DatastoreDiscoveredEvent {
}
impl HostEventTrait for DatastorePrincipalConfigured {
}
impl HostEventTrait for DatastoreRemovedOnHostEvent {
}
impl HostEventTrait for DatastoreRenamedOnHostEvent {
}
impl HostEventTrait for DrsResourceConfigureFailedEvent {
}
impl HostEventTrait for DrsResourceConfigureSyncedEvent {
}
impl HostEventTrait for DuplicateIpDetectedEvent {
}
impl HostEventTrait for DvsHealthStatusChangeEvent {
}
impl HostEventTrait for MtuMatchEvent {
}
impl HostEventTrait for MtuMismatchEvent {
}
impl HostEventTrait for TeamingMatchEvent {
}
impl HostEventTrait for TeamingMisMatchEvent {
}
impl HostEventTrait for UplinkPortMtuNotSupportEvent {
}
impl HostEventTrait for UplinkPortMtuSupportEvent {
}
impl HostEventTrait for UplinkPortVlanTrunkedEvent {
}
impl HostEventTrait for UplinkPortVlanUntrunkedEvent {
}
impl HostEventTrait for EnteredMaintenanceModeEvent {
}
impl HostEventTrait for EnteredStandbyModeEvent {
}
impl HostEventTrait for DrsEnteredStandbyModeEvent {
}
impl HostEventTrait for EnteringMaintenanceModeEvent {
}
impl HostEventTrait for EnteringStandbyModeEvent {
}
impl HostEventTrait for DrsEnteringStandbyModeEvent {
}
impl HostEventTrait for ExitMaintenanceModeEvent {
}
impl HostEventTrait for ExitStandbyModeFailedEvent {
}
impl HostEventTrait for DrsExitStandbyModeFailedEvent {
}
impl HostEventTrait for ExitedStandbyModeEvent {
}
impl HostEventTrait for DrsExitedStandbyModeEvent {
}
impl HostEventTrait for ExitingStandbyModeEvent {
}
impl HostEventTrait for DrsExitingStandbyModeEvent {
}
impl HostEventTrait for GhostDvsProxySwitchDetectedEvent {
}
impl HostEventTrait for GhostDvsProxySwitchRemovedEvent {
}
impl HostEventTrait for HostAddFailedEvent {
}
impl HostEventTrait for HostAddedEvent {
}
impl HostEventTrait for HostAdminDisableEvent {
}
impl HostEventTrait for HostAdminEnableEvent {
}
impl HostEventTrait for HostCnxFailedAccountFailedEvent {
}
impl HostEventTrait for HostCnxFailedAlreadyManagedEvent {
}
impl HostEventTrait for HostCnxFailedBadCcagentEvent {
}
impl HostEventTrait for HostCnxFailedBadUsernameEvent {
}
impl HostEventTrait for HostCnxFailedBadVersionEvent {
}
impl HostEventTrait for HostCnxFailedCcagentUpgradeEvent {
}
impl HostEventTrait for HostCnxFailedEvent {
}
impl HostEventTrait for HostCnxFailedNetworkErrorEvent {
}
impl HostEventTrait for HostCnxFailedNoAccessEvent {
}
impl HostEventTrait for HostCnxFailedNoConnectionEvent {
}
impl HostEventTrait for HostCnxFailedNoLicenseEvent {
}
impl HostEventTrait for HostCnxFailedNotFoundEvent {
}
impl HostEventTrait for HostCnxFailedTimeoutEvent {
}
impl HostEventTrait for HostComplianceCheckedEvent {
}
impl HostEventTrait for HostCompliantEvent {
}
impl HostEventTrait for HostConfigAppliedEvent {
}
impl HostEventTrait for HostConnectedEvent {
}
impl HostEventTrait for HostConnectionLostEvent {
}
impl HostEventTrait for HostDasDisabledEvent {
}
impl HostEventTrait for HostDasDisablingEvent {
}
impl HostEventTrait for HostDasEnabledEvent {
}
impl HostEventTrait for HostDasEnablingEvent {
}
impl HostEventTrait for HostDasErrorEvent {
}
impl HostEventTrait for HostDasEvent {
}
impl HostEventTrait for HostExtraNetworksEvent {
}
impl HostEventTrait for HostIsolationIpPingFailedEvent {
}
impl HostEventTrait for HostMissingNetworksEvent {
}
impl HostEventTrait for HostNoAvailableNetworksEvent {
}
impl HostEventTrait for HostNoHaEnabledPortGroupsEvent {
}
impl HostEventTrait for HostNoRedundantManagementNetworkEvent {
}
impl HostEventTrait for HostNotInClusterEvent {
}
impl HostEventTrait for HostPrimaryAgentNotShortNameEvent {
}
impl HostEventTrait for HostShortNameInconsistentEvent {
}
impl HostEventTrait for HostDasOkEvent {
}
impl HostEventTrait for HostDisconnectedEvent {
}
impl HostEventTrait for HostEnableAdminFailedEvent {
}
impl HostEventTrait for HostGetShortNameFailedEvent {
}
impl HostEventTrait for HostInAuditModeEvent {
}
impl HostEventTrait for HostIpChangedEvent {
}
impl HostEventTrait for HostIpInconsistentEvent {
}
impl HostEventTrait for HostIpToShortNameFailedEvent {
}
impl HostEventTrait for HostNonCompliantEvent {
}
impl HostEventTrait for HostProfileAppliedEvent {
}
impl HostEventTrait for HostReconnectionFailedEvent {
}
impl HostEventTrait for HostRemovedEvent {
}
impl HostEventTrait for HostShortNameToIpFailedEvent {
}
impl HostEventTrait for HostShutdownEvent {
}
impl HostEventTrait for HostSpecificationChangedEvent {
}
impl HostEventTrait for HostSpecificationRequireEvent {
}
impl HostEventTrait for HostSpecificationUpdateEvent {
}
impl HostEventTrait for HostSubSpecificationDeleteEvent {
}
impl HostEventTrait for HostSubSpecificationUpdateEvent {
}
impl HostEventTrait for HostSyncFailedEvent {
}
impl HostEventTrait for HostUpgradeFailedEvent {
}
impl HostEventTrait for HostUserWorldSwapNotEnabledEvent {
}
impl HostEventTrait for HostVnicConnectedToCustomizedDvPortEvent {
}
impl HostEventTrait for HostWwnChangedEvent {
}
impl HostEventTrait for HostWwnConflictEvent {
}
impl HostEventTrait for LocalDatastoreCreatedEvent {
}
impl HostEventTrait for LocalTsmEnabledEvent {
}
impl HostEventTrait for NasDatastoreCreatedEvent {
}
impl HostEventTrait for NoDatastoresConfiguredEvent {
}
impl HostEventTrait for RemoteTsmEnabledEvent {
}
impl HostEventTrait for TimedOutHostOperationEvent {
}
impl HostEventTrait for UpdatedAgentBeingRestartedEvent {
}
impl HostEventTrait for UserAssignedToGroup {
}
impl HostEventTrait for UserPasswordChanged {
}
impl HostEventTrait for UserUnassignedFromGroup {
}
impl HostEventTrait for VmfsDatastoreCreatedEvent {
}
impl HostEventTrait for VmfsDatastoreExpandedEvent {
}
impl HostEventTrait for VmfsDatastoreExtendedEvent {
}
impl HostEventTrait for VcAgentUninstallFailedEvent {
}
impl HostEventTrait for VcAgentUninstalledEvent {
}
impl HostEventTrait for VcAgentUpgradeFailedEvent {
}
impl HostEventTrait for VcAgentUpgradedEvent {
}
impl HostEventTrait for VimAccountPasswordChangedEvent {
}
impl HostEventTrait for IScsiBootFailureEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
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
            _ => Err(from.as_any_box()),
        }
    }
}
