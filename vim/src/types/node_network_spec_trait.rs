use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The NodeNetworkSpec class defines network specification of a node
/// in the VCHA Cluster.
pub trait NodeNetworkSpecTrait : super::data_object_trait::DataObjectTrait {
    /// VCHA Cluster network configuration of the node.
    /// 
    /// All cluster communication (state replication, heartbeat,
    /// cluster messages) happens over this network.
    fn get_ip_settings(&self) -> &CustomizationIpSettings;
}
impl<'s> serde::Serialize for dyn NodeNetworkSpecTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NodeNetworkSpecTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NodeNetworkSpecVisitor)
            }
        }

struct NodeNetworkSpecVisitor;

impl<'de> de::Visitor<'de> for NodeNetworkSpecVisitor {
    type Value = Box<dyn NodeNetworkSpecTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NodeNetworkSpecTrait JSON object with a _typeName field")
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

impl NodeNetworkSpecTrait for NodeNetworkSpec {
    fn get_ip_settings(&self) -> &CustomizationIpSettings { &self.ip_settings }
}
impl NodeNetworkSpecTrait for PassiveNodeNetworkSpec {
    fn get_ip_settings(&self) -> &CustomizationIpSettings { &self.ip_settings }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NodeNetworkSpecTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NodeNetworkSpec => Some(from.as_any_ref().downcast_ref::<NodeNetworkSpec>()?),
            StructType::PassiveNodeNetworkSpec => Some(from.as_any_ref().downcast_ref::<PassiveNodeNetworkSpec>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NodeNetworkSpec => Ok(from.as_any_box().downcast::<NodeNetworkSpec>()?),
            StructType::PassiveNodeNetworkSpec => Ok(from.as_any_box().downcast::<PassiveNodeNetworkSpec>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
