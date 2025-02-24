use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type encapsulates a typical set of ComputeResource information
/// that is useful for list views and summary pages.
pub trait ComputeResourceSummaryTrait : super::data_object_trait::DataObjectTrait {
    /// Aggregated CPU resources of all hosts, in MHz.
    fn get_total_cpu(&self) -> i32;
    /// Aggregated memory resources of all hosts, in bytes.
    fn get_total_memory(&self) -> i64;
    /// Number of physical CPU cores.
    /// 
    /// Physical CPU cores are the processors contained
    /// by a CPU package.
    fn get_num_cpu_cores(&self) -> i16;
    /// Aggregated number of CPU threads.
    fn get_num_cpu_threads(&self) -> i16;
    /// Effective CPU resources (in MHz) available to run virtual machines.
    /// 
    /// This is the
    /// aggregated effective resource level from all running hosts. Hosts that are in
    /// maintenance mode or are unresponsive are not counted. Resources used by the
    /// VMware Service Console are not included in the aggregate. This value represents
    /// the amount of resources available for the root resource pool for running
    /// virtual machines.
    fn get_effective_cpu(&self) -> i32;
    /// Effective memory resources (in MB) available to run virtual machines.
    /// 
    /// This is the aggregated effective resource level from all running hosts. Hosts
    /// that are in maintenance mode or are unresponsive are not counted.
    /// Resources used by the VMware Service Console are not included in the aggregate.
    /// This value represents the amount of resources available for the root
    /// resource pool for running virtual machines.
    fn get_effective_memory(&self) -> i64;
    /// Total number of hosts.
    fn get_num_hosts(&self) -> i32;
    /// Total number of effective hosts.
    fn get_num_effective_hosts(&self) -> i32;
    /// Overall alarm status.
    /// 
    /// In releases after vSphere API 5.0, vSphere Servers might not
    /// generate property collector update notifications for this property.
    /// To obtain the latest value of the property, you can use
    /// PropertyCollector methods RetrievePropertiesEx or WaitForUpdatesEx.
    /// If you use the PropertyCollector.WaitForUpdatesEx method, specify
    /// an empty string for the version parameter.
    /// Since this property is on a DataObject, an update returned by WaitForUpdatesEx may
    /// contain values for this property when some other property on the DataObject changes.
    /// If this update is a result of a call to WaitForUpdatesEx with a non-empty
    /// version parameter, the value for this property may not be current.
    fn get_overall_status(&self) -> &super::enums::ManagedEntityStatusEnum;
}
impl<'s> serde::Serialize for dyn ComputeResourceSummaryTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ComputeResourceSummaryTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ComputeResourceSummaryVisitor)
            }
        }

struct ComputeResourceSummaryVisitor;

impl<'de> de::Visitor<'de> for ComputeResourceSummaryVisitor {
    type Value = Box<dyn ComputeResourceSummaryTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ComputeResourceSummaryTrait JSON object with a _typeName field")
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

impl ComputeResourceSummaryTrait for ComputeResourceSummary {
    fn get_total_cpu(&self) -> i32 { self.total_cpu }
    fn get_total_memory(&self) -> i64 { self.total_memory }
    fn get_num_cpu_cores(&self) -> i16 { self.num_cpu_cores }
    fn get_num_cpu_threads(&self) -> i16 { self.num_cpu_threads }
    fn get_effective_cpu(&self) -> i32 { self.effective_cpu }
    fn get_effective_memory(&self) -> i64 { self.effective_memory }
    fn get_num_hosts(&self) -> i32 { self.num_hosts }
    fn get_num_effective_hosts(&self) -> i32 { self.num_effective_hosts }
    fn get_overall_status(&self) -> &super::enums::ManagedEntityStatusEnum { &self.overall_status }
}
impl ComputeResourceSummaryTrait for ClusterComputeResourceSummary {
    fn get_total_cpu(&self) -> i32 { self.total_cpu }
    fn get_total_memory(&self) -> i64 { self.total_memory }
    fn get_num_cpu_cores(&self) -> i16 { self.num_cpu_cores }
    fn get_num_cpu_threads(&self) -> i16 { self.num_cpu_threads }
    fn get_effective_cpu(&self) -> i32 { self.effective_cpu }
    fn get_effective_memory(&self) -> i64 { self.effective_memory }
    fn get_num_hosts(&self) -> i32 { self.num_hosts }
    fn get_num_effective_hosts(&self) -> i32 { self.num_effective_hosts }
    fn get_overall_status(&self) -> &super::enums::ManagedEntityStatusEnum { &self.overall_status }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ComputeResourceSummaryTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceSummary => Some(from.as_any_ref().downcast_ref::<ComputeResourceSummary>()?),
            StructType::ClusterComputeResourceSummary => Some(from.as_any_ref().downcast_ref::<ClusterComputeResourceSummary>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ComputeResourceSummary => Ok(from.as_any_box().downcast::<ComputeResourceSummary>()?),
            StructType::ClusterComputeResourceSummary => Ok(from.as_any_box().downcast::<ClusterComputeResourceSummary>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
