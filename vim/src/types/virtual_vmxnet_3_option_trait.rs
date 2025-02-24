use super::vim_object_trait::VimObjectTrait;
use super::dyn_serialize;
use super::convert::CastFrom;
use super::struct_enum::StructType;
use super::structs::*;
use serde::de;
use super::vim_any::VimAny;

/// The VirtualVmxnet3Option data object type contains the options for the
/// *VirtualVmxnet3* data object type.
pub trait VirtualVmxnet3OptionTrait : super::virtual_vmxnet_option_trait::VirtualVmxnetOptionTrait {
    /// Flag to indicate whether UPTv2(Uniform Pass-through version 2) is
    /// settable on this device.
    /// 
    /// ***Since:*** vSphere API Release 8.0.0.1
    fn get_uptv_2_enabled(&self) -> &Option<BoolOption>;
}
impl<'s> serde::Serialize for dyn VirtualVmxnet3OptionTrait + 's {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                dyn_serialize::serialize_polymorphic(self.as_vim_object_ref(), serializer)
            }
        }
impl<'de> serde::Deserialize<'de> for Box<dyn VirtualVmxnet3OptionTrait> {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_map(VirtualVmxnet3OptionVisitor)
            }
        }

struct VirtualVmxnet3OptionVisitor;

impl<'de> de::Visitor<'de> for VirtualVmxnet3OptionVisitor {
    type Value = Box<dyn VirtualVmxnet3OptionTrait>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid VirtualVmxnet3OptionTrait JSON object with a _typeName field")
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

impl VirtualVmxnet3OptionTrait for VirtualVmxnet3Option {
    fn get_uptv_2_enabled(&self) -> &Option<BoolOption> { &self.uptv_2_enabled }
}
impl VirtualVmxnet3OptionTrait for VirtualVmxnet3VrdmaOption {
    fn get_uptv_2_enabled(&self) -> &Option<BoolOption> { &self.uptv_2_enabled }
}
impl<From: VimObjectTrait + ?Sized + 'static> CastFrom<From> for dyn VirtualVmxnet3OptionTrait {
    fn from_ref<'a>(from: &'a From) -> Option<&'a Self> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet3Option => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Some(from.as_any_ref().downcast_ref::<VirtualVmxnet3VrdmaOption>()?),
            _ => None,
        }
    }
    
    fn from_box(from: Box<From>) -> Result<Box<Self>, Box<dyn std::any::Any + 'static>> {
        let data_type = from.data_type();
        match data_type {
            StructType::VirtualVmxnet3Option => Ok(from.as_any_box().downcast::<VirtualVmxnet3Option>()?),
            StructType::VirtualVmxnet3VrdmaOption => Ok(from.as_any_box().downcast::<VirtualVmxnet3VrdmaOption>()?),
            _ => Err(from.as_any_box()),
        }
    }
}
