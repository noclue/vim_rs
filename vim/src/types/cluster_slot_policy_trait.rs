use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The base class *ClusterSlotPolicy* is used for specifying how
/// the slot size is to be computed for the failover level HA admission control
/// policy.
/// 
/// By default, vSphere HA defines the slot size using the largest memory
/// and cpu reservations of any powered on virtual machine in the cluster.
/// Subclasses of this class define various policies to modify how the slot size
/// is chosen to prevent outlier virtual machines (i.e. those with much larger
/// reservations than the average) from skewing the slot size. If such a policy is chosen,
/// outlier virtual machines will use multiple slots. Using such a policy introduces
/// a risk that vSphere HA will be unable to failover these virtual machines because
/// of resource fragmentation.
pub trait ClusterSlotPolicyTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn ClusterSlotPolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterSlotPolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterSlotPolicyVisitor)
            }
        }

struct ClusterSlotPolicyVisitor;

impl<'de> de::Visitor<'de> for ClusterSlotPolicyVisitor {
    type Value = Box<dyn ClusterSlotPolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterSlotPolicyTrait JSON object with a _typeName field")
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

impl ClusterSlotPolicyTrait for ClusterSlotPolicy {
}
impl ClusterSlotPolicyTrait for ClusterFixedSizeSlotPolicy {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterSlotPolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterSlotPolicy => Some(from.as_any_ref().downcast_ref::<ClusterSlotPolicy>()?),
            StructType::ClusterFixedSizeSlotPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFixedSizeSlotPolicy>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterSlotPolicy => Ok(from.as_any_box().downcast::<ClusterSlotPolicy>()?),
            StructType::ClusterFixedSizeSlotPolicy => Ok(from.as_any_box().downcast::<ClusterFixedSizeSlotPolicy>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
