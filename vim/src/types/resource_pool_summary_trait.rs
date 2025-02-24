use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type encapsulates a typical set of resource
/// pool information that is useful for list views and summary pages.
pub trait ResourcePoolSummaryTrait : super::data_object_trait::DataObjectTrait {
    /// Name of resource pool.
    fn get_name(&self) -> &str;
    /// Current configuration of the resource pool.
    fn get_config(&self) -> &ResourceConfigSpec;
    /// Current runtime state of the resource pool.
    fn get_runtime(&self) -> &ResourcePoolRuntimeInfo;
    /// A set of statistics that are typically updated with near real-time regularity.
    /// 
    /// This data object type does not support notification, for scalability reasons.
    /// Therefore, changes in QuickStats do not generate property collector updates.
    /// To monitor statistics values, use the statistics and alarms modules instead.
    fn get_quick_stats(&self) -> &Option<ResourcePoolQuickStats>;
    /// Total configured memory of all virtual machines in the resource pool, in MB.
    fn get_configured_memory_mb(&self) -> Option<i32>;
}
impl<'s> serde::Serialize for dyn ResourcePoolSummaryTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ResourcePoolSummaryTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ResourcePoolSummaryVisitor)
            }
        }

struct ResourcePoolSummaryVisitor;

impl<'de> de::Visitor<'de> for ResourcePoolSummaryVisitor {
    type Value = Box<dyn ResourcePoolSummaryTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ResourcePoolSummaryTrait JSON object with a _typeName field")
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

impl ResourcePoolSummaryTrait for ResourcePoolSummary {
    fn get_name(&self) -> &str { &self.name }
    fn get_config(&self) -> &ResourceConfigSpec { &self.config }
    fn get_runtime(&self) -> &ResourcePoolRuntimeInfo { &self.runtime }
    fn get_quick_stats(&self) -> &Option<ResourcePoolQuickStats> { &self.quick_stats }
    fn get_configured_memory_mb(&self) -> Option<i32> { self.configured_memory_mb }
}
impl ResourcePoolSummaryTrait for VirtualAppSummary {
    fn get_name(&self) -> &str { &self.name }
    fn get_config(&self) -> &ResourceConfigSpec { &self.config }
    fn get_runtime(&self) -> &ResourcePoolRuntimeInfo { &self.runtime }
    fn get_quick_stats(&self) -> &Option<ResourcePoolQuickStats> { &self.quick_stats }
    fn get_configured_memory_mb(&self) -> Option<i32> { self.configured_memory_mb }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ResourcePoolSummaryTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourcePoolSummary => Some(from.as_any_ref().downcast_ref::<ResourcePoolSummary>()?),
            StructType::VirtualAppSummary => Some(from.as_any_ref().downcast_ref::<VirtualAppSummary>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ResourcePoolSummary => Ok(from.as_any_box().downcast::<ResourcePoolSummary>()?),
            StructType::VirtualAppSummary => Ok(from.as_any_box().downcast::<VirtualAppSummary>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
