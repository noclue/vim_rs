use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualVmxnetOption data object type contains the options for the
/// *VirtualVmxnet* data object type.
pub trait VirtualVmxnetOptionTrait : super::virtual_ethernet_card_option_trait::VirtualEthernetCardOptionTrait {
}
impl<'s> serde::Serialize for dyn VirtualVmxnetOptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualVmxnetOptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualVmxnetOptionVisitor)
            }
        }

struct VirtualVmxnetOptionVisitor;

impl<'de> de::Visitor<'de> for VirtualVmxnetOptionVisitor {
    type Value = Box<dyn VirtualVmxnetOptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualVmxnetOptionTrait JSON object with a _typeName field")
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

impl VirtualVmxnetOptionTrait for VirtualVmxnetOption {
}
impl VirtualVmxnetOptionTrait for VirtualVmxnet2Option {
}
impl VirtualVmxnetOptionTrait for VirtualVmxnet3Option {
}
impl VirtualVmxnetOptionTrait for VirtualVmxnet3VrdmaOption {
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualVmxnetOptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnetOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnetOption => Ok(from.as_any_box().downcast::<VirtualVmxnetOption>()?),
            StructType::VirtualVmxnet2Option => Ok(from.as_any_box().downcast::<VirtualVmxnet2Option>()?),
            StructType::VirtualVmxnet3Option => Ok(from.as_any_box().downcast::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Ok(from.as_any_box().downcast::<VirtualVmxnet3VrdmaOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
