use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This fault is thrown when a host power operation fails.
pub trait HostPowerOpFailedTrait : super::vim_fault_trait::VimFaultTrait {
}
impl<'s> serde::Serialize for dyn HostPowerOpFailedTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostPowerOpFailedTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostPowerOpFailedVisitor)
            }
        }

struct HostPowerOpFailedVisitor;

impl<'de> de::Visitor<'de> for HostPowerOpFailedVisitor {
    type Value = Box<dyn HostPowerOpFailedTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostPowerOpFailedTrait JSON object with a _typeName field")
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

impl HostPowerOpFailedTrait for HostPowerOpFailed {
}
impl HostPowerOpFailedTrait for NoPeerHostFound {
}
impl HostPowerOpFailedTrait for VmotionInterfaceNotEnabled {
}
impl HostPowerOpFailedTrait for WakeOnLanNotSupportedByVmotionNic {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostPowerOpFailedTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPowerOpFailed => Some(from.as_any_ref().downcast_ref::<HostPowerOpFailed>()?),
            StructType::NoPeerHostFound => Some(from.as_any_ref().downcast_ref::<NoPeerHostFound>()?),
            StructType::VmotionInterfaceNotEnabled => Some(from.as_any_ref().downcast_ref::<VmotionInterfaceNotEnabled>()?),
            StructType::WakeOnLanNotSupportedByVmotionNic => Some(from.as_any_ref().downcast_ref::<WakeOnLanNotSupportedByVmotionNic>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostPowerOpFailed => Ok(from.as_any_box().downcast::<HostPowerOpFailed>()?),
            StructType::NoPeerHostFound => Ok(from.as_any_box().downcast::<NoPeerHostFound>()?),
            StructType::VmotionInterfaceNotEnabled => Ok(from.as_any_box().downcast::<VmotionInterfaceNotEnabled>()?),
            StructType::WakeOnLanNotSupportedByVmotionNic => Ok(from.as_any_box().downcast::<WakeOnLanNotSupportedByVmotionNic>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
