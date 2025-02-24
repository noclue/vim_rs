use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Specifies the main parameters needed when connecting to
/// an NVMe over Fabrics controller or Discovery Service.
pub trait HostNvmeSpecTrait : super::data_object_trait::DataObjectTrait {
    /// The device name of the NVME over Fabrics host bus adapter.
    fn get_hba_name(&self) -> &str;
    /// Transport specific information necessary to connect to the controller.
    fn get_transport_parameters(&self) -> &Box<dyn super::host_nvme_transport_parameters_trait::HostNvmeTransportParametersTrait>;
}
impl<'s> serde::Serialize for dyn HostNvmeSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostNvmeSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostNvmeSpecVisitor)
            }
        }

struct HostNvmeSpecVisitor;

impl<'de> de::Visitor<'de> for HostNvmeSpecVisitor {
    type Value = Box<dyn HostNvmeSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostNvmeSpecTrait JSON object with a _typeName field")
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

impl HostNvmeSpecTrait for HostNvmeSpec {
    fn get_hba_name(&self) -> &str { &self.hba_name }
    fn get_transport_parameters(&self) -> &Box<dyn super::host_nvme_transport_parameters_trait::HostNvmeTransportParametersTrait> { &self.transport_parameters }
}
impl HostNvmeSpecTrait for HostNvmeConnectSpec {
    fn get_hba_name(&self) -> &str { &self.hba_name }
    fn get_transport_parameters(&self) -> &Box<dyn super::host_nvme_transport_parameters_trait::HostNvmeTransportParametersTrait> { &self.transport_parameters }
}
impl HostNvmeSpecTrait for HostNvmeDiscoverSpec {
    fn get_hba_name(&self) -> &str { &self.hba_name }
    fn get_transport_parameters(&self) -> &Box<dyn super::host_nvme_transport_parameters_trait::HostNvmeTransportParametersTrait> { &self.transport_parameters }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostNvmeSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostNvmeSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeSpec>()?),
            StructType::HostNvmeConnectSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeConnectSpec>()?),
            StructType::HostNvmeDiscoverSpec => Some(from.as_any_ref().downcast_ref::<HostNvmeDiscoverSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostNvmeSpec => Ok(from.as_any_box().downcast::<HostNvmeSpec>()?),
            StructType::HostNvmeConnectSpec => Ok(from.as_any_box().downcast::<HostNvmeConnectSpec>()?),
            StructType::HostNvmeDiscoverSpec => Ok(from.as_any_box().downcast::<HostNvmeDiscoverSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
