use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// These are cluster events.
pub trait ClusterEventTrait : super::event_trait::EventTrait {
}
impl<'s> serde::Serialize for dyn ClusterEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterEventVisitor)
            }
        }

struct ClusterEventVisitor;

impl<'de> de::Visitor<'de> for ClusterEventVisitor {
    type Value = Box<dyn ClusterEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterEventTrait JSON object with a _typeName field")
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

impl ClusterEventTrait for ClusterEvent {
}
impl ClusterEventTrait for ClusterComplianceCheckedEvent {
}
impl ClusterEventTrait for ClusterCreatedEvent {
}
impl ClusterEventTrait for ClusterDestroyedEvent {
}
impl ClusterEventTrait for ClusterOvercommittedEvent {
}
impl ClusterEventTrait for HostOvercommittedEvent {
}
impl ClusterEventTrait for ClusterReconfiguredEvent {
}
impl ClusterEventTrait for ClusterStatusChangedEvent {
}
impl ClusterEventTrait for HostStatusChangedEvent {
}
impl ClusterEventTrait for DasAdmissionControlDisabledEvent {
}
impl ClusterEventTrait for DasAdmissionControlEnabledEvent {
}
impl ClusterEventTrait for DasAgentFoundEvent {
}
impl ClusterEventTrait for DasAgentUnavailableEvent {
}
impl ClusterEventTrait for DasClusterIsolatedEvent {
}
impl ClusterEventTrait for DasDisabledEvent {
}
impl ClusterEventTrait for DasEnabledEvent {
}
impl ClusterEventTrait for DasHostFailedEvent {
}
impl ClusterEventTrait for DasHostIsolatedEvent {
}
impl ClusterEventTrait for DrsDisabledEvent {
}
impl ClusterEventTrait for DrsEnabledEvent {
}
impl ClusterEventTrait for DrsInvocationFailedEvent {
}
impl ClusterEventTrait for DrsRecoveredFromFailureEvent {
}
impl ClusterEventTrait for FailoverLevelRestored {
}
impl ClusterEventTrait for HostMonitoringStateChangedEvent {
}
impl ClusterEventTrait for InsufficientFailoverResourcesEvent {
}
impl ClusterEventTrait for VmHealthMonitoringStateChangedEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterEvent => Some(from.as_any_ref().downcast_ref::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Some(from.as_any_ref().downcast_ref::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Some(from.as_any_ref().downcast_ref::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Some(from.as_any_ref().downcast_ref::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Some(from.as_any_ref().downcast_ref::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Some(from.as_any_ref().downcast_ref::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Some(from.as_any_ref().downcast_ref::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Some(from.as_any_ref().downcast_ref::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Some(from.as_any_ref().downcast_ref::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Some(from.as_any_ref().downcast_ref::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Some(from.as_any_ref().downcast_ref::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Some(from.as_any_ref().downcast_ref::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Some(from.as_any_ref().downcast_ref::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Some(from.as_any_ref().downcast_ref::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Some(from.as_any_ref().downcast_ref::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Some(from.as_any_ref().downcast_ref::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Some(from.as_any_ref().downcast_ref::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Some(from.as_any_ref().downcast_ref::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Some(from.as_any_ref().downcast_ref::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Some(from.as_any_ref().downcast_ref::<VmHealthMonitoringStateChangedEvent>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterEvent => Ok(from.as_any_box().downcast::<ClusterEvent>()?),
            StructType::ClusterComplianceCheckedEvent => Ok(from.as_any_box().downcast::<ClusterComplianceCheckedEvent>()?),
            StructType::ClusterCreatedEvent => Ok(from.as_any_box().downcast::<ClusterCreatedEvent>()?),
            StructType::ClusterDestroyedEvent => Ok(from.as_any_box().downcast::<ClusterDestroyedEvent>()?),
            StructType::ClusterOvercommittedEvent => Ok(from.as_any_box().downcast::<ClusterOvercommittedEvent>()?),
            StructType::HostOvercommittedEvent => Ok(from.as_any_box().downcast::<HostOvercommittedEvent>()?),
            StructType::ClusterReconfiguredEvent => Ok(from.as_any_box().downcast::<ClusterReconfiguredEvent>()?),
            StructType::ClusterStatusChangedEvent => Ok(from.as_any_box().downcast::<ClusterStatusChangedEvent>()?),
            StructType::HostStatusChangedEvent => Ok(from.as_any_box().downcast::<HostStatusChangedEvent>()?),
            StructType::DasAdmissionControlDisabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlDisabledEvent>()?),
            StructType::DasAdmissionControlEnabledEvent => Ok(from.as_any_box().downcast::<DasAdmissionControlEnabledEvent>()?),
            StructType::DasAgentFoundEvent => Ok(from.as_any_box().downcast::<DasAgentFoundEvent>()?),
            StructType::DasAgentUnavailableEvent => Ok(from.as_any_box().downcast::<DasAgentUnavailableEvent>()?),
            StructType::DasClusterIsolatedEvent => Ok(from.as_any_box().downcast::<DasClusterIsolatedEvent>()?),
            StructType::DasDisabledEvent => Ok(from.as_any_box().downcast::<DasDisabledEvent>()?),
            StructType::DasEnabledEvent => Ok(from.as_any_box().downcast::<DasEnabledEvent>()?),
            StructType::DasHostFailedEvent => Ok(from.as_any_box().downcast::<DasHostFailedEvent>()?),
            StructType::DasHostIsolatedEvent => Ok(from.as_any_box().downcast::<DasHostIsolatedEvent>()?),
            StructType::DrsDisabledEvent => Ok(from.as_any_box().downcast::<DrsDisabledEvent>()?),
            StructType::DrsEnabledEvent => Ok(from.as_any_box().downcast::<DrsEnabledEvent>()?),
            StructType::DrsInvocationFailedEvent => Ok(from.as_any_box().downcast::<DrsInvocationFailedEvent>()?),
            StructType::DrsRecoveredFromFailureEvent => Ok(from.as_any_box().downcast::<DrsRecoveredFromFailureEvent>()?),
            StructType::FailoverLevelRestored => Ok(from.as_any_box().downcast::<FailoverLevelRestored>()?),
            StructType::HostMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<HostMonitoringStateChangedEvent>()?),
            StructType::InsufficientFailoverResourcesEvent => Ok(from.as_any_box().downcast::<InsufficientFailoverResourcesEvent>()?),
            StructType::VmHealthMonitoringStateChangedEvent => Ok(from.as_any_box().downcast::<VmHealthMonitoringStateChangedEvent>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
