use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Fibre Channel transport information about a SCSI target.
pub trait HostFibreChannelTargetTransportTrait : super::host_target_transport_trait::HostTargetTransportTrait {
    /// The world wide port name of the target.
    fn get_port_world_wide_name(&self) -> i64;
    /// The world wide node name of the target.
    fn get_node_world_wide_name(&self) -> i64;
}
impl<'s> serde::Serialize for dyn HostFibreChannelTargetTransportTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostFibreChannelTargetTransportTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostFibreChannelTargetTransportVisitor)
            }
        }

struct HostFibreChannelTargetTransportVisitor;

impl<'de> de::Visitor<'de> for HostFibreChannelTargetTransportVisitor {
    type Value = Box<dyn HostFibreChannelTargetTransportTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostFibreChannelTargetTransportTrait JSON object with a _typeName field")
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

impl HostFibreChannelTargetTransportTrait for HostFibreChannelTargetTransport {
    fn get_port_world_wide_name(&self) -> i64 { self.port_world_wide_name }
    fn get_node_world_wide_name(&self) -> i64 { self.node_world_wide_name }
}
impl HostFibreChannelTargetTransportTrait for HostFibreChannelOverEthernetTargetTransport {
    fn get_port_world_wide_name(&self) -> i64 { self.port_world_wide_name }
    fn get_node_world_wide_name(&self) -> i64 { self.node_world_wide_name }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostFibreChannelTargetTransportTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFibreChannelTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetTargetTransport>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFibreChannelTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetTargetTransport>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
