use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object represents the transport specific parameters
/// necessary to establish an NVM Express over Fabrics connection.
/// 
/// For some further information, see:
/// - "NVM Express over Fabrics 1.0", Section 1.5.7, "Connection"
pub trait HostNvmeTransportParametersTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn HostNvmeTransportParametersTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostNvmeTransportParametersTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostNvmeTransportParametersVisitor)
            }
        }

struct HostNvmeTransportParametersVisitor;

impl<'de> de::Visitor<'de> for HostNvmeTransportParametersVisitor {
    type Value = Box<dyn HostNvmeTransportParametersTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostNvmeTransportParametersTrait JSON object with a _typeName field")
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

impl HostNvmeTransportParametersTrait for HostNvmeTransportParameters {
}
impl HostNvmeTransportParametersTrait for HostNvmeOpaqueTransportParameters {
}
impl HostNvmeTransportParametersTrait for HostNvmeOverFibreChannelParameters {
}
impl HostNvmeTransportParametersTrait for HostNvmeOverRdmaParameters {
}
impl HostNvmeTransportParametersTrait for HostNvmeOverTcpParameters {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostNvmeTransportParametersTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostNvmeTransportParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeTransportParameters>()?),
            StructType::HostNvmeOpaqueTransportParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOpaqueTransportParameters>()?),
            StructType::HostNvmeOverFibreChannelParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverFibreChannelParameters>()?),
            StructType::HostNvmeOverRdmaParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverRdmaParameters>()?),
            StructType::HostNvmeOverTcpParameters => Some(from.as_any_ref().downcast_ref::<HostNvmeOverTcpParameters>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostNvmeTransportParameters => Ok(from.as_any_box().downcast::<HostNvmeTransportParameters>()?),
            StructType::HostNvmeOpaqueTransportParameters => Ok(from.as_any_box().downcast::<HostNvmeOpaqueTransportParameters>()?),
            StructType::HostNvmeOverFibreChannelParameters => Ok(from.as_any_box().downcast::<HostNvmeOverFibreChannelParameters>()?),
            StructType::HostNvmeOverRdmaParameters => Ok(from.as_any_box().downcast::<HostNvmeOverRdmaParameters>()?),
            StructType::HostNvmeOverTcpParameters => Ok(from.as_any_box().downcast::<HostNvmeOverTcpParameters>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
