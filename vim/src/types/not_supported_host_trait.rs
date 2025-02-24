use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A NotSupportedHostFault occurs when the host is of a type
/// that is not supported.
pub trait NotSupportedHostTrait : super::host_connect_fault_trait::HostConnectFaultTrait {
    /// The name of the unsupported product if available;
    /// for example, "VMware ESX Server".
    fn get_product_name(&self) -> &Option<String>;
    /// The version of the unsupported product; for example, "1.5.2"
    fn get_product_version(&self) -> &Option<String>;
}
impl<'s> serde::Serialize for dyn NotSupportedHostTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn NotSupportedHostTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(NotSupportedHostVisitor)
            }
        }

struct NotSupportedHostVisitor;

impl<'de> de::Visitor<'de> for NotSupportedHostVisitor {
    type Value = Box<dyn NotSupportedHostTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid NotSupportedHostTrait JSON object with a _typeName field")
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

impl NotSupportedHostTrait for NotSupportedHost {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NonVmwareOuiMacNotSupportedHost {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostForVFlash {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostForVmcp {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostForVmemFile {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostForVsan {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostInCluster {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailed {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedCpuFeaturesForMode {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedCpuModel {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedCpuModelForMode {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedCpuVendor {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedCpuVendorUnknown {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedHostDisconnected {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedHostSoftware {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedHostSoftwareForMode {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for EvcAdmissionFailedVmActive {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostInDvs {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl NotSupportedHostTrait for NotSupportedHostInHaCluster {
    fn get_product_name(&self) -> &Option<String> { &self.product_name }
    fn get_product_version(&self) -> &Option<String> { &self.product_version }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn NotSupportedHostTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotSupportedHost => Some(from.as_any_ref().downcast_ref::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Some(from.as_any_ref().downcast_ref::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Some(from.as_any_ref().downcast_ref::<NotSupportedHostForVsan>()?),
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
            StructType::NotSupportedHostInDvs => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Some(from.as_any_ref().downcast_ref::<NotSupportedHostInHaCluster>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::NotSupportedHost => Ok(from.as_any_box().downcast::<NotSupportedHost>()?),
            StructType::NonVmwareOuiMacNotSupportedHost => Ok(from.as_any_box().downcast::<NonVmwareOuiMacNotSupportedHost>()?),
            StructType::NotSupportedHostForVFlash => Ok(from.as_any_box().downcast::<NotSupportedHostForVFlash>()?),
            StructType::NotSupportedHostForVmcp => Ok(from.as_any_box().downcast::<NotSupportedHostForVmcp>()?),
            StructType::NotSupportedHostForVmemFile => Ok(from.as_any_box().downcast::<NotSupportedHostForVmemFile>()?),
            StructType::NotSupportedHostForVsan => Ok(from.as_any_box().downcast::<NotSupportedHostForVsan>()?),
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
            StructType::NotSupportedHostInDvs => Ok(from.as_any_box().downcast::<NotSupportedHostInDvs>()?),
            StructType::NotSupportedHostInHaCluster => Ok(from.as_any_box().downcast::<NotSupportedHostInHaCluster>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
