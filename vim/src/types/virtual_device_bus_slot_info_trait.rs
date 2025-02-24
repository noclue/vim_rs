use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// <code>*VirtualDeviceBusSlotInfo*</code> is a base data object type
/// for information about device connection to its bus.
/// 
/// This base type does not
/// define any properties. It is used as a namespace for general-purpose subtypes.
/// Specific devices types are represented by subtypes which define properties for
/// device-specific backing information.
pub trait VirtualDeviceBusSlotInfoTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn VirtualDeviceBusSlotInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualDeviceBusSlotInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualDeviceBusSlotInfoVisitor)
            }
        }

struct VirtualDeviceBusSlotInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualDeviceBusSlotInfoVisitor {
    type Value = Box<dyn VirtualDeviceBusSlotInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualDeviceBusSlotInfoTrait JSON object with a _typeName field")
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

impl VirtualDeviceBusSlotInfoTrait for VirtualDeviceBusSlotInfo {
}
impl VirtualDeviceBusSlotInfoTrait for VirtualDevicePciBusSlotInfo {
}
impl VirtualDeviceBusSlotInfoTrait for VirtualUsbControllerPciBusSlotInfo {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualDeviceBusSlotInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualDeviceBusSlotInfo>()?),
            StructType::VirtualDevicePciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Some(from.as_any_ref().downcast_ref::<VirtualUsbControllerPciBusSlotInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualDeviceBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualDeviceBusSlotInfo>()?),
            StructType::VirtualDevicePciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualDevicePciBusSlotInfo>()?),
            StructType::VirtualUsbControllerPciBusSlotInfo => Ok(from.as_any_box().downcast::<VirtualUsbControllerPciBusSlotInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
