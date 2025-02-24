use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for advanced runtime information related to the high
/// availability service for a cluster.
pub trait ClusterDasAdvancedRuntimeInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The information pertaining to the HA agents on the hosts
    fn get_das_host_info(&self) -> &Option<Box<dyn super::cluster_das_host_info_trait::ClusterDasHostInfoTrait>>;
    /// Whether HA VM Component Protection can be enabled for the cluster.
    fn get_vmcp_supported(&self) -> &Option<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo>;
    /// The map of a datastore to the set of hosts that are using
    /// the datastore for storage heartbeating.
    fn get_heartbeat_datastore_info(&self) -> &Option<Vec<DasHeartbeatDatastoreInfo>>;
}
impl<'s> serde::Serialize for dyn ClusterDasAdvancedRuntimeInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterDasAdvancedRuntimeInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterDasAdvancedRuntimeInfoVisitor)
            }
        }

struct ClusterDasAdvancedRuntimeInfoVisitor;

impl<'de> de::Visitor<'de> for ClusterDasAdvancedRuntimeInfoVisitor {
    type Value = Box<dyn ClusterDasAdvancedRuntimeInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterDasAdvancedRuntimeInfoTrait JSON object with a _typeName field")
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

impl ClusterDasAdvancedRuntimeInfoTrait for ClusterDasAdvancedRuntimeInfo {
    fn get_das_host_info(&self) -> &Option<Box<dyn super::cluster_das_host_info_trait::ClusterDasHostInfoTrait>> { &self.das_host_info }
    fn get_vmcp_supported(&self) -> &Option<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo> { &self.vmcp_supported }
    fn get_heartbeat_datastore_info(&self) -> &Option<Vec<DasHeartbeatDatastoreInfo>> { &self.heartbeat_datastore_info }
}
impl ClusterDasAdvancedRuntimeInfoTrait for ClusterDasFailoverLevelAdvancedRuntimeInfo {
    fn get_das_host_info(&self) -> &Option<Box<dyn super::cluster_das_host_info_trait::ClusterDasHostInfoTrait>> { &self.das_host_info }
    fn get_vmcp_supported(&self) -> &Option<ClusterDasAdvancedRuntimeInfoVmcpCapabilityInfo> { &self.vmcp_supported }
    fn get_heartbeat_datastore_info(&self) -> &Option<Vec<DasHeartbeatDatastoreInfo>> { &self.heartbeat_datastore_info }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterDasAdvancedRuntimeInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdvancedRuntimeInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasAdvancedRuntimeInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo => Some(from.as_any_ref().downcast_ref::<ClusterDasFailoverLevelAdvancedRuntimeInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdvancedRuntimeInfo => Ok(from.as_any_box().downcast::<ClusterDasAdvancedRuntimeInfo>()?),
            StructType::ClusterDasFailoverLevelAdvancedRuntimeInfo => Ok(from.as_any_box().downcast::<ClusterDasFailoverLevelAdvancedRuntimeInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
