use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are dvs-related events.
pub trait DvsEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn DvsEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn DvsEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(DvsEventVisitor)
            }
        }

struct DvsEventVisitor;

impl<'de> de::Visitor<'de> for DvsEventVisitor {
    type Value = Box<dyn DvsEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid DvsEventTrait JSON object with a _typeName field")
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

impl DvsEventTrait for DvsEvent {
}
impl DvsEventTrait for DvsCreatedEvent {
}
impl DvsEventTrait for DvsDestroyedEvent {
}
impl DvsEventTrait for DvsHostBackInSyncEvent {
}
impl DvsEventTrait for DvsHostJoinedEvent {
}
impl DvsEventTrait for DvsHostLeftEvent {
}
impl DvsEventTrait for DvsHostStatusUpdated {
}
impl DvsEventTrait for DvsHostWentOutOfSyncEvent {
}
impl DvsEventTrait for DvsImportEvent {
}
impl DvsEventTrait for DvsMergedEvent {
}
impl DvsEventTrait for DvsPortBlockedEvent {
}
impl DvsEventTrait for DvsPortConnectedEvent {
}
impl DvsEventTrait for DvsPortCreatedEvent {
}
impl DvsEventTrait for DvsPortDeletedEvent {
}
impl DvsEventTrait for DvsPortDisconnectedEvent {
}
impl DvsEventTrait for DvsPortEnteredPassthruEvent {
}
impl DvsEventTrait for DvsPortExitedPassthruEvent {
}
impl DvsEventTrait for DvsPortJoinPortgroupEvent {
}
impl DvsEventTrait for DvsPortLeavePortgroupEvent {
}
impl DvsEventTrait for DvsPortLinkDownEvent {
}
impl DvsEventTrait for DvsPortLinkUpEvent {
}
impl DvsEventTrait for DvsPortReconfiguredEvent {
}
impl DvsEventTrait for DvsPortRuntimeChangeEvent {
}
impl DvsEventTrait for DvsPortUnblockedEvent {
}
impl DvsEventTrait for DvsPortVendorSpecificStateChangeEvent {
}
impl DvsEventTrait for DvsReconfiguredEvent {
}
impl DvsEventTrait for DvsRenamedEvent {
}
impl DvsEventTrait for DvsRestoreEvent {
}
impl DvsEventTrait for DvsUpgradeAvailableEvent {
}
impl DvsEventTrait for DvsUpgradeInProgressEvent {
}
impl DvsEventTrait for DvsUpgradeRejectedEvent {
}
impl DvsEventTrait for DvsUpgradedEvent {
}
impl DvsEventTrait for HostLocalPortCreatedEvent {
}
impl DvsEventTrait for OutOfSyncDvsHost {
}
impl DvsEventTrait for RecoveryEvent {
}
impl DvsEventTrait for RollbackEvent {
}
impl DvsEventTrait for VmVnicPoolReservationViolationClearEvent {
}
impl DvsEventTrait for VmVnicPoolReservationViolationRaiseEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn DvsEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsEvent => Some(from.as_any_ref().downcast_ref::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Some(from.as_any_ref().downcast_ref::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Some(from.as_any_ref().downcast_ref::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Some(from.as_any_ref().downcast_ref::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Some(from.as_any_ref().downcast_ref::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Some(from.as_any_ref().downcast_ref::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Some(from.as_any_ref().downcast_ref::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Some(from.as_any_ref().downcast_ref::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Some(from.as_any_ref().downcast_ref::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Some(from.as_any_ref().downcast_ref::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Some(from.as_any_ref().downcast_ref::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Some(from.as_any_ref().downcast_ref::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Some(from.as_any_ref().downcast_ref::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Some(from.as_any_ref().downcast_ref::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Some(from.as_any_ref().downcast_ref::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Some(from.as_any_ref().downcast_ref::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Some(from.as_any_ref().downcast_ref::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Some(from.as_any_ref().downcast_ref::<RecoveryEvent>()?),
            StructType::RollbackEvent => Some(from.as_any_ref().downcast_ref::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Some(from.as_any_ref().downcast_ref::<VmVnicPoolReservationViolationRaiseEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::DvsEvent => Ok(from.as_any_box().downcast::<DvsEvent>()?),
            StructType::DvsCreatedEvent => Ok(from.as_any_box().downcast::<DvsCreatedEvent>()?),
            StructType::DvsDestroyedEvent => Ok(from.as_any_box().downcast::<DvsDestroyedEvent>()?),
            StructType::DvsHostBackInSyncEvent => Ok(from.as_any_box().downcast::<DvsHostBackInSyncEvent>()?),
            StructType::DvsHostJoinedEvent => Ok(from.as_any_box().downcast::<DvsHostJoinedEvent>()?),
            StructType::DvsHostLeftEvent => Ok(from.as_any_box().downcast::<DvsHostLeftEvent>()?),
            StructType::DvsHostStatusUpdated => Ok(from.as_any_box().downcast::<DvsHostStatusUpdated>()?),
            StructType::DvsHostWentOutOfSyncEvent => Ok(from.as_any_box().downcast::<DvsHostWentOutOfSyncEvent>()?),
            StructType::DvsImportEvent => Ok(from.as_any_box().downcast::<DvsImportEvent>()?),
            StructType::DvsMergedEvent => Ok(from.as_any_box().downcast::<DvsMergedEvent>()?),
            StructType::DvsPortBlockedEvent => Ok(from.as_any_box().downcast::<DvsPortBlockedEvent>()?),
            StructType::DvsPortConnectedEvent => Ok(from.as_any_box().downcast::<DvsPortConnectedEvent>()?),
            StructType::DvsPortCreatedEvent => Ok(from.as_any_box().downcast::<DvsPortCreatedEvent>()?),
            StructType::DvsPortDeletedEvent => Ok(from.as_any_box().downcast::<DvsPortDeletedEvent>()?),
            StructType::DvsPortDisconnectedEvent => Ok(from.as_any_box().downcast::<DvsPortDisconnectedEvent>()?),
            StructType::DvsPortEnteredPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortEnteredPassthruEvent>()?),
            StructType::DvsPortExitedPassthruEvent => Ok(from.as_any_box().downcast::<DvsPortExitedPassthruEvent>()?),
            StructType::DvsPortJoinPortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortJoinPortgroupEvent>()?),
            StructType::DvsPortLeavePortgroupEvent => Ok(from.as_any_box().downcast::<DvsPortLeavePortgroupEvent>()?),
            StructType::DvsPortLinkDownEvent => Ok(from.as_any_box().downcast::<DvsPortLinkDownEvent>()?),
            StructType::DvsPortLinkUpEvent => Ok(from.as_any_box().downcast::<DvsPortLinkUpEvent>()?),
            StructType::DvsPortReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsPortReconfiguredEvent>()?),
            StructType::DvsPortRuntimeChangeEvent => Ok(from.as_any_box().downcast::<DvsPortRuntimeChangeEvent>()?),
            StructType::DvsPortUnblockedEvent => Ok(from.as_any_box().downcast::<DvsPortUnblockedEvent>()?),
            StructType::DvsPortVendorSpecificStateChangeEvent => Ok(from.as_any_box().downcast::<DvsPortVendorSpecificStateChangeEvent>()?),
            StructType::DvsReconfiguredEvent => Ok(from.as_any_box().downcast::<DvsReconfiguredEvent>()?),
            StructType::DvsRenamedEvent => Ok(from.as_any_box().downcast::<DvsRenamedEvent>()?),
            StructType::DvsRestoreEvent => Ok(from.as_any_box().downcast::<DvsRestoreEvent>()?),
            StructType::DvsUpgradeAvailableEvent => Ok(from.as_any_box().downcast::<DvsUpgradeAvailableEvent>()?),
            StructType::DvsUpgradeInProgressEvent => Ok(from.as_any_box().downcast::<DvsUpgradeInProgressEvent>()?),
            StructType::DvsUpgradeRejectedEvent => Ok(from.as_any_box().downcast::<DvsUpgradeRejectedEvent>()?),
            StructType::DvsUpgradedEvent => Ok(from.as_any_box().downcast::<DvsUpgradedEvent>()?),
            StructType::HostLocalPortCreatedEvent => Ok(from.as_any_box().downcast::<HostLocalPortCreatedEvent>()?),
            StructType::OutOfSyncDvsHost => Ok(from.as_any_box().downcast::<OutOfSyncDvsHost>()?),
            StructType::RecoveryEvent => Ok(from.as_any_box().downcast::<RecoveryEvent>()?),
            StructType::RollbackEvent => Ok(from.as_any_box().downcast::<RollbackEvent>()?),
            StructType::VmVnicPoolReservationViolationClearEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationClearEvent>()?),
            StructType::VmVnicPoolReservationViolationRaiseEvent => Ok(from.as_any_box().downcast::<VmVnicPoolReservationViolationRaiseEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
