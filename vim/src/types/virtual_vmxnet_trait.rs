use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualVmxnet data object type represents an instance
/// of the Vmxnet virtual Ethernet adapter attached to a virtual machine.
pub trait VirtualVmxnetTrait : super::virtual_ethernet_card_trait::VirtualEthernetCardTrait {
}
impl<'s> serde::Serialize for dyn VirtualVmxnetTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualVmxnetTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualVmxnetVisitor)
            }
        }

struct VirtualVmxnetVisitor;

impl<'de> de::Visitor<'de> for VirtualVmxnetVisitor {
    type Value = Box<dyn VirtualVmxnetTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualVmxnetTrait JSON object with a _typeName field")
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

impl VirtualVmxnetTrait for VirtualVmxnet {
}
impl VirtualVmxnetTrait for VirtualVmxnet2 {
}
impl VirtualVmxnetTrait for VirtualVmxnet3 {
}
impl VirtualVmxnetTrait for VirtualVmxnet3Vrdma {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualVmxnetTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Vrdma>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet => Ok(from.as_any_box().downcast::<VirtualVmxnet>()?),
            StructType::VirtualVmxnet2 => Ok(from.as_any_box().downcast::<VirtualVmxnet2>()?),
            StructType::VirtualVmxnet3 => Ok(from.as_any_box().downcast::<VirtualVmxnet3>()?),
            StructType::VirtualVmxnet3Vrdma => Ok(from.as_any_box().downcast::<VirtualVmxnet3Vrdma>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
