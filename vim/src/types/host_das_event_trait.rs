use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Top-level event for host DAS events to extend.
pub trait HostDasEventTrait : super::host_event_trait::HostEventTrait {
}
impl<'s> serde::Serialize for dyn HostDasEventTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostDasEventTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostDasEventVisitor)
            }
        }

struct HostDasEventVisitor;

impl<'de> de::Visitor<'de> for HostDasEventVisitor {
    type Value = Box<dyn HostDasEventTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostDasEventTrait JSON object with a _typeName field")
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

impl HostDasEventTrait for HostDasEvent {
}
impl HostDasEventTrait for HostExtraNetworksEvent {
}
impl HostDasEventTrait for HostIsolationIpPingFailedEvent {
}
impl HostDasEventTrait for HostMissingNetworksEvent {
}
impl HostDasEventTrait for HostNoAvailableNetworksEvent {
}
impl HostDasEventTrait for HostNoHaEnabledPortGroupsEvent {
}
impl HostDasEventTrait for HostNoRedundantManagementNetworkEvent {
}
impl HostDasEventTrait for HostNotInClusterEvent {
}
impl HostDasEventTrait for HostPrimaryAgentNotShortNameEvent {
}
impl HostDasEventTrait for HostShortNameInconsistentEvent {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostDasEventTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
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
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
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
            _ => Err(from.as_any_box()),
        }
    }
}
