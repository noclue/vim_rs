use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Base class for specifying how admission control should be done for vSphere HA.
pub trait ClusterDasAdmissionControlPolicyTrait : super::data_object_trait::DataObjectTrait {
    /// Percentage of resource reduction that a cluster of VMs can tolerate
    /// in case of a failover.
    fn get_resource_reduction_to_tolerate_percent(&self) -> Option<i32>;
    /// Flag that determines whether strict admission control for persistent
    /// memory is enabled.
    /// 
    /// By default, this value is false.
    /// This flag can only be set to true if
    /// *ClusterDasConfigInfo.admissionControlEnabled* is set to true.
    /// When you use persistent memory admission control, the following
    /// operations are prevented, if doing so would violate the
    /// *ClusterDasConfigInfo.admissionControlEnabled*.
    /// - Creating a virtual machine with persistent memory.
    /// - Adding a virtual persistent memory device to a virtual machine.
    /// - Increasing the capacity of a virtual persistent memory device.
    ///   
    /// ***Since:*** vSphere API Release 7.0.2.0
    fn get_p_mem_admission_control_enabled(&self) -> Option<bool>;
}
impl<'s> serde::Serialize for dyn ClusterDasAdmissionControlPolicyTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn ClusterDasAdmissionControlPolicyTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(ClusterDasAdmissionControlPolicyVisitor)
            }
        }

struct ClusterDasAdmissionControlPolicyVisitor;

impl<'de> de::Visitor<'de> for ClusterDasAdmissionControlPolicyVisitor {
    type Value = Box<dyn ClusterDasAdmissionControlPolicyTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid ClusterDasAdmissionControlPolicyTrait JSON object with a _typeName field")
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

impl ClusterDasAdmissionControlPolicyTrait for ClusterDasAdmissionControlPolicy {
    fn get_resource_reduction_to_tolerate_percent(&self) -> Option<i32> { self.resource_reduction_to_tolerate_percent }
    fn get_p_mem_admission_control_enabled(&self) -> Option<bool> { self.p_mem_admission_control_enabled }
}
impl ClusterDasAdmissionControlPolicyTrait for ClusterFailoverHostAdmissionControlPolicy {
    fn get_resource_reduction_to_tolerate_percent(&self) -> Option<i32> { self.resource_reduction_to_tolerate_percent }
    fn get_p_mem_admission_control_enabled(&self) -> Option<bool> { self.p_mem_admission_control_enabled }
}
impl ClusterDasAdmissionControlPolicyTrait for ClusterFailoverLevelAdmissionControlPolicy {
    fn get_resource_reduction_to_tolerate_percent(&self) -> Option<i32> { self.resource_reduction_to_tolerate_percent }
    fn get_p_mem_admission_control_enabled(&self) -> Option<bool> { self.p_mem_admission_control_enabled }
}
impl ClusterDasAdmissionControlPolicyTrait for ClusterFailoverResourcesAdmissionControlPolicy {
    fn get_resource_reduction_to_tolerate_percent(&self) -> Option<i32> { self.resource_reduction_to_tolerate_percent }
    fn get_p_mem_admission_control_enabled(&self) -> Option<bool> { self.p_mem_admission_control_enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn ClusterDasAdmissionControlPolicyTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterDasAdmissionControlPolicy>()?),
            StructType::ClusterFailoverHostAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverHostAdmissionControlPolicy>()?),
            StructType::ClusterFailoverLevelAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverLevelAdmissionControlPolicy>()?),
            StructType::ClusterFailoverResourcesAdmissionControlPolicy => Some(from.as_any_ref().downcast_ref::<ClusterFailoverResourcesAdmissionControlPolicy>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::ClusterDasAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterDasAdmissionControlPolicy>()?),
            StructType::ClusterFailoverHostAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverHostAdmissionControlPolicy>()?),
            StructType::ClusterFailoverLevelAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverLevelAdmissionControlPolicy>()?),
            StructType::ClusterFailoverResourcesAdmissionControlPolicy => Ok(from.as_any_box().downcast::<ClusterFailoverResourcesAdmissionControlPolicy>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
