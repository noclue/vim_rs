use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// This data object type describes the options for the
/// *VirtualPCIPassthroughPluginBackingInfo* data object type.
pub trait VirtualPciPassthroughPluginBackingOptionTrait : super::virtual_device_backing_option_trait::VirtualDeviceBackingOptionTrait {
}
impl<'s> serde::Serialize for dyn VirtualPciPassthroughPluginBackingOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualPciPassthroughPluginBackingOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualPciPassthroughPluginBackingOptionVisitor)
            }
        }

struct VirtualPciPassthroughPluginBackingOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualPciPassthroughPluginBackingOptionVisitor {
    type Value = Box<dyn VirtualPciPassthroughPluginBackingOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualPciPassthroughPluginBackingOptionTrait JSON object with a _typeName field")
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

impl VirtualPciPassthroughPluginBackingOptionTrait for VirtualPciPassthroughPluginBackingOption {
}
impl VirtualPciPassthroughPluginBackingOptionTrait for VirtualPciPassthroughVmiopBackingOption {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualPciPassthroughPluginBackingOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualPciPassthroughPluginBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Some(from.as_any_ref().downcast_ref::<VirtualPciPassthroughVmiopBackingOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualPciPassthroughPluginBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughPluginBackingOption>()?),
            StructType::VirtualPciPassthroughVmiopBackingOption => Ok(from.as_any_box().downcast::<VirtualPciPassthroughVmiopBackingOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
