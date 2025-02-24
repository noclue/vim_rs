use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// An attempt to enable Enhanced VMotion Compatibility on a cluster has failed.
pub trait EvcConfigFaultTrait : super::vim_fault_trait::VimFaultTrait {
    /// The faults that caused this EVC test to fail,
    /// such as *FeatureRequirementsNotMet* faults.
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>>;
}
impl<'s> serde::Serialize for dyn EvcConfigFaultTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn EvcConfigFaultTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(EvcConfigFaultVisitor)
            }
        }

struct EvcConfigFaultVisitor;

impl<'de> de::Visitor<'de> for EvcConfigFaultVisitor {
    type Value = Box<dyn EvcConfigFaultTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid EvcConfigFaultTrait JSON object with a _typeName field")
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

impl EvcConfigFaultTrait for EvcConfigFault {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for ActiveVMsBlockingEvc {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for DisconnectedHostsBlockingEvc {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for EvcModeIllegalByVendor {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for EvcModeUnsupportedByHosts {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for EvcUnsupportedByHostHardware {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for EvcUnsupportedByHostSoftware {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl EvcConfigFaultTrait for HeterogenousHostsBlockingEvc {
    fn get_faults(&self) -> &Option<Vec<Box<dyn super::method_fault_trait::MethodFaultTrait>>> { &self.faults }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn EvcConfigFaultTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::EvcConfigFault => Some(from.as_any_ref().downcast_ref::<EvcConfigFault>()?),
            StructType::ActiveVMsBlockingEvc => Some(from.as_any_ref().downcast_ref::<ActiveVMsBlockingEvc>()?),
            StructType::DisconnectedHostsBlockingEvc => Some(from.as_any_ref().downcast_ref::<DisconnectedHostsBlockingEvc>()?),
            StructType::EvcModeIllegalByVendor => Some(from.as_any_ref().downcast_ref::<EvcModeIllegalByVendor>()?),
            StructType::EvcModeUnsupportedByHosts => Some(from.as_any_ref().downcast_ref::<EvcModeUnsupportedByHosts>()?),
            StructType::EvcUnsupportedByHostHardware => Some(from.as_any_ref().downcast_ref::<EvcUnsupportedByHostHardware>()?),
            StructType::EvcUnsupportedByHostSoftware => Some(from.as_any_ref().downcast_ref::<EvcUnsupportedByHostSoftware>()?),
            StructType::HeterogenousHostsBlockingEvc => Some(from.as_any_ref().downcast_ref::<HeterogenousHostsBlockingEvc>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::EvcConfigFault => Ok(from.as_any_box().downcast::<EvcConfigFault>()?),
            StructType::ActiveVMsBlockingEvc => Ok(from.as_any_box().downcast::<ActiveVMsBlockingEvc>()?),
            StructType::DisconnectedHostsBlockingEvc => Ok(from.as_any_box().downcast::<DisconnectedHostsBlockingEvc>()?),
            StructType::EvcModeIllegalByVendor => Ok(from.as_any_box().downcast::<EvcModeIllegalByVendor>()?),
            StructType::EvcModeUnsupportedByHosts => Ok(from.as_any_box().downcast::<EvcModeUnsupportedByHosts>()?),
            StructType::EvcUnsupportedByHostHardware => Ok(from.as_any_box().downcast::<EvcUnsupportedByHostHardware>()?),
            StructType::EvcUnsupportedByHostSoftware => Ok(from.as_any_box().downcast::<EvcUnsupportedByHostSoftware>()?),
            StructType::HeterogenousHostsBlockingEvc => Ok(from.as_any_box().downcast::<HeterogenousHostsBlockingEvc>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
