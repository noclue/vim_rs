use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// Data object describing the operational status of a physical
/// element.
pub trait HostHardwareElementInfoTrait : super::data_object_trait::DataObjectTrait {
    /// The name of the physical element
    fn get_name(&self) -> &str;
    /// The operational status of the physical element.
    /// 
    /// The status is one of
    /// the values specified in HostHardwareElementStatus.
    /// 
    /// See also *HostHardwareElementStatus_enum*.
    fn get_status(&self) -> &Box<dyn super::element_description_trait::ElementDescriptionTrait>;
}
impl<'s> serde::Serialize for dyn HostHardwareElementInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostHardwareElementInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostHardwareElementInfoVisitor)
            }
        }

struct HostHardwareElementInfoVisitor;

impl<'de> de::Visitor<'de> for HostHardwareElementInfoVisitor {
    type Value = Box<dyn HostHardwareElementInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostHardwareElementInfoTrait JSON object with a _typeName field")
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

impl HostHardwareElementInfoTrait for HostHardwareElementInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_status(&self) -> &Box<dyn super::element_description_trait::ElementDescriptionTrait> { &self.status }
}
impl HostHardwareElementInfoTrait for DpuStatusInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_status(&self) -> &Box<dyn super::element_description_trait::ElementDescriptionTrait> { &self.status }
}
impl HostHardwareElementInfoTrait for HostStorageElementInfo {
    fn get_name(&self) -> &str { &self.name }
    fn get_status(&self) -> &Box<dyn super::element_description_trait::ElementDescriptionTrait> { &self.status }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostHardwareElementInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostHardwareElementInfo => Some(from.as_any_ref().downcast_ref::<HostHardwareElementInfo>()?),
            StructType::DpuStatusInfo => Some(from.as_any_ref().downcast_ref::<DpuStatusInfo>()?),
            StructType::HostStorageElementInfo => Some(from.as_any_ref().downcast_ref::<HostStorageElementInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostHardwareElementInfo => Ok(from.as_any_box().downcast::<HostHardwareElementInfo>()?),
            StructType::DpuStatusInfo => Ok(from.as_any_box().downcast::<DpuStatusInfo>()?),
            StructType::HostStorageElementInfo => Ok(from.as_any_box().downcast::<HostStorageElementInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
