use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// A bridge connects a virtual switch to a physical network adapter.
/// 
/// There are multiple types of bridges.
pub trait HostVirtualSwitchBridgeTrait : super::data_object_trait::DataObjectTrait {
}
impl<'s> serde::Serialize for dyn HostVirtualSwitchBridgeTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn HostVirtualSwitchBridgeTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(HostVirtualSwitchBridgeVisitor)
            }
        }

struct HostVirtualSwitchBridgeVisitor;

impl<'de> de::Visitor<'de> for HostVirtualSwitchBridgeVisitor {
    type Value = Box<dyn HostVirtualSwitchBridgeTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid HostVirtualSwitchBridgeTrait JSON object with a _typeName field")
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

impl HostVirtualSwitchBridgeTrait for HostVirtualSwitchBridge {
}
impl HostVirtualSwitchBridgeTrait for HostVirtualSwitchAutoBridge {
}
impl HostVirtualSwitchBridgeTrait for HostVirtualSwitchBondBridge {
}
impl HostVirtualSwitchBridgeTrait for HostVirtualSwitchSimpleBridge {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn HostVirtualSwitchBridgeTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostVirtualSwitchBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchBridge>()?),
            StructType::HostVirtualSwitchAutoBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchAutoBridge>()?),
            StructType::HostVirtualSwitchBondBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchBondBridge>()?),
            StructType::HostVirtualSwitchSimpleBridge => Some(from.as_any_ref().downcast_ref::<HostVirtualSwitchSimpleBridge>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::HostVirtualSwitchBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchBridge>()?),
            StructType::HostVirtualSwitchAutoBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchAutoBridge>()?),
            StructType::HostVirtualSwitchBondBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchBondBridge>()?),
            StructType::HostVirtualSwitchSimpleBridge => Ok(from.as_any_box().downcast::<HostVirtualSwitchSimpleBridge>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
