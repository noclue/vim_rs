use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// General information about a network.
pub trait NetworkSummaryTrait : super::data_object_trait::DataObjectTrait {
    /// Reference to the associated managed object.
    /// 
    /// Refers instance of *Network*.
    fn get_network(&self) -> &Option<ManagedObjectReference>;
    /// Name of the network.
    fn get_name(&self) -> &str;
    /// At least one host is configured to provide this network.
    fn get_accessible(&self) -> bool;
    /// Name of the associated IP pool.
    /// 
    /// Empty if the network is not associated with an
    /// IP pool.
    fn get_ip_pool_name(&self) -> &str;
    /// Identifier of the associated IP pool.
    /// 
    /// Zero if the network is not associated
    /// with an IP pool.
    fn get_ip_pool_id(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn NetworkSummaryTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NetworkSummaryTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NetworkSummaryVisitor)
            }
        }

struct NetworkSummaryVisitor;

impl<'de> de::Visitor<'de> for NetworkSummaryVisitor {
    type Value = Box<dyn NetworkSummaryTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NetworkSummaryTrait JSON object with a _typeName field")
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

impl NetworkSummaryTrait for NetworkSummary {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
    fn get_name(&self) -> &str { &self.name }
    fn get_accessible(&self) -> bool { self.accessible }
    fn get_ip_pool_name(&self) -> &str { &self.ip_pool_name }
    fn get_ip_pool_id(&self) -> Option<i32> { self.ip_pool_id }
}
impl NetworkSummaryTrait for OpaqueNetworkSummary {
    fn get_network(&self) -> &Option<ManagedObjectReference> { &self.network }
    fn get_name(&self) -> &str { &self.name }
    fn get_accessible(&self) -> bool { self.accessible }
    fn get_ip_pool_name(&self) -> &str { &self.ip_pool_name }
    fn get_ip_pool_id(&self) -> Option<i32> { self.ip_pool_id }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NetworkSummaryTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NetworkSummary => Some(from.as_any_ref().downcast_ref::<NetworkSummary>()?),
            StructType::OpaqueNetworkSummary => Some(from.as_any_ref().downcast_ref::<OpaqueNetworkSummary>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NetworkSummary => Ok(from.as_any_box().downcast::<NetworkSummary>()?),
            StructType::OpaqueNetworkSummary => Ok(from.as_any_box().downcast::<OpaqueNetworkSummary>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
