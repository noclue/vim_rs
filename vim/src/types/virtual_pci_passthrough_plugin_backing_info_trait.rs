use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualPCIPassthrough.PluginBackingInfo is a base data object type
/// for encoding plugin-specific information.
/// 
/// This base type does not define
/// any properties. Specific plugin types are represented by subtypes which
/// define properties for subtype-specific backing information.
pub trait VirtualPciPassthroughPluginBackingInfoTrait : super::virtual_device_backing_info_trait::VirtualDeviceBackingInfoTrait {
}
impl<'s> serde::Serialize for dyn VirtualPciPassthroughPluginBackingInfoTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualPciPassthroughPluginBackingInfoTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualPciPassthroughPluginBackingInfoVisitor)
            }
        }

struct VirtualPciPassthroughPluginBackingInfoVisitor;

impl<'de> de::Visitor<'de> for VirtualPciPassthroughPluginBackingInfoVisitor {
    type Value = Box<dyn VirtualPciPassthroughPluginBackingInfoTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualPciPassthroughPluginBackingInfoTrait JSON object with a _typeName field")
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

impl VirtualPciPassthroughPluginBackingInfoTrait for VirtualPciPassthroughPluginBackingInfo {
}
impl VirtualPciPassthroughPluginBackingInfoTrait for VirtualPciPassthroughVmiopBackingInfo {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualPciPassthroughPluginBackingInfoTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualPciPassthroughPluginBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingInfo>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualPciPassthroughPluginBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingInfo>()?),
            StructType::VirtualPciPassthroughVmiopBackingInfo => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingInfo>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
