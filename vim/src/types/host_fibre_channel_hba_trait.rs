use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes the Fibre Channel host bus adapter.
pub trait HostFibreChannelHbaTrait : super::host_host_bus_adapter_trait::HostHostBusAdapterTrait {
    /// The world wide port name for the adapter.
    fn get_port_world_wide_name(&self) -> i64;
    /// The world wide node name for the adapter.
    fn get_node_world_wide_name(&self) -> i64;
    /// The type of the fiber channel port.
    fn get_port_type(&self) -> &super::enums::FibreChannelPortTypeEnum;
    /// The current operating speed of the adapter in
    /// bits per second.
    fn get_speed(&self) -> i64;
}
impl<'s> serde::Serialize for dyn HostFibreChannelHbaTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostFibreChannelHbaTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostFibreChannelHbaVisitor)
            }
        }

struct HostFibreChannelHbaVisitor;

impl<'de> de::Visitor<'de> for HostFibreChannelHbaVisitor {
    type Value = Box<dyn HostFibreChannelHbaTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostFibreChannelHbaTrait JSON object with a _typeName field")
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

impl HostFibreChannelHbaTrait for HostFibreChannelHba {
    fn get_port_world_wide_name(&self) -> i64 { self.port_world_wide_name }
    fn get_node_world_wide_name(&self) -> i64 { self.node_world_wide_name }
    fn get_port_type(&self) -> &super::enums::FibreChannelPortTypeEnum { &self.port_type }
    fn get_speed(&self) -> i64 { self.speed }
}
impl HostFibreChannelHbaTrait for HostFibreChannelOverEthernetHba {
    fn get_port_world_wide_name(&self) -> i64 { self.port_world_wide_name }
    fn get_node_world_wide_name(&self) -> i64 { self.node_world_wide_name }
    fn get_port_type(&self) -> &super::enums::FibreChannelPortTypeEnum { &self.port_type }
    fn get_speed(&self) -> i64 { self.speed }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostFibreChannelHbaTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFibreChannelHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetHba>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostFibreChannelHba => Ok(from.as_any_box().downcast::<HostFibreChannelHba>()?),
            StructType::HostFibreChannelOverEthernetHba => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetHba>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
