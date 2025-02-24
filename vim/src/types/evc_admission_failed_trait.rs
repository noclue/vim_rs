use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The host does not satisfy the admission requirements for the Enhanced
/// VMotion Compatibility mode of the cluster.
pub trait EvcAdmissionFailedTrait : super::not_supported_host_in_cluster_trait::NotSupportedHostInClusterTrait {
    /// The faults that caused this EVC test to fail
    /// (e.g.
    /// 
    /// FeatureRequirementsNotMet faults).
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>>;
}
impl<'s> serde::Serialize for dyn EvcAdmissionFailedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EvcAdmissionFailedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EvcAdmissionFailedVisitor)
            }
        }

struct EvcAdmissionFailedVisitor;

impl<'de> de::Visitor<'de> for EvcAdmissionFailedVisitor {
    type Value = Box<dyn EvcAdmissionFailedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EvcAdmissionFailedTrait JSON object with a _typeName field")
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

impl EvcAdmissionFailedTrait for EvcAdmissionFailed {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedCpuFeaturesForMode {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedCpuModel {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedCpuModelForMode {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedCpuVendor {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedCpuVendorUnknown {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedHostDisconnected {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedHostSoftware {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedHostSoftwareForMode {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcAdmissionFailedTrait for EvcAdmissionFailedVmActive {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EvcAdmissionFailedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::EvcAdmissionFailed => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Some(from.as_any_ref().downcast_ref::<EvcAdmissionFailedVmActive>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::EvcAdmissionFailed => Ok(from.as_any_box().downcast::<EvcAdmissionFailed>()?),
            StructType::EvcAdmissionFailedCpuFeaturesForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuFeaturesForMode>()?),
            StructType::EvcAdmissionFailedCpuModel => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModel>()?),
            StructType::EvcAdmissionFailedCpuModelForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuModelForMode>()?),
            StructType::EvcAdmissionFailedCpuVendor => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendor>()?),
            StructType::EvcAdmissionFailedCpuVendorUnknown => Ok(from.as_any_box().downcast::<EvcAdmissionFailedCpuVendorUnknown>()?),
            StructType::EvcAdmissionFailedHostDisconnected => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostDisconnected>()?),
            StructType::EvcAdmissionFailedHostSoftware => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftware>()?),
            StructType::EvcAdmissionFailedHostSoftwareForMode => Ok(from.as_any_box().downcast::<EvcAdmissionFailedHostSoftwareForMode>()?),
            StructType::EvcAdmissionFailedVmActive => Ok(from.as_any_box().downcast::<EvcAdmissionFailedVmActive>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
