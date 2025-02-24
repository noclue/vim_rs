use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Transport information about a SCSI target.
pub trait HostTargetTransportTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn HostTargetTransportTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostTargetTransportTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostTargetTransportVisitor)
            }
        }

struct HostTargetTransportVisitor;

impl<'de> de::Visitor<'de> for HostTargetTransportVisitor {
    type Value = Box<dyn HostTargetTransportTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostTargetTransportTrait JSON object with a _typeName field")
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

impl HostTargetTransportTrait for HostTargetTransport {
}
impl HostTargetTransportTrait for HostBlockAdapterTargetTransport {
}
impl HostTargetTransportTrait for HostFibreChannelTargetTransport {
}
impl HostTargetTransportTrait for HostFibreChannelOverEthernetTargetTransport {
}
impl HostTargetTransportTrait for HostInternetScsiTargetTransport {
}
impl HostTargetTransportTrait for HostParallelScsiTargetTransport {
}
impl HostTargetTransportTrait for HostPcieTargetTransport {
}
impl HostTargetTransportTrait for HostRdmaTargetTransport {
}
impl HostTargetTransportTrait for HostSerialAttachedTargetTransport {
}
impl HostTargetTransportTrait for HostTcpTargetTransport {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostTargetTransportTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostTargetTransport => Some(from.as_any_ref().downcast_ref::<HostTargetTransport>()?),
            StructType::HostBlockAdapterTargetTransport => Some(from.as_any_ref().downcast_ref::<HostBlockAdapterTargetTransport>()?),
            StructType::HostFibreChannelTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Some(from.as_any_ref().downcast_ref::<HostFibreChannelOverEthernetTargetTransport>()?),
            StructType::HostInternetScsiTargetTransport => Some(from.as_any_ref().downcast_ref::<HostInternetScsiTargetTransport>()?),
            StructType::HostParallelScsiTargetTransport => Some(from.as_any_ref().downcast_ref::<HostParallelScsiTargetTransport>()?),
            StructType::HostPcieTargetTransport => Some(from.as_any_ref().downcast_ref::<HostPcieTargetTransport>()?),
            StructType::HostRdmaTargetTransport => Some(from.as_any_ref().downcast_ref::<HostRdmaTargetTransport>()?),
            StructType::HostSerialAttachedTargetTransport => Some(from.as_any_ref().downcast_ref::<HostSerialAttachedTargetTransport>()?),
            StructType::HostTcpTargetTransport => Some(from.as_any_ref().downcast_ref::<HostTcpTargetTransport>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostTargetTransport => Ok(from.as_any_box().downcast::<HostTargetTransport>()?),
            StructType::HostBlockAdapterTargetTransport => Ok(from.as_any_box().downcast::<HostBlockAdapterTargetTransport>()?),
            StructType::HostFibreChannelTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelTargetTransport>()?),
            StructType::HostFibreChannelOverEthernetTargetTransport => Ok(from.as_any_box().downcast::<HostFibreChannelOverEthernetTargetTransport>()?),
            StructType::HostInternetScsiTargetTransport => Ok(from.as_any_box().downcast::<HostInternetScsiTargetTransport>()?),
            StructType::HostParallelScsiTargetTransport => Ok(from.as_any_box().downcast::<HostParallelScsiTargetTransport>()?),
            StructType::HostPcieTargetTransport => Ok(from.as_any_box().downcast::<HostPcieTargetTransport>()?),
            StructType::HostRdmaTargetTransport => Ok(from.as_any_box().downcast::<HostRdmaTargetTransport>()?),
            StructType::HostSerialAttachedTargetTransport => Ok(from.as_any_box().downcast::<HostSerialAttachedTargetTransport>()?),
            StructType::HostTcpTargetTransport => Ok(from.as_any_box().downcast::<HostTcpTargetTransport>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
