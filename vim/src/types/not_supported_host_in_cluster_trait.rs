use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A NotSupportedHostInCluster fault occurs when the host does not support
/// the necessary features to participate in the cluster.
pub trait NotSupportedHostInClusterTrait : super::not_supported_host_trait::NotSupportedHostTrait {
}
impl<'s> serde::Serialize for dyn NotSupportedHostInClusterTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NotSupportedHostInClusterTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NotSupportedHostInClusterVisitor)
            }
        }

struct NotSupportedHostInClusterVisitor;

impl<'de> de::Visitor<'de> for NotSupportedHostInClusterVisitor {
    type Value = Box<dyn NotSupportedHostInClusterTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NotSupportedHostInClusterTrait JSON object with a _typeName field")
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

impl NotSupportedHostInClusterTrait for NotSupportedHostInCluster {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailed {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedCpuFeaturesForMode {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedCpuModel {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedCpuModelForMode {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedCpuVendor {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedCpuVendorUnknown {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedHostDisconnected {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedHostSoftware {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedHostSoftwareForMode {
}
impl NotSupportedHostInClusterTrait for EvcAdmissionFailedVmActive {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NotSupportedHostInClusterTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotSupportedHostInCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInCluster>()?),
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
            StructType::NotSupportedHostInCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInCluster>()?),
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
